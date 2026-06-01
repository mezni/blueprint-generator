use nanoid::nanoid;

const ID_LENGTH: usize = 21;

const ID_ALPHABET: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '-', '_',
];

pub const PREFIX_USER: &str = "USR";
pub const PREFIX_PARTNER: &str = "PRT";
pub const PREFIX_STATION: &str = "STN";
pub const PREFIX_CHARGER: &str = "CHG";
pub const PREFIX_REVIEW: &str = "REV";

pub fn generate_id(prefix: &str) -> String {
    format!("{}-{}", prefix, nanoid!(ID_LENGTH, &ID_ALPHABET))
}

pub fn generate_user_id() -> String {
    generate_id(PREFIX_USER)
}

pub fn generate_partner_id() -> String {
    generate_id(PREFIX_PARTNER)
}

pub fn generate_station_id() -> String {
    generate_id(PREFIX_STATION)
}

pub fn generate_charger_id() -> String {
    generate_id(PREFIX_CHARGER)
}

pub fn generate_review_id() -> String {
    generate_id(PREFIX_REVIEW)
}

pub fn validate_id(id: &str, expected_prefix: &str) -> bool {
    let re = regex_lazy(expected_prefix);
    re.is_match(id)
}

fn regex_lazy(prefix: &str) -> regex::Regex {
    regex::Regex::new(&format!("^{}-[A-Za-z0-9_-]{{21}}$", prefix)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_user_id_format() {
        let id = generate_user_id();
        assert!(id.starts_with("USR-"));
        assert_eq!(id.len(), 25);
    }

    #[test]
    fn generate_partner_id_format() {
        let id = generate_partner_id();
        assert!(id.starts_with("PRT-"));
        assert_eq!(id.len(), 25);
    }

    #[test]
    fn generate_station_id_format() {
        let id = generate_station_id();
        assert!(id.starts_with("STN-"));
        assert_eq!(id.len(), 25);
    }

    #[test]
    fn generate_charger_id_format() {
        let id = generate_charger_id();
        assert!(id.starts_with("CHG-"));
        assert_eq!(id.len(), 25);
    }

    #[test]
    fn generate_review_id_format() {
        let id = generate_review_id();
        assert!(id.starts_with("REV-"));
        assert_eq!(id.len(), 25);
    }

    #[test]
    fn validate_id_accepts_valid() {
        let id = generate_user_id();
        assert!(validate_id(&id, PREFIX_USER));
    }

    #[test]
    fn validate_id_rejects_wrong_prefix() {
        let id = generate_user_id();
        assert!(!validate_id(&id, PREFIX_PARTNER));
    }

    #[test]
    fn validate_id_rejects_short() {
        assert!(!validate_id("USR-abc", PREFIX_USER));
    }

    #[test]
    fn generate_id_uniqueness() {
        let ids: std::collections::HashSet<String> = (0..1000)
            .map(|_| generate_station_id())
            .collect();
        assert_eq!(ids.len(), 1000);
    }
}
