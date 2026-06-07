use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::station::{StationListQuery, StationSearchQuery};
use crate::repository::station_repo;
use common_types::api::{ItemEnvelope, SuccessEnvelope};

use axum::Router;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/v1/driver/stations", axum::routing::get(list_stations))
        .route("/api/v1/driver/stations/{id}", axum::routing::get(get_station))
        .route("/api/v1/driver/stations/search", axum::routing::get(search_stations))
        .with_state(pool)
}

async fn list_stations(
    State(pool): State<PgPool>,
    Query(params): Query<StationListQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let pagination = PaginationParams {
        page: params.page,
        size: params.size,
    };

    let (stations, meta) = station_repo::list_visible_stations(&pool, &params, &pagination).await?;
    Ok(SuccessEnvelope::new(stations, meta))
}

async fn get_station(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Query(query): Query<StationListQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let station = station_repo::get_station_detail(&pool, &id, query.lat, query.lng).await?;
    Ok(ItemEnvelope::new(station))
}

async fn search_stations(
    State(pool): State<PgPool>,
    Query(params): Query<StationSearchQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let pagination = PaginationParams {
        page: params.page,
        size: params.size,
    };

    let (stations, meta) = station_repo::search_stations(&pool, params.q.as_deref(), &pagination).await?;
    Ok(SuccessEnvelope::new(stations, meta))
}

#[cfg(test)]
mod tests {
    use common_types::api::{ItemEnvelope, PaginationMeta, SuccessEnvelope};
    use serde_json::json;

    #[test]
    fn test_success_envelope_serialization() {
        let data = json!([{"id": "STN-001", "name": "Station A"}]);
        let meta = PaginationMeta {
            page: 1,
            size: 20,
            total: 42,
            total_pages: 3,
            has_next: true,
            has_prev: false,
        };
        let envelope = SuccessEnvelope::new(data, meta);
        let value = serde_json::to_value(envelope).unwrap();

        assert_eq!(value["success"], true);
        assert!(value.get("data").is_some());
        assert!(value.get("meta").is_some());
        assert_eq!(value["meta"]["page"], 1);
        assert_eq!(value["meta"]["size"], 20);
        assert_eq!(value["meta"]["total"], 42);
        assert_eq!(value["meta"]["total_pages"], 3);
        assert_eq!(value["meta"]["has_next"], true);
        assert_eq!(value["meta"]["has_prev"], false);
    }

    #[test]
    fn test_success_envelope_empty_data() {
        let data: Vec<String> = vec![];
        let meta = PaginationMeta {
            page: 1, size: 20, total: 0, total_pages: 0,
            has_next: false, has_prev: false,
        };
        let envelope = SuccessEnvelope::new(data, meta);
        let value = serde_json::to_value(envelope).unwrap();

        assert_eq!(value["success"], true);
        assert_eq!(value["data"], json!([]));
        assert_eq!(value["meta"]["total"], 0);
    }

    #[test]
    fn test_item_envelope_serialization() {
        let data = json!({"id": "STN-001", "name": "Station A"});
        let envelope = ItemEnvelope::new(data);
        let value = serde_json::to_value(envelope).unwrap();

        assert_eq!(value["success"], true);
        assert!(value.get("data").is_some());
        assert!(value.get("meta").is_some());
        assert_eq!(value["meta"], json!({}));
    }
}
