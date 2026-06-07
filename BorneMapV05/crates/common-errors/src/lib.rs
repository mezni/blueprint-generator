use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "UNAUTHENTICATED")]
    Unauthenticated,
    #[serde(rename = "FORBIDDEN")]
    Forbidden,
    #[serde(rename = "TOKEN_EXPIRED")]
    TokenExpired,
    #[serde(rename = "PARTNER_SCOPE_VIOLATION")]
    PartnerScopeViolation,
    #[serde(rename = "INSUFFICIENT_ROLE")]
    InsufficientRole,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "ALREADY_EXISTS")]
    AlreadyExists,
    #[serde(rename = "SOFT_DELETED")]
    SoftDeleted,
    #[serde(rename = "VALIDATION_FAILED")]
    ValidationFailed,
    #[serde(rename = "INVALID_COORDINATES")]
    InvalidCoordinates,
    #[serde(rename = "INVALID_STATE_TRANSITION")]
    InvalidStateTransition,
    #[serde(rename = "ACTIVE_STATIONS_EXIST")]
    ActiveStationsExist,
    #[serde(rename = "REVIEW_STATE_INVALID")]
    ReviewStateInvalid,
    #[serde(rename = "CONCURRENT_MODIFICATION")]
    ConcurrentModification,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ErrorCode::Unauthenticated => "UNAUTHENTICATED",
            ErrorCode::Forbidden => "FORBIDDEN",
            ErrorCode::TokenExpired => "TOKEN_EXPIRED",
            ErrorCode::PartnerScopeViolation => "PARTNER_SCOPE_VIOLATION",
            ErrorCode::InsufficientRole => "INSUFFICIENT_ROLE",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::AlreadyExists => "ALREADY_EXISTS",
            ErrorCode::SoftDeleted => "SOFT_DELETED",
            ErrorCode::ValidationFailed => "VALIDATION_FAILED",
            ErrorCode::InvalidCoordinates => "INVALID_COORDINATES",
            ErrorCode::InvalidStateTransition => "INVALID_STATE_TRANSITION",
            ErrorCode::ActiveStationsExist => "ACTIVE_STATIONS_EXIST",
            ErrorCode::ReviewStateInvalid => "REVIEW_STATE_INVALID",
            ErrorCode::ConcurrentModification => "CONCURRENT_MODIFICATION",
        };
        write!(f, "{}", s)
    }
}

impl ErrorCode {
    fn http_status(&self) -> StatusCode {
        match self {
            ErrorCode::Unauthenticated => StatusCode::UNAUTHORIZED,
            ErrorCode::TokenExpired => StatusCode::UNAUTHORIZED,
            ErrorCode::Forbidden => StatusCode::FORBIDDEN,
            ErrorCode::InsufficientRole => StatusCode::FORBIDDEN,
            ErrorCode::PartnerScopeViolation => StatusCode::FORBIDDEN,
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::AlreadyExists => StatusCode::CONFLICT,
            ErrorCode::ActiveStationsExist => StatusCode::CONFLICT,
            ErrorCode::ConcurrentModification => StatusCode::CONFLICT,
            ErrorCode::SoftDeleted => StatusCode::GONE,
            ErrorCode::ValidationFailed => StatusCode::BAD_REQUEST,
            ErrorCode::InvalidCoordinates => StatusCode::BAD_REQUEST,
            ErrorCode::InvalidStateTransition => StatusCode::BAD_REQUEST,
            ErrorCode::ReviewStateInvalid => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.code.http_status();
        let body = serde_json::json!({
            "success": false,
            "error": {
                "code": self.code.to_string(),
                "message": self.message,
                "details": self.details,
            }
        });
        (status, Json(body)).into_response()
    }
}
