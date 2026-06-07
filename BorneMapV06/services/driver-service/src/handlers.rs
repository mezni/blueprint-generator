//! HTTP handlers for API endpoints

use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::error::AppResult;

/// Nearby stations handler (public, no auth required)
///
/// Returns stations sorted by distance within the specified radius.
/// Supports rate limiting via middleware.
pub async fn nearby_handler(
    query: web::Query<NearbyQuery>,
    pool: web::Data<PgPool>,
) -> AppResult<impl Responder> {
    let query = query.into_inner();

    // Validate coordinates
    validate_coordinates(query.lat, query.lng)?;

    // Validate radius
    validate_radius(query.radius_m)?;

    // TODO: Implement actual SQLx query using ST_DWithin
    // This is a placeholder implementation
    tracing::info!("Processing nearby stations request: lat={}, lng={}, radius={}m", query.lat, query.lng, query.radius_m);

    let stations = vec![]; // TODO: Fetch from database

    Ok(HttpResponse::Ok().json(NearbyResponse {
        stations,
        query: NearbyQuery {
            latitude: query.lat,
            longitude: query.lng,
            radius_m: query.radius_m,
        },
    }))
}

/// List favorites handler (authenticated)
pub async fn list_favorites_handler(
    claims: web::Data<ev_auth::Claims>,
    query: web::Query<PageQuery>,
    pool: web::Data<PgPool>,
) -> AppResult<impl Responder> {
    let query = query.into_inner();

    // TODO: Implement actual SQLx query
    tracing::info!("Listing favorites for user: {}", claims.sub);

    let favorites = vec![]; // TODO: Fetch from database

    Ok(HttpResponse::Ok().json(FavoritesResponse {
        favorites,
        pagination: PageResponse {
            total: favorites.len() as i64,
            limit: query.limit,
            offset: query.offset,
        },
    }))
}

/// Create favorite handler (authenticated)
pub async fn create_favorite_handler(
    claims: web::Data<ev_auth::Claims>,
    input: web::Json<CreateFavoriteRequest>,
    pool: web::Data<PgPool>,
) -> AppResult<impl Responder> {
    // Validate user role (only registered_driver can create favorites)
    if claims.role != ev_auth::Role::RegisteredDriver {
        return Err("Only registered drivers can create favorites".into());
    }

    // TODO: Implement actual SQLx insert using favorites use case
    tracing::info!("Creating favorite for user: {} for station: {}", claims.sub, input.station_id);

    // TODO: Validate station exists
    // TODO: Check if station already favorited

    Ok(HttpResponse::Created().json(CreateFavoriteResponse {
        id: "FAV-mock-123".to_string(),
        user_id: claims.sub,
        station_id: input.station_id.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// List favorites handler (authenticated)
pub async fn list_favorites_handler(
    claims: web::Data<ev_auth::Claims>,
    query: web::Query<PageQuery>,
    pool: web::Data<PgPool>,
) -> AppResult<impl Responder> {
    // Validate user role (only registered_driver can list favorites)
    if claims.role != ev_auth::Role::RegisteredDriver {
        return Err("Only registered drivers can list favorites".into());
    }

    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    // TODO: Implement actual SQLx query using favorites use case
    tracing::info!("Listing favorites for user: {} with limit: {}, offset: {}", claims.sub, limit, offset);

    let favorites = vec![]; // TODO: Fetch from database

    Ok(HttpResponse::Ok().json(FavoritesResponse {
        favorites,
        pagination: PageResponse {
            total: favorites.len() as i64,
            limit: limit as i32,
            offset: offset as i32,
        },
    }))
}

/// Remove favorite handler (authenticated)
pub async fn remove_favorite_handler(
    claims: web::Data<ev_auth::Claims>,
    favorite_id: web::Path<String>,
    pool: web::Data<PgPool>,
) -> AppResult<impl Responder> {
    // Validate user role (only registered_driver can remove favorites)
    if claims.role != ev_auth::Role::RegisteredDriver {
        return Err("Only registered drivers can remove favorites".into());
    }

    // TODO: Validate user owns this favorite
    tracing::info!("Removing favorite: {} for user: {}", favorite_id, claims.sub);

    // TODO: Implement actual SQLx delete (hard delete per research decisions)
    // DELETE FROM users.favorite WHERE id = ? AND user_id = ?

    Ok(HttpResponse::NoContent().finish())
}

    Ok(HttpResponse::NoContent().finish())
}

/// Validation functions

fn validate_coordinates(lat: f64, lng: f64) -> AppResult<()> {
    use ev_domain::validation;
    validation::validate_latitude(lat)?;
    validation::validate_longitude(lng)?;
    Ok(())
}

fn validate_radius(radius_m: i32) -> AppResult<()> {
    use ev_domain::validation;
    validation::validate_radius(radius_m)?;
    Ok(())
}

// ============================================================================
// Request/Response DTOs
// ============================================================================

#[derive(Debug, serde::Deserialize)]
pub struct NearbyQuery {
    pub lat: f64,
    pub lng: f64,
    pub radius_m: i32,
}

#[derive(Debug, serde::Serialize)]
pub struct NearbyResponse {
    pub stations: Vec<NearbyStation>,
    pub query: NearbyQuery,
}

#[derive(Debug, serde::Serialize)]
pub struct NearbyStation {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub distance_m: i32,
    pub availability_status: String,
    pub capacity: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateFavoriteRequest {
    pub station_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateFavoriteResponse {
    pub id: String,
    pub user_id: String,
    pub station_id: String,
    pub created_at: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct PageQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, serde::Serialize)]
pub struct FavoritesResponse {
    pub favorites: Vec<FavoriteWithStation>,
    pub pagination: PageResponse,
}

#[derive(Debug, serde::Serialize)]
pub struct FavoriteWithStation {
    pub id: String,
    pub station: NearbyStation,
    pub created_at: String,
}

#[derive(Debug, serde::Serialize)]
pub struct PageResponse {
    pub total: i64,
    pub limit: i32,
    pub offset: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearby_query_validation() {
        // Valid coordinates
        assert!(validate_coordinates(36.8, 10.1).is_ok());
        assert!(validate_latitude(90.0).is_ok());
        assert!(validate_longitude(180.0).is_ok());

        // Invalid coordinates
        assert!(validate_latitude(91.0).is_err());
        assert!(validate_longitude(181.0).is_err());
    }

    #[test]
    fn test_nearby_query_bounds() {
        // Valid radius
        assert!(validate_radius(100).is_ok());
        assert!(validate_radius(50000).is_ok());

        // Invalid radius
        assert!(validate_radius(99).is_err());
        assert!(validate_radius(50001).is_err());
    }

    #[test]
    fn test_favorite_creation_requires_driver_role() {
        let driver_claims = ev_auth::Claims {
            sub: "driver123".to_string(),
            email: Some("driver@example.com".to_string()),
            name: Some("Driver".to_string()),
            role: ev_auth::Role::RegisteredDriver,
            partner_id: None,
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        let partner_claims = ev_auth::Claims {
            sub: "partner123".to_string(),
            email: Some("partner@example.com".to_string()),
            name: Some("Partner".to_string()),
            role: ev_auth::Role::Partner,
            partner_id: Some("PRT-123".to_string()),
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        assert_eq!(driver_claims.role, ev_auth::Role::RegisteredDriver);
        assert_eq!(partner_claims.role, ev_auth::Role::Partner);
    }
}
