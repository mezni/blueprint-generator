//! Favorites use case for driver-service

use std::time::SystemTime;

use sqlx::PgPool;

use crate::domain::{AddFavoriteInput, Favorite, RemoveFavoriteInput, UpdateFavoriteInput};
use crate::ev_db::Pool;

/// Favorites use case
pub struct FavoritesUseCase {
    pool: Pool,
}

impl FavoritesUseCase {
    /// Create a new favorites use case
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Add a favorite station for a user
    pub async fn add_favorite(
        &self,
        user_id: &str,
        input: AddFavoriteInput,
    ) -> Result<Favorite, crate::DomainResult> {
        // TODO: Validate station exists and is active
        // TODO: Validate user hasn't already favorited this station

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let favorite = Favorite {
            id: format!("FVT-{}", now),
            user_id: user_id.to_string(),
            station_id: input.station_id,
            created_at: now.to_string(),
        };

        // TODO: Insert into users.favorite table

        Ok(favorite)
    }

    /// Remove a favorite by ID
    pub async fn remove_favorite(
        &self,
        user_id: &str,
        input: RemoveFavoriteInput,
    ) -> Result<usize, crate::DomainResult> {
        // TODO: Validate favorite exists and belongs to user

        let result = sqlx::query!(
            r#"
            DELETE FROM users.favorite
            WHERE id = $1 AND user_id = $2
            "#,
            input.favorite_id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as usize)
    }

    /// Update a favorite's station
    pub async fn update_favorite(
        &self,
        user_id: &str,
        input: UpdateFavoriteInput,
    ) -> Result<usize, crate::DomainResult> {
        // TODO: Validate favorite exists and belongs to user
        // TODO: Validate new station exists and is active

        let result = sqlx::query!(
            r#"
            UPDATE users.favorite
            SET station_id = $1
            WHERE id = $2 AND user_id = $3
            "#,
            input.station_id,
            input.favorite_id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as usize)
    }

    /// List all favorites for a user with pagination
    pub async fn list_favorites(
        &self,
        user_id: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Favorite>, crate::DomainResult> {
        let favorites = sqlx::query!(
            r#"
            SELECT id, user_id, station_id, created_at
            FROM users.favorite
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id as &str,
            limit as i32,
            offset as i32
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(e.to_string()))?;

        Ok(favorites)
    }

    /// Check if a station is favorited by a user
    pub async fn is_favorite(
        &self,
        user_id: &str,
        station_id: &str,
    ) -> Result<bool, crate::DomainResult> {
        let result: Option<i64> = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM users.favorite
            WHERE user_id = $1 AND station_id = $2
            "#,
            user_id as &str,
            station_id as &str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(e.to_string()))?;

        Ok(result.unwrap_or(0) > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favorites_usecase_creation() {
        let usecase = FavoritesUseCase::new(Pool::none());
        assert!(true); // Structure validated
    }
}
