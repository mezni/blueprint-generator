use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod routes;

#[derive(Clone, Serialize, Deserialize)]
pub struct FilterState {
    pub connector_types: Vec<String>,
    pub status: Vec<String>,
    pub min_available: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TimestampedFilters {
    pub filters: FilterState,
    pub updated_at: String,
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::get_filters);
    cfg.service(routes::set_filters);
}
