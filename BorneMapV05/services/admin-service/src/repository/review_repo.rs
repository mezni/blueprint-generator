use chrono::Utc;
use common_types::api::PaginationMeta;
use common_types::ReviewStatus;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::review::{validate_review_transition, Review};

pub async fn list_reviews(
    pool: &PgPool,
    params: &PaginationParams,
    status_filter: Option<ReviewStatus>,
    station_filter: Option<&str>,
) -> Result<(Vec<Review>, PaginationMeta), ServiceError> {
    let offset = params.offset();
    let limit = params.limit();

    let (reviews, total): (Vec<Review>, i64) = match (status_filter, station_filter) {
        (Some(ref s), Some(sid)) => {
            let count: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM inventory.review WHERE status = $1 AND station_id = $2",
            )
            .bind(s.as_str())
            .bind(sid)
            .fetch_one(pool)
            .await
            .map_err(ServiceError::Db)?;

            let reviews = sqlx::query_as::<_, Review>(
                "SELECT review_id, station_id, user_id, rating, comment, \
                 status, created_at, updated_at \
                 FROM inventory.review WHERE status = $1 AND station_id = $2 \
                 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            )
            .bind(s.as_str())
            .bind(sid)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(ServiceError::Db)?;

            (reviews, count.0)
        }
        (Some(ref s), None) => {
            let count: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM inventory.review WHERE status = $1",
            )
            .bind(s.as_str())
            .fetch_one(pool)
            .await
            .map_err(ServiceError::Db)?;

            let reviews = sqlx::query_as::<_, Review>(
                "SELECT review_id, station_id, user_id, rating, comment, \
                 status, created_at, updated_at \
                 FROM inventory.review WHERE status = $1 \
                 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(s.as_str())
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(ServiceError::Db)?;

            (reviews, count.0)
        }
        (None, Some(sid)) => {
            let count: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM inventory.review WHERE station_id = $1",
            )
            .bind(sid)
            .fetch_one(pool)
            .await
            .map_err(ServiceError::Db)?;

            let reviews = sqlx::query_as::<_, Review>(
                "SELECT review_id, station_id, user_id, rating, comment, \
                 status, created_at, updated_at \
                 FROM inventory.review WHERE station_id = $1 \
                 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(sid)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(ServiceError::Db)?;

            (reviews, count.0)
        }
        (None, None) => {
            let count: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM inventory.review",
            )
            .fetch_one(pool)
            .await
            .map_err(ServiceError::Db)?;

            let reviews = sqlx::query_as::<_, Review>(
                "SELECT review_id, station_id, user_id, rating, comment, \
                 status, created_at, updated_at \
                 FROM inventory.review ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(ServiceError::Db)?;

            (reviews, count.0)
        }
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

    Ok((reviews, meta))
}

pub async fn update_review_status(
    pool: &PgPool,
    id: &str,
    new_status: ReviewStatus,
    _moderated_by: &str,
) -> Result<Review, ServiceError> {
    let current: Review = sqlx::query_as::<_, Review>(
        "SELECT review_id, station_id, user_id, rating, comment, status, created_at, updated_at \
         FROM inventory.review WHERE review_id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::not_found("Review", id),
        other => ServiceError::Db(other),
    })?;

    if !validate_review_transition(current.status, new_status) {
        return Err(ServiceError::Api(common_errors::ApiError {
            code: common_errors::ErrorCode::ReviewStateInvalid,
            message: format!("Cannot transition review from '{:?}' to '{:?}'", current.status, new_status),
            details: None,
        }));
    }

    let now = Utc::now();
    let updated = sqlx::query_as::<_, Review>(
        "UPDATE inventory.review SET status = $2, updated_at = $3 \
         WHERE review_id = $1 \
         RETURNING review_id, station_id, user_id, rating, comment, status, created_at, updated_at",
    )
    .bind(id)
    .bind(new_status.as_str())
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(updated)
}
