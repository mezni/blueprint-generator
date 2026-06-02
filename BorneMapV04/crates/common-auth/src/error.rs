use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    MissingAuthorizationHeader,
    InvalidAuthorizationHeader,
    TokenParse(String),
    UnknownKey(String),
    KeyDecode(String),
    TokenValidation(String),
    JwksNotLoaded,
    InvalidSignature,
    TokenExpired,
    InvalidIssuer,
    InvalidAudience,
    InsufficientPermissions,
    MissingTenantId,
    ConfigError(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::MissingAuthorizationHeader => write!(f, "Missing Authorization header"),
            AuthError::InvalidAuthorizationHeader => write!(f, "Invalid Authorization header format"),
            AuthError::TokenParse(e) => write!(f, "Token parse error: {e}"),
            AuthError::UnknownKey(e) => write!(f, "Unknown signing key: {e}"),
            AuthError::KeyDecode(e) => write!(f, "Key decode error: {e}"),
            AuthError::TokenValidation(e) => write!(f, "Token validation error: {e}"),
            AuthError::JwksNotLoaded => write!(f, "JWKS not yet loaded"),
            AuthError::InvalidSignature => write!(f, "Invalid token signature"),
            AuthError::TokenExpired => write!(f, "Token has expired"),
            AuthError::InvalidIssuer => write!(f, "Invalid token issuer"),
            AuthError::InvalidAudience => write!(f, "Invalid token audience"),
            AuthError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            AuthError::MissingTenantId => write!(f, "Missing tenant_id for partner role"),
            AuthError::ConfigError(e) => write!(f, "Auth config error: {e}"),
        }
    }
}

impl AuthError {
    pub fn error_code(&self) -> &'static str {
        match self {
            AuthError::MissingAuthorizationHeader => "UNAUTHENTICATED",
            AuthError::InvalidAuthorizationHeader => "UNAUTHENTICATED",
            AuthError::TokenParse(_) => "TOKEN_MALFORMED",
            AuthError::UnknownKey(_) => "TOKEN_UNKNOWN_KEY",
            AuthError::KeyDecode(_) => "TOKEN_MALFORMED",
            AuthError::TokenValidation(_) => "TOKEN_INVALID_SIGNATURE",
            AuthError::JwksNotLoaded => "JWKS_NOT_LOADED",
            AuthError::InvalidSignature => "TOKEN_INVALID_SIGNATURE",
            AuthError::TokenExpired => "TOKEN_EXPIRED",
            AuthError::InvalidIssuer => "TOKEN_INVALID_ISSUER",
            AuthError::InvalidAudience => "TOKEN_INVALID_AUDIENCE",
            AuthError::InsufficientPermissions => "FORBIDDEN",
            AuthError::MissingTenantId => "FORBIDDEN",
            AuthError::ConfigError(_) => "AUTH_CONFIG_ERROR",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingAuthorizationHeader
            | AuthError::InvalidAuthorizationHeader
            | AuthError::TokenParse(_)
            | AuthError::UnknownKey(_)
            | AuthError::KeyDecode(_)
            | AuthError::TokenValidation(_)
            | AuthError::JwksNotLoaded
            | AuthError::InvalidSignature
            | AuthError::TokenExpired
            | AuthError::InvalidIssuer
            | AuthError::InvalidAudience => StatusCode::UNAUTHORIZED,
            AuthError::InsufficientPermissions | AuthError::MissingTenantId => {
                StatusCode::FORBIDDEN
            }
            AuthError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let code = self.error_code();
        let message = self.to_string();
        let status = self.status_code();

        let body = json!({
            "success": false,
            "error": {
                "code": code,
                "message": message,
            }
        });

        (status, Json(body)).into_response()
    }
}
