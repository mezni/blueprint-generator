use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct FavoriteStation {
    pub user_id: String,
    pub station_id: String,
    pub created_at: DateTime<Utc>,
}
