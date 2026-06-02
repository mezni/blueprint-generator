# Auth Response Envelope

All auth-related error responses follow the standard API envelope defined in the constitution.

## Success (no separate auth envelope — uses existing success format)

```json
{
  "success": true,
  "data": { ... },
  "meta": {}
}
```

## Unauthenticated — HTTP 401

Returned when no valid JWT is provided.

```json
{
  "success": false,
  "error": {
    "code": "UNAUTHENTICATED",
    "message": "Authentication required"
  }
}
```

## Unauthenticated — HTTP 401 (specific error codes)

| `error.code` | HTTP 401 Scenario |
|---|---|
| `UNAUTHENTICATED` | No Authorization header provided |
| `TOKEN_EXPIRED` | JWT `exp` claim is in the past |
| `TOKEN_INVALID_SIGNATURE` | JWT signature does not match any JWKS key |
| `TOKEN_INVALID_ISSUER` | JWT `iss` claim does not match expected issuer |
| `TOKEN_INVALID_AUDIENCE` | JWT `aud` claim does not match expected audience |
| `TOKEN_MALFORMED` | JWT cannot be parsed (invalid base64, bad structure) |
| `TOKEN_UNKNOWN_KEY` | JWT `kid` header does not match any cached JWKS key |
| `JWKS_NOT_LOADED` | Service is in degraded mode (JWKS not yet available) |

## Forbidden — HTTP 403

Returned when authentication succeeds but the user's role is insufficient.

```json
{
  "success": false,
  "error": {
    "code": "FORBIDDEN",
    "message": "Insufficient permissions"
  }
}
```
