use actix_web::middleware::Logger;

#[allow(dead_code)]
pub fn logging() -> Logger {
    Logger::default()
}
