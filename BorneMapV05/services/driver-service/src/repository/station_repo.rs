use common_types::api::PaginationMeta;
use common_types::StationAvailabilityStatus;
use sqlx::{PgPool, Row};

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::station::{
    ChargerTypeInfo, GeoPoint, ReviewSummary, StationDetail, StationListItem, StationListQuery,
};

const VISIBILITY: &str = "s.is_live = true AND s.deleted_at IS NULL AND s.status = 'active' AND s.is_public = true";

#[derive(Debug, sqlx::FromRow)]
struct StationRow {
    id: String,
    name: String,
    description: Option<String>,
    latitude: f64,
    longitude: f64,
    city: Option<String>,
    country: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
struct StationRowWithDist {
    id: String,
    name: String,
    description: Option<String>,
    latitude: f64,
    longitude: f64,
    city: Option<String>,
    country: Option<String>,
    distance_km: Option<f64>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

fn build_vis_count_sql() -> String {
    format!("SELECT COUNT(*) FROM inventory.station s WHERE {}", VISIBILITY)
}

fn build_vis_list_sql() -> String {
    format!(
        "SELECT s.id, s.name, s.description, s.latitude, s.longitude, s.city, s.country, \
         s.created_at, s.updated_at \
         FROM inventory.station s WHERE {} ORDER BY s.created_at DESC LIMIT $1 OFFSET $2",
        VISIBILITY
    )
}

fn build_vis_geo_count_sql() -> String {
    format!(
        "SELECT COUNT(*) FROM inventory.station s \
         WHERE {} AND ST_DWithin(s.geom, ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography, $3)",
        VISIBILITY
    )
}

fn build_vis_geo_list_sql() -> String {
    format!(
        "SELECT s.id, s.name, s.description, s.latitude, s.longitude, s.city, s.country, \
         ROUND((ST_Distance(s.geom, ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography) / 1000.0)::numeric, 2)::float8 AS distance_km, \
         s.created_at, s.updated_at \
         FROM inventory.station s \
         WHERE {} AND ST_DWithin(s.geom, ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography, $3) \
         ORDER BY distance_km ASC NULLS LAST LIMIT $4 OFFSET $5",
        VISIBILITY
    )
}

fn build_search_count_sql() -> String {
    format!(
        "SELECT COUNT(*) FROM inventory.station s \
         WHERE {} AND (s.name ILIKE $1 OR s.city ILIKE $1 OR COALESCE(s.description, '') ILIKE $1)",
        VISIBILITY
    )
}

fn build_search_list_sql() -> String {
    format!(
        "SELECT s.id, s.name, s.description, s.latitude, s.longitude, s.city, s.country, \
         s.created_at, s.updated_at \
         FROM inventory.station s \
         WHERE {} AND (s.name ILIKE $1 OR s.city ILIKE $1 OR COALESCE(s.description, '') ILIKE $1) \
         ORDER BY s.name ASC LIMIT $2 OFFSET $3",
        VISIBILITY
    )
}

fn build_station_detail_sql() -> String {
    format!(
        "SELECT s.id, s.name, s.description, s.latitude, s.longitude, s.city, s.country, \
         s.created_at, s.updated_at \
         FROM inventory.station s WHERE s.id = $1 AND {}",
        VISIBILITY
    )
}

fn build_geo_dist_sql() -> String {
    "SELECT ROUND((ST_Distance(s.geom, \
     ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography) / 1000.0)::numeric, 2)::float8 \
     FROM inventory.station s WHERE s.id = $3".to_string()
}

async fn get_charger_types(
    pool: &PgPool,
    station_id: &str,
) -> Result<Vec<ChargerTypeInfo>, ServiceError> {
    let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(
        "SELECT c.type, c.power_kw, c.status FROM inventory.charger c \
         WHERE c.station_id = $1 AND c.deleted_at IS NULL",
    )
    .bind(station_id)
    .fetch_all(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(rows
        .iter()
        .map(|r| ChargerTypeInfo {
            connector_type: r.get("type"),
            power_kw: r.get("power_kw"),
            status: r.get("status"),
        })
        .collect())
}

async fn get_availability(
    pool: &PgPool,
    station_id: &str,
) -> Result<Option<StationAvailabilityStatus>, ServiceError> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT status FROM inventory.station_availability \
         WHERE station_id = $1 ORDER BY updated_at DESC LIMIT 1",
    )
    .bind(station_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(row.and_then(|r| StationAvailabilityStatus::from_str(&r.0)))
}

async fn get_review_summary(
    pool: &PgPool,
    station_id: &str,
) -> Result<Option<ReviewSummary>, ServiceError> {
    let row: Option<(Option<f64>, i64)> = sqlx::query_as(
        "SELECT AVG(rating)::float8, COUNT(*) FROM users.station_review \
         WHERE station_id = $1 AND status = 'published'",
    )
    .bind(station_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(row.map(|(avg, cnt)| ReviewSummary {
        average_rating: avg,
        total_reviews: cnt,
    }))
}

async fn row_to_list_item(
    pool: &PgPool,
    id: String,
    name: String,
    description: Option<String>,
    latitude: f64,
    longitude: f64,
    city: Option<String>,
    country: Option<String>,
    distance_km: Option<f64>,
) -> Result<StationListItem, ServiceError> {
    let charger_types = get_charger_types(pool, &id).await?;
    let availability = get_availability(pool, &id).await?;
    let review_summary = get_review_summary(pool, &id).await?;

    Ok(StationListItem {
        id,
        name,
        description,
        latitude,
        longitude,
        city,
        country,
        distance_km,
        geom: Some(GeoPoint { lat: latitude, lng: longitude }),
        charger_types,
        availability,
        review_summary,
    })
}

fn make_meta(total: i64, pagination: &PaginationParams) -> PaginationMeta {
    let total_i32 = total as i32;
    let size = pagination.size();
    let total_pages = total_i32.div_euclid(size) + if total_i32 % size != 0 { 1 } else { 0 };
    PaginationMeta {
        page: pagination.page(),
        size,
        total: total_i32,
        total_pages: total_pages.max(0),
        has_next: pagination.page() < total_pages,
        has_prev: pagination.page() > 1,
    }
}

pub async fn list_visible_stations(
    pool: &PgPool,
    params: &StationListQuery,
    pagination: &PaginationParams,
) -> Result<(Vec<StationListItem>, PaginationMeta), ServiceError> {
    let use_geo = params.lat.is_some() && params.lng.is_some();
    let lat = params.lat.unwrap_or(36.8065);
    let lng = params.lng.unwrap_or(10.1815);
    let radius_km = params.radius_km.unwrap_or(10.0).min(50.0);

    if use_geo {
        let count: (i64,) = sqlx::query_as(&build_vis_geo_count_sql())
            .bind(lng)
            .bind(lat)
            .bind(radius_km * 1000.0)
            .fetch_one(pool)
            .await
            .map_err(ServiceError::Db)?;

        let rows = sqlx::query_as::<_, StationRowWithDist>(&build_vis_geo_list_sql())
            .bind(lng)
            .bind(lat)
            .bind(radius_km * 1000.0)
            .bind(pagination.limit())
            .bind(pagination.offset())
            .fetch_all(pool)
            .await
            .map_err(ServiceError::Db)?;

        let mut items = Vec::with_capacity(rows.len());
        for r in rows {
            items.push(
                row_to_list_item(pool, r.id, r.name, r.description, r.latitude, r.longitude, r.city, r.country, r.distance_km)
                    .await?,
            );
        }

        Ok((items, make_meta(count.0, pagination)))
    } else {
        let count: (i64,) = sqlx::query_as(&build_vis_count_sql())
            .fetch_one(pool)
            .await
            .map_err(ServiceError::Db)?;

        let rows = sqlx::query_as::<_, StationRow>(&build_vis_list_sql())
            .bind(pagination.limit())
            .bind(pagination.offset())
            .fetch_all(pool)
            .await
            .map_err(ServiceError::Db)?;

        let mut items = Vec::with_capacity(rows.len());
        for r in rows {
            items.push(
                row_to_list_item(pool, r.id, r.name, r.description, r.latitude, r.longitude, r.city, r.country, None)
                    .await?,
            );
        }

        Ok((items, make_meta(count.0, pagination)))
    }
}

pub async fn get_station_detail(
    pool: &PgPool,
    station_id: &str,
    user_lat: Option<f64>,
    user_lng: Option<f64>,
) -> Result<StationDetail, ServiceError> {
    let row = sqlx::query_as::<_, StationRow>(&build_station_detail_sql())
        .bind(station_id)
        .fetch_optional(pool)
        .await
        .map_err(ServiceError::Db)?
        .ok_or_else(|| ServiceError::not_found("Station", station_id))?;

    let distance_km = match (user_lat, user_lng) {
        (Some(lat), Some(lng)) => {
            sqlx::query_as::<_, (Option<f64>,)>(&build_geo_dist_sql())
                .bind(lng)
                .bind(lat)
                .bind(station_id)
                .fetch_optional(pool)
                .await
                .map_err(ServiceError::Db)?
                .and_then(|d| d.0)
        }
        _ => None,
    };

    let charger_types = get_charger_types(pool, station_id).await?;
    let availability = get_availability(pool, station_id).await?;
    let review_summary = get_review_summary(pool, station_id).await?;

    let chargers: Vec<crate::models::charger::Charger> = sqlx::query_as(
        "SELECT c.id, c.station_id, c.type, c.power_kw, c.status, c.created_at, c.updated_at \
         FROM inventory.charger c WHERE c.station_id = $1 AND c.deleted_at IS NULL",
    )
    .bind(station_id)
    .fetch_all(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(StationDetail {
        id: row.id,
        name: row.name,
        description: row.description,
        latitude: row.latitude,
        longitude: row.longitude,
        city: row.city,
        country: row.country,
        distance_km,
        geom: Some(GeoPoint { lat: row.latitude, lng: row.longitude }),
        chargers,
        charger_types,
        availability,
        review_summary,
    })
}

pub async fn search_stations(
    pool: &PgPool,
    q: Option<&str>,
    pagination: &PaginationParams,
) -> Result<(Vec<StationListItem>, PaginationMeta), ServiceError> {
    let search_term = q.map(|s| format!("%{}%", s));

    if let Some(ref term) = search_term {
        let count: (i64,) = sqlx::query_as(&build_search_count_sql())
            .bind(term)
            .fetch_one(pool)
            .await
            .map_err(ServiceError::Db)?;

        let rows = sqlx::query_as::<_, StationRow>(&build_search_list_sql())
            .bind(term)
            .bind(pagination.limit())
            .bind(pagination.offset())
            .fetch_all(pool)
            .await
            .map_err(ServiceError::Db)?;

        let mut items = Vec::with_capacity(rows.len());
        for r in rows {
            items.push(
                row_to_list_item(pool, r.id, r.name, r.description, r.latitude, r.longitude, r.city, r.country, None)
                    .await?,
            );
        }

        Ok((items, make_meta(count.0, pagination)))
    } else {
        list_visible_stations(pool, &StationListQuery::default(), pagination).await
    }
}
