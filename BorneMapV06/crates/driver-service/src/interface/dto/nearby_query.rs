//! DTOs for nearby stations query

use serde::{Deserialize, Serialize};

/// Request DTO for nearby stations query
#[derive(Debug, Clone, Deserialize)]
pub struct NearbyRequest {
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub radius_km: Option<f64>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub offset: Option<usize>,
}

/// Response DTO for nearby stations query
#[derive(Debug, Clone, Serialize)]
pub struct NearbyResponse {
    pub success: bool,
    pub message: String,
    pub stations: Vec<StationDTO>,
    pub pagination: PaginationDTO,
}

/// Station DTO for API response
#[derive(Debug, Clone, Serialize)]
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
    pub status: String,
    pub distance_km: Option<f64>,
    pub distance_m: Option<f64>,
    pub formatted_address: Option<String>,
    pub formatted_distance: Option<String>,
}

/// Pagination DTO for API response
#[derive(Debug, Clone, Serialize)]
pub struct PaginationDTO {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub total_pages: usize,
    pub has_more: bool,
}

impl StationDTO {
    /// Create from station with metadata
    pub fn from_station_with_metadata(
        station: crate::domain::StationWithMetadata,
    ) -> Self {
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
            status: station.station.status.clone().unwrap_or("unknown".to_string()),
            distance_km: station.distance_km,
            distance_m: station.distance_m,
            formatted_address: None, // Can be populated if address is available
            formatted_distance: None, // Can be populated if distance is available
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
}

impl PaginationDTO {
    /// Create from pagination parameters
    pub fn from_pagination(page: usize, per_page: usize, total: usize) -> Self {
        let total_pages = (total + per_page - 1) / per_page;
        let has_more = page < total_pages;

        Self {
            page,
            per_page,
            total,
            total_pages,
            has_more,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearby_request_creation() {
        let request = NearbyRequest {
            latitude: Some(36.8065),
            longitude: Some(10.1815),
            radius_km: Some(10.0),
            limit: Some(10),
            offset: Some(0),
        };

        assert_eq!(request.latitude, Some(36.8065));
        assert_eq!(request.radius_km, Some(10.0));
    }

    #[test]
    fn test_station_dto_from_metadata() {
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
        metadata = metadata.with_distance(1.5, 1500.0);

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
            status: "active".to_string(),
            distance_km: Some(0.5),
            distance_m: None,
            formatted_address: None,
            formatted_distance: None,
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
            status: "active".to_string(),
            distance_km: Some(5.0),
            distance_m: None,
            formatted_address: None,
            formatted_distance: None,
        };

        assert_eq!(dto.format_distance(), "5 km");
    }

    #[test]
    fn test_pagination_dto_creation() {
        let pagination = PaginationDTO::from_pagination(1, 10, 25);
        assert_eq!(pagination.page, 1);
        assert_eq!(pagination.per_page, 10);
        assert_eq!(pagination.total, 25);
        assert_eq!(pagination.total_pages, 3);
        assert!(pagination.has_more);
    }

    #[test]
    fn test_pagination_dto_last_page() {
        let pagination = PaginationDTO::from_pagination(3, 10, 25);
        assert_eq!(pagination.page, 3);
        assert_eq!(pagination.total_pages, 3);
        assert!(!pagination.has_more);
    }
}
