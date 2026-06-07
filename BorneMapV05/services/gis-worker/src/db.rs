use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

pub async fn init_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;
    sqlx::query("SELECT 1").execute(&pool).await?;
    info!("Database pool initialized");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool, migrations_dir: &str) -> Result<(), sqlx::migrate::MigrateError> {
    let migrator = sqlx::migrate::Migrator::new(std::path::Path::new(migrations_dir)).await?;
    migrator.run(pool).await?;
    info!("Database migrations complete");
    Ok(())
}
