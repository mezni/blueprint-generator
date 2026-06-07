use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn init_db_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;
    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Database pool initialized");
    Ok(pool)
}
