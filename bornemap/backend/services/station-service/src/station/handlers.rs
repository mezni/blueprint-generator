use crate::auth::middleware;
use crate::error_adapter::ApiError;
use crate::station::models::*;
use crate::station::service::StationService;
use actix_web::{web, HttpResponse};
use common_utils::error::DomainError;

/// List stations inside a viewport bounding box
#[utoipa::path(
    get,
    path = "/api/v1/stations",
    params(
        ("bbox" = String, Query, description = "Comma-separated west,south,east,north")
    ),
    responses(
        (status = 200, description = "Marker summaries", body = StationListResponse),
        (status = 400, description = "Invalid bbox"),
    ),
    tag = "stations"
)]
pub async fn stations_list_by_viewport(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let bbox = query
        .get("bbox")
        .ok_or_else(|| DomainError::Validation("Missing bbox query parameter".into()))?;

    let response = StationService::list_by_viewport(pool.get_ref(), bbox).await?;
    Ok(HttpResponse::Ok().json(response))
}

/// Full station detail
#[utoipa::path(
    get,
    path = "/api/v1/stations/{id}",
    params(
        ("id" = String, Path, description = "Station UUID")
    ),
    responses(
        (status = 200, description = "Station detail", body = StationDetail),
        (status = 404, description = "Not found"),
    ),
    tag = "stations"
)]
pub async fn stations_get_by_id(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let detail = StationService::get_by_id(pool.get_ref(), &id).await?;
    Ok(HttpResponse::Ok().json(detail))
}

/// Admin paginated list of stations
#[utoipa::path(
    get,
    path = "/api/v1/admin/stations",
    params(
        ("limit" = Option<i64>, Query, description = "Page size (1-200)"),
        ("cursor" = Option<String>, Query, description = "Pagination cursor"),
        ("include_deleted" = Option<bool>, Query, description = "Include soft-deleted"),
        ("include_test" = Option<bool>, Query, description = "Include test rows"),
    ),
    responses(
        (status = 200, description = "Paginated station rows", body = AdminStationListResponse),
        (status = 401, description = "Missing/expired token"),
        (status = 403, description = "Wrong role"),
    ),
    tag = "admin",
    security(("bearerAuth" = ["admin"]))
)]
pub async fn admin_stations_list(
    pool: web::Data<sqlx::PgPool>,
    secret: web::Data<String>,
    req: actix_web::HttpRequest,
    query: web::Query<AdminListQuery>,
) -> Result<HttpResponse, ApiError> {
    let claims = middleware::extract_claims_from_req(&req, secret.get_ref())?;
    middleware::require_admin(&claims)?;

    let limit = query.limit.unwrap_or(50).clamp(1, 200);
    let response = StationService::admin_list(
        pool.get_ref(),
        limit,
        query.cursor.as_deref(),
        query.include_deleted.unwrap_or(false),
        query.include_test.unwrap_or(true),
    )
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Create a station
#[utoipa::path(
    post,
    path = "/api/v1/admin/stations",
    request_body = AdminStationCreate,
    responses(
        (status = 201, description = "Created", body = StationDetail),
        (status = 401, description = "Missing/expired token"),
        (status = 403, description = "Wrong role"),
        (status = 422, description = "Validation failed"),
    ),
    tag = "admin",
    security(("bearerAuth" = ["admin"]))
)]
pub async fn admin_stations_create(
    pool: web::Data<sqlx::PgPool>,
    secret: web::Data<String>,
    req: actix_web::HttpRequest,
    body: web::Json<AdminStationCreate>,
) -> Result<HttpResponse, ApiError> {
    let claims = middleware::extract_claims_from_req(&req, secret.get_ref())?;
    middleware::require_admin(&claims)?;

    let detail = StationService::admin_create(pool.get_ref(), &body).await?;
    Ok(HttpResponse::Created().json(detail))
}

/// Partial update
#[utoipa::path(
    patch,
    path = "/api/v1/admin/stations/{id}",
    request_body = AdminStationPatch,
    responses(
        (status = 200, description = "Updated", body = StationDetail),
        (status = 401, description = "Missing/expired token"),
        (status = 403, description = "Wrong role"),
        (status = 404, description = "Not found"),
        (status = 422, description = "Validation failed"),
    ),
    tag = "admin",
    security(("bearerAuth" = ["admin"]))
)]
pub async fn admin_stations_patch(
    pool: web::Data<sqlx::PgPool>,
    secret: web::Data<String>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<AdminStationPatch>,
) -> Result<HttpResponse, ApiError> {
    let claims = middleware::extract_claims_from_req(&req, secret.get_ref())?;
    middleware::require_admin(&claims)?;

    let id = path.into_inner();
    let detail = StationService::admin_patch(pool.get_ref(), &id, &body).await?;
    Ok(HttpResponse::Ok().json(detail))
}

/// Soft-delete a station
#[utoipa::path(
    delete,
    path = "/api/v1/admin/stations/{id}",
    responses(
        (status = 204, description = "Deleted"),
        (status = 401, description = "Missing/expired token"),
        (status = 403, description = "Wrong role"),
        (status = 404, description = "Not found"),
    ),
    tag = "admin",
    security(("bearerAuth" = ["admin"]))
)]
pub async fn admin_stations_soft_delete(
    pool: web::Data<sqlx::PgPool>,
    secret: web::Data<String>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let claims = middleware::extract_claims_from_req(&req, secret.get_ref())?;
    middleware::require_admin(&claims)?;

    let id = path.into_inner();
    StationService::admin_soft_delete(pool.get_ref(), &id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, serde::Deserialize)]
pub struct AdminListQuery {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub include_deleted: Option<bool>,
    pub include_test: Option<bool>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/stations").route(web::get().to(stations_list_by_viewport)))
        .service(web::resource("/stations/{id}").route(web::get().to(stations_get_by_id)))
        .service(
            web::resource("/admin/stations")
                .route(web::get().to(admin_stations_list))
                .route(web::post().to(admin_stations_create)),
        )
        .service(
            web::resource("/admin/stations/{id}")
                .route(web::patch().to(admin_stations_patch))
                .route(web::delete().to(admin_stations_soft_delete)),
        );
}

impl crate::auth::middleware::TokenClaimsExtractor for actix_web::HttpRequest {
    fn extract_claims(
        &self,
        secret: &str,
    ) -> Result<crate::auth::claims::TokenClaims, DomainError> {
        let auth_header = self
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| DomainError::Unauthorized("Missing Authorization header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| DomainError::Unauthorized("Invalid Authorization scheme".into()))?;

        let token_data = jsonwebtoken::decode::<crate::auth::claims::TokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|e| DomainError::Unauthorized(format!("Invalid token: {e}")))?;

        Ok(token_data.claims)
    }
}
