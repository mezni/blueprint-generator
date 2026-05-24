use common_utils::error::DomainError;

use super::claims::TokenClaims;

pub trait TokenClaimsExtractor {
    fn extract_claims(&self, secret: &str) -> Result<TokenClaims, DomainError>;
}

pub fn extract_claims_from_req(
    req: &actix_web::HttpRequest,
    secret: &str,
) -> Result<TokenClaims, DomainError> {
    req.extract_claims(secret)
}

pub fn require_admin(claims: &TokenClaims) -> Result<(), DomainError> {
    if !claims.is_admin() {
        return Err(DomainError::Forbidden(
            "Admin role required for this endpoint".into(),
        ));
    }
    Ok(())
}
