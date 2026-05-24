use actix_web::{web, HttpResponse};
use crate::error_adapter::ApiError;

use super::models::MockLoginRequest;
use super::service::MockAuthService;

/// Issue a mock JWT (MVP 1 only)
#[utoipa::path(
    post,
    path = "/api/v1/auth/mock-login",
    request_body = MockLoginRequest,
    responses(
        (status = 200, description = "JWT issued", body = MockLoginResponse),
        (status = 400, description = "Invalid role or malformed body"),
        (status = 403, description = "Username not on admin allowlist"),
    ),
    tag = "auth"
)]
pub async fn auth_mock_login(
    body: web::Json<MockLoginRequest>,
    allowlist: web::Data<Vec<String>>,
    secret: web::Data<String>,
) -> Result<HttpResponse, ApiError> {
    let response = MockAuthService::login(
        &body.username,
        &body.role,
        allowlist.get_ref(),
        secret.get_ref(),
    )?;
    Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth/mock-login")
            .route(web::post().to(auth_mock_login)),
    );
}
