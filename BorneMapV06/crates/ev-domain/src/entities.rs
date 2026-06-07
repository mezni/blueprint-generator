//! Domain entities for the EV charging platform

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Station — A charging station location managed by a Partner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: String,                           // STN-*
    pub partner_id: String,                   // PRT-*
    pub name: String,
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub availability_status: AvailabilityStatus,
    pub capacity: Option<i32>,
    pub osm_node_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Charger — A charging port at a Station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charger {
    pub id: String,                      // CHG-*
    pub station_id: String,              // STN-*
    pub connector_type: ConnectorType,
    pub power_kw: f64,
    pub status: ChargerStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Partner — Business entity that owns Stations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partner {
    pub id: String,                      // PRT-*
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub country: String,
    pub status: PartnerStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// User — Driver or Partner user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,                      // USR-*
    pub keycloak_id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: UserRole,
    pub partner_id: Option<String>,      // PRT-* (non-null only for partner role)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Favorite — User's saved Station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorite {
    pub id: String,                      // FAV-*
    pub user_id: String,                 // USR-*
    pub station_id: String,              // STN-*
    pub created_at: DateTime<Utc>,
}

/// Review — User's rating and comment on a Station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: String,                      // REV-*
    pub user_id: String,                 // USR-*
    pub station_id: String,              // STN-*
    pub rating: i32,                     // 1-5
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// ============================================================================
// Value Objects & Enums
// ============================================================================

/// Charging connector type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorType {
    #[serde(rename = "chademo")]
    ChadeMO,
    #[serde(rename = "type2")]
    Type2,
    #[serde(rename = "tesla_us")]
    TeslaUS,
    #[serde(rename = "gb_t")]
    GBT,
}

impl std::fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectorType::ChadeMO => write!(f, "ChadeMO"),
            ConnectorType::Type2 => write!(f, "Type 2"),
            ConnectorType::TeslaUS => write!(f, "Tesla US"),
            ConnectorType::GBT => write!(f, "GB/T"),
        }
    }
}

/// Charger operational status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

impl std::fmt::Display for ChargerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargerStatus::Available => write!(f, "Available"),
            ChargerStatus::InUse => write!(f, "In Use"),
            ChargerStatus::Maintenance => write!(f, "Maintenance"),
            ChargerStatus::Offline => write!(f, "Offline"),
        }
    }
}

/// Station availability status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for AvailabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvailabilityStatus::Available => write!(f, "Available"),
            AvailabilityStatus::Unavailable => write!(f, "Unavailable"),
            AvailabilityStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Partner account status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnerStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
}

impl std::fmt::Display for PartnerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartnerStatus::Active => write!(f, "Active"),
            PartnerStatus::Inactive => write!(f, "Inactive"),
            PartnerStatus::Suspended => write!(f, "Suspended"),
        }
    }
}

/// User role (strict set from constitution)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[serde(rename = "registered_driver")]
    RegisteredDriver,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "admin")]
    Admin,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::RegisteredDriver => write!(f, "Registered Driver"),
            UserRole::Partner => write!(f, "Partner"),
            UserRole::Admin => write!(f, "Admin"),
        }
    }
}
