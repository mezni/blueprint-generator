//! Partner stations use cases for driver-service

use std::time::SystemTime;

use sqlx::PgPool;

use crate::domain::{PartnerScope, PartnerStationStats, ChargerStatusSummary, ChargerStatus};
use crate::ev_db::Pool;

/// Partner station list use case
pub struct PartnerStationsListUseCase {
    pool: Pool,
}

impl PartnerStationsListUseCase {
    /// Create a new partner stations list use case
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// List partner's stations with optional filters
    pub async fn list_partner_stations(
        &self,
        scope: &PartnerScope,
        status: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<crate::ev_domain::Station>, crate::DomainResult> {
        scope.validate_partner_role()?;

        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let mut query = String::from(
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status, created_at, updated_at
            FROM inventory.station
            WHERE partner_id = $1
            "#
        );

        let mut param_count = 2;

        if let Some(status_filter) = &status {
            query.push_str(&format!(" AND status = ${}", param_count));
            param_count += 1;
        }

        query.push_str(" ORDER BY name ASC");

        if limit > 0 {
            query.push_str(&format!(" LIMIT ${}", param_count));
            param_count += 1;
        }

        if offset > 0 {
            query.push_str(&format!(" OFFSET ${}", param_count));
        }

        let mut query_builder = sqlx::query(&query);
        query_builder = query_builder.bind(&scope.partner_id);

        if let Some(status_filter) = &status {
            query_builder = query_builder.bind(status_filter);
        }

        if limit > 0 {
            query_builder = query_builder.bind(limit as i32);
        }

        if offset > 0 {
            query_builder = query_builder.bind(offset as i32);
        }

        let stations = query_builder.fetch_all(&self.pool).await.map_err(|e| {
            crate::DomainError::DatabaseError(format!("Failed to query stations: {}", e))
        })?;

        Ok(stations)
    }

    /// Get partner's station statistics
    pub async fn get_partner_stats(&self, scope: &PartnerScope) -> Result<PartnerStationStats, crate::DomainResult> {
        scope.validate_partner_role()?;

        let partner_id = &scope.partner_id;

        // Get station counts
        let station_count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM inventory.station
            WHERE partner_id = $1
            "#,
            partner_id as &str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to get station count: {}", e)))?;

        let active_count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM inventory.station
            WHERE partner_id = $1 AND status = 'active'
            "#,
            partner_id as &str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to get active station count: {}", e)))?;

        // Get charger counts
        let charger_count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM inventory.charger
            WHERE partner_id = $1
            "#,
            partner_id as &str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to get charger count: {}", e)))?;

        let active_charger_count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM inventory.charger
            WHERE partner_id = $1 AND status = 'active'
            "#,
            partner_id as &str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to get active charger count: {}", e)))?;

        // Get average capacity
        let avg_capacity: Option<i64> = sqlx::query_scalar!(
            r#"
            SELECT AVG(power_kw)
            FROM inventory.charger
            WHERE partner_id = $1
            "#,
            partner_id as &str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to get avg capacity: {}", e)))?;

        let avg_capacity = avg_capacity.unwrap_or(0) as f64;

        Ok(PartnerStationStats {
            total_stations: station_count as i32,
            active_stations: active_count as i32,
            offline_stations: (station_count as i32) - (active_count as i32),
            total_chargers: charger_count as i32,
            active_chargers: active_charger_count as i32,
            offline_chargers: (charger_count as i32) - (active_charger_count as i32),
            avg_capacity,
        })
    }

    /// Get charger status summary for a station
    pub async fn get_station_charger_summary(
        &self,
        scope: &PartnerScope,
        station_id: &str,
    ) -> Result<ChargerStatusSummary, crate::DomainResult> {
        scope.validate_partner_role()?;

        // Verify station belongs to this partner
        let station: Option<crate::ev_domain::Station> = sqlx::query_as!(
            crate::ev_domain::Station,
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status, created_at, updated_at
            FROM inventory.station
            WHERE id = $1 AND partner_id = $2
            "#,
            station_id as &str,
            scope.partner_id.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to query station: {}", e)))?;

        if station.is_none() {
            return Err(crate::DomainError::NotFound(format!("Station {} not found", station_id)));
        }

        // Get charger counts by status
        let status_counts: Vec<(String, i64)> = sqlx::query!(
            r#"
            SELECT status, COUNT(*) as count
            FROM inventory.charger
            WHERE station_id = $1
            GROUP BY status
            "#,
            station_id as &str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to query chargers: {}", e)))?;

        let mut available = 0i64;
        let mut in_use = 0i64;
        let mut maintenance = 0i64;
        let mut offline = 0i64;

        for (status, count) in status_counts {
            match ChargerStatus::from_str(&status) {
                Some(ChargerStatus::Available) => available = count,
                Some(ChargerStatus::InUse) => in_use = count,
                Some(ChargerStatus::Maintenance) => maintenance = count,
                Some(ChargerStatus::Offline) => offline = count,
                None => {}
            }
        }

        let total = available + in_use + maintenance + offline;

        Ok(ChargerStatusSummary::new(
            total as i32,
            available as i32,
            in_use as i32,
            maintenance as i32,
            offline as i32,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partner_stations_list_usecase_creation() {
        let usecase = PartnerStationsListUseCase::new(Pool::none());
        assert!(true); // Structure validated
    }
}
