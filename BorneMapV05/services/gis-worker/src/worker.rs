use crate::config::Config;
use crate::error::WorkerError;
use crate::geometry;
use crate::retry;
use sqlx::PgPool;
use tokio::sync::oneshot::Receiver;
use tracing::{info, warn, error};

pub async fn run(config: Config, pool: PgPool, mut shutdown_rx: Receiver<()>) {
    info!(
        "gis-worker poll loop starting (interval={}ms, batch_size={})",
        config.poll_interval_ms, config.batch_size
    );

    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(
        config.poll_interval_ms,
    ));

    reset_stale_processing(&pool, config.stale_processing_timeout_ms).await;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = process_batch(&config, &pool).await {
                    warn!("Batch processing error: {}", e);
                }
            }
            _ = &mut shutdown_rx => {
                info!("Worker received shutdown signal");
                break;
            }
        }
    }

    info!("gis-worker poll loop stopped");
}

async fn reset_stale_processing(pool: &PgPool, timeout_ms: i64) {
    match sqlx::query_scalar::<_, i64>(
        "WITH updated AS (
            UPDATE gis.sync_queue
            SET status = 'pending'
            WHERE status = 'processing'
              AND created_at < NOW() - ($1 || ' milliseconds')::INTERVAL
            RETURNING id
        )
        SELECT COUNT(*) FROM updated",
    )
    .bind(timeout_ms)
    .fetch_one(pool)
    .await
    {
        Ok(count) if count > 0 => info!("Reset {} stale processing rows on startup", count),
        Ok(_) => {}
        Err(e) => warn!("Failed to reset stale processing rows: {}", e),
    }
}

async fn process_batch(
    config: &Config,
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rows = claim_batch(pool, config.batch_size, config.max_retries).await?;

    if rows.is_empty() {
        return Ok(());
    }

    let batch_size = rows.len();
    info!("Claimed batch of {} row(s) for processing", batch_size);

    let futures: Vec<_> = rows
        .into_iter()
        .map(|row| process_row(pool, row))
        .collect();

    let results = futures::future::join_all(futures).await;

    let mut success_count = 0;
    let mut fail_count = 0;

    for result in &results {
        match result {
            Ok(()) => success_count += 1,
            Err(_) => fail_count += 1,
        }
    }

    info!(
        "Batch complete: {} succeeded, {} failed",
        success_count, fail_count
    );

    Ok(())
}

async fn claim_batch(
    pool: &PgPool,
    batch_size: i32,
    max_retries: u32,
) -> Result<Vec<crate::models::GisQueueEntry>, sqlx::Error> {
    // Claim pending rows (first attempt)
    let mut rows = claim_rows_by_status(pool, "pending", batch_size).await?;
    let remaining = batch_size - rows.len() as i32;

    // Claim failed rows for retry if batch not full
    if remaining > 0 {
        let retry_rows = claim_failed_rows(pool, remaining, max_retries).await?;
        rows.extend(retry_rows);
    }

    Ok(rows)
}

async fn claim_rows_by_status(
    pool: &PgPool,
    status: &str,
    limit: i32,
) -> Result<Vec<crate::models::GisQueueEntry>, sqlx::Error> {
    let rows = sqlx::query_as::<_, crate::models::GisQueueEntry>(
        r#"WITH next_batch AS (
            SELECT id
            FROM gis.sync_queue
            WHERE status = $2
            ORDER BY created_at
            LIMIT $1
            FOR UPDATE SKIP LOCKED
        )
        UPDATE gis.sync_queue
        SET status = 'processing'
        FROM next_batch
        WHERE gis.sync_queue.id = next_batch.id
        RETURNING gis.sync_queue.id, gis.sync_queue.entity_type,
                  gis.sync_queue.entity_id, gis.sync_queue.operation,
                  gis.sync_queue.payload, gis.sync_queue.status,
                  gis.sync_queue.created_at, gis.sync_queue.processed_at"#,
    )
    .bind(limit)
    .bind(status)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

async fn claim_failed_rows(
    pool: &PgPool,
    limit: i32,
    max_retries: u32,
) -> Result<Vec<crate::models::GisQueueEntry>, sqlx::Error> {
    // Fetch failed rows, increment retry_count in payload, skip rows past max retries
    let rows = sqlx::query_as::<_, crate::models::GisQueueEntry>(
        r#"WITH next_batch AS (
            SELECT id
            FROM gis.sync_queue
            WHERE status = 'failed'
            ORDER BY created_at
            LIMIT $1
            FOR UPDATE SKIP LOCKED
        )
        UPDATE gis.sync_queue
        SET status = 'processing',
            payload = jsonb_set(
                COALESCE(payload, '{}'::jsonb),
                '{retry_count}',
                to_jsonb(COALESCE(((payload->>'retry_count')::int), 0) + 1)
            )
        FROM next_batch
        WHERE gis.sync_queue.id = next_batch.id
        RETURNING gis.sync_queue.id, gis.sync_queue.entity_type,
                  gis.sync_queue.entity_id, gis.sync_queue.operation,
                  gis.sync_queue.payload, gis.sync_queue.status,
                  gis.sync_queue.created_at, gis.sync_queue.processed_at"#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    // Filter out rows that have exceeded max retries after increment
    let valid: Vec<_> = rows
        .into_iter()
        .filter(|row| {
            let retry_count = retry::extract_retry_count(&row.payload);
            if retry_count > max_retries {
                // Move to dead_letter directly
                let _ = mark_dead_letter(pool, &row.id);
                false
            } else {
                true
            }
        })
        .collect();

    Ok(valid)
}

async fn process_row(
    pool: &PgPool,
    row: crate::models::GisQueueEntry,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let row_id = &row.id;
    let entity_id = &row.entity_id;
    let operation = &row.operation;
    let old_status = &row.status;

    info!(
        row_id = %row_id,
        entity_id = %entity_id,
        operation = %operation,
        old_status = %old_status,
        "Processing outbox row"
    );

    let result = match operation.as_str() {
        "insert" | "update" => handle_geometry_update(pool, entity_id).await,
        "delete" => handle_geometry_delete(pool, entity_id).await,
        op => Err(WorkerError::Unknown(format!("Unknown operation: {}", op))),
    };

    match result {
        Ok(()) => {
            mark_done(pool, row_id).await?;
            info!(
                row_id = %row_id,
                entity_id = %entity_id,
                operation = %operation,
                old_status = %old_status,
                new_status = "done",
                "State transition"
            );
            Ok(())
        }
        Err(err) => {
            let error_code = err.error_code();
            if err.is_retryable() {
                mark_failed(pool, row_id).await?;
                warn!(
                    row_id = %row_id,
                    entity_id = %entity_id,
                    operation = %operation,
                    old_status = %old_status,
                    new_status = "failed",
                    error_code = %error_code,
                    error = %err,
                    "State transition"
                );
            } else {
                mark_dead_letter(pool, row_id).await?;
                error!(
                    row_id = %row_id,
                    entity_id = %entity_id,
                    operation = %operation,
                    old_status = %old_status,
                    new_status = "dead_letter",
                    error_code = %error_code,
                    error = %err,
                    "State transition"
                );
            }
            Ok(())
        }
    }
}

async fn handle_geometry_update(
    pool: &PgPool,
    station_id: &str,
) -> Result<(), WorkerError> {
    let coords = geometry::get_station_coords(pool, station_id).await?;

    match coords {
        Some((lat, lng)) => {
            geometry::update_station_geometry(pool, station_id, lat, lng).await
        }
        None => Err(WorkerError::InvalidCoordinates(format!(
            "Station {} has NULL lat/lng",
            station_id
        ))),
    }
}

async fn handle_geometry_delete(
    pool: &PgPool,
    station_id: &str,
) -> Result<(), WorkerError> {
    geometry::clear_station_geometry(pool, station_id).await
}

async fn mark_done(pool: &PgPool, row_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE gis.sync_queue
           SET status = 'done', processed_at = NOW()
           WHERE id = $1"#,
    )
    .bind(row_id)
    .execute(pool)
    .await?;
    Ok(())
}

async fn mark_failed(pool: &PgPool, row_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE gis.sync_queue
           SET status = 'failed'
           WHERE id = $1"#,
    )
    .bind(row_id)
    .execute(pool)
    .await?;
    Ok(())
}

async fn mark_dead_letter(pool: &PgPool, row_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE gis.sync_queue
           SET status = 'dead_letter'
           WHERE id = $1"#,
    )
    .bind(row_id)
    .execute(pool)
    .await?;
    Ok(())
}
