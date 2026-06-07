//! Database access layer for partner-service

use sqlx::PgPool;

use crate::error::AppResult;
use crate::ev_db::Pool;

/// Database module for partner-service
pub struct Database {
    pool: Pool,
}

impl Database {
    /// Create new database instance
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Get the database pool
    pub fn pool(&self) -> &Pool {
        &self.pool
    }

    /// Get a transaction
    pub async fn begin(&self) -> sqlx::Transaction<'_, Postgres> {
        self.pool.begin().await.expect("Failed to begin transaction")
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            pool: Pool::none(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::new(Pool::none());
        assert!(true);
    }
}
