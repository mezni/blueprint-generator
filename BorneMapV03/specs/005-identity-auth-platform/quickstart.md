# Quickstart: Identity, Authentication & Authorization Platform

## Prerequisites

- EPIC 2 runtime fully operational (Docker Compose, Traefik, Keycloak running)
- EPIC 3 CI/CD pipeline operational
- Rust stable toolchain installed
- Node.js 20+ installed
- Access to Keycloak admin console (`http://localhost:8080/admin`)

## Setup

### 1. Update Keycloak Realm Configuration

Apply the updated realm export with new clients and roles:

```bash
# The realm export is at infra/keycloak/realm-export.json
# It should include:
#   - Realm: ev-platform
#   - Roles: registered_driver, partner, admin
#   - Clients: driver-web, driver-mobile, admin-dashboard, partner-dashboard, backend-service

# Restart Keycloak with the updated config:
docker compose -f infra/compose/docker-compose.yml up -d keycloak
```

### 2. Update the Backend Auth Middleware

```bash
# The common-auth crate is at crates/common-auth/
# Build and verify:
cd crates/common-auth && cargo build && cargo test

# Each backend service depends on common-auth in their Cargo.toml:
#   common-auth = { path = "../../crates/common-auth" }
```

### 3. Set Up the Frontend Auth Client

```bash
# The auth-client package is at packages/auth-client/
cd packages/auth-client && npm install && npm run build

# Each frontend app depends on auth-client in their package.json:
#   "@borne-map/auth-client": "*"
```

## Development Workflow

```bash
# 1. Start the platform (EPIC 2):
docker compose -f infra/compose/docker-compose.yml -f infra/compose/docker-compose.dev.yml up -d

# 2. Verify Keycloak is running:
curl http://localhost:8080/realms/ev-platform/.well-known/openid-configuration

# 3. Test login flow (via API):
# Get an access token:
curl -X POST http://localhost:8080/realms/ev-platform/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id=backend-service" \
  -d "client_secret=<secret>" \
  -d "grant_type=client_credentials"

# 4. Test protected endpoint:
curl -H "Authorization: Bearer <token>" http://localhost/api/v1/admin/stations

# 5. Test auth rejection:
curl http://localhost/api/v1/admin/stations
# Expected: 401 Unauthorized
```

## Verification

| Test | Command | Expected |
|------|---------|----------|
| Health endpoint | `curl http://localhost/health` | 200 OK |
| Unauthenticated request | `curl http://localhost/api/v1/driver/stations` | 401 Unauthorized |
| Invalid token | `curl -H "Authorization: Bearer invalid" http://localhost/api/v1/driver/stations` | 401 Unauthorized |
| Expired token | Use an expired JWT | 401 Unauthorized |
| Role mismatch | Driver token → `/api/v1/admin/*` | 403 Forbidden |
| Public route | `curl http://localhost/health` | 200 OK (no auth) |
| Login page | Open `http://localhost:8080/realms/ev-platform/account` in browser | Keycloak login form |
| Auth audit | Check RabbitMQ queue `events.exchange` | Login/logout events present |

## CI/CD Integration

Add auth validation tests to the EPIC 3 `pr-validation.yml` workflow:

```yaml
auth-test:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Start Keycloak
      run: docker compose -f infra/compose/docker-compose.yml up -d keycloak
    - name: Run auth flow tests
      run: |
        # Token issuance test
        # Protected endpoint rejection test
        # Role enforcement test
        # Refresh flow test
```

## Shutdown

```bash
# Stop Keycloak and dependent services:
docker compose -f infra/compose/docker-compose.yml stop keycloak
```
