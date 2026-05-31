use actix_web::web;

pub mod model;
pub mod routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::get_nearby_stations);
    cfg.service(routes::update_station_status);
    cfg.service(routes::search_stations);
    cfg.service(routes::get_station_detail);
}
