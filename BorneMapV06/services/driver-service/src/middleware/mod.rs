//! Middleware modules

pub mod error;
pub mod auth;
pub mod rate_limiter;

pub use error::{ApiError, ErrorHandlerMiddleware, AppResult};
