//! EV Domain — Shared domain models and entities
//!
//! This crate contains core domain models used across all services:
//! - Station, Charger, Partner, User entities
//! - Favorite and Review models
//! - Validation rules and constraints
//! - Value objects (coordinates, identifiers, etc.)

pub mod entities;
pub mod ids;
pub mod validation;
pub mod geo;

pub use entities::*;
pub use ids::*;
pub use validation::*;
pub use geo::*;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid coordinates: {0}")]
    InvalidCoordinates(String),

    #[error("Invalid ID format: {0}")]
    InvalidId(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
