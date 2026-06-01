use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::rbac::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationDTO {
    pub id: String,
    pub name: String,
    pub partner_id: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub charger_count: u32,
    pub status: StationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StationStatus {
    Active,
    Inactive,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerDTO {
    pub id: String,
    pub name: String,
    pub contact_email: String,
    pub status: PartnerStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PartnerStatus {
    Active,
    Suspended,
    Onboarding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewDTO {
    pub id: String,
    pub station_id: String,
    pub user_id: String,
    pub rating: u8,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}
