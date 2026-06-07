use chrono::Utc;
use common_types::generate_id;
use common_types::api::PaginationMeta;
use common_types::EntityPrefix;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Transaction;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::charger::{Charger, ChargerCreate, ChargerUpdate};

pub async fn find_by_id(pool: &PgPool, charger_id: &str) -> Result<Charger, ServiceError> {
    sqlx::query_as::<_, Charger>(
        "SELECT charger_id, station_id, charger_type, power_kw, \
         status, created_at, updated_at \
         FROM inventory.charger WHERE charger_id = $1 AND deleted_at IS NULL",
    )
    .bind(charger_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::not_found("Charger", charger_id),
        other => ServiceError::Db(other),
    })
}

pub async fn verify_station_belongs_to_partner(
    pool: &PgPool,
    station_id: &str,
    partner_id: &str,
) -> Result<(), ServiceError> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT station_id FROM inventory.station \
         WHERE station_id = $1 AND partner_id = $2 AND deleted_at IS NULL",
    )
    .bind(station_id)
    .bind(partner_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    if result.is_none() {
        return Err(ServiceError::Api(common_errors::ApiError {
            code: common_errors::ErrorCode::NotFound,
            message: format!("Station '{}' not found or not owned by partner", station_id),
            details: None,
        }));
    }
    Ok(())
}

pub async fn verify_charger_belongs_to_partner(
    pool: &PgPool,
    charger_id: &str,
    partner_id: &str,
) -> Result<String, ServiceError> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT c.station_id FROM inventory.charger c \
         JOIN inventory.station s ON c.station_id = s.station_id \
         WHERE c.charger_id = $1 AND s.partner_id = $2 AND c.deleted_at IS NULL AND s.deleted_at IS NULL",
    )
    .bind(charger_id)
    .bind(partner_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    let (station_id,) = result.ok_or_else(|| ServiceError::partner_scope_violation())?;
    Ok(station_id)
}

pub async fn list_partner_chargers(
    pool: &PgPool,
    partner_id: &str,
    params: &PaginationParams,
    station_filter: Option<&str>,
) -> Result<(Vec<Charger>, PaginationMeta), ServiceError> {
    let offset = params.offset();
    let limit = params.limit();

    let (chargers, total): (Vec<Charger>, i64) = if let Some(sid) = station_filter {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM inventory.charger c \
             JOIN inventory.station s ON c.station_id = s.station_id \
             WHERE s.partner_id = $1 AND c.deleted_at IS NULL AND s.deleted_at IS NULL AND c.station_id = $2",
        )
        .bind(partner_id)
        .bind(sid)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::Db)?;

        let chargers = sqlx::query_as::<_, Charger>(
            "SELECT c.charger_id, c.station_id, c.charger_type, c.power_kw, \
             c.status, c.created_at, c.updated_at \
             FROM inventory.charger c \
             JOIN inventory.station s ON c.station_id = s.station_id \
             WHERE s.partner_id = $1 AND c.deleted_at IS NULL AND s.deleted_at IS NULL AND c.station_id = $2 \
             ORDER BY c.created_at DESC LIMIT $3 OFFSET $4",
        )
        .bind(partner_id)
        .bind(sid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(ServiceError::Db)?;

        (chargers, count.0)
    } else {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM inventory.charger c \
             JOIN inventory.station s ON c.station_id = s.station_id \
             WHERE s.partner_id = $1 AND c.deleted_at IS NULL AND s.deleted_at IS NULL",
        )
        .bind(partner_id)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::Db)?;

        let chargers = sqlx::query_as::<_, Charger>(
            "SELECT c.charger_id, c.station_id, c.charger_type, c.power_kw, \
             c.status, c.created_at, c.updated_at \
             FROM inventory.charger c \
             JOIN inventory.station s ON c.station_id = s.station_id \
             WHERE s.partner_id = $1 AND c.deleted_at IS NULL AND s.deleted_at IS NULL \
             ORDER BY c.created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(partner_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(ServiceError::Db)?;

        (chargers, count.0)
    };

    let total_i32 = total as i32;
    let size = params.size();
    let total_pages = total_i32.div_euclid(size) + if total_i32 % size != 0 { 1 } else { 0 };

    let meta = PaginationMeta {
        page: params.page(),
        size,
        total: total_i32,
        total_pages: total_pages.max(0),
        has_next: params.page() < total_pages,
        has_prev: params.page() > 1,
    };

    Ok((chargers, meta))
}

pub async fn create_charger(
    tx: &mut Transaction<'_, Postgres>,
    station_id: &str,
    req: &ChargerCreate,
) -> Result<Charger, ServiceError> {
    let charger_id = generate_id(EntityPrefix::Chg);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO inventory.charger (charger_id, station_id, charger_type, power_kw, status, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $6)",
    )
    .bind(&charger_id)
    .bind(station_id)
    .bind(req.charger_type.as_str())
    .bind(req.power_kw)
    .bind(req.status.as_str())
    .bind(now)
    .execute(tx as &mut sqlx::PgConnection)
    .await
    .map_err(ServiceError::Db)?;

    Ok(Charger {
        charger_id,
        station_id: station_id.to_string(),
        charger_type: req.charger_type,
        power_kw: req.power_kw,
        status: req.status,
        created_at: now,
        updated_at: now,
    })
}

pub async fn update_charger(
    tx: &mut Transaction<'_, Postgres>,
    id: &str,
    req: &ChargerUpdate,
    expected_updated_at: chrono::DateTime<Utc>,
) -> Result<Charger, ServiceError> {
    let now = Utc::now();

    let result = sqlx::query_as::<_, Charger>(
        "UPDATE inventory.charger SET \
         charger_type = CAST(COALESCE($1, CAST(charger_type AS text)) AS inventory.charger_type), \
         power_kw = COALESCE($2, power_kw), \
         status = CAST(COALESCE($3, CAST(status AS text)) AS inventory.charger_status), \
         updated_at = $4 \
         WHERE charger_id = $5 AND updated_at = $6 AND deleted_at IS NULL \
         RETURNING charger_id, station_id, charger_type, power_kw, status, created_at, updated_at",
    )
    .bind(req.charger_type.map(|t| t.as_str()))
    .bind(req.power_kw)
    .bind(req.status.map(|s| s.as_str()))
    .bind(now)
    .bind(id)
    .bind(expected_updated_at)
    .fetch_optional(tx as &mut sqlx::PgConnection)
    .await
    .map_err(ServiceError::Db)?;

    result.ok_or_else(|| {
        ServiceError::Api(common_errors::ApiError {
            code: common_errors::ErrorCode::ConcurrentModification,
            message: format!("Charger '{}' was modified by another request", id),
            details: None,
        })
    })
}
