use common_types::api::PaginationMeta;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::user::User;

pub async fn list_users(
    pool: &PgPool,
    params: &PaginationParams,
) -> Result<(Vec<User>, PaginationMeta), ServiceError> {
    let offset = params.offset();
    let limit = params.limit();

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM platform_db.user_account",
    )
    .fetch_one(pool)
    .await
    .map_err(ServiceError::Db)?;

    let users = sqlx::query_as::<_, User>(
        "SELECT user_id, keycloak_user_id, email, role, created_at, updated_at \
         FROM platform_db.user_account ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(ServiceError::Db)?;

    let total_i32 = count.0 as i32;
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

    Ok((users, meta))
}

pub async fn update_user(
    pool: &PgPool,
    user_id: &str,
    req: &crate::routes::admin::UserUpdateRequest,
) -> Result<User, ServiceError> {
    let now = chrono::Utc::now();
    
    let result = sqlx::query_as::<_, User>(
        "UPDATE platform_db.user_account SET \
         role = COALESCE($1, role), \
         updated_at = $2 \
         WHERE user_id = $3 \
         RETURNING user_id, keycloak_user_id, email, role, created_at, updated_at",
    )
    .bind(&req.role)
    .bind(now)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(ServiceError::Db)?;

    result.ok_or_else(|| ServiceError::not_found("User", user_id))
}
