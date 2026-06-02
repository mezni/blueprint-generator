# Research: Identity & Authentication Foundation

## 1. Rust JWT Library — jsonwebtoken

**Decision**: Use `jsonwebtoken` crate with `jwk` feature

**Rationale**: The `jsonwebtoken` crate is the most mature and widely used Rust JWT library. It supports RS256/RS384/RS512 asymmetric key validation natively. The `jwk` feature enables decoding JWK keys from the JWKS endpoint response, which is essential for validating Keycloak-issued tokens. The `Validation` struct allows configuring issuer, audience, leeway, and required algorithms — all needed for this sprint.

**Alternatives considered**: `biscuit` (more complex API, less community adoption), `aws-lc-rs` (AWS-specific, overkill), manual RS256 verification using `rsa` crate (too low-level).

**Key API**:
```rust
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};

// Parse JWKS response
let jwks: JwkSet = serde_json::from_str(&jwks_response)?;

// Decode token using JWKS
let header = decode_header(token)?;
let kid = header.kid.ok_or(AuthError::MissingKid)?;
let jwk = jwks.find(&kid).ok_or(AuthError::UnknownKey)?;
let key = DecodingKey::from_jwk(jwk)?;
let validation = Validation::new(Algorithm::RS256);
let data = decode::<Claims>(token, &key, &validation)?;
```

## 2. Axum Middleware Pattern for Route Guards

**Decision**: Use axum's `Middleware` layer with state-based route splitting

**Rationale**: axum 0.7 supports Layered middleware via `Router::layer()`. The pattern is:
- Build public routes on a base router (no auth layer)
- Nest protected routes under a separate router with the auth middleware applied
- The middleware extracts the `AuthContext` from request extensions

**Pattern**:
```rust
use axum::{
    Router, middleware,
    routing::{get, post},
};

let public_routes = Router::new()
    .route("/api/v1/stations", get(list_stations))
    .route("/api/v1/stations/{id}", get(get_station));

let protected_routes = Router::new()
    .route("/api/v1/favorites", get(list_favorites))
    .route("/api/v1/favorites", post(add_favorite))
    .layer(middleware::from_fn_with_state(state, auth_middleware));

let app = Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    .with_state(state);
```

The `require_role` / `require_any_role` functions are implemented as axum middleware that:
1. Extract `AuthContext` from request extensions (populated by `auth_middleware`)
2. Check if the user has the required role
3. Return 403 if unauthorized

## 3. Keycloak Realm Export Structure

**Decision**: Update the existing `ev-platform-realm.json` with:

- **Clients**: `driver-web` (public), `partner-dashboard` (public), `admin-dashboard` (public), `driver-mobile` (public), `platform-service` (confidential with service account)
- **Roles** (realm-level): `registered_driver`, `partner`, `admin`
- **User attribute mapper**: A client-scope mapper that reads the `tenant_id` user attribute and injects it as a custom `tenant_id` claim in the JWT
- **Protocol mappers**: Add `realm roles` mapper to include realm roles in the `realm_access.roles` claim

**JWKS URL pattern**: `{keycloak_url}/realms/ev-platform/protocol/openid-connect/certs`

**Token validation parameters**:
- Issuer: `{keycloak_url}/realms/ev-platform`
- Audience: `account` (default Keycloak audience) or specific client ID
- Algorithm: `RS256`
- Leeway: 0s (tokens are validated at exact time)

## 4. JWKS Refresh Strategy

**Decision**: Refresh on startup + periodic refresh with fallback

**Rationale**: 
- On startup, fetch JWKS synchronously (blocking) before binding the HTTP listener. If fetch fails, log warning and continue — service boots in "degraded" mode, rejecting all authenticated requests.
- Spawn a background task that refreshes JWKS at a configurable interval (default 3600s).
- If a periodic refresh fails, keep the existing JWKS cache and retry on the next interval. Log the failure.
- The refresh interval should have jitter (±10%) to avoid thundering herd on Keycloak.

**Graceful degradation**:
```
Startup:
  1. Fetch JWKS → success → cache keys → serve normally
  2. Fetch JWKS → failure → log warning → boot degraded → reject auth requests
  
Per-request:
  1. Extract token → no token → 401
  2. Validate with cached JWKS → valid → extract claims → proceed
  3. Validate with cached JWKS → invalid → 401 with error code
  
Periodic refresh:
  1. Refresh succeeds → update cache → next interval
  2. Refresh fails → keep old cache → log error → retry next interval
```

## 5. Partner Isolation at Repository Layer

**Decision**: Thread `AuthContext` through axum state into repository functions

**Rationale**: The constitution mandates repository-layer enforcement. The `AuthContext` (containing `tenant_id` for partner roles) is extracted by the auth middleware and stored in request extensions. Route handlers extract it and pass it to repository methods.

**Pattern**:
```rust
// In route handler
async fn create_station(
    auth: AuthContext,  // Extracted via axum Extension
    Json(payload): Json<CreateStationRequest>,
) -> Result<Json<StationResponse>, AppError> {
    let tenant_id = auth.require_tenant()?;  // Returns 403 if partner has no tenant_id
    let station = StationRepository::create(tenant_id, payload).await?;
    Ok(Json(station.into()))
}

// In repository
impl StationRepository {
    async fn create(tenant_id: &str, payload: CreateStationRequest) -> Result<Station, DbError> {
        sqlx::query_as!(
            Station,
            "INSERT INTO stations (partner_id, name, ...) VALUES ($1, $2, ...) RETURNING *",
            tenant_id, payload.name, ...
        )
        .fetch_one(&pool)
        .await
    }
}
```

**Key constraint**: No tenant_id is ever accepted from client JSON payloads. The repository always derives it from the authenticated `AuthContext`.

## 6. Environment Variable Schema for Auth

```text
# Auth configuration (common-auth)
JWKS_URL=https://keycloak.internal/realms/ev-platform/protocol/openid-connect/certs
JWKS_REFRESH_INTERVAL=3600       # seconds, default 3600
ALLOWED_ISSUERS=https://keycloak.internal/realms/ev-platform
REQUIRED_AUDIENCE=account         # or specific client ID
```
