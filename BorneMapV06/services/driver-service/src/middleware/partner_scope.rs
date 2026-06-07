//! Partner scope extractor

use actix_web::dev::FromRequest;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;

use crate::error::{AppResult, ApiError};
use crate::AppState;

/// Partner scope extractor
///
/// Extracts partner_id from JWT claims and validates it matches the authenticated user's scope.
/// This ensures partners can only access their own stations.
pub struct PartnerScope {
    pub user_id: String,
    pub partner_id: String,
    pub role: crate::ev_auth::Role,
}

impl PartnerScope {
    /// Validate partner scope (non-null for partner role)
    pub fn validate_partner_scope(&self) -> AppResult<String> {
        match self.role {
            crate::ev_auth::Role::Partner => {
                self.partner_id
                    .clone()
                    .ok_or(ApiError::BadRequest(
                        "Partner user missing partner_id".into(),
                    ))
            }
            _ => {
                if self.partner_id.is_some() {
                    Err(ApiError::BadRequest(
                        format!(
                            "Non-partner user {} should not have partner_id",
                            self.user_id
                        ),
                    ))
                } else {
                    Ok(String::new()) // Empty scope for non-partners
                }
            }
        }
    }
}

/// Extract partner scope from request extensions
pub async fn extract_partner_scope(
    request: &HttpRequest,
) -> Result<PartnerScope, Error> {
    let claims = request
        .extensions()
        .get::<ev_auth::Claims>()
        .ok_or_else(|| ApiError::Unauthorized)
        .map_err(|e| Error::from(e))?;

    let partner_id = claims
        .partner_id
        .clone()
        .ok_or_else(|| ApiError::Unauthorized)
        .map_err(|e| Error::from(e))?;

    Ok(PartnerScope {
        user_id: claims.sub,
        partner_id,
        role: claims.role,
    })
}

/// Create partner scope extractor for use in handlers
pub async fn partner_scope_extractor(
    request: &HttpRequest,
) -> Result<PartnerScope, Error> {
    extract_partner_scope(request).await
}

/// Middleware that validates partner scope on every request
pub async fn partner_scope_middleware(
    request: HttpRequest,
    _pool: web::Data<PgPool>,
) -> Result<HttpRequest, Error> {
    let claims = request
        .extensions()
        .get::<ev_auth::Claims>()
        .ok_or_else(|| ApiError::Unauthorized)
        .map_err(|e| Error::from(e))?;

    // Validate partner scope if role is partner
    if claims.role == crate::ev_auth::Role::Partner {
        let _partner_id = claims
            .partner_id
            .clone()
            .ok_or_else(|| ApiError::Unauthorized)
            .map_err(|e| Error::from(e))?;
    }

    // Validate role is one of the allowed roles
    if !matches!(
        claims.role,
        crate::ev_auth::Role::RegisteredDriver | crate::ev_auth::Role::Partner | crate::ev_auth::Role::Admin
    ) {
        return Err(ApiError::Forbidden.into());
    }

    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partner_scope_validation() {
        let mut claims = ev_auth::Claims {
            sub: "user123".to_string(),
            email: Some("user@example.com".to_string()),
            name: Some("User".to_string()),
            role: ev_auth::Role::RegisteredDriver,
            partner_id: None,
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        // Registered driver without partner_id should pass
        assert!(PartnerScope {
            user_id: claims.sub,
            partner_id: claims.partner_id,
            role: claims.role,
        }
        .validate_partner_scope()
        .is_ok());

        // Partner with partner_id should pass
        claims.role = ev_auth::Role::Partner;
        claims.partner_id = Some("PRT-123".to_string());
        assert!(PartnerScope {
            user_id: claims.sub,
            partner_id: claims.partner_id,
            role: claims.role,
        }
        .validate_partner_scope()
        .is_ok());

        // Partner without partner_id should fail
        claims.partner_id = None;
        assert!(PartnerScope {
            user_id: claims.sub,
            partner_id: claims.partner_id,
            role: claims.role,
        }
        .validate_partner_scope()
        .is_err());

        // Partner with empty partner_id should fail
        claims.partner_id = Some("".to_string());
        assert!(PartnerScope {
            user_id: claims.sub,
            partner_id: claims.partner_id,
            role: claims.role,
        }
        .validate_partner_scope()
        .is_err());

        // Registered driver with partner_id should fail
        claims.role = ev_auth::Role::RegisteredDriver;
        claims.partner_id = Some("PRT-123".to_string());
        assert!(PartnerScope {
            user_id: claims.sub,
            partner_id: claims.partner_id,
            role: claims.role,
        }
        .validate_partner_scope()
        .is_err());
    }

    #[test]
    fn test_allowed_roles() {
        let driver_claims = ev_auth::Claims {
            sub: "driver".to_string(),
            email: Some("driver@example.com".to_string()),
            name: Some("Driver".to_string()),
            role: ev_auth::Role::RegisteredDriver,
            partner_id: None,
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        let partner_claims = ev_auth::Claims {
            sub: "partner".to_string(),
            email: Some("partner@example.com".to_string()),
            name: Some("Partner".to_string()),
            role: ev_auth::Role::Partner,
            partner_id: Some("PRT-123".to_string()),
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        let admin_claims = ev_auth::Claims {
            sub: "admin".to_string(),
            email: Some("admin@example.com".to_string()),
            name: Some("Admin".to_string()),
            role: ev_auth::Role::Admin,
            partner_id: None,
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        // All allowed roles should pass
        assert!(partner_scope_middleware(
            HttpRequest::default(),
            web::Data::new(web::Data::<PgPool>::new(PgPool::none()))
        )
        .await
        .is_ok());
    }
}
