use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn join_database_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    log::info!("Database pool connected");
    Ok(pool)
}
