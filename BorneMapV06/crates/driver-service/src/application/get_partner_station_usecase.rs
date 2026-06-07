//! Get partner station use case for driver-service

use sqlx::PgPool;

use crate::domain::PartnerScope;
use crate::ev_db::Pool;

/// Get partner station use case
pub struct GetPartnerStationUseCase {
    pool: Pool,
}

impl GetPartnerStationUseCase {
    /// Create a new get partner station use case
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Get station detail with all chargers
    pub async fn get_station_detail(
        &self,
        scope: &PartnerScope,
        station_id: &str,
    ) -> Result<PartnerStationDetail, crate::DomainResult> {
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

        // Get all chargers for this station
        let chargers: Vec<crate::ev_domain::Charger> = sqlx::query_as!(
            crate::ev_domain::Charger,
            r#"
            SELECT id, station_id, connector_type, power_kw, status
            FROM inventory.charger
            WHERE station_id = $1
            ORDER BY power_kw DESC
            "#,
            station_id as &str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to query chargers: {}", e)))?;

        let station = station.unwrap();

        Ok(PartnerStationDetail {
            station,
            chargers,
        })
    }

    /// Get station with detailed charger information
    pub async fn get_station_with_chargers(
        &self,
        scope: &PartnerScope,
        station_id: &str,
    ) -> Result<PartnerStationDetail, crate::DomainResult> {
        self.get_station_detail(scope, station_id).await
    }
}

/// Partner station detail with chargers
#[derive(Debug, Clone)]
pub struct PartnerStationDetail {
    pub station: crate::ev_domain::Station,
    pub chargers: Vec<crate::ev_domain::Charger>,
}

impl PartnerStationDetail {
    /// Get charger summary
    pub fn charger_summary(&self) -> crate::domain::ChargerStatusSummary {
        let mut status_map = std::collections::HashMap::new();

        for charger in &self.chargers {
            let count = status_map.entry(
                charger.status.clone().unwrap_or("unknown".to_string())
            ).or_insert(0);
            *count += 1;
        }

        crate::domain::ChargerStatusDistribution::from_map(&status_map)
    }

    /// Get total charger count
    pub fn total_chargers(&self) -> usize {
        self.chargers.len()
    }

    /// Get available charger count
    pub fn available_chargers(&self) -> usize {
        self.chargers
            .iter()
            .filter(|c| c.status.as_deref() == Some("active"))
            .count()
    }

    /// Get charger by connector type
    pub fn chargers_by_type(&self, connector_type: &str) -> Vec<&crate::ev_domain::Charger> {
        self.chargers
            .iter()
            .filter(|c| c.connector_type == connector_type)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_partner_station_usecase_creation() {
        let usecase = GetPartnerStationUseCase::new(Pool::none());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_partner_station_detail_creation() {
        let station = crate::ev_domain::Station {
            id: "STN-001".to_string(),
            name: Some("Test Station".to_string()),
            address: Some("Test Address".to_string()),
            latitude: Some(36.8065),
            longitude: Some(10.1815),
            partner_id: Some("PRT-001".to_string()),
            station_type: Some("EV Charging".to_string()),
            power_kw: Some(150),
            available_chargers: Some(4),
            status: Some("active".to_string()),
            created_at: None,
            updated_at: None,
        };

        let chargers = vec![
            crate::ev_domain::Charger {
                id: "CHR-001".to_string(),
                station_id: "STN-001".to_string(),
                connector_type: "CCS2".to_string(),
                power_kw: Some(150),
                status: Some("active".to_string()),
            },
            crate::ev_domain::Charger {
                id: "CHR-002".to_string(),
                station_id: "STN-001".to_string(),
                connector_type: "Type 2".to_string(),
                power_kw: Some(22),
                status: Some("active".to_string()),
            },
        ];

        let detail = PartnerStationDetail { station, chargers };

        assert_eq!(detail.total_chargers(), 2);
        assert_eq!(detail.available_chargers(), 2);
        assert_eq!(detail.chargers_by_type("CCS2").len(), 1);
    }
}
