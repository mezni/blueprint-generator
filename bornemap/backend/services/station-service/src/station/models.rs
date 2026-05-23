use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BboxQuery {
    pub west: f64,
    pub south: f64,
    pub east: f64,
    pub north: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StationMarker {
    pub id: String,
    pub name: String,
    pub coord: [f64; 2],
    pub is_active: bool,
    pub under_maintenance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StationDetail {
    pub id: String,
    pub company: Company,
    pub name: String,
    pub address: String,
    pub coord: [f64; 2],
    pub is_active: bool,
    pub under_maintenance: bool,
    pub opening_hours_osm: Option<String>,
    pub chargers: Vec<Charger>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Company {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Charger {
    pub id: String,
    pub connector: String,
    pub power_kw: f64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StationListResponse {
    pub viewport: BboxQuery,
    pub markers: Vec<StationMarker>,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AdminStationListResponse {
    pub items: Vec<StationDetail>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AdminStationCreate {
    pub company_id: String,
    pub name: String,
    pub address: String,
    pub coord: [f64; 2],
    #[serde(default = "default_true")]
    pub is_active: bool,
    #[serde(default)]
    pub under_maintenance: bool,
    pub opening_hours_osm: Option<String>,
    pub chargers: Option<Vec<ChargerInput>>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ChargerInput {
    pub connector: String,
    pub power_kw: f64,
    #[serde(default = "default_true")]
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AdminStationPatch {
    pub name: Option<String>,
    pub address: Option<String>,
    pub coord: Option<[f64; 2]>,
    pub is_active: Option<bool>,
    pub under_maintenance: Option<bool>,
    pub opening_hours_osm: Option<String>,
}

fn default_true() -> bool {
    true
}
