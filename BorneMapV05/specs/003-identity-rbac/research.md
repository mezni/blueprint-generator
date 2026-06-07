# Research Decisions: Identity & RBAC

## Decision 1: HTTP Framework for Service Routing

**Decision**: Migrate services from raw `TcpListener` to `axum` with `tower` middleware stack.

**Rationale**: 
- Raw TCP listeners cannot support JWT middleware composition
- Axum is the most modern, ergonomic Rust web framework with first-class middleware via Tower
- Minimal dependency footprint compared to actix-web
- Follows the Rust async ecosystem standard (tokio + hyper)
- Middleware can be developed and tested independently in `common-auth`

**Alternatives considered**:
- **Raw TCP with manual JWT parsing**: Tedious, no middleware reuse, error-prone — rejected
- **actix-web**: Heavier, more complex middleware API than needed — rejected
- **warp**: Filter-based composition is powerful but less intuitive for middleware patterns — rejected
- **HTTP framework deferred to Sprint 5**: Would leave auth without a proper middleware home — rejected

**Implications**: 
- All 5 services need a main.rs refactor from TcpListener to Axum
- Services add `axum`, `tokio`, `tower-http` dependencies
- `common-auth` depends on `axum` for middleware traits
- The `/health` endpoint becomes a proper Axum route

---

## Decision 2: JWT/JWKS Validation Library

**Decision**: Use `jsonwebtoken` crate for JWT decoding/validation with manual JWKS key fetching via `reqwest`.

**Rationale**:
- `jsonwebtoken` is the de-facto standard Rust JWT library, well-maintained, supports RS256/RS384/RS512
- JWKS endpoint fetch via `reqwest` is straightforward: GET the JWKS URL, extract the matching `kid`, cache
- Keycloak issues RS256-signed tokens by default, which `jsonwebtoken` handles natively
- No need for a full OIDC client library — we only need token validation, not token issuance

**Alternatives considered**:
- **Full OIDC library (openidconnect crate)**: Heavy dependency for just token validation — rejected
- **Manual RSA signature verification**: Error-prone, no benefit — rejected
- **Keycloak token introspection endpoint**: Requires admin credentials per request — rejected (validating locally is faster)

**Implications**:
- Dependency: `jsonwebtoken`, `reqwest` (with rustls-tls), `serde`, `serde_json`
- JWKS response parsing: Keycloak's JWKS endpoint returns standard JWKS JSON
- Token validation: signature → expiration → issuer → audience → role extraction

---

## Decision 3: JWKS Caching Strategy

**Decision**: Cache JWKS keys in a `tokio::sync::RwLock<HashMap<String, CachedKey>>` with a configurable TTL (default 1 hour), refreshed on cache miss if TTL has expired.

**Rationale**:
- Keycloak rotates keys infrequently; per-request JWKS fetch is wasteful
- Cache invalidation: Keycloak sets `Cache-Control: max-age` on JWKS response; use this as TTL hint
- Thread-safe: RwLock allows concurrent reads, exclusive write on refresh
- Memory-safe: Cache holds at most 2-3 keys at a time

**Alternatives considered**:
- **No caching**: JWKS fetch on every request — 50-100ms added latency per auth call — rejected
- **moka/ cached libraries**: Overkill for a small key set — rejected
- **Manual lazy_static cache**: Simple, sufficient — chosen

**Implications**:
- On startup: no JWKS fetch (lazy, first request triggers fetch)
- On validation: check cache → if miss or expired, fetch JWKS → validate
- On validation failure with cached key: force-refresh JWKS and retry once (handles rotation mid-flight)
- **Degraded mode**: If JWKS endpoint is unreachable and cache has stale keys, use stale keys for validation (cached sessions continue to work). Requests without a cached key (cache miss) are rejected with UNAUTHENTICATED. Health endpoints remain accessible.

---

## Decision 4: Keycloak Realm Configuration as Code

**Decision**: Commit a Keycloak realm export JSON (`infra/compose/keycloak/realm-export.json`) and import it on first Keycloak startup via the Keycloak API.

**Rationale**:
- Realm export contains all configuration: roles, OIDC clients, identity providers, required actions
- Import via `kcadm.sh` or Keycloak Startup Command (`--import-realm`) on first boot
- Committed to Git = reproducible, reviewable, environment-independent
- Allows CI/test environments to create identical realms

**Alternatives considered**:
- **Manual admin console configuration**: Not reproducible, error-prone, blocks CI — rejected
- **Keycloak Admin API via script**: Flexible but adds script maintenance — acceptable for dynamic config, but export covers static setup
- **Terraform / Keycloak provider**: Overkill for 1 realm with 3 roles — rejected

**Implications**:
- Realm export must include: client `bornemap-api`, roles, identity providers (Google, Facebook stubs), required actions
- Identity provider client IDs/secrets from environment variables (not in export)
- Export is re-importable: idempotent for roles, clients (Keycloak skips existing)
- **Network topology**: Keycloak is internal-only. Traefik proxies `/auth/*` to Keycloak for OIDC flows. Backend services fetch JWKS directly via Keycloak's internal Docker hostname. gis-worker and analytics-writer are also internal-only.

---

## Decision 5: Auth Middleware Pattern

**Decision**: Axum middleware layers with three guard modes applied as route-layer filters.

**Pattern**:
1. **AuthLayer**: Extracts `Authorization: Bearer <token>`, validates JWT, populates `CurrentUser` in request extensions
2. **`require_role(Role)`**: Layer that checks `CurrentUser.role >= required`, returns `INSUFFICIENT_ROLE` if insufficient
3. **`require_authenticated()`**: Layer that rejects if no `CurrentUser` in extensions, returns `UNAUTHENTICATED`
4. **Public routes**: No middleware — handlers work without `CurrentUser` extension

**Rationale**:
- Tower layer pattern = composable, testable, route-scoped
- Each service applies auth layer globally, then role guards per route group
- Health endpoints excluded via `.layer()` ordering (apply auth after health routes)

**Alternatives considered**:
- **Single middleware with role check inside**: Less composable, harder to test — rejected
- **Macro-based guards**: Less flexible, harder to debug — rejected

**Implications**:
- `common-auth` exposes: `AuthLayer`, `CurrentUser` struct, `require_role()`, `require_authenticated()`
- Each service: imports `common-auth`, applies layers, defines route groups
- Test utilities: `TestAuthLayer` for injecting known users into test requests
