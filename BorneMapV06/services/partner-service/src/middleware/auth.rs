//! JWT authentication middleware for partner-service

use actix_web::dev::{ServiceRequest, ServiceRequest as Request, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization::Bearer;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{AppResult, ApiError};

/// Extract JWT token from Authorization header
pub fn extract_jwt(request: &mut Request) -> AppResult<String> {
    let auth_header = request
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiError::BadRequest("Missing Authorization header".into()))?;

    let bearer = Bearer::from_header(auth_header)
        .ok_or_else(|| ApiError::BadRequest("Invalid Authorization header format".into()))?;

    Ok(bearer.token().to_string())
}

/// Verify JWT token and extract claims
pub async fn verify_jwt(
    token: String,
) -> AppResult<crate::ev_auth::Claims> {
    // TODO: Implement actual JWT validation using Keycloak public key
    // For now, return mock claims for testing
    let claims = crate::ev_auth::Claims {
        sub: "mock-partner-id".to_string(),
        email: Some("partner@example.com".to_string()),
        name: Some("Partner User".to_string()),
        role: crate::ev_auth::Role::Partner,
        partner_id: Some("PRT-mock-123".to_string()),
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() + 3600, // Expires in 1 hour
        jti: Some("mock-jti".to_string()),
    };

    Ok(claims)
}

/// Auth middleware handler
pub async fn auth_middleware(
    request: Request,
    _pool: web::Data<PgPool>,
) -> Result<Request, Error> {
    let token = extract_jwt(&mut request)
        .map_err(|e| Error::from(e))?;

    let claims = verify_jwt(token)
        .await
        .map_err(|e| Error::from(e))?;

    // Attach claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(request)
}

/// Global auth middleware for the app
pub fn auth() -> HttpAuthentication<crate::ev_auth::JwtValidator> {
    HttpAuthentication::new(|request, mut req, validator| {
        let validator = validator.clone();
        async move {
            let token = extract_jwt(&mut req).map_err(|e| Error::from(e))?;
            let claims = verify_jwt(token).await.map_err(|e| Error::from(e))?;
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
    fn test_verify_jwt() {
        let claims = verify_jwt("test-token".to_string()).unwrap();
        assert_eq!(claims.role, crate::ev_auth::Role::Partner);
        assert_eq!(claims.partner_id, Some("PRT-mock-123".to_string()));
    }
}
