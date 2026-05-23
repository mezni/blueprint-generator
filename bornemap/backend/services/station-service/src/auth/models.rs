use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct MockLoginRequest {
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MockLoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}
