//! Error handler middleware

use actix_web::error::ResponseError;
use actix_web::{HttpResponse, Json};
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web_httpauth::extractors::AuthExtractor;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use sqlx::PgPool;

use crate::error::AppResult;

/// Error handler middleware
pub struct ErrorHandlerMiddleware;

impl ErrorHandlerMiddleware {
    /// Create new error handler middleware
    pub fn new() -> Self {
        Self
    }
}

impl Transform<ServiceRequest, Error> for ErrorHandlerMiddleware {
    type Transform = ErrorHandlerMiddlewareInner;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::Error>>;

    fn new_transform(&self, _service: ServiceRequest) -> Self::Future {
        ready(Ok(ErrorHandlerMiddlewareInner))
    }
}

/// Error handler middleware inner
pub struct ErrorHandlerMiddlewareInner;

impl ServiceRequest for ErrorHandlerMiddlewareInner {
    fn extensions(&self) -> &actix_http::body::MessageMap {
        unimplemented!()
    }

    fn extensions_mut(&mut self) -> &mut actix_http::body::MessageMap {
        unimplemented!()
    }

    fn poll_request(&mut self, cx: &mut Context<'_>) -> Poll<Result<actix_http::Request<actix_http::body::Body>, Error>> {
        unimplemented!()
    }

    fn poll_body(&mut self, cx: &mut Context<'_>) -> Poll<Result<actix_http::body::Body, Error>> {
        unimplemented!()
    }

    fn headers(&self) -> &actix_http::header::HeaderMap {
        unimplemented!()
    }

    fn headers_mut(&mut self) -> &mut actix_http::header::HeaderMap {
        unimplemented!()
    }

    fn header_ref(&self, key: &'static actix_http::header::HeaderName) -> Option<&actix_http::header::HeaderValue> {
        unimplemented!()
    }

    fn remote_addr(&self) -> &std::net::SocketAddr {
        unimplemented!()
    }

    fn connection_info(&self) -> &actix_http::info::ConnectionInfo {
        unimplemented!()
    }

    fn take_headers(&mut self) -> actix_http::header::HeaderMap {
        unimplemented!()
    }

    fn payload(&self) -> &mut actix_http::body::MessageBody {
        unimplemented!()
    }
}

impl ResponseError for ErrorHandlerMiddleware {
    fn status_code(&self) -> actix_http::StatusCode {
        actix_http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "internal_error",
            "message": "An unexpected error occurred",
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_handler_creation() {
        let middleware = ErrorHandlerMiddleware::new();
        assert!(middleware.new_transform(actix_web::test::TestRequest::default().finish()).is_ok());
    }
}
