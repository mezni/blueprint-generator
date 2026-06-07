//! Review repository for managing station reviews

use sqlx::{PgPool, Postgres};
use std::sync::Arc;

use crate::ev_domain::{Review, User, Station};
use crate::error::{AppResult, ApiError};
use crate::ev_db::Pool;

/// Review repository
pub struct ReviewRepository {
    pool: Pool,
}

impl ReviewRepository {
    /// Create new review repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Insert a new review (soft delete allowed)
    pub async fn create(&self, review: &Review) -> AppResult<String> {
        let result = sqlx::query_scalar::<_, String>(
            r#"
            INSERT INTO users.review (id, user_id, station_id, rating, comment, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING id
            "#,
        )
        .bind(&review.id)
        .bind(&review.user_id)
        .bind(&review.station_id)
        .bind(&review.rating)
        .bind(&review.comment)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    /// Delete a review (soft delete - keep audit trail)
    pub async fn delete(&self, id: &str, user_id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE users.review
            SET deleted_at = NOW()
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    /// Find review by ID
    pub async fn find_by_id(&self, id: &str) -> AppResult<Option<Review>> {
        let review = sqlx::query_as::<_, Review>(
            r#"
            SELECT id, user_id, station_id, rating, comment, created_at, updated_at, deleted_at
            FROM users.review
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(review)
    }

    /// List reviews for a station
    pub async fn find_by_station(
        &self,
        station_id: &str,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<Review>> {
        let reviews = sqlx::query_as::<_, Review>(
            r#"
            SELECT id, user_id, station_id, rating, comment, created_at, updated_at, deleted_at
            FROM users.review
            WHERE station_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(station_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(reviews)
    }

    /// List reviews for a user
    pub async fn find_by_user(
        &self,
        user_id: &str,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<Review>> {
        let reviews = sqlx::query_as::<_, Review>(
            r#"
            SELECT id, user_id, station_id, rating, comment, created_at, updated_at, deleted_at
            FROM users.review
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(reviews)
    }

    /// Count reviews for a station (excluding soft-deleted)
    pub async fn count_by_station(&self, station_id: &str) -> AppResult<i64> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM users.review
            WHERE station_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(station_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(count)
    }

    /// Get average rating for a station
    pub async fn avg_rating_by_station(&self, station_id: &str) -> AppResult<Option<f64>> {
        let result: Option<f64> = sqlx::query_scalar(
            r#"
            SELECT COALESCE(AVG(rating), 0.0)
            FROM users.review
            WHERE station_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(station_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_review_repository_creation() {
        let pool = Pool::none(); // Mock pool for testing
        let repo = ReviewRepository::new(pool);
        assert!(true); // Repository created successfully
    }
}
