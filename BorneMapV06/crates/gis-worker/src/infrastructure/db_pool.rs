//! Database pool manager for gis-worker

use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};

/// Database pool configuration
#[derive(Debug, Clone)]
pub struct DatabasePoolConfig {
    pub database_url: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub acquire_timeout: Option<std::time::Duration>,
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://postgres@localhost/borne_map".to_string()),
            min_connections: 2,
            max_connections: 10,
            acquire_timeout: Some(std::time::Duration::from_secs(30)),
        }
    }
}

/// Database pool manager
pub struct DatabasePoolManager {
    pool: PgPool,
}

impl DatabasePoolManager {
    /// Create a new database pool manager
    pub fn new(config: DatabasePoolConfig) -> Result<Self, sqlx::Error> {
        debug!(
            "Creating database pool: min={}, max={}, timeout={:?}",
            config.min_connections,
            config.max_connections,
            config.acquire_timeout
        );

        let pool = PgPoolOptions::new()
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .acquire_timeout(config.acquire_timeout.unwrap_or_default())
            .connect(&config.database_url)?;

        debug!("Database pool created successfully");

        Ok(Self { pool })
    }

    /// Get the database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get the pool as a mutable reference
    pub fn pool_mut(&mut self) -> &mut PgPool {
        &mut self.pool
    }

    /// Perform a health check
    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        let result = sqlx::query("SELECT 1").fetch_one(&self.pool).await?;
        debug!("Health check passed: {:?}", result);
        Ok(())
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        let info = self.pool.get_stats();
        PoolStats {
            min_connections: info.min_size,
            max_connections: info.max_size,
            idle_connections: info.idle_size,
            in_use_connections: info.active_size,
        }
    }

    /// Close the pool
    pub async fn close(self) -> Result<(), sqlx::Error> {
        debug!("Closing database pool");
        self.pool.close().await;
        Ok(())
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub min_connections: u32,
    pub max_connections: u32,
    pub idle_connections: u32,
    pub in_use_connections: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_pool_config_default() {
        let config = DatabasePoolConfig::default();
        assert_eq!(config.min_connections, 2);
        assert_eq!(config.max_connections, 10);
    }

    #[test]
    fn test_database_pool_manager_creation() {
        let manager = DatabasePoolManager::new(DatabasePoolConfig::default());
        assert!(manager.is_ok());
    }
}
