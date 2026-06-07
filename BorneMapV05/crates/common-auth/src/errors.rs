use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Authentication required")]
    Unauthenticated,

    #[error("Token has expired")]
    TokenExpired,

    #[error("Insufficient permissions")]
    InsufficientRole,

    #[error("Forbidden")]
    Forbidden,

    #[error("Partner scope violation")]
    PartnerScopeViolation,

    #[error("Token validation failed: {0}")]
    ValidationError(String),

    #[error("JWKS fetch failed: {0}")]
    JwksFetchError(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AuthError::Unauthenticated => (StatusCode::UNAUTHORIZED, "UNAUTHENTICATED", self.to_string()),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED", self.to_string()),
            AuthError::InsufficientRole => (StatusCode::FORBIDDEN, "INSUFFICIENT_ROLE", self.to_string()),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN", self.to_string()),
            AuthError::PartnerScopeViolation => (StatusCode::FORBIDDEN, "PARTNER_SCOPE_VIOLATION", self.to_string()),
            AuthError::ValidationError(_) => (StatusCode::UNAUTHORIZED, "UNAUTHENTICATED", self.to_string()),
            AuthError::JwksFetchError(_) => (StatusCode::SERVICE_UNAVAILABLE, "AUTH_UNAVAILABLE", self.to_string()),
        };

        let body = json!({
            "success": false,
            "error": {
                "code": code,
                "message": message,
                "details": null
            }
        });

        (status, Json(body)).into_response()
    }
}
