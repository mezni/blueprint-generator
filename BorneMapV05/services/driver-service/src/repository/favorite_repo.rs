use chrono::Utc;
use sqlx::PgPool;

use crate::error::ServiceError;

pub async fn add_favorite(
    pool: &PgPool,
    user_id: &str,
    station_id: &str,
) -> Result<(), ServiceError> {
    sqlx::query(
        "INSERT INTO users.favorite_station (user_id, station_id, created_at) \
         VALUES ($1, $2, $3) ON CONFLICT (user_id, station_id) DO NOTHING",
    )
    .bind(user_id)
    .bind(station_id)
    .bind(Utc::now())
    .execute(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(())
}

pub async fn remove_favorite(
    pool: &PgPool,
    user_id: &str,
    station_id: &str,
) -> Result<(), ServiceError> {
    let result = sqlx::query(
        "DELETE FROM users.favorite_station WHERE user_id = $1 AND station_id = $2",
    )
    .bind(user_id)
    .bind(station_id)
    .execute(pool)
    .await
    .map_err(ServiceError::Db)?;

    if result.rows_affected() == 0 {
        return Err(ServiceError::not_found("Favorite", station_id));
    }

    Ok(())
}

pub async fn list_favorites(
    pool: &PgPool,
    user_id: &str,
) -> Result<Vec<String>, ServiceError> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT station_id FROM users.favorite_station WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(rows.into_iter().map(|r| r.0).collect())
}

pub async fn is_favorite(
    pool: &PgPool,
    user_id: &str,
    station_id: &str,
) -> Result<bool, ServiceError> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM users.favorite_station WHERE user_id = $1 AND station_id = $2",
    )
    .bind(user_id)
    .bind(station_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(row.map(|r| r.0 > 0).unwrap_or(false))
}
