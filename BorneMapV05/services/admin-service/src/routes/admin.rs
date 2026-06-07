use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use common_auth::CurrentUser;
use common_types::{PartnerStatus, ReviewStatus, StationStatus};
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::station::{validate_coordinates, validate_status_transition, StationUpdate};
use crate::repository::{
    outbox_repo, partner_repo, review_repo, station_repo, user_repo,
};
use crate::repository::partner_repo::PartnerUpdateRequest;
use common_types::api::{ItemEnvelope, SuccessEnvelope};

#[derive(serde::Serialize)]
pub struct OverviewMetrics {
    pub total_partners: i64,
    pub total_stations: i64,
    pub active_stations: i64,
    pub pending_reviews: i64,
}

fn get_header(headers: &HeaderMap, name: &str) -> Result<String, ServiceError> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| ServiceError::validation(format!("Missing required header: {}", name)))
}

async fn get_overview(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ServiceError> {
    let total_partners: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM inventory.partner WHERE deleted_at IS NULL"
    )
    .fetch_one(&pool)
    .await
    .map_err(ServiceError::Db)?;

    let total_stations: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM inventory.station WHERE deleted_at IS NULL"
    )
    .fetch_one(&pool)
    .await
    .map_err(ServiceError::Db)?;

    let active_stations: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM inventory.station WHERE deleted_at IS NULL AND status = 'active'"
    )
    .fetch_one(&pool)
    .await
    .map_err(ServiceError::Db)?;

    let pending_reviews: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM inventory.review WHERE status = 'submitted'"
    )
    .fetch_one(&pool)
    .await
    .map_err(ServiceError::Db)?;

    let metrics = OverviewMetrics {
        total_partners: total_partners.0,
        total_stations: total_stations.0,
        active_stations: active_stations.0,
        pending_reviews: pending_reviews.0,
    };

    Ok(ItemEnvelope::new(metrics))
}

pub fn station_routes(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .route("/api/v1/admin/stations", axum::routing::get(list_stations))
        .route(
            "/api/v1/admin/stations/{id}",
            axum::routing::patch(update_station).delete(delete_station),
        )
        .with_state(pool)
}

pub fn partner_routes(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .route("/api/v1/admin/overview", axum::routing::get(get_overview))
        .route("/api/v1/admin/partners", axum::routing::get(list_partners).post(create_partner))
        .route(
            "/api/v1/admin/partners/{id}",
            axum::routing::patch(update_partner).delete(delete_partner),
        )
        .route("/api/v1/admin/users", axum::routing::get(list_users))
        .route("/api/v1/admin/users/{id}", axum::routing::patch(update_user))
        .route("/api/v1/admin/reviews", axum::routing::get(list_reviews))
        .route("/api/v1/admin/reviews/{id}/status", axum::routing::patch(moderate_review))
        .with_state(pool)
}

#[derive(serde::Deserialize)]
pub struct AdminStationQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub include_deleted: Option<bool>,
    pub status: Option<String>,
}

async fn list_stations(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Query(params): Query<AdminStationQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let pagination = PaginationParams {
        page: params.page,
        size: params.size,
    };

    let (stations, meta) = station_repo::admin_list_stations(
        &pool,
        &pagination,
        params.include_deleted.unwrap_or(false),
        None::<StationStatus>,
    )
    .await?;

    Ok(SuccessEnvelope::new(stations, meta))
}

async fn update_station(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(body): Json<StationUpdate>,
) -> Result<impl IntoResponse, ServiceError> {
    let etag = get_header(&headers, "If-Match")?;

    let current = station_repo::admin_find_by_id(&pool, &id).await?;

    if let Some(ref new_status) = body.status {
        validate_status_transition(current.status, *new_status)?;
    }

    if let Some(lat) = body.latitude {
        let lng = body.longitude.unwrap_or(current.longitude);
        validate_coordinates(lat, lng)?;
    } else if let Some(lng) = body.longitude {
        validate_coordinates(current.latitude, lng)?;
    }

    let expected_updated_at = chrono::DateTime::parse_from_rfc3339(&etag)
        .map_err(|_| ServiceError::validation("If-Match must be an RFC 3339 timestamp"))?
        .with_timezone(&chrono::Utc);

    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    let updated = station_repo::update_station(&mut tx, &id, &body, expected_updated_at).await?;

    outbox_repo::insert_outbox_entry(&mut tx, "station", &id, "update")
        .await
        .map_err(ServiceError::Db)?;

    tx.commit().await.map_err(ServiceError::Db)?;

    Ok(ItemEnvelope::new(updated))
}

async fn delete_station(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let current = station_repo::admin_find_by_id(&pool, &id).await?;

    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    station_repo::admin_delete_station(&mut tx, &id).await?;

    outbox_repo::insert_outbox_entry(&mut tx, "station", &id, "delete")
        .await
        .map_err(ServiceError::Db)?;

    tx.commit().await.map_err(ServiceError::Db)?;

    Ok(ItemEnvelope::new(current))
}

// ---- Partner handlers ----

#[derive(serde::Deserialize)]
pub struct AdminPartnerQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub include_deleted: Option<bool>,
    pub status: Option<String>,
}

async fn list_partners(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Query(params): Query<AdminPartnerQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let pagination = PaginationParams {
        page: params.page,
        size: params.size,
    };

    let status_filter = params
        .status
        .as_deref()
        .and_then(PartnerStatus::from_str);

    let (partners, meta) = partner_repo::list_admin_partners(
        &pool,
        &pagination,
        params.include_deleted.unwrap_or(false),
        status_filter,
    )
    .await?;

    Ok(SuccessEnvelope::new(partners, meta))
}

#[derive(serde::Deserialize)]
pub struct CreatePartnerBody {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

async fn create_partner(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Json(body): Json<CreatePartnerBody>,
) -> Result<impl IntoResponse, ServiceError> {
    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    let partner = partner_repo::create_partner(
        &mut tx,
        &body.name,
        body.email.as_deref(),
        body.phone.as_deref(),
    )
    .await?;

    tx.commit().await.map_err(ServiceError::Db)?;

    Ok((StatusCode::CREATED, ItemEnvelope::new(partner)))
}

async fn update_partner(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(body): Json<PartnerUpdateRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let etag = get_header(&headers, "If-Match")?;

    let expected_updated_at = chrono::DateTime::parse_from_rfc3339(&etag)
        .map_err(|_| ServiceError::validation("If-Match must be an RFC 3339 timestamp"))?
        .with_timezone(&chrono::Utc);

    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    let updated = partner_repo::update_partner(&mut tx, &id, &body, expected_updated_at).await?;

    tx.commit().await.map_err(ServiceError::Db)?;

    Ok(ItemEnvelope::new(updated))
}

async fn delete_partner(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let has_active = partner_repo::check_active_stations(&pool, &id).await?;
    if has_active {
        return Err(ServiceError::Api(common_errors::ApiError {
            code: common_errors::ErrorCode::ActiveStationsExist,
            message: format!("Cannot delete partner '{}' because it has active stations", id),
            details: None,
        }));
    }

    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    partner_repo::soft_delete_partner(&mut tx, &id).await?;

    tx.commit().await.map_err(ServiceError::Db)?;

    Ok(ItemEnvelope::new(serde_json::json!({"id": id, "deleted": true})))
}

// ---- User handlers ----

#[derive(serde::Deserialize)]
pub struct UserUpdateRequest {
    pub role: Option<String>,
}

async fn list_users(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, ServiceError> {
    let (users, meta) = user_repo::list_users(&pool, &params).await?;
    Ok(SuccessEnvelope::new(users, meta))
}

async fn update_user(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(body): Json<UserUpdateRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let updated = user_repo::update_user(&pool, &id, &body).await?;
    Ok(ItemEnvelope::new(updated))
}

// ---- Review handlers ----

#[derive(serde::Deserialize)]
pub struct AdminReviewQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub status: Option<String>,
    pub station_id: Option<String>,
}

async fn list_reviews(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Query(params): Query<AdminReviewQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let pagination = PaginationParams {
        page: params.page,
        size: params.size,
    };

    let status_filter = params.status.as_deref().and_then(ReviewStatus::from_str);

    let (reviews, meta) = review_repo::list_reviews(
        &pool,
        &pagination,
        status_filter,
        params.station_id.as_deref(),
    )
    .await?;

    Ok(SuccessEnvelope::new(reviews, meta))
}

#[derive(serde::Deserialize)]
pub struct ModerateReviewBody {
    pub status: ReviewStatus,
}

async fn moderate_review(
    Extension(_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(body): Json<ModerateReviewBody>,
) -> Result<impl IntoResponse, ServiceError> {
    let updated = review_repo::update_review_status(&pool, &id, body.status, "admin")
        .await?;

    Ok(ItemEnvelope::new(updated))
}
