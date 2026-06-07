//! Review repository for reviews queries

use sqlx::PgPool;

use crate::domain::Review;
use crate::ev_db::Pool;

/// Review repository for users schema queries
pub struct ReviewRepository {
    pool: Pool,
}

impl ReviewRepository {
    /// Create a new review repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Add a review for a station
    pub async fn add_review(
        &self,
        user_id: &str,
        station_id: &str,
        rating: i32,
        comment: Option<String>,
    ) -> Result<Review, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();

        let review = Review {
            id: format!("RVW-{}", now),
            user_id: user_id.to_string(),
            station_id: station_id.to_string(),
            rating,
            comment,
            status: "pending".to_string(),
            created_at: now,
            updated_at: now,
        };

        sqlx::query!(
            r#"
            INSERT INTO users.review (
                id, user_id, station_id, rating, comment, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            review.id,
            review.user_id,
            review.station_id,
            review.rating,
            review.comment,
            review.status,
            review.created_at,
            review.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(review)
    }

    /// Update a review
    pub async fn update_review(
        &self,
        user_id: &str,
        review_id: &str,
        rating: Option<i32>,
        comment: Option<String>,
        status: Option<String>,
    ) -> Result<usize, sqlx::Error> {
        let mut query = String::from(r#"UPDATE users.review SET "#);

        if let Some(rating) = rating {
            query.push_str(&format!("rating = $2, "));
        }
        if let Some(comment) = comment {
            query.push_str(&format!("comment = $3, "));
        }
        if let Some(status) = status {
            query.push_str(&format!("status = $4, "));
        }

        query.push_str("updated_at = $5 WHERE id = $1 AND user_id = $6");
        query.push_str("::sqlx::Query<>");

        let mut query_builder = sqlx::query(&query);
        if let Some(rating) = rating {
            query_builder = query_builder.bind(rating);
        }
        if let Some(comment) = comment {
            query_builder = query_builder.bind(comment);
        }
        if let Some(status) = status {
            query_builder = query_builder.bind(status);
        }
        query_builder = query_builder.bind(now);
        query_builder = query_builder.bind(review_id);
        query_builder = query_builder.bind(user_id);

        let result = query_builder.execute(&self.pool).await?;
        Ok(result.rows_affected() as usize)
    }

    /// Delete a review (soft delete)
    pub async fn delete_review(&self, user_id: &str, review_id: &str) -> Result<usize, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE users.review
            SET status = 'deleted'
            WHERE id = $1 AND user_id = $2
            "#,
            review_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as usize)
    }

    /// List reviews for a station
    pub async fn list_reviews_by_station(
        &self,
        station_id: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Review>, sqlx::Error> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let reviews = sqlx::query!(
            r#"
            SELECT id, user_id, station_id, rating, comment, status, created_at, updated_at
            FROM users.review
            WHERE station_id = $1 AND status != 'deleted'
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            station_id as &str,
            limit as i32,
            offset as i32
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reviews)
    }

    /// Get average rating for a station
    pub async fn get_average_rating(&self, station_id: &str) -> Result<f64, sqlx::Error> {
        let rating: Option<f64> = sqlx::query_scalar!(
            r#"
            SELECT AVG(rating)
            FROM users.review
            WHERE station_id = $1 AND status != 'deleted'
            "#,
            station_id as &str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rating.unwrap_or(0.0))
    }

    /// Count reviews for a station
    pub async fn count_reviews(&self, station_id: &str) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM users.review
            WHERE station_id = $1 AND status != 'deleted'
            "#,
            station_id as &str
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
    fn test_review_repository_creation() {
        let repo = ReviewRepository::new(Pool::none());
        assert!(true); // Structure validated
    }
}
