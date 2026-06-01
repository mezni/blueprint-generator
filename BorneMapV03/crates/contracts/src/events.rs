use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    StationSearched,
    StationViewed,
    ChargingStarted,
    ChargingCompleted,
    ReviewSubmitted,
    PartnerStationCreated,
    PartnerStationUpdated,
    UserRegistered,
    ErrorOccurred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickstreamEventEnvelope {
    pub event_id: String,
    pub event_type: EventType,
    pub user_id: Option<String>,
    pub session_id: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub trace_id: String,
}
