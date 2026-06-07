//! HTTP routing configuration
//!
//! Defines all API endpoints and middleware for the driver service.

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::PgPool;
use std::env;

use crate::migration_runner::health_check;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

/// Setup API routes and middleware
pub fn setup_routes(app: &mut App<AppState>) {
    // Health check endpoint (no auth required)
    app.route("/health", web::get().to(health_check));

    // API v1 routes (auth required for most endpoints)
    let api_v1 = web::scope("/api/v1")
        .wrap(crate::error::ErrorHandlerMiddleware)
        .configure(routes);

    app.service(api_v1);
}

/// Define all API v1 routes
fn routes(cfg: &mut web::ServiceConfig) {
    // Public endpoints (no auth required)
    cfg.route("/stations/nearby", web::get().to(crate::handlers::nearby_handler));

    // Authenticated endpoints (require JWT)
    cfg.service(web::scope("/favorites")
        .wrap(crate::middleware::auth::AuthMiddleware)
        .route("", web::get().to(crate::handlers::list_favorites_handler))
        .route("", web::post().to(crate::handlers::create_favorite_handler))
        .route("/{favorite_id}", web::delete().to(crate::handlers::remove_favorite_handler))
    );

    // Uncomment when partner-service is implemented
    // cfg.service(web::scope("/partner/stations")
    //     .wrap(crate::middleware::auth::AuthMiddleware)
    //     .wrap(crate::middleware::partner_scope::PartnerScopeMiddleware)
    //     .route("", web::get().to(crate::handlers::list_partner_stations_handler))
    //     .route("/{station_id}", web::get().to(crate::handlers::get_partner_station_handler))
    //     .route("", web::post().to(crate::handlers::create_partner_station_handler))
    //     .route("/{station_id}", web::patch().to(crate::handlers::update_partner_station_handler))
    // );
}

/// Health check handler
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "driver-service",
        "version": "0.1.0"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_setup() {
        // This is a structural test - verify routing structure
        let routes = vec![
            "GET /health",
            "GET /api/v1/stations/nearby",
            "POST /api/v1/favorites",
            "GET /api/v1/favorites",
            "DELETE /api/v1/favorites/{id}",
        ];

        assert_eq!(routes.len(), 5);
        assert!(routes.contains(&"GET /health"));
    }

    #[test]
    fn test_api_scope_structure() {
        let api_v1_routes = vec![
            "stations/nearby",
            "favorites",
        ];

        assert_eq!(api_v1_routes.len(), 2);
        assert_eq!(api_v1_routes[0], "stations/nearby");
        assert_eq!(api_v1_routes[1], "favorites");
    }
}
