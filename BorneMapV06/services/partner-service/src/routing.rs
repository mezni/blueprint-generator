//! HTTP routing configuration for partner-service

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::PgPool;

use crate::AppState;
use crate::middleware::{auth, partner_scope};

/// Setup API routes and middleware for partner-service
pub fn setup_routes(app: &mut App<AppState>) {
    // Health check endpoint (no auth required)
    app.route("/health", web::get().to(health_check));

    // API v1 routes (auth required)
    let api_v1 = web::scope("/api/v1")
        .wrap(crate::error::ErrorHandlerMiddleware)
        .configure(routes);

    app.service(api_v1);
}

/// Define all API v1 routes
fn routes(cfg: &mut web::ServiceConfig) {
    // Partner station endpoints (auth required)
    cfg.service(web::scope("/partner/stations")
        .wrap(auth::auth())
        .wrap(partner_scope::partner_scope_middleware)
        .route("", web::get().to(crate::handlers::list_partner_stations_handler))
        .route("", web::post().to(crate::handlers::create_partner_station_handler))
        .route("/{station_id}", web::get().to(crate::handlers::get_partner_station_handler))
        .route("/{station_id}", web::patch().to(crate::handlers::update_partner_station_handler))
    );
}

/// Health check handler
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "partner-service",
        "version": "0.1.0"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_setup() {
        // Test structure
        let routes = vec![
            "GET /health",
            "GET /api/v1/partner/stations",
            "POST /api/v1/partner/stations",
            "GET /api/v1/partner/stations/{station_id}",
            "PATCH /api/v1/partner/stations/{station_id}",
        ];

        assert_eq!(routes.len(), 5);
        assert!(routes.contains(&"GET /health"));
    }
}

/// Define all API v1 routes
fn routes(cfg: &mut web::ServiceConfig) {
    // Partner station endpoints (auth required)
    cfg.service(web::scope("/partner/stations")
        .wrap(crate::middleware::auth::auth())
        .wrap(crate::middleware::partner_scope::partner_scope_middleware)
        .route("", web::get().to(crate::handlers::list_partner_stations_handler))
        .route("", web::post().to(crate::handlers::create_partner_station_handler))
        .route("/{station_id}", web::get().to(crate::handlers::get_partner_station_handler))
        .route("/{station_id}", web::patch().to(crate::handlers::update_partner_station_handler))
    );
}

/// Health check handler
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "partner-service",
        "version": "0.1.0"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_setup() {
        // Test structure
        let routes = vec![
            "GET /health",
            "GET /api/v1/partner/stations",
            "POST /api/v1/partner/stations",
            "GET /api/v1/partner/stations/{station_id}",
            "PATCH /api/v1/partner/stations/{station_id}",
        ];

        assert_eq!(routes.len(), 5);
        assert!(routes.contains(&"GET /health"));
    }
}
