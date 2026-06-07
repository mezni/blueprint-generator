use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn health() -> Json<Value> {
    Json(json!({"status":"ok"}))
}

pub fn routes() -> Router {
    Router::new().route("/health", get(health))
}
