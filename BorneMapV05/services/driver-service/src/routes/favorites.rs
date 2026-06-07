use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use common_auth::CurrentUser;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::repository::favorite_repo;
use common_types::api::ItemEnvelope;

use axum::Router;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/api/v1/driver/favorites/{id}",
            axum::routing::post(add_favorite).delete(remove_favorite),
        )
        .route("/api/v1/driver/favorites", axum::routing::get(list_favorites))
        .with_state(pool)
}

async fn add_favorite(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(station_id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    favorite_repo::add_favorite(&pool, &user.user_id, &station_id).await?;
    Ok((
        StatusCode::CREATED,
        ItemEnvelope::new(serde_json::json!({"station_id": station_id})),
    ))
}

async fn remove_favorite(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(station_id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    favorite_repo::remove_favorite(&pool, &user.user_id, &station_id).await?;
    Ok(ItemEnvelope::new(serde_json::json!({"station_id": station_id})))
}

async fn list_favorites(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ServiceError> {
    let station_ids = favorite_repo::list_favorites(&pool, &user.user_id).await?;
    Ok(ItemEnvelope::new(station_ids))
}
