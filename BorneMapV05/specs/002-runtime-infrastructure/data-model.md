# Data Model: Runtime Infrastructure

**Sprint 2 data architecture — database layout, network topology, and port assignments**

## Database Layout

Three PostgreSQL databases, all hosted on the same `postgres` container:

| Database | Purpose | Extensions | Owned By |
|----------|---------|------------|----------|
| `keycloak_db` | Keycloak realm, users, sessions, roles | None | Keycloak only |
| `platform_db` | Business data: stations, chargers, reviews, partners, users | PostGIS (`postgis`) | All backend services (read/write) |
| `analytics_db` | Event/clickstream data, analytics materialized views | None (partitioned by time) | clickstream-service (write), analytics-writer (write) |

Constraints:
- No cross-database joins (by design — prevents coupling between identity, business, and analytics domains)
- Keycloak connects via `KC_DB_URL=jdbc:postgresql://postgres.internal:5432/keycloak_db`
- Backend services connect via libpq (Rust `sqlx` or `diesel`) to `postgres.internal:5432/platform_db`

## Network Topology

```
┌─────────────────────────────────────────────────────────────┐
│  Docker bridge network: bornemap_internal (internal: true)  │
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │  Traefik  │  │ Keycloak │  │ Postgres │  │ RabbitMQ │    │
│  │  :80,443  │  │  :8080   │  │  :5432   │  │ :5672    │    │
│  │           │  │  :9000   │  │          │  │ :15672   │    │
│  └─────┬─────┘  └──────────┘  └──────────┘  └──────────┘    │
│        │                                                    │
│  ┌─────┴────────────────────────────────────────────────┐   │
│  │  /api/v1/drivers/*     → driver-service:8081         │   │
│  │  /api/v1/admin/*       → admin-service:8082          │   │
│  │  /api/v1/clickstream/* → clickstream-service:8083    │   │
│  │  /api/v1/gis/*         → gis-worker:8084             │   │
│  │  /api/v1/analytics/*   → analytics-writer:8085       │   │
│  └────────────────────────────────────────────────────────┘  │
│                                                              │
│  Host network (external)      ←── Traefik only              │
│  localhost:80 → Traefik:80                                   │
│  localhost:443 → Traefik:443                                 │
└─────────────────────────────────────────────────────────────┘
```

**Internal DNS names** (Docker Compose service names):
- `traefik.internal`
- `keycloak.internal`
- `postgres.internal`
- `rabbitmq.internal`
- `driver-service.internal`
- `admin-service.internal`
- `clickstream-service.internal`
- `gis-worker.internal`
- `analytics-writer.internal`

## Port Assignments

| Service | Internal Port | Host Port (base) | Host Port (override) |
|---------|--------------|-------------------|----------------------|
| traefik | 80, 443 | 80:80, 443:443 | — |
| keycloak | 8080, 9000 | — | 8080:8080 |
| postgres | 5432 | — | 5432:5432 |
| rabbitmq | 5672, 15672 | — | 5672:5672, 15672:15672 |
| driver-service | 8081 | — | — |
| admin-service | 8082 | — | — |
| clickstream-service | 8083 | — | — |
| gis-worker | 8084 | — | — |
| analytics-writer | 8085 | — | — |

## Environment Variables

### Shared (docker-compose.yml)

```yaml
# Internal network DNS resolution
services:
  postgres:
    environment:
      - POSTGRES_USER=${POSTGRES_USER}
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
      - POSTGRES_DB=platform_db

  keycloak:
    environment:
      - KC_DB_URL=jdbc:postgresql://postgres.internal:5432/keycloak_db
      - KC_DB_USERNAME=${KEYCLOAK_DB_USER}
      - KC_DB_PASSWORD=${KEYCLOAK_DB_PASSWORD}
      - KC_HOSTNAME=keycloak.internal
      - KC_HTTP_PORT=8080
      - KC_HEALTH_ENABLED=true
      - KC_METRICS_ENABLED=true

  rabbitmq:
    environment:
      - RABBITMQ_DEFAULT_USER=${RABBITMQ_USER}
      - RABBITMQ_DEFAULT_PASS=${RABBITMQ_PASSWORD}
      - RABBITMQ_DEFAULT_VHOST=/

  traefik:
    # Static config mounted; env vars for file provider path
    - TRAEFIK_CONFIG_FILE=/etc/traefik/config.yml

  driver-service:
    environment:
      - PORT=8081
      - DB_URL=postgres://${PLATFORM_DB_USER}:${PLATFORM_DB_PASSWORD}@postgres.internal:5432/platform_db
      - RABBITMQ_URL=amqp://${RABBITMQ_USER}:${RABBITMQ_PASSWORD}@rabbitmq.internal:5672/
      - KEYCLOAK_URL=http://keycloak.internal:8080
```

(Other backend services follow the same pattern with their respective ports and env.)

## Traefik Routing Rules

Defined in `infra/compose/traefik/config.yml` (static file provider):

| Path Prefix | Stripped To | Target |
|-------------|-------------|--------|
| `/api/v1/drivers/` | `/` | `http://driver-service:8081` |
| `/api/v1/admin/` | `/` | `http://admin-service:8082` |
| `/api/v1/clickstream/` | `/` | `http://clickstream-service:8083` |
| `/api/v1/gis/` | `/` | `http://gis-worker:8084` |
| `/api/v1/analytics/` | `/` | `http://analytics-writer:8085` |

Each route uses `PathPrefix` matcher + `StripPrefix` middleware so backend services receive clean paths (e.g., `GET /health` not `GET /api/v1/drivers/health`).
