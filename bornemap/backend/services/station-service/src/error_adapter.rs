use actix_web::{HttpResponse, http::StatusCode};
use common_utils::error::DomainError;
use serde_json::json;

pub struct ApiError(pub DomainError);

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status = StatusCode::from_u16(self.0.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let code = self.0.error_code();

        HttpResponse::build(status).json(json!({
            "type": "about:blank",
            "title": status.canonical_reason().unwrap_or("Error"),
            "status": status.as_u16(),
            "detail": self.0.to_string(),
            "code": code,
        }))
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        ApiError(err)
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError(DomainError::Internal(err.to_string()))
    }
}
