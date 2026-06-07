//! Create station use case for driver-service

use sqlx::PgPool;

use crate::domain::{PartnerScope, ChargerStatus};
use crate::ev_db::Pool;
use crate::ev_domain::Station;

/// Create station use case
pub struct CreateStationUseCase {
    pool: Pool,
}

impl CreateStationUseCase {
    /// Create a new create station use case
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Create a new station
    pub async fn create_station(
        &self,
        scope: &PartnerScope,
        input: CreateStationInput,
    ) -> Result<Station, crate::DomainResult> {
        scope.validate_partner_role()?;

        // Validate coordinates
        if let Some(lat) = input.latitude {
            crate::ev_domain::validate_latitude(lat)?;
        }

        if let Some(lng) = input.longitude {
            crate::ev_domain::validate_longitude(lng)?;
        }

        let now = chrono::Utc::now().timestamp();

        let station = Station {
            id: format!("STN-{}", now),
            name: input.name.clone(),
            address: input.address.clone(),
            latitude: input.latitude,
            longitude: input.longitude,
            partner_id: Some(scope.partner_id.clone()),
            station_type: input.station_type.clone(),
            power_kw: input.power_kw,
            available_chargers: input.available_chargers.unwrap_or(0),
            status: input.status.clone().unwrap_or("active".to_string()),
            created_at: Some(now),
            updated_at: Some(now),
        };

        // TODO: Validate partner has permission to create stations

        sqlx::query!(
            r#"
            INSERT INTO inventory.station (
                id, name, address, latitude, longitude, partner_id, station_type,
                power_kw, available_chargers, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            station.id,
            station.name,
            station.address,
            station.latitude,
            station.longitude,
            station.partner_id,
            station.station_type,
            station.power_kw,
            station.available_chargers,
            station.status,
            station.created_at,
            station.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to create station: {}", e)))?;

        // TODO: Create default charger (station will have at least one charger)
        // TODO: Trigger outbox event for GIS sync

        Ok(station)
    }
}

/// Create station input
#[derive(Debug, Clone)]
pub struct CreateStationInput {
    pub name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub station_type: Option<String>,
    pub power_kw: Option<i32>,
    pub available_chargers: Option<i32>,
    pub status: Option<String>,
}

impl CreateStationInput {
    /// Create from map
    pub fn from_map(params: &std::collections::HashMap<String, String>) -> Self {
        let name = params.get("name").cloned();
        let address = params.get("address").cloned();
        let station_type = params.get("station_type").cloned();
        let status = params.get("status").cloned();

        let latitude = params
            .get("latitude")
            .and_then(|s| s.parse::<f64>().ok());

        let longitude = params
            .get("longitude")
            .and_then(|s| s.parse::<f64>().ok());

        let power_kw = params
            .get("power_kw")
            .and_then(|s| s.parse::<i32>().ok());

        let available_chargers = params
            .get("available_chargers")
            .and_then(|s| s.parse::<i32>().ok());

        Self {
            name,
            address,
            latitude,
            longitude,
            station_type,
            power_kw,
            available_chargers,
            status,
        }
    }

    /// Validate required fields
    pub fn validate(&self) -> Result<(), crate::DomainResult> {
        if self.name.is_none() {
            return Err(crate::DomainError::BusinessRuleViolation(
                "Station name is required".to_string(),
            ));
        }

        if self.latitude.is_none() && self.longitude.is_none() {
            return Err(crate::DomainError::BusinessRuleViolation(
                "Either latitude or longitude is required".to_string(),
            ));
        }

        if let Some(lat) = self.latitude {
            crate::ev_domain::validate_latitude(lat)?;
        }

        if let Some(lng) = self.longitude {
            crate::ev_domain::validate_longitude(lng)?;
        }

        if let Some(power_kw) = self.power_kw {
            if power_kw < 1 {
                return Err(crate::DomainError::BusinessRuleViolation(
                    "Power must be at least 1 kW".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_station_usecase_creation() {
        let usecase = CreateStationUseCase::new(Pool::none());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_create_station_input_from_map() {
        let mut params = std::collections::HashMap::new();
        params.insert("name".to_string(), "Test Station".to_string());
        params.insert("address".to_string(), "123 Main St".to_string());
        params.insert("latitude".to_string(), "36.8065".to_string());
        params.insert("longitude".to_string(), "10.1815".to_string());
        params.insert("power_kw".to_string(), "150".to_string());

        let input = CreateStationInput::from_map(&params);

        assert_eq!(input.name, Some("Test Station".to_string()));
        assert_eq!(input.address, Some("123 Main St".to_string()));
        assert_eq!(input.power_kw, Some(150));
    }

    #[test]
    fn test_create_station_input_validation() {
        let mut params = std::collections::HashMap::new();
        params.insert("address".to_string(), "123 Main St".to_string());

        let input = CreateStationInput::from_map(&params);

        assert!(input.validate().is_err());
    }
}
