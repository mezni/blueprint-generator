use actix_web::{get, put, web, HttpResponse, Responder};
use crate::AppState;
use crate::domains::filters::{FilterState, TimestampedFilters};

#[get("/filters")]
pub async fn get_filters(
    state: web::Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let session_id = match query.get("session_id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "missing session_id"})),
    };

    let store = state.filter_store.lock().unwrap();
    match store.get(session_id) {
        Some(entry) => HttpResponse::Ok().json(entry),
        None => HttpResponse::Ok().json(serde_json::json!({
            "filters": {
                "connector_types": [],
                "status": [],
                "min_available": null
            },
            "updated_at": null
        })),
    }
}

#[put("/filters")]
pub async fn set_filters(
    state: web::Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
    body: web::Json<FilterState>,
) -> impl Responder {
    let session_id = match query.get("session_id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "missing session_id"})),
    };

    let entry = TimestampedFilters {
        filters: body.into_inner(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    let mut store = state.filter_store.lock().unwrap();
    store.insert(session_id.clone(), entry.clone());

    HttpResponse::Ok().json(entry)
}
