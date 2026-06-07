use chrono::{DateTime, Utc};
use common_types::PartnerStatus;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Partner {
    pub partner_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: PartnerStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
