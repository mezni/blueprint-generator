# Data Model: Identity & Authentication

## Entity: AuthContext

The `AuthContext` is the request-scoped security context produced by JWT validation. It represents the authenticated user for the duration of a single HTTP request. No DB persistence.

| Field | Type | Source | Description |
|-------|------|--------|-------------|
| `sub` | `String` | JWT `sub` claim | Keycloak user ID — the canonical user identifier |
| `roles` | `Vec<Role>` | JWT `realm_access.roles` | Assigned realm roles for this user |
| `tenant_id` | `Option<String>` | JWT `tenant_id` claim (custom mapper) | Partner tenant ID — `Some` for partner role, `None` for driver/admin |

## Enum: Role

Three and only three roles exist per constitution:

| Variant | JWT Value | Description |
|---------|-----------|-------------|
| `RegisteredDriver` | `registered_driver` | Authenticated EV driver — can manage favorites, reviews, profile |
| `Partner` | `partner` | Charging station partner — tenant-scoped station management |
| `Admin` | `admin` | Platform administrator — global access |

## Keycloak Realm Configuration

### Clients

| Client ID | Type | Description |
|-----------|------|-------------|
| `driver-web` | Public | Driver-facing web application |
| `driver-mobile` | Public | Driver-facing mobile application |
| `partner-dashboard` | Public | Partner-facing dashboard |
| `admin-dashboard` | Public | Admin-facing dashboard |
| `platform-service` | Confidential | Service-to-service machine-to-machine auth (future use) |

### Realm Roles

| Role Name | Description |
|-----------|-------------|
| `registered_driver` | Standard authenticated driver |
| `partner` | Station partner with tenant-scoped access |
| `admin` | Platform administrator |

### Protocol Mappers (on `ev-platform` realm)

| Mapper Name | Type | Claim | Description |
|-------------|------|-------|-------------|
| `realm roles` | `realm roles` | `realm_access.roles` | Maps realm roles into JWT realm_access.roles claim (built-in) |
| `tenant_id` | User Attribute | `tenant_id` | Maps Keycloak user attribute `tenant_id` to JWT `tenant_id` claim |

### JWT Claim Structure (after validation)

```json
{
  "sub": "a1b2c3d4-e5f6-...",
  "realm_access": {
    "roles": ["partner"]
  },
  "tenant_id": "prt-001",
  "iss": "https://keycloak.internal/realms/ev-platform",
  "aud": "account",
  "exp": 1717200000,
  "iat": 1717196400
}
```

## State Transitions

- **Unauthenticated → Authenticated**: Client obtains JWT from Keycloak via OAuth2 login. JWT is presented as Bearer token on each request.
- **Authenticated → Unauthenticated**: JWT expires (checked on every request). Client must obtain a new token via refresh or re-login.
- **Role change**: When a Keycloak admin changes a user's realm role, the change takes effect when the user obtains a new JWT (existing tokens retain old roles until expiry).
- **Tenant change**: When a partner's `tenant_id` changes in Keycloak, the change takes effect on next token refresh.

## Validation Rules

- `sub` must be a non-empty UUID (v4) string
- `roles` must contain at least one recognized role; unknown roles are silently ignored
- `tenant_id` must be non-empty when role is `Partner`; service returns 403 if missing
- `tenant_id` is ignored for `RegisteredDriver` and `Admin` roles
- Token must not be expired (`exp` claim checked with 0s leeway)
- Issuer must match `ALLOWED_ISSUERS` env var
- Signature must be verifiable with a key from the configured JWKS endpoint
