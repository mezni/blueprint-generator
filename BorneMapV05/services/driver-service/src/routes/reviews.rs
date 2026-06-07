use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use common_auth::CurrentUser;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::review::{ReviewCreate, ReviewUpdate};
use crate::repository::review_repo;
use common_types::api::{ItemEnvelope, SuccessEnvelope};

use axum::Router;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/v1/driver/reviews", axum::routing::post(create_review).get(list_reviews))
        .route(
            "/api/v1/driver/reviews/{id}",
            axum::routing::patch(update_review).delete(delete_review),
        )
        .with_state(pool)
}

async fn create_review(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Json(body): Json<ReviewCreate>,
) -> Result<impl IntoResponse, ServiceError> {
    if body.rating < 1 || body.rating > 5 {
        return Err(ServiceError::validation("Rating must be between 1 and 5"));
    }

    let review = review_repo::create_review(&pool, &user.user_id, &body).await?;
    Ok((StatusCode::CREATED, ItemEnvelope::new(review)))
}

async fn update_review(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(body): Json<ReviewUpdate>,
) -> Result<impl IntoResponse, ServiceError> {
    if let Some(rating) = body.rating {
        if rating < 1 || rating > 5 {
            return Err(ServiceError::validation("Rating must be between 1 and 5"));
        }
    }

    let review = review_repo::update_review(&pool, &id, &user.user_id, &body).await?;
    Ok(ItemEnvelope::new(review))
}

async fn delete_review(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    review_repo::soft_delete_review(&pool, &id, &user.user_id).await?;
    Ok(ItemEnvelope::new(serde_json::json!({"id": id, "deleted": true})))
}

async fn list_reviews(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, ServiceError> {
    let (reviews, meta) = review_repo::list_user_reviews(&pool, &user.user_id, &params).await?;
    Ok(SuccessEnvelope::new(reviews, meta))
}
