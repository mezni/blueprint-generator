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
async fn test_migrations_apply_and_rollback() {
    let pool = setup_test_pool().await;

    common_db::run_migrations(&pool)
        .await
        .expect("Migrations should apply successfully");

    let schemas: Vec<(String,)> = sqlx::query_as(
        "SELECT schema_name FROM information_schema.schemata WHERE schema_name IN ('inventory', 'users', 'gis', 'analytics') ORDER BY schema_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query schemas");

    let schema_names: Vec<&str> = schemas.iter().map(|s| s.0.as_str()).collect();
    assert!(schema_names.contains(&"analytics"), "analytics schema should exist");
    assert!(schema_names.contains(&"gis"), "gis schema should exist");
    assert!(schema_names.contains(&"inventory"), "inventory schema should exist");
    assert!(schema_names.contains(&"users"), "users schema should exist");

    let extensions: Vec<(String,)> = sqlx::query_as(
        "SELECT extname FROM pg_extension WHERE extname IN ('postgis', 'uuid-ossp') ORDER BY extname"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query extensions");

    let ext_names: Vec<&str> = extensions.iter().map(|e| e.0.as_str()).collect();
    assert!(ext_names.contains(&"postgis"), "postgis extension should exist");
    assert!(ext_names.contains(&"uuid-ossp"), "uuid-ossp extension should exist");

    let roles: Vec<(String,)> = sqlx::query_as(
        "SELECT rolname FROM pg_roles WHERE rolname IN ('admin_service', 'driver_service', 'gis_worker', 'clickstream_service') ORDER BY rolname"
    )
    .fetch_all(&pool)
    .await
    .expect("Should query roles");

    let role_names: Vec<&str> = roles.iter().map(|r| r.0.as_str()).collect();
    assert!(role_names.contains(&"admin_service"), "admin_service role should exist");
    assert!(role_names.contains(&"clickstream_service"), "clickstream_service role should exist");
    assert!(role_names.contains(&"driver_service"), "driver_service role should exist");
    assert!(role_names.contains(&"gis_worker"), "gis_worker role should exist");
}

#[tokio::test]
#[ignore]
async fn test_migration_idempotency() {
    let pool = setup_test_pool().await;

    common_db::run_migrations(&pool)
        .await
        .expect("First migration run should succeed");

    common_db::run_migrations(&pool)
        .await
        .expect("Second migration run should succeed (idempotent)");
}
