//! Nearby stations HTTP handler

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::application::NearbyStationsUseCase;
use crate::domain::{NearbyQuery, Pagination};
use crate::errors::{ApiError, AppResult};
use crate::interface::dto::{NearbyRequest, NearbyResponse, PaginationDTO, StationDTO};

/// Nearby stations handler
pub async fn nearby_handler(
    request: HttpRequest,
    query: web::Query<NearbyRequest>,
    pool: web::Data<PgPool>,
) -> AppResult<impl Responder> {
    // Parse query parameters
    let latitude = query.latitude.ok_or_else(|| {
        ApiError::BadRequest("latitude parameter is required".to_string())
    })?;

    let longitude = query.longitude.ok_or_else(|| {
        ApiError::BadRequest("longitude parameter is required".to_string())
    })?;

    let radius_km = query.radius_km.unwrap_or(10.0);
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    // Create nearby query
    let nearby_query = NearbyQuery::new(latitude, longitude, radius_km)
        .with_limit(limit)
        .with_offset(offset);

    // Use case
    let usecase = NearbyStationsUseCase::new(pool.clone());

    // Find nearby stations
    let result = usecase.find_nearby(nearby_query).await?;

    // Build response
    let pagination = PaginationDTO::from_pagination(offset / limit + 1, limit, result.total);

    let stations: Vec<StationDTO> = result
        .stations
        .iter()
        .map(|s| StationDTO::from_station_with_metadata(s.clone()))
        .collect();

    let response = NearbyResponse {
        success: true,
        message: format!("Found {} stations within {}km", result.total, radius_km),
        stations,
        pagination,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Health check handler
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "driver-service",
        "version": "0.1.0"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearby_handler_structure() {
        // Test structure only - actual tests would need mock database
        assert!(true);
    }
}
