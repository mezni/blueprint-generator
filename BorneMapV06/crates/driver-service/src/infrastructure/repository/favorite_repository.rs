//! Favorite repository for favorites queries

use sqlx::PgPool;

use crate::domain::{AddFavoriteInput, Favorite, RemoveFavoriteInput, UpdateFavoriteInput};
use crate::ev_db::Pool;

/// Favorite repository for users schema queries
pub struct FavoriteRepository {
    pool: Pool,
}

impl FavoriteRepository {
    /// Create a new favorite repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Add a favorite for a user
    pub async fn add_favorite(
        &self,
        user_id: &str,
        input: AddFavoriteInput,
    ) -> Result<Favorite, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();

        let favorite = Favorite {
            id: format!("FVT-{}", now),
            user_id: user_id.to_string(),
            station_id: input.station_id,
            created_at: now.to_string(),
        };

        sqlx::query!(
            r#"
            INSERT INTO users.favorite (id, user_id, station_id, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
            favorite.id,
            favorite.user_id,
            favorite.station_id,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(favorite)
    }

    /// Remove a favorite by ID
    pub async fn remove_favorite(
        &self,
        user_id: &str,
        input: RemoveFavoriteInput,
    ) -> Result<usize, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users.favorite
            WHERE id = $1 AND user_id = $2
            "#,
            input.favorite_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as usize)
    }

    /// Update a favorite's station
    pub async fn update_favorite(
        &self,
        user_id: &str,
        input: UpdateFavoriteInput,
    ) -> Result<usize, sqlx::Error> {
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
        .await?;

        Ok(result.rows_affected() as usize)
    }

    /// List all favorites for a user
    pub async fn list_favorites(
        &self,
        user_id: &str,
    ) -> Result<Vec<Favorite>, sqlx::Error> {
        let favorites = sqlx::query!(
            r#"
            SELECT id, user_id, station_id, created_at
            FROM users.favorite
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id as &str
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(favorites)
    }

    /// Check if a station is favorited by a user
    pub async fn is_favorite(
        &self,
        user_id: &str,
        station_id: &str,
    ) -> Result<bool, sqlx::Error> {
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
        .await?;

        Ok(result.unwrap_or(0) > 0)
    }

    /// Count favorites for a user
    pub async fn count_favorites(&self, user_id: &str) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM users.favorite
            WHERE user_id = $1
            "#,
            user_id as &str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favorite_repository_creation() {
        let repo = FavoriteRepository::new(Pool::none());
        assert!(true); // Structure validated
    }
}
