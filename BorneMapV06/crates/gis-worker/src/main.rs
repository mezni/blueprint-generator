//! GIS Sync Worker — Async worker for station location projection
//!
//! This worker polls the outbox table for station change events and projects
//! them to the GIS schema for efficient spatial queries.

mod application;
mod domain;
mod infrastructure;

use std::time::Duration;
use tracing::{error, info, warn};

use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::application::GisSyncUseCase;
use crate::domain::{EventReader, EventType};
use crate::ev_db::Pool;

/// Application state
pub struct AppState {
    pub pool: Pool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,gis_worker=debug".into()),
        )
        .init();

    info!("Starting GIS Sync Worker");

    // Load configuration
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        warn!("DATABASE_URL not set, using default: postgresql://postgres@localhost/borne_map");
        "postgresql://postgres@localhost/borne_map".to_string()
    });

    info!("Connecting to database...");

    // Create database pool
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    info!("Database pool created");

    // Run migrations
    info!("Running database migrations...");
    let migration_runner = crate::infrastructure::MigrationRunner::new(pool.clone());
    match migration_runner.run_migrations().await {
        Ok(migrations_applied) if !migrations_applied.is_empty() => {
            info!("Migrations applied: {:?}", migrations_applied);
        }
        Ok(_) => {
            info!("No migrations applied");
        }
        Err(e) => {
            warn!("Migration failed (continuing): {}", e);
        }
    }

    // Create application state
    let state = AppState {
        pool: Pool::new(pool),
    };

    info!("GIS Sync Worker started successfully");

    // Start polling loop
    let sync_usecase = GisSyncUseCase::new(Pool::new(pool.clone()));
    let poll_interval = Duration::from_secs(5);

    loop {
        info!("Polling for new events...");

        match sync_usecase.has_pending_events().await {
            Ok(has_pending) => {
                if has_pending {
                    info!("Found pending events, processing...");
                    match sync_usecase.sync_all().await {
                        Ok(count) => {
                            info!("Processed {} events", count);
                        }
                        Err(e) => {
                            error!("Failed to process events: {}", e);
                        }
                    }
                } else {
                    debug!("No pending events found");
                }
            }
            Err(e) => {
                warn!("Error checking for pending events: {}", e);
            }
        }

        tokio::time::sleep(poll_interval).await;
    }
}

/// Health check endpoint
async fn health_check() -> impl actix_web::Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "gis-worker",
        "version": "0.1.0"
    }))
}
