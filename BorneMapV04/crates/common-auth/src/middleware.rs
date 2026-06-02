use crate::claims::{AuthContext, Role};
use crate::error::AuthError;
use crate::jwt::JwtValidator;
use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

#[async_trait::async_trait]
impl<S: Send + Sync + 'static> FromRequestParts<S> for AuthContext {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<AuthContext>().cloned().ok_or(AuthError::MissingAuthorizationHeader)
    }
}

pub async fn auth_middleware(
    validator: Arc<JwtValidator>,
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AuthError::MissingAuthorizationHeader)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidAuthorizationHeader)?;

    let auth_context = validator.validate_token(token).await?;

    req.extensions_mut().insert(auth_context);
    Ok(next.run(req).await)
}

async fn require_role_inner<B>(
    req: Request<B>,
    next: Next<B>,
    role: Role,
) -> Result<Response, AuthError> {
    let auth_context = req
        .extensions()
        .get::<AuthContext>()
        .ok_or(AuthError::InsufficientPermissions)?;

    if !auth_context.has_role(&role) {
        return Err(AuthError::InsufficientPermissions);
    }

    Ok(next.run(req).await)
}

async fn require_any_role_inner<B>(
    req: Request<B>,
    next: Next<B>,
    roles: Vec<Role>,
) -> Result<Response, AuthError> {
    let auth_context = req
        .extensions()
        .get::<AuthContext>()
        .ok_or(AuthError::InsufficientPermissions)?;

    if !auth_context.has_any_role(&roles) {
        return Err(AuthError::InsufficientPermissions);
    }

    Ok(next.run(req).await)
}

pub fn require_role(role: Role) -> impl axum::response::IntoResponse + Clone + Send + 'static {
    let role = role.clone();
    axum::middleware::from_fn(move |req: Request<Body>, next: Next<Body>| {
        let role = role.clone();
        async move { require_role_inner(req, next, role).await }
    })
}

pub fn require_any_role(roles: Vec<Role>) -> impl axum::response::IntoResponse + Clone + Send + 'static {
    axum::middleware::from_fn(move |req: Request<Body>, next: Next<Body>| {
        let roles = roles.clone();
        async move { require_any_role_inner(req, next, roles).await }
    })
}
