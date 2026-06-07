# Quickstart: Runtime Infrastructure

**Goal**: Bring the full Bornemap platform online locally in under 5 minutes

## Prerequisites

- Docker Engine 24+ and Docker Compose v2 (`docker compose` — not `docker-compose`)
- Git clone of the Bornemap monorepo on branch `002-runtime-infrastructure`

## Bring Up the Stack

```bash
# 1. Navigate to compose directory
cd infra/compose

# 2. Copy env files (one-time setup)
cp ../env/postgres.env.example .env
# Edit .env with real secrets (the defaults work for local dev)

# 3. Start all services
docker compose up -d

# 4. Watch startup (press Ctrl+C to detach)
docker compose logs -f
```

## Verify Everything is Running

```bash
# List all services and their health status
docker compose ps

# Expected: all 9 services showing "healthy" or "Up"

# Check Traefik routing → driver-service health
curl -f http://localhost/api/v1/drivers/health
# → {"status":"ok"}

# Check all backend services
for svc in drivers admin clickstream gis analytics; do
  echo "=== $svc ==="
  curl -f "http://localhost/api/v1/$svc/health"
done

# Verify infrastructure reachability (from inside a container)
docker compose exec driver-service sh -c \
  'cat /dev/null > /dev/tcp/postgres.internal/5432 && echo "Postgres OK"'

docker compose exec driver-service sh -c \
  'cat /dev/null > /dev/tcp/rabbitmq.internal/5672 && echo "RabbitMQ OK"'

docker compose exec driver-service sh -c \
  'cat /dev/null > /dev/tcp/keycloak.internal/8080 && echo "Keycloak OK"'
```

## Startup Order

The compose file enforces this dependency chain via `depends_on` with health checks:

```
PostgreSQL (healthy) → RabbitMQ (healthy) → Keycloak (healthy)
    ↓
Traefik (healthy) → Backend services (in parallel)
```

No backend service starts before all three infrastructure dependencies are healthy.

## Docker Override for Local Development

The base compose file keeps all infrastructure ports internal. For local development tooling:

```bash
# Ports exposed on localhost (via docker-compose.override.yml):
#   - PostgreSQL: localhost:5432
#   - RabbitMQ: localhost:5672, localhost:15672 (management UI)
#   - Keycloak: localhost:8080 (auth server)
```

To disable port exposure for a clean test:
```bash
docker compose -f docker-compose.yml up -d
```

## Stop the Stack

```bash
# Stop all services (preserves volumes)
docker compose down

# Stop and delete all volumes (fresh start)
docker compose down -v
```

## Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|-------------|-----|
| PostgreSQL restarts in loop | Init script error | `docker compose logs postgres` and fix init-dbs.sh |
| Keycloak fails to start | DB not ready or realm JSON invalid | Check postgres health first; validate JSON |
| Backend service unhealthy | Port mismatch or missing env | Verify PORT env and check logs: `docker compose logs driver-service` |
| Traefik 404 on routes | Config file not mounted correctly | `docker compose exec traefik cat /etc/traefik/config.yml` |
| `port already allocated` | Override port conflict | Stop local Postgres/RabbitMQ services on host |
