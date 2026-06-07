//! JWT validation using jsonwebtoken

use crate::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum JwtValidatorError {
    #[error("Invalid token: {0}")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),

    #[error("Invalid claims: {0}")]
    InvalidClaims(String),

    #[error("Token expired")]
    TokenExpired,
}

pub type JwtResult<T> = Result<T, JwtValidatorError>;

/// JWT Validator for Keycloak tokens
pub struct JwtValidator {
    decoding_key: Arc<DecodingKey>,
    validation: Validation,
}

impl JwtValidator {
    /// Create a new JWT validator with a public key
    ///
    /// # Arguments
    /// * `public_key` - PEM-encoded public key from Keycloak
    ///
    /// # Errors
    /// Returns error if the public key is invalid
    pub fn new(public_key: &str) -> JwtResult<Self> {
        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.validate_exp = true;

        Ok(Self {
            decoding_key: Arc::new(decoding_key),
            validation,
        })
    }

    /// Validate and decode a JWT token
    ///
    /// # Arguments
    /// * `token` - JWT token string
    ///
    /// # Errors
    /// Returns error if token is invalid, expired, or claims are invalid
    pub fn validate(&self, token: &str) -> JwtResult<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)?;
        let claims = token_data.claims;

        // Validate partner scope
        claims
            .validate_partner_scope()
            .map_err(|e| JwtValidatorError::InvalidClaims(e.to_string()))?;

        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_is_expired() {
        let claims = Claims {
            sub: "user123".to_string(),
            email: Some("user@example.com".to_string()),
            name: Some("User".to_string()),
            role: crate::Role::RegisteredDriver,
            partner_id: None,
            iat: 1000,
            exp: 2000,
            jti: None,
        };

        assert!(!claims.is_expired(1500)); // Current time before expiry
        assert!(claims.is_expired(2001)); // Current time after expiry
    }

    #[test]
    fn test_partner_scope_validation() {
        let mut claims = Claims {
            sub: "user123".to_string(),
            email: Some("user@example.com".to_string()),
            name: Some("User".to_string()),
            role: crate::Role::Partner,
            partner_id: Some("partner123".to_string()),
            iat: 1000,
            exp: 2000,
            jti: None,
        };

        // Partner with partner_id should validate
        assert!(claims.validate_partner_scope().is_ok());

        // Partner without partner_id should fail
        claims.partner_id = None;
        assert!(claims.validate_partner_scope().is_err());

        // Non-partner without partner_id should validate
        claims.role = crate::Role::RegisteredDriver;
        assert!(claims.validate_partner_scope().is_ok());

        // Non-partner with partner_id should fail
        claims.partner_id = Some("partner123".to_string());
        assert!(claims.validate_partner_scope().is_err());
    }
}
