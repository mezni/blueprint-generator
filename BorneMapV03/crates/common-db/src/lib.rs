pub mod identity;
pub mod partition;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub use sqlx;

const DEFAULT_DATABASE_URL: &str = "postgres://borne:devpassword@localhost:5432/borne_map";
const MAX_CONNECTIONS: u32 = 10;
const ACQUIRE_TIMEOUT_SECS: u64 = 30;

pub async fn create_pool(database_url: Option<&str>) -> Result<PgPool, sqlx::Error> {
    let url = match database_url {
        Some(url) => url.to_string(),
        None => std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_string()),
    };

    PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .acquire_timeout(Duration::from_secs(ACQUIRE_TIMEOUT_SECS))
        .connect(&url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("../../services/admin-service/migrations")
        .run(pool)
        .await
}

pub async fn check_connection(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    Ok(())
}
