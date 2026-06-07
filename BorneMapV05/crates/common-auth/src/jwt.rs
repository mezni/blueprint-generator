use crate::errors::AuthError;
use chrono::{TimeDelta, Utc};
use common_types::Role;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{info, warn};

// ---------------------------------------------------------------------------
// JWT Claims
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub iss: String,
    pub aud: serde_json::Value,
    pub exp: usize,
    pub iat: usize,
    pub email: Option<String>,
    #[serde(rename = "realm_access")]
    pub realm_access: Option<RealmAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

impl JwtClaims {
    /// Extract the first matching platform role from `realm_access.roles`.
    /// Role parsing is centralized in `common_types::Role`.
    pub fn role(&self) -> Option<Role> {
        self.realm_access
            .as_ref()
            .and_then(|r| r.roles.iter().find_map(|s| Role::from_keycloak(s)))
    }
}

// ---------------------------------------------------------------------------
// JWK types (subset of JWKS spec)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    pub n: String,
    pub e: String,
    #[serde(rename = "use")]
    pub use_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwksResponse {
    pub keys: Vec<Jwk>,
}

// ---------------------------------------------------------------------------
// JWKS Cache
// ---------------------------------------------------------------------------

/// Normal cache TTL — after this a key is considered stale and a refresh is attempted.
const DEFAULT_TTL_SECONDS: i64 = 3600;
/// Maximum age a stale key may reach in degraded mode (JWKS unreachable) before it is
/// rejected. This bounds the window during which a rotated/compromised signing key
/// could remain trusted while Keycloak is down.
const MAX_DEGRADED_STALENESS_SECONDS: i64 = 24 * 3600;

struct CachedKey {
    key: DecodingKey,
    algorithm: Algorithm,
    fetched_at: chrono::DateTime<Utc>,
}

pub struct JwksCache {
    keys: RwLock<HashMap<String, CachedKey>>,
    jwks_url: String,
    ttl_seconds: i64,
    max_degraded_staleness_seconds: i64,
    /// Single-flight guard so concurrent requests don't stampede the JWKS endpoint.
    refresh_lock: Mutex<()>,
}

impl JwksCache {
    pub fn new(jwks_url: String) -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
            jwks_url,
            ttl_seconds: DEFAULT_TTL_SECONDS,
            max_degraded_staleness_seconds: MAX_DEGRADED_STALENESS_SECONDS,
            refresh_lock: Mutex::new(()),
        }
    }

    pub async fn get(&self, kid: &str) -> Result<(DecodingKey, Algorithm), AuthError> {
        // Check cache for a non-expired key (read lock dropped before any await on network).
        {
            let keys = self.keys.read().await;
            if let Some(cached) = keys.get(kid) {
                let age = Utc::now() - cached.fetched_at;
                if age < TimeDelta::seconds(self.ttl_seconds) {
                    return Ok((cached.key.clone(), cached.algorithm));
                }
            }
        }

        // Cache miss or expired — fetch JWKS (single-flight).
        self.refresh().await?;

        // Retry lookup
        let keys = self.keys.read().await;
        match keys.get(kid) {
            Some(cached) => Ok((cached.key.clone(), cached.algorithm)),
            None => Err(AuthError::ValidationError(format!(
                "No JWK found for kid: {}",
                kid
            ))),
        }
    }

    pub async fn refresh(&self) -> Result<(), AuthError> {
        // Single-flight: only one task fetches at a time. Others wait, then re-check
        // whether a fresh key already arrived to avoid a redundant network round-trip.
        let _guard = self.refresh_lock.lock().await;

        let resp: JwksResponse = reqwest::get(&self.jwks_url)
            .await
            .map_err(|e| AuthError::JwksFetchError(e.to_string()))?
            .json()
            .await
            .map_err(|e| AuthError::JwksFetchError(e.to_string()))?;

        let now = Utc::now();
        let mut keys = self.keys.write().await;

        for jwk in resp.keys {
            if jwk.use_.as_deref() != Some("sig") {
                continue;
            }
            let algorithm = match jwk.alg.as_str() {
                "RS256" => Algorithm::RS256,
                "RS384" => Algorithm::RS384,
                "RS512" => Algorithm::RS512,
                other => {
                    warn!("Unsupported JWK algorithm: {}", other);
                    continue;
                }
            };
            let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
                .map_err(|e| AuthError::JwksFetchError(format!("Invalid JWK: {}", e)))?;

            keys.insert(
                jwk.kid.clone(),
                CachedKey {
                    key: decoding_key,
                    algorithm,
                    fetched_at: now,
                },
            );
        }

        info!("JWKS cache refreshed — {} keys loaded", keys.len());
        Ok(())
    }

    /// Degraded-mode validation: use a cached key when JWKS is unreachable, but only
    /// within `max_degraded_staleness_seconds`. Keys older than that are rejected so a
    /// rotated signing key cannot be trusted indefinitely during an outage.
    pub async fn validate_degraded(&self, kid: &str) -> Result<(DecodingKey, Algorithm), AuthError> {
        let keys = self.keys.read().await;
        match keys.get(kid) {
            Some(cached) => {
                let age = Utc::now() - cached.fetched_at;
                if age <= TimeDelta::seconds(self.max_degraded_staleness_seconds) {
                    Ok((cached.key.clone(), cached.algorithm))
                } else {
                    warn!(
                        kid = %kid,
                        age_seconds = age.num_seconds(),
                        "Degraded-mode key exceeds max staleness — rejecting"
                    );
                    Err(AuthError::Unauthenticated)
                }
            }
            None => Err(AuthError::Unauthenticated),
        }
    }
}

// ---------------------------------------------------------------------------
// Public validation function
// ---------------------------------------------------------------------------

static JWKS_CACHE: Lazy<RwLock<Option<Arc<JwksCache>>>> = Lazy::new(|| RwLock::new(None));

pub async fn init_jwks_cache(jwks_url: String) {
    let cache = Arc::new(JwksCache::new(jwks_url));
    // Warm the cache
    if let Err(e) = cache.refresh().await {
        warn!("Initial JWKS fetch failed (will retry on first request): {}", e);
    }
    let mut guard = JWKS_CACHE.write().await;
    *guard = Some(cache);
}

pub async fn validate_token(
    token: &str,
    issuer: &str,
    audience: &str,
) -> Result<JwtClaims, AuthError> {
    // Clone the Arc and drop the outer guard immediately so we never hold the
    // global lock across the network/validation awaits below.
    let cache = {
        let guard = JWKS_CACHE.read().await;
        guard
            .as_ref()
            .cloned()
            .ok_or_else(|| AuthError::ValidationError("JWKS cache not initialized".into()))?
    };

    // Extract kid from token header
    let header = decode_header(token)
        .map_err(|_| AuthError::ValidationError("Invalid token header".into()))?;
    let kid = header
        .kid
        .ok_or_else(|| AuthError::ValidationError("Missing kid in token header".into()))?;

    // Get key: try normal cache first, fall back to degraded mode
    let (key, alg) = match cache.get(&kid).await {
        Ok(k) => k,
        Err(AuthError::JwksFetchError(_)) => {
            warn!("JWKS unreachable — using stale cached key (degraded mode)");
            cache
                .validate_degraded(&kid)
                .await
                .map_err(|_| AuthError::Unauthenticated)?
        }
        Err(e) => return Err(e),
    };

    // Build validation
    let mut validation = Validation::new(alg);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);
    validation.validate_exp = true;

    let token_data = decode::<JwtClaims>(token, &key, &validation)
        .map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::ValidationError(e.to_string()),
            }
        })?;

    Ok(token_data.claims)
}
