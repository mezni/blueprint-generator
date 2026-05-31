use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event_id: String,
    pub client_platform: String,
    pub app_version: String,
    pub connected_at: String,
}
