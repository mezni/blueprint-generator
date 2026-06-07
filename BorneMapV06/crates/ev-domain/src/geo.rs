//! Geographic coordinate validation

use crate::DomainResult;

/// Validate latitude is in valid range [-90, 90]
pub fn validate_latitude(latitude: f64) -> DomainResult<()> {
    if latitude < -90.0 || latitude > 90.0 {
        return Err(crate::DomainError::InvalidCoordinates(
            format!("Latitude must be between -90 and 90, got {}", latitude)
        ));
    }
    Ok(())
}

/// Validate longitude is in valid range [-180, 180]
pub fn validate_longitude(longitude: f64) -> DomainResult<()> {
    if longitude < -180.0 || longitude > 180.0 {
        return Err(crate::DomainError::InvalidCoordinates(
            format!("Longitude must be between -180 and 180, got {}", longitude)
        ));
    }
    Ok(())
}

/// Validate both latitude and longitude are in valid ranges
pub fn validate_coordinates(latitude: f64, longitude: f64) -> DomainResult<()> {
    validate_latitude(latitude)?;
    validate_longitude(longitude)?;
    Ok(())
}

/// Validate latitude is in valid range for Tunisia (approximate)
pub fn validate_latitude_tunisia(latitude: f64) -> DomainResult<()> {
    if latitude < 33.7 || latitude > 37.4 {
        return Err(crate::DomainError::InvalidCoordinates(
            format!("Latitude must be between 33.7 and 37.4 (Tunisia range), got {}", latitude)
        ));
    }
    Ok(())
}

/// Validate longitude is in valid range for Tunisia (approximate)
pub fn validate_longitude_tunisia(longitude: f64) -> DomainResult<()> {
    if longitude < 7.5 || longitude > 11.5 {
        return Err(crate::DomainError::InvalidCoordinates(
            format!("Longitude must be between 7.5 and 11.5 (Tunisia range), got {}", longitude)
        ));
    }
    Ok(())
}

/// Validate both coordinates are in Tunisia's range
pub fn validate_tunisia_coordinates(latitude: f64, longitude: f64) -> DomainResult<()> {
    validate_latitude_tunisia(latitude)?;
    validate_longitude_tunisia(longitude)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_latitude_valid() {
        assert!(validate_latitude(0.0).is_ok());
        assert!(validate_latitude(45.0).is_ok());
        assert!(validate_latitude(90.0).is_ok());
    }

    #[test]
    fn test_validate_latitude_invalid_low() {
        assert!(validate_latitude(-91.0).is_err());
    }

    #[test]
    fn test_validate_latitude_invalid_high() {
        assert!(validate_latitude(91.0).is_err());
    }

    #[test]
    fn test_validate_longitude_valid() {
        assert!(validate_longitude(0.0).is_ok());
        assert!(validate_longitude(180.0).is_ok());
    }

    #[test]
    fn test_validate_longitude_invalid_low() {
        assert!(validate_longitude(-181.0).is_err());
    }

    #[test]
    fn test_validate_longitude_invalid_high() {
        assert!(validate_longitude(181.0).is_err());
    }

    #[test]
    fn test_validate_coordinates_valid() {
        assert!(validate_coordinates(36.8065, 10.1815).is_ok());
    }

    #[test]
    fn test_validate_coordinates_invalid() {
        assert!(validate_coordinates(-91.0, 10.0).is_err());
        assert!(validate_coordinates(36.8065, 181.0).is_err());
    }

    #[test]
    fn test_validate_tunisia_latitude_valid() {
        assert!(validate_latitude_tunisia(35.0).is_ok());
    }

    #[test]
    fn test_validate_tunisia_latitude_invalid() {
        assert!(validate_latitude_tunisia(30.0).is_err());
        assert!(validate_latitude_tunisia(40.0).is_err());
    }

    #[test]
    fn test_validate_tunisia_longitude_valid() {
        assert!(validate_longitude_tunisia(10.0).is_ok());
    }

    #[test]
    fn test_validate_tunisia_longitude_invalid() {
        assert!(validate_longitude_tunisia(5.0).is_err());
        assert!(validate_longitude_tunisia(12.0).is_err());
    }
}
