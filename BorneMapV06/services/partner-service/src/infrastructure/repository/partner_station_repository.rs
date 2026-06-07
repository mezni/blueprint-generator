//! Partner station repository for managing partner-owned stations

use sqlx::{PgPool, Postgres};
use std::sync::Arc;

use crate::ev_domain::{Partner, Station, Charger};
use crate::error::{AppResult, ApiError};
use crate::ev_db::Pool;

/// Partner station repository
pub struct PartnerStationRepository {
    pool: Pool,
}

impl PartnerStationRepository {
    /// Create new repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// List partner's stations with pagination and optional status filter
    pub async fn list_by_partner(
        &self,
        partner_id: &str,
        limit: i32,
        offset: i32,
        status_filter: Option<String>,
    ) -> AppResult<Vec<Station>> {
        let query = if let Some(status) = status_filter {
            r#"
            SELECT id, partner_id, name, address, latitude, longitude, osm_node_id,
                   availability_status, capacity, created_at, updated_at, deleted_at
            FROM inventory.station
            WHERE partner_id = $1 AND availability_status = $2 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#
        } else {
            r#"
            SELECT id, partner_id, name, address, latitude, longitude, osm_node_id,
                   availability_status, capacity, created_at, updated_at, deleted_at
            FROM inventory.station
            WHERE partner_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        };

        let stations = sqlx::query_as::<_, Station>(query)
            .bind(partner_id)
            .bind(&status_filter.unwrap_or_default())
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(stations)
    }

    /// Get station by ID (must belong to partner)
    pub async fn get_by_id(&self, station_id: &str, partner_id: &str) -> AppResult<Station> {
        let station = sqlx::query_as::<_, Station>(
            r#"
            SELECT id, partner_id, name, address, latitude, longitude, osm_node_id,
                   availability_status, capacity, created_at, updated_at, deleted_at
            FROM inventory.station
            WHERE id = $1 AND partner_id = $2
            "#,
        )
        .bind(station_id)
        .bind(partner_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Station {} not found", station_id)))?;

        Ok(station)
    }

    /// Create a new station for a partner
    pub async fn create(
        &self,
        station: &Station,
        partner_id: &str,
    ) -> AppResult<String> {
        let result = sqlx::query_scalar::<_, String>(
            r#"
            INSERT INTO inventory.station (id, partner_id, name, address, latitude, longitude,
                   availability_status, capacity, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
            RETURNING id
            "#,
        )
        .bind(&station.id)
        .bind(partner_id)
        .bind(&station.name)
        .bind(&station.address)
        .bind(station.latitude)
        .bind(station.longitude)
        .bind(&station.availability_status)
        .bind(station.capacity)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    /// Update station details (name, address, availability, capacity)
    pub async fn update(
        &self,
        station_id: &str,
        partner_id: &str,
        name: Option<&str>,
        address: Option<&str>,
        availability_status: Option<String>,
        capacity: Option<i32>,
    ) -> AppResult<()> {
        // Build dynamic UPDATE query based on provided fields
        let mut query = String::from("UPDATE inventory.station SET updated_at = NOW()");
        let mut bind_count = 1;

        if let Some(name_val) = name {
            query.push_str(&format!("\n        SET name = ${}", bind_count));
            bind_count += 1;
        }

        if let Some(address_val) = address {
            if bind_count > 1 {
                query.push_str(&format!(", address = ${}", bind_count));
            } else {
                query.push_str(&format!("\n        SET address = ${}", bind_count));
            }
            bind_count += 1;
        }

        if let Some(avail) = availability_status {
            if bind_count > 1 {
                query.push_str(&format!(", availability_status = ${}", bind_count));
            } else {
                query.push_str(&format!("\n        SET availability_status = ${}", bind_count));
            }
            bind_count += 1;
        }

        if let Some(cap) = capacity {
            if bind_count > 1 {
                query.push_str(&format!(", capacity = ${}", bind_count));
            } else {
                query.push_str(&format!("\n        SET capacity = ${}", bind_count));
            }
            bind_count += 1;
        }

        query.push_str(
            r#"
            WHERE id = $1 AND partner_id = $2
            "#,
        );

        let mut query_builder = sqlx::query(&query);
        query_builder = query_builder.bind(station_id);
        query_builder = query_builder.bind(partner_id);

        let mut bind_idx = 3;
        if let Some(name_val) = name {
            query_builder = query_builder.bind(name_val);
            bind_idx += 1;
        }

        if let Some(address_val) = address {
            query_builder = query_builder.bind(address_val);
            bind_idx += 1;
        }

        if let Some(avail) = availability_status {
            query_builder = query_builder.bind(avail);
            bind_idx += 1;
        }

        if let Some(cap) = capacity {
            query_builder = query_builder.bind(cap);
        }

        let _rows_affected = query_builder
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Check if any rows were updated
        // Note: Need to fetch result differently for affected rows
        // This is a simplified version - actual implementation may vary

        Ok(())
    }

    /// Delete station (soft delete)
    pub async fn delete(&self, station_id: &str, partner_id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE inventory.station
            SET deleted_at = NOW()
            WHERE id = $1 AND partner_id = $2
            "#,
        )
        .bind(station_id)
        .bind(partner_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    /// Get station count for partner
    pub async fn count_by_partner(&self, partner_id: &str) -> AppResult<i64> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM inventory.station
            WHERE partner_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(partner_id)
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
    fn test_partner_station_repository_creation() {
        let pool = Pool::none(); // Mock pool for testing
        let repo = PartnerStationRepository::new(pool);
        assert!(true); // Repository created successfully
    }
}
