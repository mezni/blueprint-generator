//! EV Auth — Keycloak JWT authentication and authorization
//!
//! This crate provides JWT validation and claims extraction for Keycloak-based authentication.
//!
//! # Features
//! - JWT token validation
//! - Claims extraction and verification
//! - Role-based access control support
//! - Partner scope validation

pub mod claims;
pub mod jwt_validator;

pub use claims::{Claims, Role};
pub use jwt_validator::{JwtValidator, JwtValidatorError};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid JWT: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Missing partner ID in token")]
    MissingPartnerScope,

    #[error("Invalid role: {0}")]
    InvalidRole(String),

    #[error("Unauthorized")]
    Unauthorized,
}

pub type AuthResult<T> = Result<T, AuthError>;
