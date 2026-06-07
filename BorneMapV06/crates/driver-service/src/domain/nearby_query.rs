//! Nearby query domain model

use std::collections::HashMap;

use crate::ev_geo::distance::haversine_distance;
use crate::ev_geo::point::LatLng;
use crate::DomainResult;

/// Nearby query parameters
#[derive(Debug, Clone)]
pub struct NearbyQuery {
    pub latitude: f64,
    pub longitude: f64,
    pub radius_km: f64,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub filter: Option<StationFilter>,
}

/// Nearby query result
#[derive(Debug, Clone)]
pub struct NearbyQueryResult {
    pub stations: Vec<StationWithDistance>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

/// Station with distance to query point
#[derive(Debug, Clone)]
pub struct StationWithDistance {
    pub station: crate::ev_domain::Station,
    pub distance_km: f64,
    pub distance_m: f64,
}

impl NearbyQuery {
    /// Create a new nearby query
    pub fn new(latitude: f64, longitude: f64, radius_km: f64) -> DomainResult<Self> {
        validate_latitude(latitude)?;
        validate_longitude(longitude)?;
        validate_radius(radius_km)?;

        Ok(Self {
            latitude,
            longitude,
            radius_km,
            limit: None,
            offset: None,
            filter: None,
        })
    }

    /// Set the limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set filter
    pub fn with_filter(mut self, filter: crate::ev_domain::StationFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Validate coordinates are within Tunisia (optional)
    pub fn validate_tunisia(&self) -> DomainResult<()> {
        // Optional: Can be enabled for Tunisia-specific validation
        // For now, skip strict Tunisia validation to allow worldwide use
        Ok(())
    }

    /// Get query point as LatLng
    pub fn to_latlng(&self) -> LatLng {
        LatLng::new(self.latitude, self.longitude).unwrap()
    }
}

impl NearbyQueryResult {
    /// Calculate statistics for the result
    pub fn statistics(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();

        if let Some(stations) = self.stations.first() {
            stats.insert("closest_distance_km".to_string(), stations.distance_km);
        }

        if let Some(last) = self.stations.last() {
            stats.insert("farthest_distance_km".to_string(), last.distance_km);
        }

        stats.insert("total_stations".to_string(), self.total as f64);
        stats.insert("page_size".to_string(), self.limit as f64);
        stats.insert("page_offset".to_string(), self.offset as f64);

        stats
    }
}

/// Helper function to calculate distance to a station
pub fn calculate_station_distance(
    station: &crate::ev_domain::Station,
    query_latitude: f64,
    query_longitude: f64,
) -> DomainResult<StationWithDistance> {
    let latlng = LatLng::new(query_latitude, query_longitude)?;

    let distance_m = haversine_distance(&latlng, &station.to_latlng()?);
    let distance_km = distance_m / 1000.0;

    Ok(StationWithDistance {
        station: station.clone(),
        distance_km,
        distance_m,
    })
}

/// Validation error messages
const ERR_LATITUDE_OUT_OF_RANGE: &str = "Latitude must be between -90 and 90 degrees";
const ERR_LONGITUDE_OUT_OF_RANGE: &str = "Longitude must be between -180 and 180 degrees";
const ERR_RADIUS_TOO_SMALL: &str = "Radius must be at least 100 meters";
const ERR_RADIUS_TOO_LARGE: &str = "Radius must be at most 50000 meters (50km)";

fn validate_latitude(latitude: f64) -> DomainResult<()> {
    if latitude < -90.0 || latitude > 90.0 {
        return Err(crate::DomainError::InvalidCoordinates(
            ERR_LATITUDE_OUT_OF_RANGE.to_string(),
        ));
    }
    Ok(())
}

fn validate_longitude(longitude: f64) -> DomainResult<()> {
    if longitude < -180.0 || longitude > 180.0 {
        return Err(crate::DomainError::InvalidCoordinates(
            ERR_LONGITUDE_OUT_OF_RANGE.to_string(),
        ));
    }
    Ok(())
}

fn validate_radius(radius_km: f64) -> DomainResult<()> {
    let radius_m = radius_km * 1000.0;
    if radius_m < 100.0 {
        return Err(crate::DomainError::BusinessRuleViolation(
            ERR_RADIUS_TOO_SMALL.to_string(),
        ));
    }
    if radius_m > 50_000.0 {
        return Err(crate::DomainError::BusinessRuleViolation(
            ERR_RADIUS_TOO_LARGE.to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearby_query_creation_valid() {
        let query = NearbyQuery::new(36.8065, 10.1815, 10.0);
        assert!(query.is_ok());
    }

    #[test]
    fn test_nearby_query_creation_invalid_latitude() {
        let query = NearbyQuery::new(-91.0, 10.0, 10.0);
        assert!(query.is_err());
    }

    #[test]
    fn test_nearby_query_creation_invalid_longitude() {
        let query = NearbyQuery::new(36.0, 181.0, 10.0);
        assert!(query.is_err());
    }

    #[test]
    fn test_nearby_query_creation_invalid_radius_small() {
        let query = NearbyQuery::new(36.0, 10.0, 0.05); // 50m is too small
        assert!(query.is_err());
    }

    #[test]
    fn test_nearby_query_creation_invalid_radius_large() {
        let query = NearbyQuery::new(36.0, 10.0, 60.0); // 60km is too large
        assert!(query.is_err());
    }

    #[test]
    fn test_nearby_query_with_limit() {
        let query = NearbyQuery::new(36.8065, 10.1815, 10.0)
            .unwrap()
            .with_limit(10);
        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn test_to_latlng() {
        let query = NearbyQuery::new(36.8065, 10.1815, 10.0).unwrap();
        let latlng = query.to_latlng();
        assert_eq!(latlng.latitude, 36.8065);
        assert_eq!(latlng.longitude, 10.1815);
    }

    #[test]
    fn test_nearby_query_result_statistics() {
        let result = NearbyQueryResult {
            stations: vec![
                StationWithDistance {
                    station: crate::ev_domain::Station {
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
                    distance_km: 0.5,
                    distance_m: 500.0,
                },
                StationWithDistance {
                    station: crate::ev_domain::Station {
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
                    distance_km: 1.5,
                    distance_m: 1500.0,
                },
            ],
            total: 2,
            limit: 10,
            offset: 0,
        };

        let stats = result.statistics();
        assert_eq!(stats.get("total_stations"), Some(&2.0));
        assert_eq!(stats.get("page_size"), Some(&10.0));
        assert_eq!(stats.get("page_offset"), Some(&0.0));
        assert_eq!(stats.get("closest_distance_km"), Some(&0.5));
        assert_eq!(stats.get("farthest_distance_km"), Some(&1.5));
    }
}
