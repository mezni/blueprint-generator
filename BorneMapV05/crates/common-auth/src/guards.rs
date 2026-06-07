use crate::errors::AuthError;
use crate::CurrentUser;
use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use common_types::Role;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AuthConfig {
    pub issuer: String,
    pub audience: String,
    pub jwks_url: String,
}

static AUTH_CONFIG: once_cell::sync::OnceCell<AuthConfig> = once_cell::sync::OnceCell::new();

pub fn set_auth_config(config: AuthConfig) {
    AUTH_CONFIG.set(config).ok();
}

fn extract_bearer_token(header: Option<&HeaderValue>) -> Option<String> {
    let header = header?.to_str().ok()?;
    header.strip_prefix("Bearer ").map(|s| s.to_string())
}

async fn try_authenticate(
    token: Option<String>,
    pool: Option<&PgPool>,
) -> Result<Option<CurrentUser>, AuthError> {
    let config = AUTH_CONFIG
        .get()
        .ok_or_else(|| AuthError::ValidationError("Auth not configured".into()))?;

    let token = match token {
        Some(t) => t,
        None => return Ok(None),
    };

    let claims = crate::jwt::validate_token(&token, &config.issuer, &config.audience).await?;
    let role = claims.role().ok_or(AuthError::InsufficientRole)?;
    let provisioned =
        crate::provisioning::provision_user(pool, &claims.sub, claims.email.as_deref(), role).await;

    Ok(Some(provisioned.into()))
}

/// Mandatory authentication layer.
///
/// Extracts `PgPool` (if present) from request extensions and passes it to
/// the provisioning layer for partner_id resolution.
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, AuthError> {
    let token = extract_bearer_token(req.headers().get("authorization"));
    let pool = req.extensions().get::<PgPool>().cloned();
    let user = try_authenticate(token, pool.as_ref()).await?.ok_or(AuthError::Unauthenticated)?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

/// Optional authentication layer.
pub async fn optional_auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let token = extract_bearer_token(req.headers().get("authorization"));
    let pool = req.extensions().get::<PgPool>().cloned();
    if let Some(user) = try_authenticate(token, pool.as_ref()).await? {
        req.extensions_mut().insert(user);
    }
    Ok(next.run(req).await)
}

/// Middleware that rejects requests without an authenticated `CurrentUser`.
pub async fn require_authenticated(req: Request, next: Next) -> Result<Response, AuthError> {
    if req.extensions().get::<CurrentUser>().is_none() {
        return Err(AuthError::Unauthenticated);
    }
    Ok(next.run(req).await)
}

pub fn require_role(
    required: Role,
) -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, AuthError>> + Send>,
> + Clone {
    move |req: Request, next: Next| {
        Box::pin(async move {
            let satisfied = req
                .extensions()
                .get::<CurrentUser>()
                .map(|u| u.role.satisfies(required))
                .ok_or(AuthError::Unauthenticated)?;
            if satisfied {
                Ok(next.run(req).await)
            } else {
                Err(AuthError::InsufficientRole)
            }
        })
    }
}

pub fn extract_current_user(req: &Request) -> Result<CurrentUser, AuthError> {
    req.extensions()
        .get::<CurrentUser>()
        .cloned()
        .ok_or(AuthError::Unauthenticated)
}
