/// Errors related to geographic operations
#[derive(Debug, Clone, PartialEq)]
pub enum GeoError {
    InvalidLatitude(f64),
    InvalidLongitude(f64),
    InvalidBbox(String),
}

impl std::fmt::Display for GeoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoError::InvalidLatitude(lat) => {
                write!(f, "invalid latitude: {} (must be -90 to 90)", lat)
            }
            GeoError::InvalidLongitude(lng) => {
                write!(f, "invalid longitude: {} (must be -180 to 180)", lng)
            }
            GeoError::InvalidBbox(msg) => write!(f, "invalid bounding box: {}", msg),
        }
    }
}

impl std::error::Error for GeoError {}
