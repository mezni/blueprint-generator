# Auth Middleware Contract

**Enforced by**: `crates/common-auth` library
**Target runtime**: All Rust backend services (admin-service, driver-service, clickstream-service, gis-sync-worker)

## Responsibilities

- Extract Bearer token from `Authorization` header
- Validate JWT: signature (RS256 via JWKS), issuer (`https://keycloak:8080/realms/ev-platform`), expiry, audience
- Parse `realm_access.roles` claim
- Inject authenticated identity into request context
- Return 401 for missing/invalid/expired tokens
- Return 403 for valid tokens with insufficient role permissions

## Request Flow

```
Incoming Request
  → Traefik Gateway (fast token rejection: expiry + signature)
  → Backend Service
    → common-auth middleware
      → Extract Bearer token
      → Validate JWT (signature, issuer, exp, aud)
      → Extract roles
      → Inject UserContext into request
    → Route handler (checks role against route)
    → Response
```

## Public Endpoints (no auth required)

```
GET  /health          → Health check endpoint
GET  /ready           → Readiness endpoint
POST /auth/*          → Authentication gateway routes
```

## Error Response Format

```json
{
  "error_code": "UNAUTHORIZED",
  "message": "Token is missing or invalid",
  "trace_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

| HTTP Status | `error_code` | When |
|-------------|-------------|------|
| 401 | `UNAUTHORIZED` | Missing, expired, or invalid token |
| 403 | `FORBIDDEN` | Valid token but insufficient role |
| 500 | `AUTH_UNAVAILABLE` | Identity provider unreachable (gateway fallback) |

## Token Requirements

| Requirement | Specification |
|-------------|---------------|
| Format | JWT (signed, not encrypted) |
| Signing algorithm | RS256 (RSA with SHA-256) |
| JWKS endpoint | `https://keycloak:8080/realms/ev-platform/protocol/openid-connect/certs` |
| Required claims | `sub`, `exp`, `iss`, `aud`, `realm_access.roles` |
| Access token lifetime | 15 minutes |
| Role claim path | `realm_access.roles` (array of strings) |
