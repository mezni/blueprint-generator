use serde::{Deserialize, Serialize};

/// Types of EV charging connectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectorType {
    #[serde(rename = "CCS2")]
    Ccs2,
    #[serde(rename = "Type2")]
    Type2,
    #[serde(rename = "TeslaSupercharger")]
    TeslaSupercharger,
    #[serde(rename = "CHAdeMO")]
    Chademo,
    #[serde(rename = "Type1")]
    Type1,
}

impl ConnectorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectorType::Ccs2 => "CCS2",
            ConnectorType::Type2 => "Type2",
            ConnectorType::TeslaSupercharger => "TeslaSupercharger",
            ConnectorType::Chademo => "CHAdeMO",
            ConnectorType::Type1 => "Type1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "CCS2" => Some(ConnectorType::Ccs2),
            "Type2" => Some(ConnectorType::Type2),
            "TeslaSupercharger" => Some(ConnectorType::TeslaSupercharger),
            "CHAdeMO" => Some(ConnectorType::Chademo),
            "Type1" => Some(ConnectorType::Type1),
            _ => None,
        }
    }
}

/// Operational status of a charger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChargerStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "in_use")]
    InUse,
    #[serde(rename = "maintenance")]
    Maintenance,
    #[serde(rename = "offline")]
    Offline,
}

impl ChargerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChargerStatus::Available => "available",
            ChargerStatus::InUse => "in_use",
            ChargerStatus::Maintenance => "maintenance",
            ChargerStatus::Offline => "offline",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "available" => Some(ChargerStatus::Available),
            "in_use" => Some(ChargerStatus::InUse),
            "maintenance" => Some(ChargerStatus::Maintenance),
            "offline" => Some(ChargerStatus::Offline),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connector_type_as_str() {
        assert_eq!(ConnectorType::Ccs2.as_str(), "CCS2");
        assert_eq!(ConnectorType::Type2.as_str(), "Type2");
        assert_eq!(ConnectorType::TeslaSupercharger.as_str(), "TeslaSupercharger");
    }

    #[test]
    fn test_connector_type_from_str() {
        assert_eq!(ConnectorType::from_str("CCS2"), Some(ConnectorType::Ccs2));
        assert_eq!(ConnectorType::from_str("invalid"), None);
    }

    #[test]
    fn test_charger_status_as_str() {
        assert_eq!(ChargerStatus::Available.as_str(), "available");
        assert_eq!(ChargerStatus::InUse.as_str(), "in_use");
        assert_eq!(ChargerStatus::Maintenance.as_str(), "maintenance");
        assert_eq!(ChargerStatus::Offline.as_str(), "offline");
    }

    #[test]
    fn test_charger_status_from_str() {
        assert_eq!(ChargerStatus::from_str("available"), Some(ChargerStatus::Available));
        assert_eq!(ChargerStatus::from_str("in_use"), Some(ChargerStatus::InUse));
        assert_eq!(ChargerStatus::from_str("invalid"), None);
    }

    #[test]
    fn test_connector_type_serde() {
        let json = serde_json::to_string(&ConnectorType::Ccs2).unwrap();
        assert_eq!(json, "\"CCS2\"");
        let deserialized: ConnectorType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ConnectorType::Ccs2);
    }

    #[test]
    fn test_charger_status_serde() {
        let json = serde_json::to_string(&ChargerStatus::InUse).unwrap();
        assert_eq!(json, "\"in_use\"");
        let deserialized: ChargerStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ChargerStatus::InUse);
    }
}
