//! Partner Service — Partner-facing API for station management
//!
//! This service provides endpoints for partners to view and manage their own stations.
//! Partners can only see their own stations (scoped by partner_id from JWT).

pub mod config;
pub mod error;
pub mod infrastructure;
pub mod routing;

pub use config::Config;
pub use error::{ApiError, AppResult};

/// Application state
#[derive(Debug)]
pub struct AppState {
    pub config: Config,
    pub pool: sqlx::PgPool,
}
