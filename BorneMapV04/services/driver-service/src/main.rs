mod config;
mod db;

use axum::{Json, Router, http::StatusCode, routing::get};
use common_config::{ConfigLoader, log_redacted, load_env_map};
use common_observability::SERVICE_VERSION;
use config::DriverServiceConfig;
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct ReadyResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
    dependencies: Vec<DependencyStatus>,
}

#[derive(Serialize)]
struct DependencyStatus {
    name: &'static str,
    status: &'static str,
}

struct AppState {
    pg_pool: Option<PgPool>,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "driver-service",
        version: SERVICE_VERSION,
    })
}

async fn ready(state: axum::extract::State<Arc<AppState>>) -> (StatusCode, Json<ReadyResponse>) {
    let pg_ok = match &state.pg_pool {
        Some(pool) => pool.acquire().await.is_ok(),
        None => false,
    };

    let deps = vec![DependencyStatus {
        name: "postgresql",
        status: if pg_ok { "connected" } else { "disconnected" },
    }];

    if pg_ok {
        (StatusCode::OK, Json(ReadyResponse {
            status: "ok",
            service: "driver-service",
            version: SERVICE_VERSION,
            dependencies: deps,
        }))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(ReadyResponse {
            status: "degraded",
            service: "driver-service",
            version: SERVICE_VERSION,
            dependencies: deps,
        }))
    }
}

#[tokio::main]
async fn main() {
    common_observability::init_default("driver-service");

    let env_map = load_env_map();
    let cfg = DriverServiceConfig::load().unwrap_or_else(|e| {
        tracing::error!("Configuration error: {e}");
        std::process::exit(1);
    });

    log_redacted(&env_map);
    info!(stage = "config_load", service = %cfg.service_name, port = %cfg.service_port, "Configuration loaded");

    info!(stage = "dependency_check", "Connecting to PostgreSQL");
    let pg_pool = match db::connect(&cfg.database_url).await {
        Ok(pool) => {
            info!("Connected to PostgreSQL");
            Some(pool)
        }
        Err(e) => {
            tracing::error!("Failed to connect to PostgreSQL: {e}");
            None
        }
    };

    let addr = format!("0.0.0.0:{}", cfg.service_port);
    let state = Arc::new(AppState { pg_pool });

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .with_state(state);

    info!(stage = "route_registration", port = %cfg.service_port, "Registering routes");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!(stage = "ready", addr = %addr, "Service ready");
    axum::serve(listener, app).await.unwrap();
}
