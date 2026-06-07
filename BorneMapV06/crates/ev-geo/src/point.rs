use serde::{Deserialize, Serialize};

use crate::geo_error::GeoError;

/// A geographic coordinate in WGS84 (EPSG:4326).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LatLng {
    pub latitude: f64,
    pub longitude: f64,
}

impl LatLng {
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, GeoError> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(GeoError::InvalidLatitude(latitude));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(GeoError::InvalidLongitude(longitude));
        }
        Ok(LatLng { latitude, longitude })
    }

    pub fn latitude_radians(&self) -> f64 {
        self.latitude * std::f64::consts::PI / 180.0
    }

    pub fn longitude_radians(&self) -> f64 {
        self.longitude * std::f64::consts::PI / 180.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latlng_valid() {
        let p = LatLng::new(36.806389, 10.181667).unwrap();
        assert!((p.latitude - 36.806389).abs() < 1e-6);
    }

    #[test]
    fn test_latlng_invalid_latitude() {
        assert!(LatLng::new(100.0, 10.0).is_err());
        assert!(LatLng::new(-100.0, 10.0).is_err());
    }

    #[test]
    fn test_latlng_invalid_longitude() {
        assert!(LatLng::new(36.0, 190.0).is_err());
        assert!(LatLng::new(36.0, -190.0).is_err());
    }

    #[test]
    fn test_latlng_boundary() {
        assert!(LatLng::new(90.0, 180.0).is_ok());
        assert!(LatLng::new(-90.0, -180.0).is_ok());
    }
}
