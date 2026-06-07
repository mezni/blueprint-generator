use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;
    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Database pool initialized");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    let path = std::env::var("ADMIN_SERVICE_MIGRATIONS_DIR")
        .unwrap_or_else(|_| "services/admin-service/migrations".into());
    sqlx::migrate::Migrator::new(std::path::Path::new(&path))
        .await?
        .run(pool)
        .await?;
    tracing::info!("Database migrations complete");
    Ok(())
}
