# Research: Runtime Infrastructure (Docker Compose v1)

**Phase 0 research for Sprint 2 implementation plan**

## 1. Traefik Routing: File Provider vs Docker Provider

**Decision**: File provider with static YAML configuration file

**Rationale**:
- The spec requires "static routing rules" — file provider is explicit, testable, and version-controllable
- Docker provider labels work for dynamic environments but mix routing config into service declarations
- File provider allows Traefik config to live in its own directory (`infra/compose/traefik/config.yml`) separate from the compose file
- Supports `PathPrefix` matchers with `StripPrefixMiddleware` for the `/api/v1/{service}/*` routing scheme
- Single `localhost` entrypoint on port 80 handles all routing

**Alternatives considered**:
- Docker provider (labels on each service in compose): Implicit, harder to audit at a glance
- Combined approach: Unnecessary complexity for 5 routes

**Configuration pattern**:
```yaml
# localhost entrypoint
entryPoints:
  web:
    address: ":80"

# Router: match PathPrefix, strip prefix, forward to internal service
routers:
  driver:
    rule: "PathPrefix(`/api/v1/drivers`)"
    middlewares:
      - strip-drivers
    service: driver-service

middlewares:
  strip-drivers:
    stripPrefix:
      prefixes:
        - "/api/v1/drivers"

services:
  driver-service:
    loadBalancer:
      servers:
        - url: "http://driver-service:8081"
```

## 2. Docker Health Checks

**Decision**: Use shell-based probes compatible with each base image

**Rationale**:
- Rust images (distroless or slim) lack `curl` — must use `/dev/tcp` shell probes or scripts
- PostgreSQL image includes `pg_isready` — use the image's built-in health check
- RabbitMQ image includes `rabbitmq-diagnostics` — use `rabbitmq-diagnostics check_port_connectivity`
- Keycloak image (UBI-based) includes `curl` — use HTTP health check on its port
- Health check interval: 15s, retries: 5, timeout: 10s, start period: 30s (PostgreSQL may need 60s)

**Health check patterns**:
| Service | Check | Reason |
|---------|-------|--------|
| postgres | `pg_isready -U $POSTGRES_USER` | Built into postgres image |
| rabbitmq | `rabbitmq-diagnostics check_port_connectivity` | Tests actual connectivity |
| keycloak | `curl -sf http://localhost:9000/health/ready` | Keycloak provides its own health endpoint on admin port |
| driver-service | `/bin/sh -c 'cat /dev/null > /dev/tcp/localhost/8081 2>/dev/null || exit 1'` | No curl in Rust image |
| admin-service | Same at port 8082 | — |
| clickstream-service | Same at port 8083 | — |
| gis-worker | Same at port 8084 | — |
| analytics-writer | Same at port 8085 | — |
| traefik | `/bin/sh -c 'cat /dev/null > /dev/tcp/localhost/80 2>/dev/null || exit 1'` | Built-in health check also available |

Note: `cat /dev/null > /dev/tcp/...` checks TCP connectivity. For services that may not accept raw TCP (HTTP servers), a more robust check would be to pipe an HTTP GET via `/dev/tcp`. For Sprint 2, simple TCP connectivity to the port is sufficient since the process is the service.

## 3. Keycloak Realm Import

**Decision**: Volume-mount realm JSON to `/opt/keycloak/data/import/` and pass `--import-realm`

**Rationale**:
- Keycloak 26 (Quarkus) supports auto-import from `/opt/keycloak/data/import/` directory
- Set `KC_IMPORT_REALM=true` or use `--import-realm` argument — both work; env var is cleaner in Compose
- Realm JSON defines `bornemap` realm with 3 roles and 3 users
- Keycloak must use `postgres.internal:5432/keycloak_db` as its database (not embedded H2)
- Use `KC_DB_URL`, `KC_DB_USERNAME`, `KC_DB_PASSWORD` env vars for DB configuration

**Alternatives considered**:
- Keycloak Admin API import: Requires auth token, more complex in startup
- Manual import via admin console: Not reproducible

## 4. PostgreSQL Database Initialization

**Decision**: Shell script mounted to `/docker-entrypoint-initdb.d/`

**Rationale**:
- Standard PostgreSQL pattern: scripts in `docker-entrypoint-initdb.d/` run once on first DB init
- Script creates 3 databases: `keycloak_db`, `platform_db` (with PostGIS), `analytics_db`
- PostGIS extension enabled on `platform_db` only (`CREATE EXTENSION IF NOT EXISTS postgis`)
- Script idempotent: uses `CREATE DATABASE IF NOT EXISTS` pattern via shell check

**Init script pattern**:
```bash
#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE DATABASE keycloak_db;
    CREATE DATABASE platform_db;
    CREATE DATABASE analytics_db;
EOSQL

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname platform_db <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS postgis;
EOSQL
```

## 5. Docker Compose Override Pattern

**Decision**: `docker-compose.override.yml` in the same directory as the base compose file

**Rationale**:
- Docker Compose v2 automatically loads `docker-compose.override.yml` when present alongside `docker-compose.yml`
- Override exposes infrastructure ports (5432, 5672, 15672, 8080) for local dev tooling
- Base compose file remains production-safe with no host port exposures (except Traefik)
- Override is in `.gitignore`? No — `docker-compose.override.yml` should be committed since it's a shared dev tool. But if the project has sensitive defaults, it could be gitignored. For Bornemap, committing it is fine — it only exposes ports.

**Alternatives considered**:
- Single compose file with all ports: Not safe for production-like deployments
- Separate `docker-compose.dev.yml`: Requires explicit `-f` flag, less ergonomic
- Multiple override files: Unnecessary complexity

## 6. Rust Health Endpoint Implementation

**Decision**: Raw TCP listener with minimal HTTP response (no framework)

**Rationale**:
- Sprint 2 has no business logic — only needs to return HTTP 200
- `std::net::TcpListener` keeps dependencies minimal and build fast
- Framework (axum/actix) can be added when actual API endpoints are needed
- Response format: HTTP/1.1 200 OK with `Content-Type: application/json` and body `{"status":"ok"}`
- Port read from `PORT` env var with a sensible default (8081-8085 per service)

**Alternatives considered**:
- axum framework: Adds compile time and dependency weight for a single endpoint
- warp: Same concern
- Raw TCP: Minimal, fast to build, trivially replaceable in later sprints

## Key Decisions Summary

| Decision | Choice | Why |
|----------|--------|-----|
| Traefik routing | File provider (YAML) | Explicit, testable, separate from compose |
| Backend health checks | `/dev/tcp` shell probe | No curl in Rust base images |
| PostgreSQL init | `docker-entrypoint-initdb.d/` script | Standard, idempotent, built-in pattern |
| Keycloak import | Volume mount + `--import-realm` | Reproducible, no manual steps |
| Override file | Committed `docker-compose.override.yml` | Shared dev ergonomic, production-safe base |
| Rust health endpoint | Raw `TcpListener` | Minimal deps, fast build, easy to replace |
