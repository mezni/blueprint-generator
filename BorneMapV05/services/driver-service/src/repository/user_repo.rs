use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::user::{DriverProfile, ProfileUpdate, UserProfile};

pub async fn get_profile(
    pool: &PgPool,
    user_id: &str,
) -> Result<DriverProfile, ServiceError> {
    let user: (String, Option<String>, Option<DateTime<Utc>>, Option<DateTime<Utc>>) = sqlx::query_as(
        "SELECT id, email, created_at, last_login_at FROM users.user_account WHERE id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::not_found("User", user_id),
        other => ServiceError::Db(other),
    })?;

    let profile: Option<UserProfile> = sqlx::query_as::<_, UserProfile>(
        "SELECT user_id, display_name, avatar_url, preferred_language, preferences \
         FROM users.user_profile WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    Ok(DriverProfile {
        user_id: user.0,
        email: user.1,
        display_name: profile.as_ref().and_then(|p| p.display_name.clone()),
        avatar_url: profile.as_ref().and_then(|p| p.avatar_url.clone()),
        preferred_language: profile.as_ref().and_then(|p| p.preferred_language.clone()),
        preferences: profile.as_ref().and_then(|p| p.preferences.clone()),
        created_at: user.2,
        last_login_at: user.3,
    })
}

pub async fn upsert_profile(
    pool: &PgPool,
    user_id: &str,
    req: &ProfileUpdate,
) -> Result<DriverProfile, ServiceError> {
    let existing: Option<UserProfile> = sqlx::query_as::<_, UserProfile>(
        "SELECT user_id, display_name, avatar_url, preferred_language, preferences \
         FROM users.user_profile WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    if existing.is_some() {
        sqlx::query(
            "UPDATE users.user_profile SET \
             display_name = COALESCE($1, display_name), \
             avatar_url = COALESCE($2, avatar_url), \
             preferred_language = COALESCE($3, preferred_language), \
             preferences = COALESCE($4, preferences) \
             WHERE user_id = $5",
        )
        .bind(&req.display_name)
        .bind(&req.avatar_url)
        .bind(&req.preferred_language)
        .bind(&req.preferences)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(ServiceError::Db)?;
    } else {
        sqlx::query(
            "INSERT INTO users.user_profile (user_id, display_name, avatar_url, preferred_language, preferences) \
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(user_id)
        .bind(&req.display_name)
        .bind(&req.avatar_url)
        .bind(&req.preferred_language)
        .bind(&req.preferences)
        .execute(pool)
        .await
        .map_err(ServiceError::Db)?;
    }

    get_profile(pool, user_id).await
}
