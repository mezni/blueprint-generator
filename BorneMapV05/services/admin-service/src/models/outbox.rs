use chrono::{DateTime, Utc};
use common_types::GisQueueStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GisQueueEntry {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: String,
    pub status: GisQueueStatus,
    pub created_at: DateTime<Utc>,
}
