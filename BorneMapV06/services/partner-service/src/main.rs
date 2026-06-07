//! Partner Service — Partner-facing API for station management
//!
//! This service provides endpoints for partners to view and manage their own stations.
//! Partners can only see their own stations (scoped by partner_id from JWT).

mod config;
mod error;
mod handlers;
mod infrastructure;
mod routing;

pub use config::Config;
pub use error::{ApiError, AppResult};

use actix_web::{web, HttpServer, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::info;
use tracing_subscriber::fmt;

use crate::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env();
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    tracing::info!("Starting partner-service on port {}", config.server_port);

    // Create database pool
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database pool created");

    // Create application state
    let state = AppState {
        config: config.clone(),
        pool,
    };

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(routing::setup_routes)
    })
    .bind(("0.0.0.0", config.server_port))?
    .run()
    .await
}

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "partner-service",
        "version": "0.1.0"
    }))
}
