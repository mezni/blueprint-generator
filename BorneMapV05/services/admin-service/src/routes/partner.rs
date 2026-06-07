use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use common_auth::CurrentUser;
use common_types::{AvailabilitySource, StationAvailabilityStatus, StationStatus};
use serde::Serialize;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::charger::{ChargerCreate, ChargerUpdate};
use crate::models::station::{validate_coordinates, validate_status_transition, StationCreate, StationUpdate};
use crate::repository::{availability_repo, charger_repo, idempotency_repo, outbox_repo, station_repo};
use common_types::api::{ItemEnvelope, SuccessEnvelope};

fn get_header(headers: &HeaderMap, name: &str) -> Result<String, ServiceError> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| ServiceError::validation(format!("Missing required header: {}", name)))
}

pub fn station_routes(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .route("/api/v1/partner/stations", axum::routing::get(list_stations).post(create_station))
        .route(
            "/api/v1/partner/stations/{id}",
            axum::routing::patch(update_station).delete(delete_station),
        )
        .with_state(pool)
}

pub fn charger_routes(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .route("/api/v1/partner/chargers", axum::routing::get(list_chargers).post(create_charger))
        .route("/api/v1/partner/chargers/{id}", axum::routing::patch(update_charger))
        .route("/api/v1/partner/stations/{id}/availability", axum::routing::patch(update_availability))
        .with_state(pool)
}

pub fn profile_routes(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .route("/api/v1/partner/me", axum::routing::get(get_profile))
        .with_state(pool)
}

// ---- Station handlers ----

async fn list_stations(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = user.partner_id.ok_or(ServiceError::partner_scope_violation())?;
    let (stations, meta) = station_repo::list_partner_stations(
        &pool,
        &partner_id,
        &params,
        false,
        None::<StationStatus>,
    )
    .await?;
    Ok(SuccessEnvelope::new(stations, meta))
}

async fn create_station(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(body): Json<StationCreate>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = user.partner_id.ok_or(ServiceError::partner_scope_violation())?;
    let key = get_header(&headers, "Idempotency-Key")?;

    validate_coordinates(body.latitude, body.longitude)?;

    let is_duplicate = idempotency_repo::check_and_insert(&pool, &key, "pending")
        .await
        .map_err(|e| ServiceError::internal(format!("idempotency check failed: {e}")))?;

    if is_duplicate {
        return Err(ServiceError::Api(common_errors::ApiError {
            code: common_errors::ErrorCode::AlreadyExists,
            message: "Request with this idempotency key has already been processed".into(),
            details: None,
        }));
    }

    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    let station = station_repo::create_station(&mut tx, &partner_id, &body).await?;

    idempotency_repo::insert_in_tx(&mut tx, &key, &station.station_id)
        .await
        .map_err(ServiceError::Db)?;

    outbox_repo::insert_outbox_entry(&mut tx, "station", &station.station_id, "insert")
        .await
        .map_err(ServiceError::Db)?;

    tx.commit().await.map_err(ServiceError::Db)?;

    tracing::info!(
        entity_type = "station",
        entity_id = %station.station_id,
        partner_id = %partner_id,
        operation = "insert",
        "Station created"
    );

    Ok((StatusCode::CREATED, ItemEnvelope::new(station)))
}

async fn update_station(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(body): Json<StationUpdate>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = user.partner_id.ok_or(ServiceError::partner_scope_violation())?;
    let etag = get_header(&headers, "If-Match")?;

    let current = station_repo::find_by_id(&pool, &id).await?;
    if current.partner_id != partner_id {
        return Err(ServiceError::partner_scope_violation());
    }

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
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = user.partner_id.ok_or(ServiceError::partner_scope_violation())?;

    let current = station_repo::find_by_id(&pool, &id).await?;
    if current.partner_id != partner_id {
        return Err(ServiceError::partner_scope_violation());
    }

    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    station_repo::soft_delete_station(&mut tx, &id).await?;

    outbox_repo::insert_outbox_entry(&mut tx, "station", &id, "delete")
        .await
        .map_err(ServiceError::Db)?;

    tx.commit().await.map_err(ServiceError::Db)?;

    Ok(ItemEnvelope::new(current))
}

// ---- Charger handlers ----

#[derive(serde::Deserialize)]
pub struct ListChargersQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub station_id: Option<String>,
}

async fn list_chargers(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Query(params): Query<ListChargersQuery>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = user.partner_id.ok_or(ServiceError::partner_scope_violation())?;

    let pagination = PaginationParams {
        page: params.page,
        size: params.size,
    };

    let (chargers, meta) = charger_repo::list_partner_chargers(
        &pool,
        &partner_id,
        &pagination,
        params.station_id.as_deref(),
    )
    .await?;

    Ok(SuccessEnvelope::new(chargers, meta))
}

async fn create_charger(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Json(body): Json<ChargerCreate>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = user.partner_id.ok_or(ServiceError::partner_scope_violation())?;

    charger_repo::verify_station_belongs_to_partner(&pool, &body.station_id, &partner_id).await?;

    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    let charger = charger_repo::create_charger(&mut tx, &body.station_id, &body).await?;

    tx.commit().await.map_err(ServiceError::Db)?;

    Ok((StatusCode::CREATED, ItemEnvelope::new(charger)))
}

async fn update_charger(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(body): Json<ChargerUpdate>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = user.partner_id.ok_or(ServiceError::partner_scope_violation())?;
    let etag = get_header(&headers, "If-Match")?;

    charger_repo::verify_charger_belongs_to_partner(&pool, &id, &partner_id).await?;

    let expected_updated_at = chrono::DateTime::parse_from_rfc3339(&etag)
        .map_err(|_| ServiceError::validation("If-Match must be an RFC 3339 timestamp"))?
        .with_timezone(&chrono::Utc);

    let mut tx = pool.begin().await.map_err(ServiceError::Db)?;

    let updated = charger_repo::update_charger(&mut tx, &id, &body, expected_updated_at).await?;

    tx.commit().await.map_err(ServiceError::Db)?;

    Ok(ItemEnvelope::new(updated))
}

#[derive(serde::Deserialize)]
struct AvailabilityUpdate {
    status: StationAvailabilityStatus,
}

async fn update_availability(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(body): Json<AvailabilityUpdate>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = user.partner_id.ok_or(ServiceError::partner_scope_violation())?;

    availability_repo::verify_station_belongs_to_partner(&pool, &id, &partner_id).await?;

    let availability = availability_repo::upsert_availability(
        &pool,
        &id,
        body.status,
        AvailabilitySource::ManualPartner,
    )
    .await?;

    Ok(ItemEnvelope::new(availability))
}

// ---- Profile handler ----

#[derive(Serialize)]
struct ProfileResponse {
    user_id: String,
    email: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    membership_role: Option<String>,
}

async fn get_profile(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ServiceError> {
    let partner_id = match user.partner_id.as_deref() {
        Some(pid) => pid.to_string(),
        None => {
            return Ok(ItemEnvelope::new(ProfileResponse {
                user_id: user.user_id.clone(),
                email: user.email.clone(),
                partner_id: None,
                partner_name: None,
                membership_role: None,
            }))
        }
    };

    let profile: Option<(String, String)> = sqlx::query_as(
        "SELECT p.name, pm.role FROM inventory.partner p \
         JOIN platform_db.partner_membership pm ON p.partner_id = pm.partner_id \
         WHERE p.partner_id = $1 AND pm.user_id = $2 AND p.deleted_at IS NULL",
    )
    .bind(&partner_id)
    .bind(&user.user_id)
    .fetch_optional(&pool)
    .await
    .map_err(ServiceError::Db)?;

    let (partner_name, membership_role) = match profile {
        Some((name, role)) => (Some(name), Some(role)),
        None => (None, None),
    };

    Ok(ItemEnvelope::new(ProfileResponse {
        user_id: user.user_id.clone(),
        email: user.email.clone(),
        partner_id: user.partner_id.clone(),
        partner_name,
        membership_role,
    }))
}
