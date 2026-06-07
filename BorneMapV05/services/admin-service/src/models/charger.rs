use chrono::{DateTime, Utc};
use common_types::ChargerStatus;
use common_types::ChargerType;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Charger {
    pub charger_id: String,
    pub station_id: String,
    pub charger_type: ChargerType,
    pub power_kw: f64,
    pub status: ChargerStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerCreate {
    pub station_id: String,
    pub charger_type: ChargerType,
    pub power_kw: f64,
    pub status: ChargerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerUpdate {
    pub charger_type: Option<ChargerType>,
    pub power_kw: Option<f64>,
    pub status: Option<ChargerStatus>,
}
