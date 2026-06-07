//! Domain validation rules for driver-service

use crate::DomainResult;

/// Validate latitude is in valid range [-90, 90]
pub fn validate_latitude(latitude: f64) -> DomainResult<()> {
    if latitude < -90.0 || latitude > 90.0 {
        return Err(crate::DomainError::InvalidCoordinates(
            "Latitude must be between -90 and 90 degrees".to_string(),
        ));
    }
    Ok(())
}

/// Validate longitude is in valid range [-180, 180]
pub fn validate_longitude(longitude: f64) -> DomainResult<()> {
    if longitude < -180.0 || longitude > 180.0 {
        return Err(crate::DomainError::InvalidCoordinates(
            "Longitude must be between -180 and 180 degrees".to_string(),
        ));
    }
    Ok(())
}

/// Validate radius is in valid range [100, 50000] meters
pub fn validate_radius(radius_meters: f64) -> DomainResult<()> {
    if radius_meters < 100.0 {
        return Err(crate::DomainError::BusinessRuleViolation(
            "Radius must be at least 100 meters".to_string(),
        ));
    }
    if radius_meters > 50_000.0 {
        return Err(crate::DomainError::BusinessRuleViolation(
            "Radius must be at most 50000 meters (50km)".to_string(),
        ));
    }
    Ok(())
}

/// Validate coordinates and radius
pub fn validate_query(latitude: f64, longitude: f64, radius_km: f64) -> DomainResult<()> {
    validate_latitude(latitude)?;
    validate_longitude(longitude)?;
    validate_radius(radius_km * 1000.0)?;
    Ok(())
}

/// Validate coordinate pair for Tunisia (optional)
pub fn validate_tunisia_coordinates(latitude: f64, longitude: f64) -> DomainResult<()> {
    // Tunisia: 33.7 to 37.4 (lat), 7.5 to 11.5 (lon)
    if latitude < 33.7 || latitude > 37.4 {
        return Err(crate::DomainError::InvalidCoordinates(
            "Latitude must be between 33.7 and 37.4 for Tunisia".to_string(),
        ));
    }
    if longitude < 7.5 || longitude > 11.5 {
        return Err(crate::DomainError::InvalidCoordinates(
            "Longitude must be between 7.5 and 11.5 for Tunisia".to_string(),
        ));
    }
    Ok(())
}

/// Validate page number is >= 1
pub fn validate_page(page: usize) -> DomainResult<()> {
    if page < 1 {
        return Err(crate::DomainError::BusinessRuleViolation(
            "Page number must be >= 1".to_string(),
        ));
    }
    Ok(())
}

/// Validate per_page is in valid range [1, 100]
pub fn validate_per_page(per_page: usize) -> DomainResult<()> {
    if per_page < 1 {
        return Err(crate::DomainError::BusinessRuleViolation(
            "Per page must be >= 1".to_string(),
        ));
    }
    if per_page > 100 {
        return Err(crate::DomainError::BusinessRuleViolation(
            "Per page must be <= 100".to_string(),
        ));
    }
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
    fn test_validate_latitude_invalid() {
        assert!(validate_latitude(-91.0).is_err());
        assert!(validate_latitude(91.0).is_err());
    }

    #[test]
    fn test_validate_longitude_valid() {
        assert!(validate_longitude(0.0).is_ok());
        assert!(validate_longitude(180.0).is_ok());
    }

    #[test]
    fn test_validate_longitude_invalid() {
        assert!(validate_longitude(-181.0).is_err());
        assert!(validate_longitude(181.0).is_err());
    }

    #[test]
    fn test_validate_radius_valid() {
        assert!(validate_radius(100.0).is_ok()); // 100m min
        assert!(validate_radius(5000.0).is_ok());
        assert!(validate_radius(50000.0).is_ok()); // 50km max
    }

    #[test]
    fn test_validate_radius_invalid() {
        assert!(validate_radius(99.0).is_err()); // Too small
        assert!(validate_radius(50001.0).is_err()); // Too large
    }

    #[test]
    fn test_validate_query_valid() {
        assert!(validate_query(36.8065, 10.1815, 10.0).is_ok());
    }

    #[test]
    fn test_validate_query_invalid_latitude() {
        assert!(validate_query(-91.0, 10.0, 10.0).is_err());
    }

    #[test]
    fn test_validate_query_invalid_radius() {
        assert!(validate_query(36.0, 10.0, 60.0).is_err()); // 60km is too large
    }

    #[test]
    fn test_validate_page_valid() {
        assert!(validate_page(1).is_ok());
        assert!(validate_page(10).is_ok());
    }

    #[test]
    fn test_validate_page_invalid() {
        assert!(validate_page(0).is_err());
    }

    #[test]
    fn test_validate_per_page_valid() {
        assert!(validate_per_page(1).is_ok());
        assert!(validate_per_page(50).is_ok());
        assert!(validate_per_page(100).is_ok());
    }

    #[test]
    fn test_validate_per_page_invalid() {
        assert!(validate_per_page(0).is_err());
        assert!(validate_per_page(101).is_err());
    }
}
