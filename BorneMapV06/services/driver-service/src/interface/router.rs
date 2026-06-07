use actix_web::web;
use crate::interface::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Root-level health check (for Docker, load balancers, etc.)
    cfg.configure(handlers::health::configure);
    
    // API v1 routes
    cfg.service(
        web::scope("/api/v1")
            .configure(handlers::health::configure)
    );
}
