use actix_web::web;

pub mod routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::log_client_connection);
    cfg.service(routes::get_aggregates);
}
