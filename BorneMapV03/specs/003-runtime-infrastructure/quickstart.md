# Quickstart: Platform Boot

```bash
# From repo root — boot the full platform
docker compose -f infra/compose/docker-compose.yml -f infra/compose/docker-compose.dev.yml up -d

# Verify all services are healthy
docker compose ps

# Test API gateway routing
curl http://localhost/api/v1/driver/health
curl http://localhost/api/v1/admin/health
curl http://localhost/api/v1/events/health

# Test version enforcement (should be rejected)
curl http://localhost/stations

# Access frontends
# Driver:   http://localhost
# Admin:    http://localhost/admin
# Partner:  http://localhost/partner

# Shut down
docker compose -f infra/compose/docker-compose.yml -f infra/compose/docker-compose.dev.yml down
```

## Production

```bash
docker compose -f infra/compose/docker-compose.yml -f infra/compose/docker-compose.prod.yml up -d
```

## Prerequisites

- Docker Engine 24+ with Compose v2 plugin
- Copy `.env.example` to `.env` and fill in values
