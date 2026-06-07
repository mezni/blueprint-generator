# Configuration

## Environment Files

Location: `infra/env/`

| File | Purpose |
|------|---------|
| `.env.example` | Common environment template |
| `driver-service.env.example` | Driver service variables |
| `admin-service.env.example` | Admin service variables |
| `clickstream-service.env.example` | Clickstream service variables |
| `gis-sync-worker.env.example` | GIS sync worker variables |

## Docker Compose

| File | Purpose |
|------|---------|
| `infra/compose/docker-compose.yml` | Base compose file |
| `infra/compose/docker-compose.override.yml` | Local development overrides |
| `infra/compose/docker-compose.prod.yml` | Production configuration |

## Secrets

- Secrets stored only on host environment files
- Not committed to repository
- Not passed through CI/CD pipelines
- Manual deployment copies env files to host
