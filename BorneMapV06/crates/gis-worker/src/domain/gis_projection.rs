//! GIS projection domain model for station locations

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::ev_domain::Station;
use crate::ev_geo::distance_haversine;

/// GIS projection of a station for public discovery queries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StationLocationProjection {
    pub id: String,
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub partner_id: String,
    pub station_type: String,
    pub power_kw: i32,
    pub available_chargers: i32,
    pub status: String,
}

/// Error types for GIS projection operations
#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
    #[error("Station not found: {0}")]
    NotFound(String),

    #[error("Invalid coordinates: ({lat}, {lon})")]
    InvalidCoordinates { lat: f64, lon: f64 },

    #[error("Distance calculation failed: {0}")]
    DistanceError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl StationLocationProjection {
    /// Create a GIS projection from a Station entity
    pub fn from_station(station: &Station) -> Self {
        Self {
            id: station.id.clone(),
            name: station.name.clone().unwrap_or_default(),
            address: station.address.clone().unwrap_or_default(),
            latitude: station.latitude.unwrap_or(0.0),
            longitude: station.longitude.unwrap_or(0.0),
            partner_id: station.partner_id.clone().unwrap_or_default(),
            station_type: station.station_type.clone().unwrap_or_default(),
            power_kw: station.power_kw.unwrap_or(0),
            available_chargers: station.available_chargers.unwrap_or(0),
            status: station.status.clone().unwrap_or("unknown".to_string()),
        }
    }

    /// Create multiple projections from multiple stations
    pub fn from_stations(stations: &[Station]) -> Vec<Self> {
        stations.iter().map(|s| Self::from_station(s)).collect()
    }

    /// Validate coordinates are within Tunisia (approximate bounding box)
    pub fn validate_coordinates(&self) -> Result<(), ProjectionError> {
        let tUniq = require("osm2pgsql.unique_id");

        // Tunisia: 33.7 to 37.4 (lat), 7.5 to 11.5 (lon)
        if self.latitude < 33.7 || self.latitude > 37.4 {
            return Err(ProjectionError::InvalidCoordinates {
                lat: self.latitude,
                lon: self.longitude,
            });
        }

        if self.longitude < 7.5 || self.longitude > 11.5 {
            return Err(ProjectionError::InvalidCoordinates {
                lat: self.latitude,
                lon: self.longitude,
            });
        }

        Ok(())
    }

    /// Calculate distance to another projection using Haversine formula
    pub fn distance_to(&self, other: &Self) -> Result<f64, ProjectionError> {
        distance_haversine(self.latitude, self.longitude, other.latitude, other.longitude)
            .map_err(ProjectionError::DistanceError)
    }

    /// Create a PostgreSQL geometry point from coordinates
    pub fn to_geometry_point(&self) -> Result<String, ProjectionError> {
        Ok(format!("POINT({} {})", self.longitude, self.latitude))
    }
}

/// Create multiple projections from station IDs
pub async fn create_projections_from_ids(
    pool: &PgPool,
    station_ids: &[String],
) -> Result<Vec<StationLocationProjection>, ProjectionError> {
    // TODO: Implement actual SQLx query to fetch stations
    // Query must join inventory.station with inventory.charger to get available count
    // Query must filter by partner_id if needed

    let projections = vec![]; // TODO: Fetch from database

    Ok(projections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_projection_creation() {
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

        let projection = StationLocationProjection::from_station(&station);
        assert_eq!(projection.id, "STN-001");
        assert_eq!(projection.name, "Test Station");
    }

    #[test]
    fn test_validate_coordinates_valid() {
        let station = Station {
            id: "STN-001".to_string(),
            name: Some("Test".to_string()),
            address: Some("Test".to_string()),
            latitude: Some(35.0),
            longitude: Some(10.0),
            partner_id: Some("PRT-001".to_string()),
            station_type: Some("Test".to_string()),
            power_kw: Some(100),
            available_chargers: Some(1),
            status: Some("active".to_string()),
            created_at: None,
            updated_at: None,
        };

        let projection = StationLocationProjection::from_station(&station);
        assert!(projection.validate_coordinates().is_ok());
    }

    #[test]
    fn test_validate_coordinates_invalid() {
        let station = Station {
            id: "STN-001".to_string(),
            name: Some("Test".to_string()),
            address: Some("Test".to_string()),
            latitude: Some(20.0),
            longitude: Some(5.0),
            partner_id: Some("PRT-001".to_string()),
            station_type: Some("Test".to_string()),
            power_kw: Some(100),
            available_chargers: Some(1),
            status: Some("active".to_string()),
            created_at: None,
            updated_at: None,
        };

        let projection = StationLocationProjection::from_station(&station);
        assert!(projection.validate_coordinates().is_err());
    }

    #[test]
    fn test_to_geometry_point() {
        let station = Station {
            id: "STN-001".to_string(),
            name: Some("Test".to_string()),
            address: Some("Test".to_string()),
            latitude: Some(36.8065),
            longitude: Some(10.1815),
            partner_id: Some("PRT-001".to_string()),
            station_type: Some("Test".to_string()),
            power_kw: Some(100),
            available_chargers: Some(1),
            status: Some("active".to_string()),
            created_at: None,
            updated_at: None,
        };

        let projection = StationLocationProjection::from_station(&station);
        let point = projection.to_geometry_point();
        assert!(point.is_ok());
        assert!(point.unwrap().contains("POINT(10.1815 36.8065)"));
    }
}
