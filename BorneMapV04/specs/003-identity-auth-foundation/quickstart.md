# Quickstart: Identity & Authentication

## Prerequisites

- Docker Compose stack running (from Sprint 2)
- `common-auth` crate built (`cargo build -p common-auth`)

## Testing Authentication

### 1. Verify Keycloak Realm

```bash
# Check Keycloak admin API is reachable
curl -s http://localhost:8080/realms/ev-platform/.well-known/openid-configuration | jq .
```

### 2. Obtain a Token

```bash
# Using the admin-cli client (built-in Keycloak client)
ACCESS_TOKEN=$(curl -s -X POST \
  http://localhost:8080/realms/ev-platform/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id=admin-cli" \
  -d "username=admin" \
  -d "password=admin" \
  -d "grant_type=password" | jq -r '.access_token')
echo "$ACCESS_TOKEN"
```

### 3. Test Protected Endpoint

```bash
# Without token (expect 401)
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/api/v1/favorites

# With token (expect 200)
curl -s -o /dev/null -w "%{http_code}" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  http://localhost:3000/api/v1/favorites
```

### 4. Run Auth Smoke Test

```bash
./scripts/auth-smoke-test.sh
```

## Environment Variables (New for Sprint 3)

Add to each service's `.env` file:

```ini
JWKS_URL=https://keycloak.internal/realms/ev-platform/protocol/openid-connect/certs
JWKS_REFRESH_INTERVAL=3600
ALLOWED_ISSUERS=https://keycloak.internal/realms/ev-platform
REQUIRED_AUDIENCE=account
```

## Key Files

| File | Purpose |
|---|---|
| `crates/common-auth/src/lib.rs` | Auth library public API |
| `infra/keycloak/realm-export/ev-platform-realm.json` | Code-managed IdP config |
| `scripts/auth-smoke-test.sh` | Auth integration test suite |
| `specs/003-identity-auth-foundation/contracts/auth-envelope.md` | Error response formats |
| `specs/003-identity-auth-foundation/contracts/auth-middleware-api.md` | Middleware API docs |
