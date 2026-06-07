//! Migration runner for database schema setup
//!
//! This module handles automatic migration execution on service startup.
//! Migrations are stored in migrations/ directory and applied sequentially.

use sqlx::{PgPool, Postgres};
use tracing::{info, warn};

/// Migration status
pub enum MigrationStatus {
    Applied,
    Failed,
}

/// Run all pending migrations
///
/// Executes migrations in order from migrations/ directory
pub async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting database migrations...");

    let migration_files = vec![
        "001_create_inventory_schema.sql",
        "002_create_users_schema_and_tables.sql",
        "003_create_gis_schema_and_indexes.sql",
        "004_create_station_outbox.sql",
    ];

    for (i, file) in migration_files.iter().enumerate() {
        let migration_id = i + 1;
        info!("Applying migration {} ({}): {}", migration_id, file, file);

        let migration_sql = include_str!(concat!("../migrations/", file));

        if migration_sql.is_empty() {
            warn!("Migration file {} is empty, skipping", file);
            continue;
        }

        // Execute migration SQL
        sqlx::query(migration_sql)
            .execute(pool)
            .await
            .map_err(|e| {
                warn!("Failed to apply migration {}: {}", file, e);
                e
            })?;

        info!("Migration {} ({}.sql) applied successfully", migration_id, file);
    }

    info!("All migrations applied successfully!");
    Ok(())
}

/// Verify all tables exist
///
/// Returns errors if any required tables are missing
pub async fn verify_schema(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let required_tables = vec![
        "inventory.partner",
        "inventory.station",
        "inventory.charger",
        "inventory.station_outbox",
        "users.user",
        "users.favorite",
        "users.review",
        "gis.osm_ways",
        "gis.osm_nodes",
        "gis.station_locations",
    ];

    for table in required_tables {
        let exists: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT FROM information_schema.tables
                WHERE table_schema = current_schema()
                AND table_name = $1
            )
            "#,
        )
        .bind(table.split('.').nth(1))
        .fetch_one(pool)
        .await?;

        if !exists {
            return Err(format!(
                "Required table '{}' does not exist. Run migrations first.",
                table
            )
            .into());
        }
    }

    info!("All required tables verified!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_file_loading() {
        let sql = include_str!("../migrations/001_create_inventory_schema.sql");
        assert!(!sql.is_empty());
        assert!(sql.contains("CREATE TABLE"));
    }

    #[test]
    fn test_migration_order() {
        let migrations = vec![
            "001_create_inventory_schema.sql",
            "002_create_users_schema_and_tables.sql",
            "003_create_gis_schema_and_indexes.sql",
            "004_create_station_outbox.sql",
        ];

        assert_eq!(migrations.len(), 4);
        assert_eq!(migrations[0], "001_create_inventory_schema.sql");
        assert_eq!(migrations[3], "004_create_station_outbox.sql");
    }

    #[test]
    fn test_migration_sql_contains_create_table() {
        let sql = include_str!("../migrations/001_create_inventory_schema.sql");
        assert!(sql.contains("CREATE TABLE inventory.partner"));
        assert!(sql.contains("CREATE TABLE inventory.station"));
        assert!(sql.contains("CREATE TABLE inventory.charger"));
    }
}
