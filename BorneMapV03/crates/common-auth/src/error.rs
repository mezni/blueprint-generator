use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token is missing or invalid")]
    Unauthorized,

    #[error("Insufficient permissions for this resource")]
    Forbidden,

    #[error("Token has expired")]
    TokenExpired,

    #[error("Identity provider unavailable")]
    AuthUnavailable,

    #[error("JWT validation failed: {0}")]
    JwtError(String),

    #[error("JWKS fetch failed: {0}")]
    JwksError(String),
}

impl AuthError {
    pub fn status_code(&self) -> u16 {
        match self {
            AuthError::Unauthorized | AuthError::TokenExpired => 401,
            AuthError::Forbidden => 403,
            AuthError::AuthUnavailable => 500,
            AuthError::JwtError(_) => 401,
            AuthError::JwksError(_) => 500,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            AuthError::Unauthorized => "UNAUTHORIZED",
            AuthError::Forbidden => "FORBIDDEN",
            AuthError::TokenExpired => "TOKEN_EXPIRED",
            AuthError::AuthUnavailable => "AUTH_UNAVAILABLE",
            AuthError::JwtError(_) => "UNAUTHORIZED",
            AuthError::JwksError(_) => "AUTH_UNAVAILABLE",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AuditEventType {
    LoginSuccess,
    LoginFailure,
    Logout,
    TokenRefresh,
    RoleChange,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditEvent {
    pub event_type: AuditEventType,
    pub user_id: Option<String>,
    pub client_id: Option<String>,
    pub ip_address: Option<String>,
    pub outcome: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthErrorResponse {
    pub error_code: &'static str,
    pub message: String,
    pub trace_id: String,
}
