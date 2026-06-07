use chrono::{DateTime, Utc};
use common_types::AvailabilitySource;
use common_types::StationAvailabilityStatus;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Availability {
    pub station_id: String,
    pub availability_status: StationAvailabilityStatus,
    pub source: AvailabilitySource,
    pub updated_at: DateTime<Utc>,
}
