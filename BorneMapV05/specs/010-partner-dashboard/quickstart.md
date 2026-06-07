# Quickstart — Sprint 10 Partner Dashboard

## Prerequisites

- Docker Compose stack running (services: postgres, rabbitmq, keycloak, traefik, admin-service, driver-service, clickstream-service)
- Node.js 20+ and npm 10+
- Partner user seeded in Keycloak (kc-partner-001 / partner1@example.tn)

## Start the Frontend

```bash
npm run dev:partner
```

Opens at `http://localhost:5173`. Proxies `/api` and `/auth` to `http://localhost:80` (Traefik).

## Verify Backend

```bash
# Unauthenticated — should return 401
curl -s http://localhost/api/v1/partner/me

# With token — should return partner profile
TOKEN=$(curl -s -X POST http://localhost/auth/realms/bornemap/protocol/openid-connect/token \
  -d "client_id=bornemap-api" \
  -d "username=partner1@example.tn" \
  -d "password=changeme" \
  -d "grant_type=password" | jq -r '.access_token')
curl -s -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/partner/me
```

## Available Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/partner/stations` | List stations (paginated) |
| POST | `/api/v1/partner/stations` | Create station (Idempotency-Key required) |
| PATCH | `/api/v1/partner/stations/{id}` | Update station (If-Match required) |
| DELETE | `/api/v1/partner/stations/{id}` | Soft-delete station |
| PATCH | `/api/v1/partner/stations/{id}/availability` | Update availability |
| GET | `/api/v1/partner/chargers` | List chargers (optional station_id filter) |
| POST | `/api/v1/partner/chargers` | Create charger |
| PATCH | `/api/v1/partner/chargers/{id}` | Update charger (If-Match required) |
| GET | `/api/v1/partner/me` | Get partner profile |

## Build

```bash
npm run build -w apps/partner-dashboard
```

## Architecture

```
Browser → Vite Dev Server (:5173)
  ├── /api/v1/partner/* → Traefik (:80) → admin-service (:8082)
  └── /auth/* → Traefik (:80) → keycloak (:8080)
```
