mod config;
mod db;
mod error;
pub mod extractors;
mod models;
mod repository;
mod routes;

use std::net::SocketAddr;

use axum::http::Method;
use axum::{middleware, Extension, Router};
use common_auth::{auth_middleware, require_role, set_auth_config, AuthConfig};
use common_types::Role;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

use self::config::AppConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env();

    let pool = db::init_db_pool(&config.database_url)
        .await
        .expect("Failed to init database pool");

    if let Err(e) = common_db::run_migrations(&pool).await {
        tracing::warn!("Skipping migrations (already applied?): {e}");
    }

    set_auth_config(AuthConfig {
        issuer: config.auth_issuer.clone(),
        audience: config.auth_audience.clone(),
        jwks_url: config.auth_jwks_url.clone(),
    });

    common_auth::init_jwks_cache(config.auth_jwks_url.clone()).await;

    // CORS — allow everything for partner UI dev; tighten in production
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(Any)
        .allow_headers(Any);

    // Partner-facing API — authenticate + require partner role
    let partner_routes = routes::partner_routes(pool.clone())
        .layer(middleware::from_fn(require_role(Role::Partner)))
        .layer(middleware::from_fn(auth_middleware));

    // Admin-facing API — authenticate + require admin role
    let admin_routes = routes::admin_routes(pool.clone())
        .layer(middleware::from_fn(require_role(Role::Admin)))
        .layer(middleware::from_fn(auth_middleware));

    // Public routes (health)
    let public = routes::public_routes();

    let app = Router::new()
        .merge(partner_routes)
        .merge(admin_routes)
        .merge(public)
        .layer(Extension(pool))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("admin-service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
