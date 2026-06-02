# Data Model: Runtime Foundation

**Phase**: 1 | **Date**: 2026-06-01 | **Plan**: [plan.md](plan.md)

> This sprint defines runtime infrastructure — no business data schemas. The models below describe the Docker Compose topology, service configuration schema, health probe contracts, and environment profiles.

## 1. Container Topology

### Docker Compose Service Definitions

| Service | Image | Internal DNS | Port(s) | Dependencies | Health Check |
|---------|-------|-------------|---------|--------------|-------------|
| postgres | postgres:16-alpine | postgres.internal | 5432 | — | pg_isready |
| rabbitmq | rabbitmq:4-management | rabbitmq.internal | 5672, 15672 | postgres | rabbitmq-diagnostics ping |
| keycloak | quay.io/keycloak/keycloak:26+ | keycloak.internal | 8080 | postgres | curl realm endpoint; DB: keycloak_db |
| traefik | traefik:v3 | — | 80, 8080 | — | built-in /api/http/routers |
| driver-service | local build | driver.internal | 8081 | postgres, keycloak | /health |
| admin-service | local build | admin.internal | 8082 | postgres, keycloak | /health |
| clickstream-service | local build | clickstream.internal | 8083 | rabbitmq | /health |
| gis-worker | local build | gis.internal | 8084 | postgres, rabbitmq | /health |
| analytics-writer | local build | analytics.internal | 8085 | postgres, rabbitmq | /health |

### Networks

- **bornemap-net** (internal overlay): All 9 containers
- No external network exposure except Traefik ports 80/8080

### Volumes

| Volume | Mount | Service |
|--------|-------|---------|
| pg-data | /var/lib/postgresql/data | postgres |
| rmq-data | /var/lib/rabbitmq | rabbitmq |
| kc-data | /opt/keycloak/data | keycloak (H2 fallback; production uses keycloak_db) |

## 2. Service Configuration Schema

### Common Variables (all services)

| Variable | Required | Type | Default | Description |
|----------|----------|------|---------|-------------|
| APP_ENV | yes | enum | — | local / docker / staging |
| APP_NAME | yes | string | — | service identifier (e.g., "driver-service") |
| SERVICE_PORT | yes | uint16 | — | HTTP listen port |
| LOG_LEVEL | no | enum | info | trace / debug / info / warn / error |
| LOG_FORMAT | no | enum | json | json / text |

### Database Connectivity (driver-service, admin-service, analytics-writer)

| Variable | Required | Type | Description |
|----------|----------|------|-------------|
| DB_HOST | yes | hostname | PostgreSQL host |
| DB_PORT | yes | uint16 | PostgreSQL port |
| DB_NAME | yes | string | Database name |
| DB_USER | yes | string | Database user |
| DB_PASSWORD | yes | string | Database password |
| DB_MAX_CONNECTIONS | no | uint16 | Connection pool size (default 10) |

### RabbitMQ Connectivity (clickstream-service, gis-worker, analytics-writer)

| Variable | Required | Type | Description |
|----------|----------|------|-------------|
| RABBITMQ_HOST | yes | hostname | RabbitMQ host |
| RABBITMQ_PORT | yes | uint16 | AMQP port |
| RABBITMQ_USER | yes | string | AMQP user |
| RABBITMQ_PASSWORD | yes | string | AMQP password |
| RABBITMQ_VHOST | yes | string | Virtual host |

### Auth Configuration (driver-service, admin-service)

| Variable | Required | Type | Description |
|----------|----------|------|-------------|
| AUTH_ISSUER | yes | url | Keycloak realm URL |
| AUTH_JWKS_URL | yes | url | JWKS endpoint |
| AUTH_AUDIENCE | yes | string | Expected JWT audience |

## 3. Environment Profiles

### Profile: local

- Default for `APP_ENV=local`
- Host port mapping enabled for all management UIs
- Relaxed validation: missing optional vars use defaults
- Debug logging enabled
- Keycloak in dev mode (no TLS)

### Profile: docker

- Default for `APP_ENV=docker`
- Internal networking only (except Traefik)
- Strict validation: all required vars must be present
- Info logging
- Keycloak uses PostgreSQL backend

### Profile: staging

- Placeholder for future use
- Same as docker profile with future additions

## 4. Runtime Boot Stages

Each service progresses through these stages, emitting structured JSON logs:

1. **config_load** — environment parsed and validated
2. **dependency_check** — connection attempts to DB / RabbitMQ
3. **route_registration** — HTTP routes registered on axum router
4. **ready** — service responds to /ready with HTTP 200

## 5. Queue Topology (RabbitMQ)

| Queue | Type | Durable | Binding | Consumer |
|-------|------|---------|---------|----------|
| clickstream.raw | classic | yes | clickstream.topic | clickstream-service |
| gis.sync | classic | yes | gis.topic | gis-worker |
| analytics.ingest | classic | yes | analytics.topic | analytics-writer |

