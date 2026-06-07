pub mod discovery;
pub mod favorites;
pub mod reviews;
pub mod profile;
pub mod public;

use axum::Router;
use sqlx::PgPool;

pub fn public_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(discovery::routes(pool.clone()))
        .merge(public::routes())
}

pub fn authenticated_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(favorites::routes(pool.clone()))
        .merge(reviews::routes(pool.clone()))
        .merge(profile::routes(pool))
}
