use actix_web::{get, web, HttpResponse};

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}
