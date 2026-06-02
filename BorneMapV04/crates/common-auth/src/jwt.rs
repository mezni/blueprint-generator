use crate::claims::{AuthContext, KeycloakClaims, Role};
use crate::config::AuthConfig;
use crate::error::AuthError;
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct JwtValidator {
    jwks: Arc<RwLock<Option<JwkSet>>>,
    config: AuthConfig,
}

impl JwtValidator {
    pub fn new(config: AuthConfig) -> Self {
        JwtValidator {
            jwks: Arc::new(RwLock::new(None)),
            config,
        }
    }

    pub async fn init(&self) {
        match self.fetch_jwks().await {
            Ok(keys) => {
                *self.jwks.write().await = Some(keys);
                info!("JWKS loaded successfully");
            }
            Err(e) => {
                warn!("Failed to fetch JWKS on startup: {e}. Service will reject authenticated requests.");
            }
        }

        let interval = std::time::Duration::from_secs(self.config.jwks_refresh_interval);
        let validator = self.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                if let Err(e) = validator.fetch_jwks().await {
                    error!("Periodic JWKS refresh failed: {e}. Keeping existing keys.");
                } else {
                    info!("JWKS refreshed successfully");
                }
            }
        });
    }

    async fn fetch_jwks(&self) -> Result<JwkSet, AuthError> {
        let response = reqwest::get(&self.config.jwks_url)
            .await
            .map_err(|e| AuthError::TokenValidation(format!("JWKS fetch failed: {e}")))?;

        let jwks: JwkSet = response
            .json()
            .await
            .map_err(|e| AuthError::TokenValidation(format!("JWKS parse failed: {e}")))?;

        *self.jwks.write().await = Some(jwks.clone());
        Ok(jwks)
    }

    pub async fn validate_token(&self, token: &str) -> Result<AuthContext, AuthError> {
        let jwks_lock = self.jwks.read().await;
        let jwks = jwks_lock
            .as_ref()
            .ok_or(AuthError::JwksNotLoaded)?;

        let header = decode_header(token)
            .map_err(|e| AuthError::TokenParse(e.to_string()))?;

        let kid = header
            .kid
            .ok_or_else(|| AuthError::TokenParse("Missing kid header".into()))?;

        let jwk = jwks
            .find(&kid)
            .ok_or_else(|| AuthError::UnknownKey(kid.clone()))?;

        let key =
            DecodingKey::from_jwk(jwk).map_err(|e| AuthError::KeyDecode(e.to_string()))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&self.config.allowed_issuers);
        validation.set_audience(&[&self.config.required_audience]);
        validation.validate_exp = true;
        validation.leeway = 0;

        let token_data = decode::<KeycloakClaims>(token, &key, &validation)
            .map_err(|e| {
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                    jsonwebtoken::errors::ErrorKind::InvalidSignature => AuthError::InvalidSignature,
                    jsonwebtoken::errors::ErrorKind::InvalidIssuer => AuthError::InvalidIssuer,
                    jsonwebtoken::errors::ErrorKind::InvalidAudience => AuthError::InvalidAudience,
                    _ => AuthError::TokenValidation(e.to_string()),
                }
            })?;

        let claims = token_data.claims;

        let roles: Vec<Role> = claims
            .realm_access
            .as_ref()
            .map(|ra| {
                ra.roles
                    .iter()
                    .filter_map(|r| Role::try_from(r.as_str()).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(AuthContext {
            sub: claims.sub,
            roles,
            tenant_id: claims.tenant_id,
        })
    }

    pub fn config(&self) -> &AuthConfig {
        &self.config
    }
}
