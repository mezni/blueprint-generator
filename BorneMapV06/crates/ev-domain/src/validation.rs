//! Validation rules for domain models

use crate::DomainError;

/// Validate coordinate bounds (WGS84)
pub fn validate_latitude(lat: f64) -> Result<(), DomainError> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(DomainError::InvalidCoordinates(format!(
            "Invalid latitude: {}, must be between -90 and 90",
            lat
        )));
    }
    Ok(())
}

/// Validate longitude bounds (WGS84)
pub fn validate_longitude(lng: f64) -> Result<(), DomainError> {
    if !(-180.0..=180.0).contains(&lng) {
        return Err(DomainError::InvalidCoordinates(format!(
            "Invalid longitude: {}, must be between -180 and 180",
            lng
        )));
    }
    Ok(())
}

/// Validate coordinates as a pair
pub fn validate_coordinates(lat: f64, lng: f64) -> Result<(), DomainError> {
    validate_latitude(lat)?;
    validate_longitude(lng)?;
    Ok(())
}

/// Validate search radius bounds
pub fn validate_radius(radius_m: i32) -> Result<(), DomainError> {
    if !(100..=50000).contains(&radius_m) {
        return Err(DomainError::ValidationError(format!(
            "Invalid radius: {}, must be between 100 and 50000 meters",
            radius_m
        )));
    }
    Ok(())
}

/// Validate string length with bounds
pub fn validate_string_length(value: &str, min: usize, max: usize) -> Result<(), DomainError> {
    let len = value.len();
    if len < min || len > max {
        return Err(DomainError::ValidationError(format!(
            "String length {} not within bounds [{}, {}]",
            len, min, max
        )));
    }
    Ok(())
}

/// Validate email format (basic)
pub fn validate_email(email: &str) -> Result<(), DomainError> {
    if !email.contains('@') || !email.contains('.') {
        return Err(DomainError::ValidationError(format!(
            "Invalid email format: {}",
            email
        )));
    }
    Ok(())
}

/// Validate rating (1-5 scale)
pub fn validate_rating(rating: i32) -> Result<(), DomainError> {
    if !(1..=5).contains(&rating) {
        return Err(DomainError::ValidationError(format!(
            "Invalid rating: {}, must be between 1 and 5",
            rating
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_coordinates() {
        assert!(validate_coordinates(36.8, 10.1).is_ok());
        assert!(validate_coordinates(0.0, 0.0).is_ok());
        assert!(validate_latitude(91.0).is_err());
        assert!(validate_longitude(181.0).is_err());
    }

    #[test]
    fn test_validate_radius() {
        assert!(validate_radius(5000).is_ok());
        assert!(validate_radius(100).is_ok());
        assert!(validate_radius(50000).is_ok());
        assert!(validate_radius(99).is_err());
        assert!(validate_radius(50001).is_err());
    }

    #[test]
    fn test_validate_rating() {
        assert!(validate_rating(1).is_ok());
        assert!(validate_rating(5).is_ok());
        assert!(validate_rating(3).is_ok());
        assert!(validate_rating(0).is_err());
        assert!(validate_rating(6).is_err());
    }
}
