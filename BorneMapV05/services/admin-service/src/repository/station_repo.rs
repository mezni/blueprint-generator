use chrono::{DateTime, Utc};
use common_types::generate_id;
use common_types::api::PaginationMeta;
use common_types::EntityPrefix;
use common_types::StationAvailabilityStatus;
use common_types::StationStatus;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Transaction;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::station::{Station, StationCreate, StationUpdate};

pub async fn find_by_id(pool: &PgPool, station_id: &str) -> Result<Station, ServiceError> {
    sqlx::query_as::<_, Station>(
        "SELECT station_id, partner_id, name, address, latitude, longitude, \
         status, availability_status, created_at, updated_at \
         FROM inventory.station WHERE station_id = $1 AND deleted_at IS NULL",
    )
    .bind(station_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::not_found("Station", station_id),
        other => ServiceError::Db(other),
    })
}

pub async fn list_partner_stations(
    pool: &PgPool,
    partner_id: &str,
    params: &PaginationParams,
    include_deleted: bool,
    status_filter: Option<StationStatus>,
) -> Result<(Vec<Station>, PaginationMeta), ServiceError> {
    let offset = params.offset();
    let limit = params.limit();

    let (stations, total): (Vec<Station>, i64) = if let Some(ref status) = status_filter {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM inventory.station \
             WHERE partner_id = $1 AND ($2 OR deleted_at IS NULL) AND status = $3",
        )
        .bind(partner_id)
        .bind(include_deleted)
        .bind(status.as_str())
        .fetch_one(pool)
        .await
        .map_err(ServiceError::Db)?;

        let stations = sqlx::query_as::<_, Station>(
            "SELECT station_id, partner_id, name, address, latitude, longitude, \
             status, availability_status, created_at, updated_at \
             FROM inventory.station \
             WHERE partner_id = $1 AND ($2 OR deleted_at IS NULL) AND status = $3 \
             ORDER BY created_at DESC LIMIT $4 OFFSET $5",
        )
        .bind(partner_id)
        .bind(include_deleted)
        .bind(status.as_str())
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(ServiceError::Db)?;

        (stations, count.0)
    } else {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM inventory.station \
             WHERE partner_id = $1 AND ($2 OR deleted_at IS NULL)",
        )
        .bind(partner_id)
        .bind(include_deleted)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::Db)?;

        let stations = sqlx::query_as::<_, Station>(
            "SELECT station_id, partner_id, name, address, latitude, longitude, \
             status, availability_status, created_at, updated_at \
             FROM inventory.station \
             WHERE partner_id = $1 AND ($2 OR deleted_at IS NULL) \
             ORDER BY created_at DESC LIMIT $3 OFFSET $4",
        )
        .bind(partner_id)
        .bind(include_deleted)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(ServiceError::Db)?;

        (stations, count.0)
    };

    let total_i32 = total as i32;
    let size = params.size();
    let total_pages = total_i32.div_euclid(size) + if total_i32 % size != 0 { 1 } else { 0 };

    let meta = PaginationMeta {
        page: params.page(),
        size,
        total: total_i32,
        total_pages: total_pages.max(0),
        has_next: params.page() < total_pages,
        has_prev: params.page() > 1,
    };

    Ok((stations, meta))
}

pub async fn get_station_by_id(pool: &PgPool, id: &str) -> Result<Station, ServiceError> {
    find_by_id(pool, id).await
}

pub async fn create_station(
    tx: &mut Transaction<'_, Postgres>,
    partner_id: &str,
    req: &StationCreate,
) -> Result<Station, ServiceError> {
    let station_id = generate_id(EntityPrefix::Stn);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO inventory.station \
         (station_id, partner_id, name, address, latitude, longitude, \
          status, availability_status, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)",
    )
    .bind(&station_id)
    .bind(partner_id)
    .bind(&req.name)
    .bind(&req.address)
    .bind(req.latitude)
    .bind(req.longitude)
    .bind("draft")
    .bind("unavailable")
    .bind(now)
    .execute(tx as &mut sqlx::PgConnection)
    .await?;

    Ok(Station {
        station_id,
        partner_id: partner_id.to_string(),
        name: req.name.clone(),
        address: req.address.clone(),
        latitude: req.latitude,
        longitude: req.longitude,
        status: StationStatus::Draft,
        availability_status: StationAvailabilityStatus::Unavailable,
        created_at: now,
        updated_at: now,
    })
}

pub async fn update_station(
    tx: &mut Transaction<'_, Postgres>,
    id: &str,
    req: &StationUpdate,
    expected_updated_at: DateTime<Utc>,
) -> Result<Station, ServiceError> {
    let now = Utc::now();

    let result = sqlx::query_as::<_, Station>(
        "UPDATE inventory.station SET \
         name = COALESCE($1, name), \
         address = COALESCE($2, address), \
         latitude = COALESCE($3, latitude), \
         longitude = COALESCE($4, longitude), \
         status = CAST(COALESCE($5, CAST(status AS text)) AS inventory.station_status), \
         availability_status = CAST(COALESCE($6, CAST(availability_status AS text)) AS inventory.station_availability_status), \
         updated_at = $7 \
         WHERE station_id = $8 AND updated_at = $9 AND deleted_at IS NULL \
         RETURNING station_id, partner_id, name, address, latitude, longitude, \
         status, availability_status, created_at, updated_at",
    )
    .bind(&req.name)
    .bind(&req.address)
    .bind(req.latitude)
    .bind(req.longitude)
    .bind(req.status.map(|s| s.as_str()))
    .bind(req.availability_status.map(|s| s.as_str()))
    .bind(now)
    .bind(id)
    .bind(expected_updated_at)
    .fetch_optional(tx as &mut sqlx::PgConnection)
    .await
    .map_err(ServiceError::Db)?;

    result.ok_or_else(|| {
        ServiceError::Api(common_errors::ApiError {
            code: common_errors::ErrorCode::ConcurrentModification,
            message: format!("Station '{}' was modified by another request", id),
            details: None,
        })
    })
}

pub async fn soft_delete_station(
    tx: &mut Transaction<'_, Postgres>,
    id: &str,
) -> Result<(), ServiceError> {
    let result = sqlx::query(
        "UPDATE inventory.station SET deleted_at = now() \
         WHERE station_id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(tx as &mut sqlx::PgConnection)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ServiceError::not_found("Station", id));
    }
    Ok(())
}

/// Admin variants — no partner_id filter
pub async fn admin_list_stations(
    pool: &PgPool,
    params: &PaginationParams,
    include_deleted: bool,
    _status_filter: Option<StationStatus>,
) -> Result<(Vec<Station>, PaginationMeta), ServiceError> {
    let offset = params.offset();
    let limit = params.limit();

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM inventory.station WHERE $1 OR deleted_at IS NULL",
    )
    .bind(include_deleted)
    .fetch_one(pool)
    .await
    .map_err(ServiceError::Db)?;

    let stations: Vec<Station> = sqlx::query_as::<_, Station>(
        "SELECT station_id, partner_id, name, address, latitude, longitude, \
         status, availability_status, created_at, updated_at \
         FROM inventory.station WHERE $1 OR deleted_at IS NULL \
         ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(include_deleted)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(ServiceError::Db)?;

    let total_i32 = count.0 as i32;
    let size = params.size();
    let total_pages = total_i32.div_euclid(size) + if total_i32 % size != 0 { 1 } else { 0 };

    let meta = PaginationMeta {
        page: params.page(),
        size,
        total: total_i32,
        total_pages: total_pages.max(0),
        has_next: params.page() < total_pages,
        has_prev: params.page() > 1,
    };

    Ok((stations, meta))
}

pub async fn admin_find_by_id(pool: &PgPool, id: &str) -> Result<Station, ServiceError> {
    sqlx::query_as::<_, Station>(
        "SELECT station_id, partner_id, name, address, latitude, longitude, \
         status, availability_status, created_at, updated_at \
         FROM inventory.station WHERE station_id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::not_found("Station", id),
        other => ServiceError::Db(other),
    })
}

pub async fn admin_delete_station(tx: &mut Transaction<'_, Postgres>, id: &str) -> Result<(), ServiceError> {
    let result = sqlx::query(
        "UPDATE inventory.station SET deleted_at = now() WHERE station_id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(tx as &mut sqlx::PgConnection)
    .await?;
    if result.rows_affected() == 0 {
        return Err(ServiceError::not_found("Station", id));
    }
    Ok(())
}
