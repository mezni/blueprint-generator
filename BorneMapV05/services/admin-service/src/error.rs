use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use common_auth::AuthError;
use common_errors::{ApiError, ErrorCode};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Auth error: {0}")]
    Auth(#[from] AuthError),

    #[error("API error: {0}")]
    Api(#[from] ApiError),

    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl ServiceError {
    pub fn validation(msg: impl Into<String>) -> Self {
        ServiceError::Validation(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        ServiceError::Internal(msg.into())
    }

    pub fn not_found(entity: &str, id: &str) -> Self {
        ServiceError::Api(ApiError {
            code: ErrorCode::NotFound,
            message: format!("{} with id '{}' not found", entity, id),
            details: None,
        })
    }

    pub fn partner_scope_violation() -> Self {
        ServiceError::Api(ApiError {
            code: ErrorCode::PartnerScopeViolation,
            message: "Resource belongs to a different partner".into(),
            details: None,
        })
    }

    pub fn invalid_coordinates() -> Self {
        ServiceError::Api(ApiError {
            code: ErrorCode::InvalidCoordinates,
            message: "Latitude must be between -90 and 90, longitude between -180 and 180".into(),
            details: None,
        })
    }

    pub fn invalid_state_transition(from: &str, to: &str) -> Self {
        ServiceError::Api(ApiError {
            code: ErrorCode::InvalidStateTransition,
            message: format!("Cannot transition station status from '{}' to '{}'", from, to),
            details: None,
        })
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        match self {
            ServiceError::Auth(auth_err) => auth_err.into_response(),
            ServiceError::Api(api_err) => api_err.into_response(),
            ServiceError::Db(sqlx_err) => {
                let (status, code, message) = match &sqlx_err {
                    sqlx::Error::RowNotFound => {
                        (StatusCode::NOT_FOUND, "NOT_FOUND", "Resource not found")
                    }
                    sqlx::Error::Database(db_err) => {
                        let constraint = db_err.constraint().unwrap_or("");
                        if constraint.contains("active_stations_exist") || constraint.contains("trg_partner_delete_guard") {
                            (StatusCode::CONFLICT, "ACTIVE_STATIONS_EXIST", "Cannot delete partner with active stations")
                        } else if constraint.contains("unique") || constraint.contains("duplicate") {
                            (StatusCode::CONFLICT, "ALREADY_EXISTS", "Resource already exists")
                        } else if constraint.contains("fk") || constraint.contains("foreign_key") {
                            (StatusCode::BAD_REQUEST, "VALIDATION_FAILED", "Referenced entity does not exist")
                        } else {
                            (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "A database error occurred")
                        }
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "A database error occurred"),
                };
                let body = json!({
                    "success": false,
                    "error": { "code": code, "message": message, "details": null }
                });
                (status, Json(body)).into_response()
            }
            ServiceError::Validation(msg) => {
                let body = json!({
                    "success": false,
                    "error": { "code": "VALIDATION_FAILED", "message": msg, "details": null }
                });
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
            ServiceError::Internal(msg) => {
                let body = json!({
                    "success": false,
                    "error": { "code": "INTERNAL_ERROR", "message": msg, "details": null }
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}
