use actix_web::{get, post, web, HttpResponse, Responder};
use core::AnalyticsEvent;
use futures_util::stream::StreamExt;
use lapin::{options::*, BasicProperties};
use mongodb::bson::doc;
use regex::Regex;
use std::sync::OnceLock;

use crate::AppState;

fn event_id_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^evt-[a-f0-9]{8}$").expect("invalid event_id regex"))
}

#[post("/analytics/connect")]
pub async fn log_client_connection(
    state: web::Data<AppState>,
    payload: web::Json<AnalyticsEvent>,
) -> impl Responder {
    let event = payload.into_inner();

    if !event_id_regex().is_match(&event.event_id) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "invalid event_id: must match pattern evt-[a-f0-9]{8}"
        }));
    }

    let payload_bytes = match serde_json::to_vec(&event) {
        Ok(b) => b,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "failed to serialize payload"
            }));
        }
    };

    match state
        .amqp_channel
        .basic_publish(
            "",
            "analytics.connections",
            BasicPublishOptions::default(),
            &payload_bytes,
            BasicProperties::default(),
        )
        .await
    {
        Ok(_) => HttpResponse::Accepted().json(serde_json::json!({"accepted": true})),
        Err(e) => {
            log::error!("Failed to enqueue analytics event: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "failed to enqueue event"
            }))
        }
    }
}

#[get("/analytics/connections")]
pub async fn get_aggregates(
    state: web::Data<AppState>,
) -> impl Responder {
    let collection = state.mongo_db.collection::<mongodb::bson::Document>("connection_aggregates");

    let mut cursor = match collection.find(doc! {}, None).await {
        Ok(c) => c,
        Err(e) => {
            log::error!("MongoDB query failed: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "failed to query aggregates"
            }));
        }
    };

    let mut aggregates = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                aggregates.push(doc);
            }
            Err(e) => {
                log::error!("Failed to read aggregate document: {}", e);
            }
        }
    }

    HttpResponse::Ok().json(serde_json::json!({ "aggregates": aggregates }))
}
