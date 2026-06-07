//! NanoID-based identifiers for all entities
//!
//! All entities use 16-character prefixed NanoIDs:
//! - STN-* for Stations
//! - CHG-* for Chargers
//! - PRT-* for Partners
//! - USR-* for Users
//! - FAV-* for Favorites
//! - REV-* for Reviews

use nanoid::nanoid;

const NANOID_SIZE: usize = 13; // 16 total with 3-char prefix

/// Generate a new Station ID (STN-*)
pub fn generate_station_id() -> String {
    format!("STN-{}", nanoid!(NANOID_SIZE))
}

/// Generate a new Charger ID (CHG-*)
pub fn generate_charger_id() -> String {
    format!("CHG-{}", nanoid!(NANOID_SIZE))
}

/// Generate a new Partner ID (PRT-*)
pub fn generate_partner_id() -> String {
    format!("PRT-{}", nanoid!(NANOID_SIZE))
}

/// Generate a new User ID (USR-*)
pub fn generate_user_id() -> String {
    format!("USR-{}", nanoid!(NANOID_SIZE))
}

/// Generate a new Favorite ID (FAV-*)
pub fn generate_favorite_id() -> String {
    format!("FAV-{}", nanoid!(NANOID_SIZE))
}

/// Generate a new Review ID (REV-*)
pub fn generate_review_id() -> String {
    format!("REV-{}", nanoid!(NANOID_SIZE))
}

/// Validate NanoID format with prefix
pub fn validate_id(id: &str, expected_prefix: &str) -> bool {
    if !id.starts_with(expected_prefix) || !id.starts_with(&format!("{}-", expected_prefix)) {
        return false;
    }

    let parts: Vec<&str> = id.split('-').collect();
    if parts.len() != 2 {
        return false;
    }

    parts[1].len() == NANOID_SIZE && parts[1].chars().all(|c| c.is_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_id_generation() {
        let id = generate_station_id();
        assert!(id.starts_with("STN-"));
        assert_eq!(id.len(), 17); // 4 prefix + 13 nanoid
        assert!(validate_id(&id, "STN"));
    }

    #[test]
    fn test_validate_id() {
        let station_id = generate_station_id();
        assert!(validate_id(&station_id, "STN"));
        assert!(!validate_id(&station_id, "CHG"));

        assert!(!validate_id("INVALID", "STN"));
        assert!(!validate_id("STN-", "STN"));
    }
}
