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
use common_auth::{auth_middleware, optional_auth_middleware, require_role, set_auth_config, AuthConfig};
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

    set_auth_config(AuthConfig {
        issuer: config.auth_issuer.clone(),
        audience: config.auth_audience.clone(),
        jwks_url: config.auth_jwks_url.clone(),
    });

    common_auth::init_jwks_cache(config.auth_jwks_url.clone()).await;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    // Public routes: station discovery (optional auth for distance info)
    let discovery = routes::public_routes(pool.clone())
        .layer(middleware::from_fn(optional_auth_middleware));

    // Authenticated routes: require registered_driver role
    let authenticated = routes::authenticated_routes(pool.clone())
        .layer(middleware::from_fn(require_role(Role::RegisteredDriver)))
        .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .merge(discovery)
        .merge(authenticated)
        .layer(Extension(pool))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("driver-service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use common_auth::AuthError;

    use crate::error::ServiceError;

    #[test]
    fn test_auth_error_unauthenticated_status() {
        let err = ServiceError::Auth(AuthError::Unauthenticated);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_auth_error_insufficient_role_status() {
        let err = ServiceError::Auth(AuthError::InsufficientRole);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_auth_error_token_expired_status() {
        let err = ServiceError::Auth(AuthError::TokenExpired);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_auth_error_forbidden_status() {
        let err = ServiceError::Auth(AuthError::Forbidden);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
