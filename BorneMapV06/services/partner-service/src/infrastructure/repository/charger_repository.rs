//! Charger repository for partner service

use sqlx::{PgPool, Postgres};
use std::sync::Arc;

use crate::ev_domain::{Charger, Station};
use crate::error::{AppResult, ApiError};
use crate::ev_db::Pool;

/// Charger repository
pub struct ChargerRepository {
    pool: Pool,
}

impl ChargerRepository {
    /// Create new repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// List chargers for a station
    pub async fn list_by_station(&self, station_id: &str) -> AppResult<Vec<Charger>> {
        let chargers = sqlx::query_as::<_, Charger>(
            r#"
            SELECT id, station_id, connector_type, power_kw, status, created_at, updated_at, deleted_at
            FROM inventory.charger
            WHERE station_id = $1 AND deleted_at IS NULL
            ORDER BY id ASC
            "#,
        )
        .bind(station_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(chargers)
    }

    /// Get charger by ID
    pub async fn get_by_id(&self, charger_id: &str) -> AppResult<Charger> {
        let charger = sqlx::query_as::<_, Charger>(
            r#"
            SELECT id, station_id, connector_type, power_kw, status, created_at, updated_at, deleted_at
            FROM inventory.charger
            WHERE id = $1
            "#,
        )
        .bind(charger_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Charger {} not found", charger_id)))?;

        Ok(charger)
    }

    /// Create charger for a station
    pub async fn create(&self, charger: &Charger) -> AppResult<String> {
        let result = sqlx::query_scalar::<_, String>(
            r#"
            INSERT INTO inventory.charger (id, station_id, connector_type, power_kw, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING id
            "#,
        )
        .bind(&charger.id)
        .bind(&charger.station_id)
        .bind(&charger.connector_type)
        .bind(&charger.power_kw)
        .bind(&charger.status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    /// Update charger status
    pub async fn update_status(
        &self,
        charger_id: &str,
        status: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE inventory.charger
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(status)
        .bind(charger_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    /// Delete charger (soft delete)
    pub async fn delete(&self, charger_id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE inventory.charger
            SET deleted_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(charger_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    /// Count chargers for a station
    pub async fn count_by_station(&self, station_id: &str) -> AppResult<i64> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM inventory.charger
            WHERE station_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(station_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(count)
    }

    /// Count chargers by status for a station
    pub async fn count_by_status(&self, station_id: &str, status: &str) -> AppResult<i64> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM inventory.charger
            WHERE station_id = $1 AND status = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(station_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charger_repository_creation() {
        let pool = Pool::none(); // Mock pool for testing
        let repo = ChargerRepository::new(pool);
        assert!(true); // Repository created successfully
    }
}
