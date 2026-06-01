use sqlx::postgres::PgPool;

async fn setup_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://borne:devpassword@localhost:5432/borne_map_test".to_string());
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

#[tokio::test]
#[ignore]
async fn test_four_schemas_exist() {
    let pool = setup_test_pool().await;

    let schemas: Vec<(String,)> = sqlx::query_as(
        "SELECT schema_name FROM information_schema.schemata \
         WHERE schema_name IN ('inventory', 'users', 'gis', 'analytics') \
         ORDER BY schema_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query schemas");

    assert_eq!(schemas.len(), 4, "Exactly four schemas should exist");
    let names: Vec<&str> = schemas.iter().map(|s| s.0.as_str()).collect();
    assert_eq!(names, vec!["analytics", "gis", "inventory", "users"]);
}

#[tokio::test]
#[ignore]
async fn test_inventory_tables_exist() {
    let pool = setup_test_pool().await;

    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables \
         WHERE table_schema = 'inventory' \
         ORDER BY table_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query inventory tables");

    let names: Vec<&str> = tables.iter().map(|t| t.0.as_str()).collect();
    assert!(names.contains(&"charger"), "charger table should exist");
    assert!(names.contains(&"partner"), "partner table should exist");
    assert!(names.contains(&"review"), "review table should exist");
    assert!(names.contains(&"station"), "station table should exist");
}

#[tokio::test]
#[ignore]
async fn test_users_tables_exist() {
    let pool = setup_test_pool().await;

    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables \
         WHERE table_schema = 'users' \
         ORDER BY table_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query users tables");

    let names: Vec<&str> = tables.iter().map(|t| t.0.as_str()).collect();
    assert!(names.contains(&"favorite"), "favorite table should exist");
    assert!(names.contains(&"user_account"), "user_account table should exist");
}

#[tokio::test]
#[ignore]
async fn test_gis_tables_exist() {
    let pool = setup_test_pool().await;

    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables \
         WHERE table_schema = 'gis' \
         ORDER BY table_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query gis tables");

    let names: Vec<&str> = tables.iter().map(|t| t.0.as_str()).collect();
    assert!(names.contains(&"geo_boundary"), "geo_boundary table should exist");
    assert!(names.contains(&"station_enrichment"), "station_enrichment table should exist");
}

#[tokio::test]
#[ignore]
async fn test_analytics_tables_exist() {
    let pool = setup_test_pool().await;

    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables \
         WHERE table_schema = 'analytics' \
         AND table_name LIKE 'raw_event%' \
         ORDER BY table_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query analytics tables");

    let names: Vec<&str> = tables.iter().map(|t| t.0.as_str()).collect();
    assert!(names.contains(&"raw_event"), "raw_event parent table should exist");
}

#[tokio::test]
#[ignore]
async fn test_station_location_has_gist_index() {
    let pool = setup_test_pool().await;

    let indexes: Vec<(String,)> = sqlx::query_as(
        "SELECT indexname FROM pg_indexes \
         WHERE schemaname = 'inventory' AND tablename = 'station' \
         AND indexname = 'idx_station_location'"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query indexes");

    assert!(!indexes.is_empty(), "GIST index on station.location should exist");
}

#[tokio::test]
#[ignore]
async fn test_admin_service_can_write_inventory() {
    let pool = setup_test_pool().await;

    let result = sqlx::query(
        "INSERT INTO inventory.partner (id, name, email) VALUES ('PRT-test1234567890123456', 'Test', 'test-perm@example.com')"
    )
    .execute(&pool)
    .await;

    assert!(result.is_ok(), "Admin (borne) should be able to write to inventory");

    sqlx::query("DELETE FROM inventory.partner WHERE id = 'PRT-test1234567890123456'")
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
#[ignore]
async fn test_role_schema_permissions() {
    let pool = setup_test_pool().await;

    let has_usage: Vec<(String, String)> = sqlx::query_as(
        "SELECT grantee, schema_name \
         FROM information_schema.schema_privileges \
         WHERE grantee IN ('admin_service', 'driver_service', 'gis_worker', 'clickstream_service') \
         AND schema_name IN ('inventory', 'users', 'gis', 'analytics') \
         ORDER BY grantee, schema_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query schema privileges");

    assert!(!has_usage.is_empty(), "Service roles should have schema privileges");
}
