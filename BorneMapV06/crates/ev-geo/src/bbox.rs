use crate::geo_error::GeoError;
use crate::point::LatLng;
use serde::{Deserialize, Serialize};

/// A bounding box defined by minimum and maximum coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub min_lng: f64,
    pub max_lat: f64,
    pub max_lng: f64,
}

impl BoundingBox {
    pub fn new(min_lat: f64, min_lng: f64, max_lat: f64, max_lng: f64) -> Result<Self, GeoError> {
        if !(-90.0..=90.0).contains(&min_lat) || !(-90.0..=90.0).contains(&max_lat) {
            return Err(GeoError::InvalidBbox(
                "latitude values must be between -90 and 90".to_string(),
            ));
        }
        if !(-180.0..=180.0).contains(&min_lng) || !(-180.0..=180.0).contains(&max_lng) {
            return Err(GeoError::InvalidBbox(
                "longitude values must be between -180 and 180".to_string(),
            ));
        }
        if min_lat > max_lat {
            return Err(GeoError::InvalidBbox(
                "min_lat must be less than or equal to max_lat".to_string(),
            ));
        }
        if min_lng > max_lng {
            return Err(GeoError::InvalidBbox(
                "min_lng must be less than or equal to max_lng".to_string(),
            ));
        }
        Ok(BoundingBox {
            min_lat,
            min_lng,
            max_lat,
            max_lng,
        })
    }

    /// Check if a point is within this bounding box
    pub fn contains(&self, point: &LatLng) -> bool {
        (self.min_lat..=self.max_lat).contains(&point.latitude)
            && (self.min_lng..=self.max_lng).contains(&point.longitude)
    }

    /// Parse a bbox string in the format "min_lat,min_lng,max_lat,max_lng"
    pub fn from_str(s: &str) -> Result<Self, GeoError> {
        let parts: Vec<f64> = s
            .split(',')
            .map(|p| p.trim().parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| GeoError::InvalidBbox("failed to parse coordinates".to_string()))?;

        if parts.len() != 4 {
            return Err(GeoError::InvalidBbox(
                "bbox must have exactly 4 comma-separated values".to_string(),
            ));
        }

        BoundingBox::new(parts[0], parts[1], parts[2], parts[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::LatLng;

    #[test]
    fn test_bbox_valid() {
        let bbox = BoundingBox::new(33.0, 8.0, 37.0, 12.0).unwrap();
        assert!((bbox.min_lat - 33.0).abs() < 1e-6);
    }

    #[test]
    fn test_bbox_invalid_order() {
        assert!(BoundingBox::new(37.0, 8.0, 33.0, 12.0).is_err());
    }

    #[test]
    fn test_bbox_contains() {
        let bbox = BoundingBox::new(33.0, 8.0, 37.0, 12.0).unwrap();
        let tunis = LatLng::new(36.806389, 10.181667).unwrap();
        assert!(bbox.contains(&tunis));
        let sydney = LatLng::new(-33.86, 151.21).unwrap();
        assert!(!bbox.contains(&sydney));
    }

    #[test]
    fn test_bbox_from_str() {
        let bbox = BoundingBox::from_str("33.0,8.0,37.0,12.0").unwrap();
        assert!((bbox.min_lat - 33.0).abs() < 1e-6);
    }

    #[test]
    fn test_bbox_from_str_invalid() {
        assert!(BoundingBox::from_str("invalid").is_err());
    }
}
