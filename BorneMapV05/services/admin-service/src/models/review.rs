use chrono::{DateTime, Utc};
use common_types::ReviewStatus;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub review_id: String,
    pub station_id: String,
    pub user_id: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub status: ReviewStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerateReviewRequest {
    pub status: ReviewStatus,
}

pub fn validate_review_transition(from: ReviewStatus, to: ReviewStatus) -> bool {
    match (from, to) {
        (ReviewStatus::Published, ReviewStatus::Hidden) => true,
        (ReviewStatus::Published, ReviewStatus::Flagged) => true,
        (ReviewStatus::Flagged, ReviewStatus::Hidden) => true,
        (ReviewStatus::Flagged, ReviewStatus::Published) => true,
        (ReviewStatus::Hidden, ReviewStatus::Published) => true,
        (_, ReviewStatus::Deleted) => true,
        (f, t) if f == t => true,
        _ => false,
    }
}
