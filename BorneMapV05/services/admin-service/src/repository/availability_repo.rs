use chrono::Utc;
use common_types::AvailabilitySource;
use common_types::StationAvailabilityStatus;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::availability::Availability;

pub async fn find_by_station(
    pool: &PgPool,
    station_id: &str,
) -> Result<Availability, ServiceError> {
    sqlx::query_as::<_, Availability>(
        "SELECT station_id, availability_status, source, updated_at \
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

pub async fn upsert_availability(
    pool: &PgPool,
    station_id: &str,
    status: StationAvailabilityStatus,
    source: AvailabilitySource,
) -> Result<Availability, ServiceError> {
    let now = Utc::now();

    sqlx::query(
        "UPDATE inventory.station SET availability_status = $2, updated_at = $3 \
         WHERE station_id = $1 AND deleted_at IS NULL",
    )
    .bind(station_id)
    .bind(status.as_str())
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::not_found("Station", station_id),
        other => ServiceError::Db(other),
    })?;

    Ok(Availability {
        station_id: station_id.to_string(),
        availability_status: status,
        source,
        updated_at: now,
    })
}

pub async fn verify_station_belongs_to_partner(
    pool: &PgPool,
    station_id: &str,
    partner_id: &str,
) -> Result<(), ServiceError> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT station_id FROM inventory.station \
         WHERE station_id = $1 AND partner_id = $2 AND deleted_at IS NULL",
    )
    .bind(station_id)
    .bind(partner_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    if result.is_none() {
        return Err(ServiceError::Api(common_errors::ApiError {
            code: common_errors::ErrorCode::NotFound,
            message: format!("Station '{}' not found or not owned by partner", station_id),
            details: None,
        }));
    }
    Ok(())
}
