# Quickstart: Runtime Foundation

**Date**: 2026-06-01

## Prerequisites

- Docker Engine 24+
- Docker Compose v2
- Git

## Build & Boot

```bash
# 1. Build all Rust services
cargo build --workspace

# 2. Start the full platform
docker compose -f infra/compose/docker-compose.yml up -d

# 3. Check container health (wait 60s for boot)
docker compose -f infra/compose/docker-compose.yml ps

# 4. Run smoke tests
bash scripts/smoke-test.sh
```

## Verify

### Health Endpoints

```bash
curl -s http://localhost:80/api/driver/health | jq .
curl -s http://localhost:80/api/admin/health | jq .
curl -s http://localhost:80/api/clickstream/health | jq .
```

### Infrastructure UIs (local profile only)

| UI | URL |
|----|-----|
| Traefik Dashboard | http://localhost:8080/dashboard/ |
| RabbitMQ Management | http://localhost:15672 |
| Keycloak Admin | http://localhost:8090 |

## Environment Profiles

```bash
# Default (docker profile)
docker compose up

# Local development profile
APP_ENV=local docker compose --profile local up
```

## Stopping

```bash
# Stop all containers (preserves data volumes)
docker compose -f infra/compose/docker-compose.yml down

# Stop and wipe all data
docker compose -f infra/compose/docker-compose.yml down -v
```

## Architecture

```
┌─────────┐     ┌──────────┐     ┌──────────┐
│ Traefik │────▶│ Services │────▶│ Postgres │
│  :80    │     │  Rust    │     │  :5432   │
└─────────┘     └────┬─────┘     └──────────┘
                     │                │
                     │         ┌──────┴──────┐
                     │         │  RabbitMQ   │
                     └────────▶│  :5672      │
                               └─────────────┘
```
