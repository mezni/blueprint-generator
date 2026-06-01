use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::AuthError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub iss: String,
    pub aud: serde_json::Value,
    pub exp: usize,
    pub iat: usize,
    #[serde(rename = "realm_access")]
    pub realm_access: Option<RealmAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ValidatedToken {
    pub subject: String,
    pub roles: Vec<String>,
    pub raw_token: String,
}

pub struct JwtValidator {
    jwks_uri: String,
    issuer: String,
    audience: String,
    jwks_keys: Arc<RwLock<Vec<JwkKey>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwkKey {
    pub kty: String,
    pub kid: Option<String>,
    pub n: String,
    pub e: String,
    pub alg: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<JwkKey>,
}

impl JwtValidator {
    pub fn new(jwks_uri: String, issuer: String, audience: String) -> Self {
        Self {
            jwks_uri,
            issuer,
            audience,
            jwks_keys: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn refresh_jwks(&self) -> Result<(), AuthError> {
        let response = reqwest::get(&self.jwks_uri)
            .await
            .map_err(|e| AuthError::JwksError(format!("HTTP request failed: {e}")))?;

        let jwks: JwksResponse = response
            .json()
            .await
            .map_err(|e| AuthError::JwksError(format!("JSON parse failed: {e}")))?;

        let mut keys = self.jwks_keys.write().await;
        *keys = jwks.keys;
        Ok(())
    }

    pub async fn validate_token(&self, token: &str) -> Result<ValidatedToken, AuthError> {
        let header = jsonwebtoken::decode_header(token)
            .map_err(|e| AuthError::JwtError(format!("Invalid header: {e}")))?;

        let kid = header.kid.as_deref().unwrap_or("default");
        let keys = self.jwks_keys.read().await;
        let jwk = keys
            .iter()
            .find(|k| k.kid.as_deref() == Some(kid))
            .or_else(|| keys.first())
            .ok_or_else(|| AuthError::JwksError("No matching JWK found".into()))?;

        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| AuthError::JwtError(format!("Invalid JWK key: {e}")))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        validation.validate_exp = true;

        let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::JwtError(format!("Token validation failed: {e}")),
            })?;

        let roles = token_data
            .claims
            .realm_access
            .as_ref()
            .map(|r| r.roles.clone())
            .unwrap_or_default();

        Ok(ValidatedToken {
            subject: token_data.claims.sub,
            roles,
            raw_token: token.to_string(),
        })
    }

    pub fn required_roles_for_path(path: &str) -> HashSet<String> {
        let mut roles = HashSet::new();
        if path.starts_with("/api/v1/admin") {
            roles.insert("admin".to_string());
        } else if path.starts_with("/api/v1/partner") {
            roles.insert("partner".to_string());
            roles.insert("admin".to_string());
        } else if path.starts_with("/api/v1/driver") {
            roles.insert("registered_driver".to_string());
            roles.insert("partner".to_string());
            roles.insert("admin".to_string());
        }
        roles
    }
}
