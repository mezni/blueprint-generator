use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: String,
    pub keycloak_user_id: String,
    pub email: Option<String>,
    pub role: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user_id: String,
    pub keycloak_user_id: String,
    pub email: Option<String>,
    pub role: Option<String>,
    pub created_at: DateTime<Utc>,
}
