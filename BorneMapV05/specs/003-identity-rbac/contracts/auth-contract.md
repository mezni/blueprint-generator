# Auth Contract API

## Auth Error Responses

All auth-related errors use the standard error envelope:

```json
{
  "success": false,
  "error": {
    "code": "UNAUTHENTICATED",
    "message": "Authentication required",
    "details": null
  }
}
```

### Error Codes (Auth)

| Code | HTTP Status | When Returned |
|------|-------------|---------------|
| `UNAUTHENTICATED` | 401 | No token provided or token missing required claims |
| `TOKEN_EXPIRED` | 401 | Token is past its `exp` claim |
| `INSUFFICIENT_ROLE` | 403 | Token is valid but role lacks permission for the endpoint |
| `FORBIDDEN` | 403 | Token is valid but user is not the resource owner |
| `PARTNER_SCOPE_VIOLATION` | 403 | Partner query attempts to access another partner's data |

---

## JWT Validation Contract

### Required Request Header

```
Authorization: Bearer <jwt_token>
```

### Validation Steps (in order)

1. Extract token from `Authorization: Bearer <token>` header
2. Decode JWT header to extract `kid`
3. Fetch JWK matching `kid` from cache or JWKS endpoint
4. Verify token signature using the JWK
5. Validate `exp` (must be future)
6. Validate `iss` (must match `AUTH_ISSUER`)
7. Validate `aud` (must contain `AUTH_AUDIENCE`)
8. Extract role from `realm_access.roles` (first matching role wins)
9. Look up or auto-provision `user_account`
10. Populate `CurrentUser` with identity and permissions

### On Validation Failure

| Failure Point | Error Code | Details |
|---------------|------------|---------|
| Missing header | `UNAUTHENTICATED` | `"Authentication required"` |
| Malformed token | `UNAUTHENTICATED` | `"Invalid token format"` |
| Signature invalid | `UNAUTHENTICATED` | `"Token signature verification failed"` |
| Token expired | `TOKEN_EXPIRED` | `"Token has expired"` |
| Issuer mismatch | `UNAUTHENTICATED` | `"Invalid token issuer"` |
| Audience mismatch | `UNAUTHENTICATED` | `"Invalid token audience"` |
| No valid role | `INSUFFICIENT_ROLE` | `"No valid role found in token"` |

---

## Auth Guard Contract

### Guard Levels

| Guard | Behavior | Used For |
|-------|----------|----------|
| **Public** | No auth check; `CurrentUser` is `None` | Health checks, station discovery, search (Sprint 7) |
| **Authenticated** | Token required; `CurrentUser` populated with identity | Favorites, reviews, profile (Sprint 7+) |
| **Role(admin)** | Token required + `role == admin` | Admin CRUD endpoints (Sprint 5+) |
| **Role(partner)** | Token required + `role == partner` | Partner CRUD endpoints (Sprint 5+) |
| **Role(registered_driver)** | Token required + `role == registered_driver` | Driver authenticated actions (Sprint 7+) |

### Middleware Architecture

```
Request
  │
  ▼
Auth Layer (JWT validation, CurrentUser population)
  │
  ├── Public route ──→ Handler
  │
  ├── Role-guard layer ──→ Handler (role check before handler)
  │
  └── Resource-owner check (handler-level, if applicable)
```

---

## First-Login Provisioning Contract

### Flow

```
1. Valid JWT received
2. Look up user_account by keycloak_user_id (JWT.sub)
3. If found:
   - Update last_login_at
   - Return CurrentUser with existing profile
4. If not found (first login):
   - INSERT user_account (id = new ULID with USR- prefix, keycloak_user_id = JWT.sub, email = JWT.email, status = 'active')
   - If Keycloak user has custom attribute `partner_id`:
     - Validate partner_id exists in inventory.partner
     - INSERT partner_membership (user_id, partner_id, role from Keycloak attribute or default 'viewer')
   - Return CurrentUser with new identity
```

### Idempotency

- `user_account.keycloak_user_id` is UNIQUE — duplicate insert fails, but the SELECT-before-INSERT ensures exactly one
- In case of race condition (concurrent first logins), second insert fails UNIQUE constraint; retry with SELECT

---

## CurrentUser Data Structure

```json
{
  "user_id": "USR-01ABCDEF",
  "keycloak_user_id": "a1b2c3d4-...",
  "email": "user@example.com",
  "role": "partner",
  "partner_id": "PRT-01XYZ"
}
```

Passed through Axum request extensions. Available to all handlers downstream of the auth layer.
