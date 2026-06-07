//! Driver Service — Station Discovery & Favorites API
//!
//! This service provides:
//! - Public station discovery endpoint (GET /api/v1/stations/nearby)
//! - User favorites management (POST/GET/DELETE /api/v1/favorites)
//! - Rate limiting, authentication middleware, error handling
//!
//! Architecture:
//! - Domain layer: Pure business logic (station discovery, favorites)
//! - Application layer: Use cases (orchestration)
//! - Infrastructure layer: DB access, repositories, middleware
//! - Interface layer: HTTP handlers (Actix-Web)

mod config;
mod error;
mod migration_runner;
mod routing;

pub use config::Config;
pub use error::{ApiError, AppResult};

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::PgPool;

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

    tracing::info!("Starting driver-service on port {}", config.server_port);

    // Create database pool
    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database pool created");

    // Run migrations
    match migration_runner::run_migrations(&pool).await {
        Ok(()) => tracing::info!("Migrations applied successfully"),
        Err(e) => {
            tracing::error!("Failed to apply migrations: {}", e);
            std::process::exit(1);
        }
    }

    // Verify schema
    match migration_runner::verify_schema(&pool).await {
        Ok(()) => tracing::info!("Schema verified successfully"),
        Err(e) => {
            tracing::error!("Schema verification failed: {}", e);
            std::process::exit(1);
        }
    }

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
        "service": "driver-service",
        "version": "0.1.0"
    }))
}
