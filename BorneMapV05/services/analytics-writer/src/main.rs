use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

async fn health() -> Json<Value> {
    Json(json!({"status":"ok"}))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // analytics-writer is internal-only — no auth middleware needed.
    let app = Router::new().route("/health", get(health));

    let port: u16 = std::env::var("ANALYTICS_WRITER_PORT")
        .unwrap_or_else(|_| "8085".into())
        .parse()
        .unwrap_or(8085);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("analytics-writer listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
