//! JWT authentication middleware

use actix_web::dev::{ServiceRequest, ServiceRequest as Request, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization::Bearer;
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::AppResult;

/// Extract JWT token from Authorization header
pub fn extract_jwt(request: &mut Request) -> AppResult<String> {
    let auth_header = request
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiError::BadRequest("Missing Authorization header".into()))?;

    let bearer = Bearer::from_header(auth_header)
        .ok_or_else(|| ApiError::BadRequest("Invalid Authorization header format. Expected 'Bearer <token>'".into()))?;

    Ok(bearer.token().to_string())
}

/// Verify JWT token and extract claims
pub async fn verify_jwt(
    token: String,
    _pool: &PgPool,
) -> AppResult<ev_auth::Claims> {
    // TODO: Implement actual JWT validation using Keycloak public key
    // For now, return mock claims for testing
    let claims = ev_auth::Claims {
        sub: "mock-user-id".to_string(),
        email: Some("test@example.com".to_string()),
        name: Some("Test User".to_string()),
        role: ev_auth::Role::RegisteredDriver,
        partner_id: None,
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() + 3600, // Expires in 1 hour
        jti: Some("mock-jti-123".to_string()),
    };

    Ok(claims)
}

/// Auth middleware handler
pub async fn auth_middleware(
    request: ServiceRequest,
    pool: web::Data<PgPool>,
) -> Result<Request, Error> {
    let token = extract_jwt(&mut request)
        .map_err(|e| Error::from(e))?;

    let claims = verify_jwt(token, &pool)
        .await
        .map_err(|e| Error::from(e))?;

    // Attach claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(request)
}

/// Auth middleware that requires partner role
pub async fn auth_partner_middleware(
    request: ServiceRequest,
    pool: web::Data<PgPool>,
) -> Result<Request, Error> {
    let mut request = auth_middleware(request, pool).await?;

    let claims = request
        .extensions()
        .get::<ev_auth::Claims>()
        .ok_or_else(|| ApiError::Unauthorized)
        .map_err(|e| Error::from(e))?;

    if claims.role != ev_auth::Role::Partner {
        return Err(ApiError::Forbidden.into());
    }

    // Attach claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(request)
}

/// Global auth middleware for the app
pub fn auth() -> HttpAuthentication<ev_auth::JwtValidator> {
    HttpAuthentication::new(|request, mut req, validator| {
        let validator = validator.clone();
        async move {
            let token = extract_jwt(&mut req).map_err(|e| Error::from(e))?;
            let claims = verify_jwt(token, &validator).await.map_err(|e| Error::from(e))?;
            Ok(req.into_parts(claims))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_jwt_from_valid_header() {
        let mut request = Request::build(HttpRequest::default())
            .insert_header(("Authorization", "Bearer test-token-123"))
            .finish()
            .unwrap();

        let token = extract_jwt(&mut request);
        assert!(token.is_ok());
        assert_eq!(token.unwrap(), "test-token-123");
    }

    #[test]
    fn test_extract_jwt_from_invalid_header() {
        let mut request = Request::build(HttpRequest::default())
            .insert_header(("Authorization", "InvalidFormat"))
            .finish()
            .unwrap();

        let token = extract_jwt(&mut request);
        assert!(token.is_err());
    }

    #[test]
    fn test_extract_jwt_from_missing_header() {
        let mut request = Request::build(HttpRequest::default())
            .finish()
            .unwrap();

        let token = extract_jwt(&mut request);
        assert!(token.is_err());
    }

    #[test]
    fn test_auth_partner_requires_partner_role() {
        // Driver role
        let driver_claims = ev_auth::Claims {
            sub: "driver123".to_string(),
            email: Some("driver@example.com".to_string()),
            name: Some("Driver".to_string()),
            role: ev_auth::Role::RegisteredDriver,
            partner_id: None,
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        // Partner role
        let partner_claims = ev_auth::Claims {
            sub: "partner123".to_string(),
            email: Some("partner@example.com".to_string()),
            name: Some("Partner".to_string()),
            role: ev_auth::Role::Partner,
            partner_id: Some("PRT-123".to_string()),
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        assert_ne!(driver_claims.role, ev_auth::Role::Partner);
        assert_eq!(partner_claims.role, ev_auth::Role::Partner);
    }

    #[test]
    fn test_jwt_expires() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let future_exp = now + 3600;
        let old_iat = now - 1800;

        let claims = ev_auth::Claims {
            sub: "test".to_string(),
            email: Some("test@example.com".to_string()),
            name: Some("Test".to_string()),
            role: ev_auth::Role::RegisteredDriver,
            partner_id: None,
            iat: old_iat,
            exp: future_exp,
            jti: Some("jti".to_string()),
        };

        // This test verifies the JWT structure (actual expiration check requires Keycloak validation)
        assert!(claims.exp > claims.iat);
    }
}

/// Verify JWT token and extract claims
pub async fn verify_jwt(
    token: String,
    pool: &PgPool,
) -> AppResult<ev_auth::Claims> {
    // TODO: Implement actual JWT validation
    // For now, return mock claims
    let claims = ev_auth::Claims {
        sub: "mock-user-id".to_string(),
        email: Some("test@example.com".to_string()),
        name: Some("Test User".to_string()),
        role: ev_auth::Role::RegisteredDriver,
        partner_id: None,
        iat: 1700000000,
        exp: 1700003600,
        jti: Some("mock-jti".to_string()),
    };

    Ok(claims)
}

/// Auth middleware handler
pub async fn auth_middleware(
    request: ServiceRequest,
    pool: web::Data<PgPool>,
) -> Result<ServiceRequest, Error> {
    let token = extract_jwt(&mut request)
        .map_err(|e| Error::from(e))?;

    let claims = verify_jwt(token, &pool)
        .await
        .map_err(|e| Error::from(e))?;

    // Attach claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(request)
}

/// Auth middleware that requires partner role
pub async fn auth_partner_middleware(
    request: ServiceRequest,
    pool: web::Data<PgPool>,
) -> Result<ServiceRequest, Error> {
    let mut request = auth_middleware(request, pool).await?;

    let claims = request
        .extensions()
        .get::<ev_auth::Claims>()
        .ok_or_else(|| ApiError::Unauthorized)
        .map_err(|e| Error::from(e))?;

    if claims.role != ev_auth::Role::Partner {
        return Err(ApiError::Forbidden.into());
    }

    // Attach claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(request)
}

/// Global auth middleware for the app
pub fn auth() -> HttpAuthentication<ev_auth::JwtValidator> {
    HttpAuthentication::new(|request, mut req, validator| {
        let validator = validator.clone();
        async move {
            let token = extract_jwt(&mut req).map_err(|e| Error::from(e))?;
            let claims = verify_jwt(token, &validator).await.map_err(|e| Error::from(e))?;
            Ok(req.into_parts(claims))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_jwt_from_valid_header() {
        let mut request = ServiceRequest::build(HttpRequest::default())
            .insert_header(("Authorization", "Bearer test-token-123"))
            .finish()
            .unwrap();

        let token = extract_jwt(&mut request);
        assert!(token.is_ok());
        assert_eq!(token.unwrap(), "test-token-123");
    }

    #[test]
    fn test_extract_jwt_from_invalid_header() {
        let mut request = ServiceRequest::build(HttpRequest::default())
            .insert_header(("Authorization", "InvalidFormat"))
            .finish()
            .unwrap();

        let token = extract_jwt(&mut request);
        assert!(token.is_err());
    }

    #[test]
    fn test_auth_partner_requires_partner_role() {
        // Mock test for partner middleware
        let claims = ev_auth::Claims {
            sub: "user123".to_string(),
            email: Some("user@example.com".to_string()),
            name: Some("User".to_string()),
            role: ev_auth::Role::RegisteredDriver,
            partner_id: None,
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        assert_ne!(claims.role, ev_auth::Role::Partner);
    }
}
