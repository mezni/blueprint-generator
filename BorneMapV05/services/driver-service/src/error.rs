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

    pub fn forbidden() -> Self {
        ServiceError::Api(ApiError {
            code: ErrorCode::Forbidden,
            message: "You do not have permission to perform this action".into(),
            details: None,
        })
    }

    pub fn already_exists(msg: impl Into<String>) -> Self {
        ServiceError::Api(ApiError {
            code: ErrorCode::AlreadyExists,
            message: msg.into(),
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
                        if constraint.contains("unique") || constraint.contains("duplicate") {
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

#[cfg(test)]
mod tests {
    use crate::error::ServiceError;

    #[test]
    fn test_validation_error_creation() {
        let err = ServiceError::validation("test error");
        let msg = format!("{}", err);
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_not_found_error() {
        let err = ServiceError::not_found("Station", "STN-123");
        let msg = format!("{}", err);
        assert!(msg.contains("Station with id 'STN-123' not found"));
    }

    #[test]
    fn test_forbidden_error() {
        let err = ServiceError::forbidden();
        let msg = format!("{}", err);
        assert!(msg.contains("do not have permission"));
    }

    #[test]
    fn test_already_exists_error() {
        let err = ServiceError::already_exists("Item already exists");
        let msg = format!("{}", err);
        assert!(msg.contains("Item already exists"));
    }
}
