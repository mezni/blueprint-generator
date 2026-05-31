use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartnerSnapshot {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub partner_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Charger {
    pub id: String,
    pub plug_type: String,
    pub power_output: i32,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub partner: PartnerSnapshot,
    pub latitude: f64,
    pub longitude: f64,
    pub status: String,
    pub chargers: Vec<Charger>,
    pub is_live: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StationRow {
    pub id: String,
    pub name: String,
    pub partner_id: String,
    pub partner_name: String,
    pub partner_type: String,
    pub latitude: f64,
    pub longitude: f64,
    pub status: String,
    pub is_live: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ChargerRow {
    pub id: String,
    pub station_id: String,
    pub plug_type: String,
    pub power_output: i32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct NearbyQuery {
    pub lat: f64,
    pub lng: f64,
    pub distance: Option<f64>,
    pub show_staged: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct StatusUpdate {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub filters: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StatusUpdateResponse {
    pub id: String,
    pub status: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
}
