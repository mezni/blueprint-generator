use sqlx::postgres::PgPool;

async fn setup_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://borne:devpassword@localhost:5432/borne_map_test".to_string());
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

async fn seed_stations(pool: &PgPool, count: usize) -> Vec<String> {
    let partner_id = format!("PRT-{}", &nanoid::nanoid!(21));

    sqlx::query(
        "INSERT INTO inventory.partner (id, name, email) VALUES ($1, 'Perf Test Partner', 'perf@example.com')"
    )
    .bind(&partner_id)
    .execute(pool)
    .await
    .expect("Should insert partner");

    let mut station_ids = Vec::new();
    for i in 0..count {
        let station_id = format!("STN-{}", &nanoid::nanoid!(21));
        let lng = 2.0 + (i as f64 / count as f64) * 4.0;
        let lat = 46.0 + (i as f64 / count as f64) * 4.0;

        sqlx::query(
            "INSERT INTO inventory.station (id, partner_id, name, location) \
             VALUES ($1, $2, $3, ST_SetSRID(ST_MakePoint($4, $5), 4326)::geography)"
        )
        .bind(&station_id)
        .bind(&partner_id)
        .bind(format!("Station {i}"))
        .bind(lng)
        .bind(lat)
        .execute(pool)
        .await
        .expect("Should insert station");

        station_ids.push(station_id);
    }

    station_ids
}

async fn cleanup(pool: &PgPool) {
    sqlx::query("DELETE FROM inventory.station").execute(pool).await.ok();
    sqlx::query("DELETE FROM inventory.partner").execute(pool).await.ok();
}

#[tokio::test]
#[ignore]
async fn test_nearby_station_search_performance() {
    let pool = setup_test_pool().await;
    cleanup(&pool).await;

    let station_ids = seed_stations(&pool, 10_000).await;

    let start = std::time::Instant::now();

    let stations: Vec<(String,)> = sqlx::query_as(
        "SELECT id FROM inventory.station \
         WHERE ST_DWithin(location, ST_SetSRID(ST_MakePoint(4.0, 48.0), 4326)::geography, 50000) \
           AND status = 'active' \
         ORDER BY location <-> ST_SetSRID(ST_MakePoint(4.0, 48.0), 4326)::geography \
         LIMIT 50"
    )
    .fetch_all(&pool)
    .await
    .expect("Should execute nearby search");

    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 300, "Nearby search should complete under 300ms, took {}ms", elapsed.as_millis());
    assert!(!stations.is_empty(), "Should find at least one station within 50km");

    cleanup(&pool).await;
}

#[tokio::test]
#[ignore]
async fn test_analytics_partition_creation() {
    let pool = setup_test_pool().await;

    let tomorrow = chrono::Utc::now().date_naive() + chrono::Duration::days(1);

    common_db::partition::create_daily_partition(&pool, tomorrow)
        .await
        .expect("Should create daily partition");

    let partition_name = format!("raw_event_{}", tomorrow.format("%Y%m%d"));

    let exists: Vec<(String,)> = sqlx::query_as(
        "SELECT tablename FROM pg_tables WHERE schemaname = 'analytics' AND tablename = $1"
    )
    .bind(&partition_name)
    .fetch_all(&pool)
    .await
    .expect("Should query partitions");

    assert!(!exists.is_empty(), "Partition {} should exist", partition_name);
}

#[tokio::test]
#[ignore]
async fn test_ensure_tomorrow_partition() {
    let pool = setup_test_pool().await;

    common_db::partition::ensure_tomorrow_partition(&pool)
        .await
        .expect("Should ensure tomorrow partition exists");

    let tomorrow = chrono::Utc::now().date_naive() + chrono::Duration::days(1);
    let partition_name = format!("raw_event_{}", tomorrow.format("%Y%m%d"));

    let exists: Vec<(String,)> = sqlx::query_as(
        "SELECT tablename FROM pg_tables WHERE schemaname = 'analytics' AND tablename = $1"
    )
    .bind(&partition_name)
    .fetch_all(&pool)
    .await
    .expect("Should query partitions");

    assert!(!exists.is_empty(), "Tomorrow's partition should exist after ensure_tomorrow_partition");
}
