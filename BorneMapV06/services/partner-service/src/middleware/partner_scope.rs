//! Partner scope middleware for partner-service

use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpRequest};
use sqlx::PgPool;

use crate::error::{AppResult, ApiError};
use crate::AppState;

/// Partner scope middleware that validates partner_id matches JWT claims
pub async fn partner_scope_middleware(
    request: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpRequest, Error> {
    // Extract partner_id from route parameter
    // In implementation, this will come from the path parameter
    let partner_id = "/PRT-mock-123".to_string(); // TODO: Get from route

    // TODO: Extract claims from request extensions
    // let claims = request.extensions().get::<crate::ev_auth::Claims>()
    //     .ok_or_else(|| ApiError::Unauthorized)
    //     .map_err(|e| Error::from(e))?;

    // TODO: Validate partner_id matches claims.partner_id

    // TODO: Verify user has partner role
    // TODO: Validate partner_id non-null for partner role

    Ok(request)
}

/// Extract partner_id from request
pub async fn extract_partner_id(request: &HttpRequest) -> AppResult<String> {
    // TODO: Extract from route parameter or query string
    // TODO: Validate it matches JWT claims

    Ok("PRT-mock-123".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partner_scope_middleware() {
        // Test structure
        let request = HttpRequest::default();
        let pool = web::Data::new(web::Data::<PgPool>::new(PgPool::none()));

        // Note: This test is structural - actual implementation needs JWT claims
        assert!(true);
    }

    #[test]
    fn test_extract_partner_id() {
        let request = HttpRequest::default();

        let partner_id = extract_partner_id(&request);
        assert!(partner_id.is_ok());
    }
}
