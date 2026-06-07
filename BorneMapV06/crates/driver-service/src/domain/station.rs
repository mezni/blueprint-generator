//! Station domain model

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::ev_domain::Station;

/// Station with extended metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationWithMetadata {
    pub station: Station,
    pub partner_name: Option<String>,
    pub available_chargers_count: i32,
    pub distance_km: Option<f64>,
    pub distance_m: Option<f64>,
    pub station_type_display: Option<String>,
    pub power_kw_display: Option<String>,
    pub last_updated: Option<String>,
    pub is_favorite: bool,
}

impl StationWithMetadata {
    /// Create from a station entity
    pub fn from_station(
        station: &Station,
        partner_name: Option<String>,
    ) -> Self {
        let available_chargers = station.available_chargers.unwrap_or(0);

        Self {
            station: station.clone(),
            partner_name,
            available_chargers_count: available_chargers,
            distance_km: None,
            distance_m: None,
            station_type_display: station.station_type.clone(),
            power_kw_display: station.power_kw.map(|kw| format!("{} kW", kw)),
            last_updated: station.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M").to_string()),
            is_favorite: false,
        }
    }

    /// Set distance to query point
    pub fn with_distance(mut self, distance_km: f64, distance_m: f64) -> Self {
        self.distance_km = Some(distance_km);
        self.distance_m = Some(distance_m);
        self
    }

    /// Set if this station is a favorite for the user
    pub fn with_favorite(mut self, is_favorite: bool) -> Self {
        self.is_favorite = is_favorite;
        self
    }

    /// Format station type for display
    pub fn format_station_type(&self) -> String {
        if let Some(ref type_str) = self.station_type_display {
            match type_str.as_str() {
                "EV Charging" => "Electric Vehicle Charging".to_string(),
                "Fast Charging" => "Fast Charging".to_string(),
                "Supercharger" => "Supercharger".to_string(),
                _ => type_str.clone(),
            }
        } else {
            "Charging Station".to_string()
        }
    }

    /// Format power for display
    pub fn format_power(&self) -> String {
        self.power_kw_display.clone().unwrap_or_else(|| "N/A".to_string())
    }
}

/// Sort stations by distance (ascending)
pub fn sort_by_distance(mut stations: Vec<StationWithMetadata>) -> Vec<StationWithMetadata> {
    stations.sort_by(|a, b| {
        let distance_a = a.distance_km.unwrap_or(f64::MAX);
        let distance_b = b.distance_km.unwrap_or(f64::MAX);
        distance_a.partial_cmp(&distance_b).unwrap()
    });
    stations
}

/// Pagination parameters for station listings
#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

impl Pagination {
    /// Create new pagination
    pub fn new(page: usize, per_page: usize) -> Self {
        Self {
            page: Some(page),
            per_page: Some(per_page),
        }
    }

    /// Get offset for query
    pub fn offset(&self) -> usize {
        self.page.unwrap_or(1) * self.per_page.unwrap_or(20) - self.per_page.unwrap_or(20)
    }

    /// Get limit for query
    pub fn limit(&self) -> usize {
        self.per_page.unwrap_or(20).min(100) // Max 100 per page
    }
}

/// Filter parameters for station search
#[derive(Debug, Clone)]
pub struct StationFilter {
    pub partner_id: Option<String>,
    pub status: Option<String>,
    pub station_type: Option<String>,
    pub min_power_kw: Option<i32>,
    pub max_power_kw: Option<i32>,
    pub available_chargers_min: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_with_metadata_creation() {
        let station = Station {
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

        let metadata = StationWithMetadata::from_station(&station, Some("AutoMotive".to_string()));
        assert_eq!(metadata.station.id, "STN-001");
        assert_eq!(metadata.partner_name, Some("AutoMotive".to_string()));
        assert_eq!(metadata.available_chargers_count, 4);
    }

    #[test]
    fn test_station_with_distance() {
        let station = Station {
            id: "STN-001".to_string(),
            name: Some("Test".to_string()),
            address: None,
            latitude: Some(36.8),
            longitude: Some(10.1),
            partner_id: Some("PRT-001".to_string()),
            station_type: Some("Test".to_string()),
            power_kw: Some(100),
            available_chargers: Some(1),
            status: Some("active".to_string()),
            created_at: None,
            updated_at: None,
        };

        let mut metadata = StationWithMetadata::from_station(&station, None);
        metadata = metadata.with_distance(1.5, 1500.0);
        metadata = metadata.with_favorite(true);

        assert_eq!(metadata.distance_km, Some(1.5));
        assert_eq!(metadata.distance_m, Some(1500.0));
        assert!(metadata.is_favorite);
    }

    #[test]
    fn test_format_station_type() {
        let station = Station {
            id: "STN-001".to_string(),
            name: Some("Test".to_string()),
            address: None,
            latitude: Some(36.8),
            longitude: Some(10.1),
            partner_id: Some("PRT-001".to_string()),
            station_type: Some("EV Charging".to_string()),
            power_kw: Some(100),
            available_chargers: Some(1),
            status: Some("active".to_string()),
            created_at: None,
            updated_at: None,
        };

        let metadata = StationWithMetadata::from_station(&station, None);
        assert_eq!(metadata.format_station_type(), "Electric Vehicle Charging");
    }

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(1, 10);
        assert_eq!(pagination.offset(), 0);
        assert_eq!(pagination.limit(), 10);
    }

    #[test]
    fn test_pagination_with_page_2() {
        let pagination = Pagination::new(2, 10);
        assert_eq!(pagination.offset(), 10);
        assert_eq!(pagination.limit(), 10);
    }

    #[test]
    fn test_sort_by_distance() {
        let mut stations = vec![
            StationWithMetadata {
                station: Station {
                    id: "STN-001".to_string(),
                    name: Some("Station 1".to_string()),
                    address: None,
                    latitude: Some(36.8),
                    longitude: Some(10.1),
                    partner_id: Some("PRT-001".to_string()),
                    station_type: Some("Test".to_string()),
                    power_kw: Some(100),
                    available_chargers: Some(1),
                    status: Some("active".to_string()),
                    created_at: None,
                    updated_at: None,
                },
                partner_name: None,
                available_chargers_count: 1,
                distance_km: Some(2.0),
                distance_m: Some(2000.0),
                station_type_display: None,
                power_kw_display: None,
                last_updated: None,
                is_favorite: false,
            },
            StationWithMetadata {
                station: Station {
                    id: "STN-002".to_string(),
                    name: Some("Station 2".to_string()),
                    address: None,
                    latitude: Some(36.9),
                    longitude: Some(10.2),
                    partner_id: Some("PRT-001".to_string()),
                    station_type: Some("Test".to_string()),
                    power_kw: Some(100),
                    available_chargers: Some(1),
                    status: Some("active".to_string()),
                    created_at: None,
                    updated_at: None,
                },
                partner_name: None,
                available_chargers_count: 1,
                distance_km: Some(1.0),
                distance_m: Some(1000.0),
                station_type_display: None,
                power_kw_display: None,
                last_updated: None,
                is_favorite: false,
            },
        ];

        let sorted = sort_by_distance(stations);
        assert_eq!(sorted[0].station.id, "STN-002");
        assert_eq!(sorted[1].station.id, "STN-001");
    }
}
