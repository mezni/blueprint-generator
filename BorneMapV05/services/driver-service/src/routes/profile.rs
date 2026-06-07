use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use common_auth::CurrentUser;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::user::ProfileUpdate;
use crate::repository::user_repo;
use common_types::api::ItemEnvelope;

use axum::Router;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/v1/driver/me", axum::routing::get(get_profile).patch(update_profile))
        .with_state(pool)
}

async fn get_profile(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ServiceError> {
    let profile = user_repo::get_profile(&pool, &user.user_id).await?;
    Ok(ItemEnvelope::new(profile))
}

async fn update_profile(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Json(body): Json<ProfileUpdate>,
) -> Result<impl IntoResponse, ServiceError> {
    let profile = user_repo::upsert_profile(&pool, &user.user_id, &body).await?;
    Ok(ItemEnvelope::new(profile))
}
