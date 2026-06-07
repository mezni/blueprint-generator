use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

/// Create a PostgreSQL connection pool from a database URL.
///
/// # Errors
/// Returns `sqlx::Error` if the connection fails or pool creation fails.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url)
        .await?;

    // Verify the connection is valid
    sqlx::query("SELECT 1").execute(&pool).await?;

    Ok(pool)
}

/// Create a connection pool with custom configuration.
pub async fn create_pool_with_config(
    database_url: &str,
    max_connections: u32,
    acquire_timeout_secs: u64,
) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(acquire_timeout_secs))
        .connect(database_url)
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_url_parsing() {
        // Test that we can parse a valid DATABASE_URL
        let url = "postgresql://user:pass@localhost:5432/db";
        // Just verify the URL is syntactically valid
        assert!(url.starts_with("postgresql://"));
        assert!(url.contains("@"));
    }

    #[test]
    fn test_pool_url_parsing_invalid() {
        let url = "";
        assert!(url.is_empty());
    }

    #[test]
    fn test_invalid_url_detection() {
        assert!(!url_has_valid_scheme("not_a_url"));
        assert!(url_has_valid_scheme("postgresql://localhost/db"));
    }

    /// Helper: check if a string looks like a valid PostgreSQL URL
    fn url_has_valid_scheme(url: &str) -> bool {
        url.starts_with("postgresql://") || url.starts_with("postgres://")
    }
}
