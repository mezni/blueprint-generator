# API Gateway Routing Contract

**Config file**: `infra/traefik/dynamic.yml`
**Enforced by**: Traefik v3

## Routing Table

| Priority | Router | Path | Target Service | Target Port |
|----------|--------|------|----------------|-------------|
| 10 | `driver-api` | `/api/v1/driver/*` | driver-service | 8080 |
| 10 | `admin-api` | `/api/v1/admin/*` | admin-service | 8080 |
| 10 | `events-api` | `/api/v1/events/*` | clickstream-service | 8080 |
| 5 | `auth` | `/auth/*` | keycloak | 8080 |
| 1 | `driver-web` | `/` | driver-web | 80 |
| 1 | `admin-dashboard` | `/admin` | admin-dashboard | 80 |
| 1 | `partner-dashboard` | `/partner` | partner-dashboard | 80 |

## Rejection Rules

- Backend routes not matching `/api/v1/*` → HTTP 404 with `{"error": "unversioned_route"}`
- Frontend routes not matching `/`, `/admin`, `/partner` → HTTP 404
- Health/ready requests are passed through without version check

## Middleware

- `strip-prefix`: Strips the `/api/v1/<service>` prefix before forwarding to backend
- `rate-limit`: Applied to driver-api (100 req/min per IP, burst 20)
