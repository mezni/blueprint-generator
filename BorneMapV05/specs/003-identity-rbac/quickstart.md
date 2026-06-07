# Quickstart: Identity & RBAC Sprint

## Prerequisites

- Sprint 2 infrastructure running (`docker compose up` with Keycloak, PostgreSQL, RabbitMQ, Traefik)
- Rust toolchain (edition 2024)
- `cargo build` succeeds on the monorepo

## Setup

### 1. Keycloak Realm

The Keycloak realm configuration is at `infra/compose/keycloak/realm-export.json` (to be created during implementation). On first Keycloak startup, the realm is imported automatically via the `--import-realm` startup option.

To manually import (e.g., for development):

```bash
docker compose exec keycloak /opt/keycloak/bin/kcadm.sh create realms -f /opt/keycloak/data/import/realm-export.json --server http://localhost:8080
```

### 2. Environment Variables

Each service needs the following auth-related environment variables (in their `.env` files under `infra/env/`):

```env
AUTH_ISSUER=http://keycloak:8080/realms/bornemap
AUTH_JWKS_URL=http://keycloak:8080/realms/bornemap/protocol/openid-connect/certs
AUTH_AUDIENCE=bornemap-api
```

Clients access Keycloak through Traefik (`/auth/*` routes), not directly. Backend services fetch JWKS directly via Keycloak's internal Docker hostname.

### 3. Build & Test

```bash
# Build the crate
cargo build -p common-auth

# Run auth tests
cargo test -p common-auth

# Run all integration tests
cargo test --test '*' --features integration-test
```

## Manual Verification

### Health Check (no auth required)

```bash
curl http://localhost:8081/health
# → {"status":"ok"}
```

### Get a Test Token

Login via Keycloak's OIDC flow:

```bash
# Using password grant (development only - OAuth2 providers for prod)
TOKEN=$(curl -s -X POST http://localhost:8080/realms/bornemap/protocol/openid-connect/token \
  -d "client_id=bornemap-api" \
  -d "username=test-driver" \
  -d "password=test-password" \
  -d "grant_type=password" | jq -r '.access_token')
echo $TOKEN
```

### Test Authenticated Endpoint

```bash
curl -H "Authorization: Bearer $TOKEN" http://localhost:8082/api/v1/driver/me
# → {"success":true,"data":{"user_id":"USR-...","role":"registered_driver"},"meta":{...}}
```

### Test Auth Errors

```bash
# No token
curl http://localhost:8082/api/v1/driver/me
# → {"success":false,"error":{"code":"UNAUTHENTICATED","message":"Authentication required","details":null}}

# Expired token (modify exp to past)
# → {"success":false,"error":{"code":"TOKEN_EXPIRED","message":"Token has expired","details":null}}

# Wrong role (admin token on partner endpoint)
# → {"success":false,"error":{"code":"INSUFFICIENT_ROLE","message":"Insufficient permissions","details":null}}
```

## Architecture Overview

```
┌─────────────┐     ┌──────────────┐     ┌──────────────┐
│   Client    │────→│   Traefik    │────→│   Service    │
│  (app/web)  │     │  (reverse    │     │  (axum HTTP) │
│             │     │   proxy)     │     │              │
└──────┬──────┘     └──────────────┘     └──────┬───────┘
       │                                        │
       │  1. Login                              │  2. Auth Layer
       │                                        │     ├─ JWT validate
       ▼                                        │     ├─ Role check
┌──────────────┐                                │     └─ CurrentUser
│   Keycloak   │                                │
│  (identity)  │←──── JWKS fetch ──────────────│
└──────┬───────┘                                │
       │                                        │
       │  3. First-login provisioning           │
       ▼                                        ▼
┌────────────────┐                    ┌─────────────────┐
│  keycloak_db   │                    │   platform_db    │
│  (users,       │                    │  users schema    │
│   roles)       │                    │  user_account    │
└────────────────┘                    │  partner_membership│
                                      └─────────────────┘
```
