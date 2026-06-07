use axum::routing::{get, post};
use axum::{Json, Router};
use common_auth::{optional_auth_middleware, set_auth_config, AuthConfig};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

async fn health() -> Json<Value> {
    Json(json!({"status":"ok"}))
}

/// Placeholder for clickstream event ingestion (Sprint 13).
async fn ingest() -> Json<Value> {
    Json(json!({
        "success": true,
        "data": {"message": "Event ingestion stub"},
        "meta": {}
    }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let port: u16 = std::env::var("CLICKSTREAM_PORT")
        .unwrap_or_else(|_| "8083".into())
        .parse()
        .unwrap_or(8083);

    let issuer = std::env::var("AUTH_ISSUER")
        .unwrap_or_else(|_| "http://keycloak:8080/realms/bornemap".into());
    let jwks_url = std::env::var("AUTH_JWKS_URL")
        .unwrap_or_else(|_| "http://keycloak:8080/realms/bornemap/protocol/openid-connect/certs".into());
    let audience = std::env::var("AUTH_AUDIENCE")
        .unwrap_or_else(|_| "bornemap-api".into());

    set_auth_config(AuthConfig {
        issuer: issuer.clone(),
        audience: audience.clone(),
        jwks_url: jwks_url.clone(),
    });

    common_auth::init_jwks_cache(jwks_url).await;

    // Clickstream ingestion accepts anonymous events: optional auth populates
    // CurrentUser when a valid token is present but does not reject anonymous callers.
    let events = Router::new()
        .route("/api/v1/clickstream/events", post(ingest))
        .layer(axum::middleware::from_fn(optional_auth_middleware));

    // /health is exempt from auth (liveness/readiness probes).
    let app = Router::new()
        .route("/health", get(health))
        .merge(events);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("clickstream-service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
