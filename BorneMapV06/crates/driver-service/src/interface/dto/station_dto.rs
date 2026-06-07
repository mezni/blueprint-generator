//! Station DTOs for API responses

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Station DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationDTO {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub partner_id: String,
    pub partner_name: Option<String>,
    pub station_type: String,
    pub power_kw: i32,
    pub available_chargers: i32,
    pub total_chargers: i32,
    pub status: String,
    pub is_favorite: bool,
    pub distance_km: Option<f64>,
    pub distance_m: Option<f64>,
    pub formatted_address: Option<String>,
    pub formatted_distance: Option<String>,
    pub last_updated: Option<String>,
}

impl StationDTO {
    /// Create from station with metadata
    pub fn from_station_with_metadata(
        station: crate::domain::StationWithMetadata,
    ) -> Self {
        let total_chargers = station.station.available_chargers.unwrap_or(0) + 1; // Mock count

        Self {
            id: station.station.id.clone(),
            name: station.station.name.clone().unwrap_or_default(),
            address: station.station.address.clone(),
            latitude: station.station.latitude.unwrap_or(0.0),
            longitude: station.station.longitude.unwrap_or(0.0),
            partner_id: station.station.partner_id.clone().unwrap_or_default(),
            partner_name: station.partner_name,
            station_type: station.format_station_type(),
            power_kw: station.station.power_kw.unwrap_or(0),
            available_chargers: station.available_chargers_count,
            total_chargers,
            status: station.station.status.clone().unwrap_or("unknown".to_string()),
            is_favorite: station.is_favorite,
            distance_km: station.distance_km,
            distance_m: station.distance_m,
            formatted_address: None, // Can be populated if address is available
            formatted_distance: None, // Can be populated if distance is available
            last_updated: station.last_updated,
        }
    }

    /// Format distance for display
    pub fn format_distance(&self) -> String {
        match (self.distance_km, self.distance_m) {
            (Some(km), _) => {
                if km < 1.0 {
                    format!("{} m", self.distance_m.unwrap_or(0))
                } else {
                    format!("{} km", km)
                }
            }
            (_, Some(m)) => format!("{} m", m),
            _ => "Unknown".to_string(),
        }
    }

    /// Format address for display
    pub fn format_address(&self) -> String {
        if let Some(ref address) = self.address {
            address.clone()
        } else {
            self.name.clone()
        }
    }

    /// Check if station is available (status == 'active')
    pub fn is_available(&self) -> bool {
        self.status == "active"
    }

    /// Check if station is fully occupied
    pub fn is_full(&self) -> bool {
        self.available_chargers <= 0
    }

    /// Check if station has free chargers
    pub fn has_free_chargers(&self) -> bool {
        self.available_chargers > 0
    }
}

/// Station detail DTO with charger details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationDetailDTO {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub partner_id: String,
    pub partner_name: Option<String>,
    pub station_type: String,
    pub power_kw: i32,
    pub total_chargers: i32,
    pub available_chargers: i32,
    pub status: String,
    pub chargers: Vec<ChargerDTO>,
    pub distance_km: Option<f64>,
    pub formatted_address: Option<String>,
    pub formatted_distance: Option<String>,
}

/// Charger DTO for station details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerDTO {
    pub id: String,
    pub station_id: String,
    pub type: String,
    pub connector_type: String,
    pub power_kw: i32,
    pub status: String,
    pub is_available: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_dto_creation() {
        let station = crate::domain::Station {
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

        let metadata = crate::domain::StationWithMetadata::from_station(&station, Some("AutoMotive".to_string()));

        let dto = StationDTO::from_station_with_metadata(metadata);
        assert_eq!(dto.id, "STN-001");
        assert_eq!(dto.name, "Test Station");
        assert_eq!(dto.partner_name, Some("AutoMotive".to_string()));
        assert_eq!(dto.power_kw, 150);
    }

    #[test]
    fn test_format_distance_short() {
        let dto = StationDTO {
            id: "STN-001".to_string(),
            name: "Test".to_string(),
            address: None,
            latitude: 36.8,
            longitude: 10.1,
            partner_id: "PRT-001".to_string(),
            partner_name: None,
            station_type: "Test".to_string(),
            power_kw: 100,
            available_chargers: 1,
            total_chargers: 2,
            status: "active".to_string(),
            is_favorite: false,
            distance_km: Some(0.5),
            distance_m: None,
            formatted_address: None,
            formatted_distance: None,
            last_updated: None,
        };

        assert_eq!(dto.format_distance(), "500 m");
    }

    #[test]
    fn test_format_distance_long() {
        let dto = StationDTO {
            id: "STN-001".to_string(),
            name: "Test".to_string(),
            address: None,
            latitude: 36.8,
            longitude: 10.1,
            partner_id: "PRT-001".to_string(),
            partner_name: None,
            station_type: "Test".to_string(),
            power_kw: 100,
            available_chargers: 1,
            total_chargers: 2,
            status: "active".to_string(),
            is_favorite: false,
            distance_km: Some(5.0),
            distance_m: None,
            formatted_address: None,
            formatted_distance: None,
            last_updated: None,
        };

        assert_eq!(dto.format_distance(), "5 km");
    }

    #[test]
    fn test_format_address() {
        let dto = StationDTO {
            id: "STN-001".to_string(),
            name: "Test Station".to_string(),
            address: Some("123 Main St".to_string()),
            latitude: 36.8,
            longitude: 10.1,
            partner_id: "PRT-001".to_string(),
            partner_name: None,
            station_type: "Test".to_string(),
            power_kw: 100,
            available_chargers: 1,
            total_chargers: 2,
            status: "active".to_string(),
            is_favorite: false,
            distance_km: None,
            distance_m: None,
            formatted_address: None,
            formatted_distance: None,
            last_updated: None,
        };

        assert_eq!(dto.format_address(), "123 Main St");
    }

    #[test]
    fn test_address_format_without_address() {
        let dto = StationDTO {
            id: "STN-001".to_string(),
            name: "Test Station".to_string(),
            address: None,
            latitude: 36.8,
            longitude: 10.1,
            partner_id: "PRT-001".to_string(),
            partner_name: None,
            station_type: "Test".to_string(),
            power_kw: 100,
            available_chargers: 1,
            total_chargers: 2,
            status: "active".to_string(),
            is_favorite: false,
            distance_km: None,
            distance_m: None,
            formatted_address: None,
            formatted_distance: None,
            last_updated: None,
        };

        assert_eq!(dto.format_address(), "Test Station");
    }

    #[test]
    fn test_station_availability() {
        let dto = StationDTO {
            id: "STN-001".to_string(),
            name: "Test".to_string(),
            address: None,
            latitude: 36.8,
            longitude: 10.1,
            partner_id: "PRT-001".to_string(),
            partner_name: None,
            station_type: "Test".to_string(),
            power_kw: 100,
            available_chargers: 1,
            total_chargers: 2,
            status: "active".to_string(),
            is_favorite: false,
            distance_km: None,
            distance_m: None,
            formatted_address: None,
            formatted_distance: None,
            last_updated: None,
        };

        assert!(dto.is_available());
        assert!(dto.has_free_chargers());
        assert!(!dto.is_full());
    }

    #[test]
    fn test_station_full() {
        let dto = StationDTO {
            id: "STN-001".to_string(),
            name: "Test".to_string(),
            address: None,
            latitude: 36.8,
            longitude: 10.1,
            partner_id: "PRT-001".to_string(),
            partner_name: None,
            station_type: "Test".to_string(),
            power_kw: 100,
            available_chargers: 0,
            total_chargers: 2,
            status: "active".to_string(),
            is_favorite: false,
            distance_km: None,
            distance_m: None,
            formatted_address: None,
            formatted_distance: None,
            last_updated: None,
        };

        assert!(dto.is_available());
        assert!(!dto.has_free_chargers());
        assert!(dto.is_full());
    }
}
