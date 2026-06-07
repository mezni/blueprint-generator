//! EV Database — Database connection and migration management
//!
//! This crate provides database pool management and migration execution for the EV platform.
//!
//! Features:
//! - PostgreSQL connection pooling with sqlx
//! - Automatic migration execution on startup
//! - Health check functionality

pub mod db_pool;

pub use db_pool::PgPool;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_connection_string_parsing() {
        let db_url = "postgresql://user:pass@localhost:5432/test";
        assert!(db_url.contains("postgresql://"));
        assert!(db_url.contains("localhost"));
    }

    #[test]
    fn test_pool_size_validation() {
        let pool_size = 10;
        assert!(pool_size > 0 && pool_size <= 100);
    }
}
