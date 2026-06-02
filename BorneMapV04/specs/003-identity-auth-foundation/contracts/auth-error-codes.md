# Auth Error Codes

## Error Enum (internal)

```rust
pub enum AuthError {
    /// No Authorization header present
    MissingAuthorizationHeader,
    /// Authorization header is not a valid Bearer token
    InvalidAuthorizationHeader,
    /// JWT parse failure (bad base64, malformed structure)
    TokenParse(String),
    /// JWT `kid` not found in cached JWKS
    UnknownKey(String),
    /// JWKS key decode failure
    KeyDecode(String),
    /// JWT validation failed (signature, expiry, issuer, audience)
    TokenValidation(String),
    /// JWKS not yet loaded (service in degraded mode)
    JwksNotLoaded,
    /// JWT signature is invalid
    InvalidSignature,
    /// Token has expired
    TokenExpired,
    /// JWT issuer does not match allowed issuers
    InvalidIssuer,
    /// JWT audience does not match required audience
    InvalidAudience,
    /// Role check failed (user lacks required role)
    InsufficientPermissions,
    /// Partner user has no tenant_id claim
    MissingTenantId,
}
```

## HTTP Mapping

| `AuthError` variant | HTTP Status | `error.code` |
|---|---|---|
| `MissingAuthorizationHeader` | 401 | `UNAUTHENTICATED` |
| `InvalidAuthorizationHeader` | 401 | `UNAUTHENTICATED` |
| `TokenParse` | 401 | `TOKEN_MALFORMED` |
| `UnknownKey` | 401 | `TOKEN_UNKNOWN_KEY` |
| `KeyDecode` | 401 | `TOKEN_MALFORMED` |
| `TokenValidation` | 401 | `TOKEN_INVALID_SIGNATURE` |
| `JwksNotLoaded` | 401 | `JWKS_NOT_LOADED` |
| `InvalidSignature` | 401 | `TOKEN_INVALID_SIGNATURE` |
| `TokenExpired` | 401 | `TOKEN_EXPIRED` |
| `InvalidIssuer` | 401 | `TOKEN_INVALID_ISSUER` |
| `InvalidAudience` | 401 | `TOKEN_INVALID_AUDIENCE` |
| `InsufficientPermissions` | 403 | `FORBIDDEN` |
| `MissingTenantId` | 403 | `FORBIDDEN` |
