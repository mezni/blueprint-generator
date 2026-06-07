use crate::error::WorkerError;
use sqlx::PgPool;
use tracing::info;

pub fn validate_coordinates(lat: f64, lng: f64) -> Result<(), WorkerError> {
    if lat < -90.0 || lat > 90.0 {
        return Err(WorkerError::InvalidCoordinates(format!(
            "latitude {} out of range [-90, 90]",
            lat
        )));
    }
    if lng < -180.0 || lng > 180.0 {
        return Err(WorkerError::InvalidCoordinates(format!(
            "longitude {} out of range [-180, 180]",
            lng
        )));
    }
    Ok(())
}

pub async fn update_station_geometry(
    pool: &PgPool,
    station_id: &str,
    lat: f64,
    lng: f64,
) -> Result<(), WorkerError> {
    validate_coordinates(lat, lng)?;

    let result = sqlx::query(
        "UPDATE inventory.station
         SET geom = ST_SetSRID(ST_MakePoint($1, $2), 4326)
         WHERE id = $3",
    )
    .bind(lng)
    .bind(lat)
    .bind(station_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(WorkerError::StationNotFound(station_id.to_string()));
    }

    info!(
        "Updated geometry for station {} at ({}, {})",
        station_id, lat, lng
    );

    Ok(())
}

pub async fn clear_station_geometry(pool: &PgPool, station_id: &str) -> Result<(), WorkerError> {
    let result = sqlx::query(
        "UPDATE inventory.station
         SET geom = NULL
         WHERE id = $1",
    )
    .bind(station_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(WorkerError::StationNotFound(station_id.to_string()));
    }

    info!("Cleared geometry for station {}", station_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_coordinates_valid() {
        assert!(validate_coordinates(0.0, 0.0).is_ok());
        assert!(validate_coordinates(90.0, 180.0).is_ok());
        assert!(validate_coordinates(-90.0, -180.0).is_ok());
        assert!(validate_coordinates(36.8065, 10.1815).is_ok());
    }

    #[test]
    fn test_validate_coordinates_invalid_lat() {
        let err = validate_coordinates(91.0, 0.0).unwrap_err();
        assert!(err.to_string().contains("INVALID_COORDINATES"));
        assert!(err.to_string().contains("latitude"));
    }

    #[test]
    fn test_validate_coordinates_invalid_lng() {
        let err = validate_coordinates(0.0, 200.0).unwrap_err();
        assert!(err.to_string().contains("INVALID_COORDINATES"));
        assert!(err.to_string().contains("longitude"));
    }

    #[test]
    fn test_validate_coordinates_negative_bounds() {
        assert!(validate_coordinates(-90.0, -180.0).is_ok());
        let err = validate_coordinates(-91.0, 0.0).unwrap_err();
        assert!(err.to_string().contains("INVALID_COORDINATES"));
    }
}

pub async fn get_station_coords(
    pool: &PgPool,
    station_id: &str,
) -> Result<Option<(f64, f64)>, WorkerError> {
    let row = sqlx::query_as::<_, (Option<f64>, Option<f64>)>(
        "SELECT latitude, longitude FROM inventory.station WHERE id = $1",
    )
    .bind(station_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some((Some(lat), Some(lng))) => Ok(Some((lat, lng))),
        Some(_) => Ok(None),
        None => Err(WorkerError::StationNotFound(station_id.to_string())),
    }
}
