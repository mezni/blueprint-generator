# Quickstart: Admin Service MVP

**Branch**: `005-admin-service-mvp` | **Date**: 2026-06-02

## Prerequisites

- Rust 1.87+ (edition 2024)
- PostgreSQL 16+ with PostGIS extension
- Running Keycloak instance (realm: `bornemap`, roles: `admin`, `partner`, `registered_driver`)
- Docker Compose (for full stack)

## Environment Variables

```bash
# Database
PLATFORM_DB_HOST=localhost
PLATFORM_DB_PORT=5432
PLATFORM_DB_NAME=platform_db
PLATFORM_DB_USER=bornemap
PLATFORM_DB_PASSWORD=changeme
PLATFORM_DB_SSL_MODE=disable
PLATFORM_DB_MAX_CONNECTIONS=20

# Service
ADMIN_SERVICE_PORT=8082

# Auth
AUTH_ISSUER=http://localhost:8080/realms/bornemap
AUTH_JWKS_URL=http://localhost:8080/realms/bornemap/protocol/openid-connect/certs
AUTH_AUDIENCE=bornemap-api

# Feature flags
PARTNER_DELETE_BLOCK_ACTIVE_STATIONS=true

# Observability
LOG_LEVEL=info
LOG_FORMAT=json
REQUEST_ID_HEADER=x-request-id
```

## Quick Start

### 1. Database Setup

```bash
# Run migrations (Sprint 4 schema + Sprint 5 idempotency table)
cd services/admin-service
sqlx database create
sqlx migrate run
```

### 2. Build & Run

```bash
# From repo root
cargo build -p admin-service
cargo run -p admin-service
```

The service starts on `http://0.0.0.0:8082`.

### 3. Verify

```bash
# Health check (no auth)
curl http://localhost:8082/health
# → {"status":"ok"}

# Admin endpoint (requires admin JWT)
curl -H "Authorization: Bearer <admin-jwt>" \
     http://localhost:8082/api/v1/admin/partners

# Partner endpoint (requires partner JWT)
curl -H "Authorization: Bearer <partner-jwt>" \
     http://localhost:8082/api/v1/partner/me
```

### 4. Docker Compose

```bash
# Full stack
docker compose up -d

# Just admin-service
docker compose up -d postgres keycloak traefik admin-service
```

## Testing

```bash
# Unit tests
cargo test -p admin-service

# Integration tests (requires running PostgreSQL)
cargo test -p admin-service --features integration

# All workspace tests
cargo test --workspace
```

## Key Endpoints Summary

### Partner API (partner role required)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/partner/me` | View own profile + membership |
| GET | `/api/v1/partner/stations` | List own stations |
| POST | `/api/v1/partner/stations` | Create station (Idempotency-Key required) |
| PATCH | `/api/v1/partner/stations/{id}` | Update own station (If-Match required) |
| DELETE | `/api/v1/partner/stations/{id}` | Soft-delete own station |
| GET | `/api/v1/partner/chargers` | List own chargers |
| POST | `/api/v1/partner/chargers` | Create charger at own station |
| PATCH | `/api/v1/partner/chargers/{id}` | Update own charger (If-Match required) |
| PATCH | `/api/v1/partner/stations/{id}/availability` | Update station availability |

### Admin API (admin role required)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/admin/users` | List all users |
| GET | `/api/v1/admin/partners` | List all partners |
| POST | `/api/v1/admin/partners` | Create partner |
| PATCH | `/api/v1/admin/partners/{id}` | Update partner (If-Match required) |
| DELETE | `/api/v1/admin/partners/{id}` | Soft-delete partner (blocked if active stations) |
| GET | `/api/v1/admin/stations` | List all stations |
| PATCH | `/api/v1/admin/stations/{id}` | Update any station (If-Match required) |
| DELETE | `/api/v1/admin/stations/{id}` | Soft-delete any station |
| GET | `/api/v1/admin/reviews` | List all reviews |
| PATCH | `/api/v1/admin/reviews/{id}/status` | Moderate review status |
