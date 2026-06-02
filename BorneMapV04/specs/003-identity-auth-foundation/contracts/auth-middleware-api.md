# common-auth Middleware API Contract

## Public API (from `common-auth` crate)

### Types

```rust
/// The three allowed realm roles.
pub enum Role {
    RegisteredDriver,
    Partner,
    Admin,
}

/// Request-scoped security context extracted from a validated JWT.
pub struct AuthContext {
    pub sub: String,
    pub roles: Vec<Role>,
    pub tenant_id: Option<String>,
}

impl AuthContext {
    /// Returns tenant_id if role is Partner, otherwise returns Forbidden error.
    pub fn require_tenant(&self) -> Result<&str, AuthError>;
}
```

### Configuration

```rust
/// Auth configuration loaded from environment variables.
pub struct AuthConfig {
    pub jwks_url: String,
    pub jwks_refresh_interval: u64,  // seconds, default 3600
    pub allowed_issuers: Vec<String>,
    pub required_audience: String,
}

impl AuthConfig {
    /// Loads auth config from env vars. Crashes on missing required vars.
    pub fn from_env() -> Result<Self, ConfigError>;
}
```

### Middleware Functions

```rust
/// Axum middleware: validates JWT from Authorization header.
/// Injects AuthContext into request extensions on success.
/// Returns 401 on invalid/missing token.
pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode>;

/// Axum middleware guard: rejects request if user lacks the required role.
/// Must be placed AFTER auth_middleware in the layer stack.
pub fn require_role(role: Role) -> Middleware;

/// Axum middleware guard: rejects request if user lacks any of the listed roles.
/// Must be placed AFTER auth_middleware in the layer stack.
pub fn require_any_role(roles: &[Role]) -> Middleware;
```

### Usage Pattern

```rust
use common_auth::{auth_middleware, require_role, Role};

let public_routes = Router::new()
    .route("/api/v1/stations", get(list_stations));

let protected_routes = Router::new()
    .route("/api/v1/favorites", get(list_favorites))
    .route_layer(middleware::from_fn(auth_middleware));

let partner_routes = Router::new()
    .route("/api/v1/partner/stations", get(list_partner_stations))
    .route_layer(middleware::from_fn(auth_middleware))
    .layer(require_role(Role::Partner));

let app = Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    .merge(partner_routes);
```

### AuthConfig Environment Variables

| Variable | Required | Default | Description |
|---|---|---|---|
| `JWKS_URL` | Yes | — | Full URL to Keycloak JWKS endpoint |
| `JWKS_REFRESH_INTERVAL` | No | `3600` | Seconds between JWKS cache refreshes |
| `ALLOWED_ISSUERS` | Yes | — | Comma-separated list of allowed JWT issuers |
| `REQUIRED_AUDIENCE` | Yes | — | Expected JWT audience claim value |
