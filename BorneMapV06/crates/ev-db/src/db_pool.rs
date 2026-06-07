//! PostgreSQL connection pool management

use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::time::Duration;

/// Database connection pool
pub type Pool = Pool<Postgres>;

/// Database connection pool options
#[derive(Debug, Clone)]
pub struct PoolOptions {
    pub max_connections: u32,
    pub acquire_timeout: Duration,
}

impl Default for PoolOptions {
    fn default() -> Self {
        Self {
            max_connections: 10,
            acquire_timeout: Duration::from_secs(30),
        }
    }
}

impl PoolOptions {
    /// Create new pool options with custom connection count
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Create new pool options with custom acquire timeout
    pub fn with_acquire_timeout(mut self, timeout: Duration) -> Self {
        self.acquire_timeout = timeout;
        self
    }
}

/// Create a PostgreSQL connection pool
///
/// # Arguments
/// * `database_url` - PostgreSQL connection string (e.g., "postgresql://user:pass@localhost:5432/db")
/// * `options` - Pool connection options
///
/// # Errors
/// Returns error if connection fails or pool creation fails
pub async fn create_pool(database_url: &str, options: PoolOptions) -> sqlx::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(options.max_connections)
        .acquire_timeout(options.acquire_timeout)
        .connect(database_url)
        .await?;

    Ok(pool)
}

/// Check if database pool is healthy
///
/// Executes a simple query to verify connection
pub async fn health_check(pool: &PgPool) -> sqlx::Result<bool> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map(|_| true)
        .map_err(|e| {
            tracing::error!("Health check failed: {}", e);
            e
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_options_defaults() {
        let opts = PoolOptions::default();
        assert_eq!(opts.max_connections, 10);
        assert_eq!(opts.acquire_timeout.as_secs(), 30);
    }

    #[test]
    fn test_pool_options_custom() {
        let opts = PoolOptions::default()
            .with_max_connections(20)
            .with_acquire_timeout(Duration::from_secs(60));
        assert_eq!(opts.max_connections, 20);
        assert_eq!(opts.acquire_timeout.as_secs(), 60);
    }

    #[test]
    fn test_database_url_format() {
        let url = "postgresql://test:pass@localhost:5432/testdb";
        assert!(url.contains("postgresql://"));
        assert!(url.contains("localhost"));
        assert!(url.contains(":5432"));
    }
}
