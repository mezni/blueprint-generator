pub mod admin;
pub mod partner;
pub mod public;

use axum::Router;
use sqlx::PgPool;

/// Build all partner-facing route groups (US1, US2, US4).
pub fn partner_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(partner::station_routes(pool.clone()))
        .merge(partner::charger_routes(pool.clone()))
        .merge(partner::profile_routes(pool))
}

/// Build all admin-facing route groups (US3, US5).
pub fn admin_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(admin::station_routes(pool.clone()))
        .merge(admin::partner_routes(pool))
}

/// Build public (unauthenticated) routes.
pub fn public_routes() -> Router {
    Router::new().merge(public::routes())
}
