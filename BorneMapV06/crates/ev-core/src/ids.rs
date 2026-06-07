use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fmt;

const ID_LENGTH: usize = 16;
const ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Identifier prefixes for each entity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityPrefix {
    Station,
    Charger,
    Partner,
    User,
    Review,
    Event,
}

impl EntityPrefix {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityPrefix::Station => "STN",
            EntityPrefix::Charger => "CHG",
            EntityPrefix::Partner => "PRT",
            EntityPrefix::User => "USR",
            EntityPrefix::Review => "REV",
            EntityPrefix::Event => "EVT",
        }
    }
}

impl fmt::Display for EntityPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Generate a prefixed NanoID for an entity
///
/// # Example
/// ```
/// use ev_core::ids::{EntityPrefix, generate_id};
/// let id = generate_id(EntityPrefix::Station);
/// assert!(id.starts_with("STN-"));
/// assert_eq!(id.len(), 20); // "STN-" (4) + 16 chars = 20
/// ```
pub fn generate_id(prefix: EntityPrefix) -> String {
    let random_part = nanoid!(ID_LENGTH, &ALPHABET.chars().collect::<Vec<_>>());
    format!("{}-{}", prefix.as_str(), random_part)
}

/// Check if a string is a valid prefixed NanoID
pub fn is_valid_id(id: &str, prefix: EntityPrefix) -> bool {
    let expected_prefix = format!("{}-", prefix.as_str());
    if !id.starts_with(&expected_prefix) {
        return false;
    }
    let suffix = &id[expected_prefix.len()..];
    suffix.len() == ID_LENGTH && suffix.chars().all(|c| ALPHABET.contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_station_id() {
        let id = generate_id(EntityPrefix::Station);
        assert!(id.starts_with("STN-"));
        assert_eq!(id.len(), 20);
    }

    #[test]
    fn test_generate_charger_id() {
        let id = generate_id(EntityPrefix::Charger);
        assert!(id.starts_with("CHG-"));
        assert_eq!(id.len(), 20);
    }

    #[test]
    fn test_generate_partner_id() {
        let id = generate_id(EntityPrefix::Partner);
        assert!(id.starts_with("PRT-"));
        assert_eq!(id.len(), 20);
    }

    #[test]
    fn test_generate_user_id() {
        let id = generate_id(EntityPrefix::User);
        assert!(id.starts_with("USR-"));
        assert_eq!(id.len(), 20);
    }

    #[test]
    fn test_generate_review_id() {
        let id = generate_id(EntityPrefix::Review);
        assert!(id.starts_with("REV-"));
        assert_eq!(id.len(), 20);
    }

    #[test]
    fn test_generate_event_id() {
        let id = generate_id(EntityPrefix::Event);
        assert!(id.starts_with("EVT-"));
        assert_eq!(id.len(), 20);
    }

    #[test]
    fn test_is_valid_id() {
        let id = generate_id(EntityPrefix::Station);
        assert!(is_valid_id(&id, EntityPrefix::Station));
        assert!(!is_valid_id(&id, EntityPrefix::Charger));
        assert!(!is_valid_id("invalid", EntityPrefix::Station));
    }

    #[test]
    fn test_uniqueness() {
        let ids: std::collections::HashSet<String> =
            (0..1000).map(|_| generate_id(EntityPrefix::Station)).collect();
        assert_eq!(ids.len(), 1000);
    }

    #[test]
    fn test_entity_prefix_display() {
        assert_eq!(EntityPrefix::Station.to_string(), "STN");
        assert_eq!(EntityPrefix::Charger.to_string(), "CHG");
    }
}
