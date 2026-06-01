use crate::error::AuthError;
use crate::validator::JwtValidator;
use std::time::Instant;

pub struct ClientCredentials {
    client_id: String,
    client_secret: String,
    token_url: String,
    cached_token: Option<CachedToken>,
    validator: JwtValidator,
}

struct CachedToken {
    raw: String,
    expires_at: Instant,
}

impl ClientCredentials {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        token_url: &str,
        jwks_uri: &str,
        issuer: &str,
    ) -> Self {
        let validator = JwtValidator::new(
            jwks_uri.into(),
            issuer.into(),
            client_id.into(),
        );
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            token_url: token_url.into(),
            cached_token: None,
            validator,
        }
    }

    pub async fn acquire_token(&mut self) -> Result<String, AuthError> {
        if let Some(cached) = &self.cached_token {
            if Instant::now() < cached.expires_at {
                return Ok(cached.raw.clone());
            }
        }

        let client = reqwest::Client::new();
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "client_credentials"),
        ];

        let resp = client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|_e| AuthError::AuthUnavailable)?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|_| AuthError::AuthUnavailable)?;

        let access_token = body["access_token"]
            .as_str()
            .ok_or(AuthError::AuthUnavailable)?
            .to_string();

        let expires_in = body["expires_in"].as_u64().unwrap_or(300);

        let _validated = self.validator.validate_token(&access_token).await?;

        self.cached_token = Some(CachedToken {
            raw: access_token.clone(),
            expires_at: Instant::now()
                + std::time::Duration::from_secs(expires_in.saturating_sub(60)),
        });

        Ok(access_token)
    }

    pub fn cached_token(&self) -> Option<&str> {
        self.cached_token
            .as_ref()
            .filter(|c| Instant::now() < c.expires_at)
            .map(|c| c.raw.as_str())
    }
}
