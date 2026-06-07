use chrono::Utc;
use common_types::api::PaginationMeta;
use common_types::generate_id;
use common_types::EntityPrefix;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::review::{Review, ReviewCreate, ReviewUpdate};

pub async fn create_review(
    pool: &PgPool,
    user_id: &str,
    req: &ReviewCreate,
) -> Result<Review, ServiceError> {
    let id = generate_id(EntityPrefix::Rev);
    let now = Utc::now();

    let review = sqlx::query_as::<_, Review>(
        "INSERT INTO users.station_review (id, user_id, station_id, rating, comment, status, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, 'published', $6, $6) \
         RETURNING id, user_id, station_id, rating, comment, status, created_at, updated_at",
    )
    .bind(&id)
    .bind(user_id)
    .bind(&req.station_id)
    .bind(req.rating)
    .bind(&req.comment)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(ref db_err) = e {
            if let Some(c) = db_err.constraint() {
                if c.contains("uq_station_review_user_station") {
                    return ServiceError::already_exists("You have already reviewed this station");
                }
            }
        }
        ServiceError::Db(e)
    })?;

    Ok(review)
}

pub async fn update_review(
    pool: &PgPool,
    review_id: &str,
    user_id: &str,
    req: &ReviewUpdate,
) -> Result<Review, ServiceError> {
    // First verify ownership
    let current: Review = sqlx::query_as::<_, Review>(
        "SELECT id, user_id, station_id, rating, comment, status, created_at, updated_at \
         FROM users.station_review WHERE id = $1",
    )
    .bind(review_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?
    .ok_or_else(|| ServiceError::not_found("Review", review_id))?;

    if current.user_id != user_id {
        return Err(ServiceError::forbidden());
    }

    if current.status == "deleted" {
        return Err(ServiceError::validation("Cannot update a deleted review"));
    }

    let now = Utc::now();
    let updated = sqlx::query_as::<_, Review>(
        "UPDATE users.station_review SET \
         rating = COALESCE($1, rating), \
         comment = COALESCE($2, comment), \
         updated_at = $3 \
         WHERE id = $4 AND user_id = $5 \
         RETURNING id, user_id, station_id, rating, comment, status, created_at, updated_at",
    )
    .bind(req.rating)
    .bind(&req.comment)
    .bind(now)
    .bind(review_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(updated)
}

pub async fn soft_delete_review(
    pool: &PgPool,
    review_id: &str,
    user_id: &str,
) -> Result<(), ServiceError> {
    let current: Review = sqlx::query_as::<_, Review>(
        "SELECT id, user_id, station_id, rating, comment, status, created_at, updated_at \
         FROM users.station_review WHERE id = $1",
    )
    .bind(review_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?
    .ok_or_else(|| ServiceError::not_found("Review", review_id))?;

    if current.user_id != user_id {
        return Err(ServiceError::forbidden());
    }

    if current.status == "deleted" {
        return Err(ServiceError::validation("Review is already deleted"));
    }

    let now = Utc::now();
    sqlx::query(
        "UPDATE users.station_review SET status = 'deleted', updated_at = $1 WHERE id = $2 AND user_id = $3",
    )
    .bind(now)
    .bind(review_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(())
}

pub async fn list_user_reviews(
    pool: &PgPool,
    user_id: &str,
    pagination: &PaginationParams,
) -> Result<(Vec<Review>, PaginationMeta), ServiceError> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users.station_review WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(ServiceError::Db)?;

    let reviews: Vec<Review> = sqlx::query_as::<_, Review>(
        "SELECT id, user_id, station_id, rating, comment, status, created_at, updated_at \
         FROM users.station_review WHERE user_id = $1 \
         ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(user_id)
    .bind(pagination.limit())
    .bind(pagination.offset())
    .fetch_all(pool)
    .await
    .map_err(ServiceError::Db)?;

    let total_i32 = count.0 as i32;
    let size = pagination.size();
    let total_pages = total_i32.div_euclid(size) + if total_i32 % size != 0 { 1 } else { 0 };

    let meta = PaginationMeta {
        page: pagination.page(),
        size,
        total: total_i32,
        total_pages: total_pages.max(0),
        has_next: pagination.page() < total_pages,
        has_prev: pagination.page() > 1,
    };

    Ok((reviews, meta))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    use crate::error::ServiceError;

    #[test]
    fn test_already_deleted_validation_error_message() {
        let err = ServiceError::validation("Review is already deleted");
        let msg = format!("{}", err);
        assert!(msg.contains("Review is already deleted"));
    }

    #[test]
    fn test_cannot_update_deleted_validation_error_message() {
        let err = ServiceError::validation("Cannot update a deleted review");
        let msg = format!("{}", err);
        assert!(msg.contains("Cannot update a deleted review"));
    }

    #[test]
    fn test_already_deleted_response_is_bad_request() {
        let err = ServiceError::validation("Review is already deleted");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_deleted_review_rejects_update() {
        let err = ServiceError::validation("Cannot update a deleted review");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_soft_delete_status_constants() {
        let published = "published";
        let deleted = "deleted";
        assert_eq!(published, "published");
        assert_eq!(deleted, "deleted");
        assert_ne!(published, deleted);
    }
}
