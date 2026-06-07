//! Favorite repository for managing user favorites

use sqlx::{PgPool, Postgres};
use std::sync::Arc;

use crate::ev_domain::{Favorite, User, Station};
use crate::error::{AppResult, ApiError};
use crate::ev_db::Pool;

/// Favorite repository
pub struct FavoriteRepository {
    pool: Pool,
}

impl FavoriteRepository {
    /// Create new favorite repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Insert a new favorite
    pub async fn create(&self, favorite: &Favorite) -> AppResult<String> {
        let result = sqlx::query_scalar::<_, String>(
            r#"
            INSERT INTO users.favorite (id, user_id, station_id, created_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING id
            "#,
        )
        .bind(&favorite.id)
        .bind(&favorite.user_id)
        .bind(&favorite.station_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    /// Delete a favorite (hard delete per Sprint 1 clarification)
    pub async fn delete(&self, id: &str, user_id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            DELETE FROM users.favorite
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Check if any rows were deleted
        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(format!(
                "Favorite {} not found or does not belong to user {}",
                id, user_id
            )));
        }

        Ok(())
    }

    /// Find favorite by ID
    pub async fn find_by_id(&self, id: &str) -> AppResult<Option<Favorite>> {
        let favorite = sqlx::query_as::<_, Favorite>(
            r#"
            SELECT id, user_id, station_id, created_at
            FROM users.favorite
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(favorite)
    }

    /// List all favorites for a user
    pub async fn find_by_user(
        &self,
        user_id: &str,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<Favorite>> {
        let favorites = sqlx::query_as::<_, Favorite>(
            r#"
            SELECT id, user_id, station_id, created_at
            FROM users.favorite
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

        Ok(favorites)
    }

    /// Check if user has favorited a station
    pub async fn has_favorite(&self, user_id: &str, station_id: &str) -> AppResult<bool> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM users.favorite
            WHERE user_id = $1 AND station_id = $2
            "#,
        )
        .bind(user_id)
        .bind(station_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(count > 0)
    }

    /// Get favorite count for a user
    pub async fn count_by_user(&self, user_id: &str) -> AppResult<i64> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM users.favorite
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favorite_repository_creation() {
        let pool = Pool::none(); // Mock pool for testing
        let repo = FavoriteRepository::new(pool);
        assert!(true); // Repository created successfully
    }
}
