# Environment Configuration Contract

**Source**: `.env` file at repository root
**Loading**: Docker Compose `env_file` directive
**Scope**: All 11 runtime services

## Required Variables

| Variable | Services | Example (dev) | Required |
|----------|----------|---------------|----------|
| `POSTGRES_USER` | postgres, all backend | `borne` | Yes |
| `POSTGRES_PASSWORD` | postgres, all backend | `devpassword` | Yes |
| `POSTGRES_DB` | postgres, all backend | `borne_map` | Yes |
| `DATABASE_URL` | admin-service, driver-service, gis-sync-worker | `postgres://borne:devpassword@postgres:5432/borne_map` | Yes |
| `RABBITMQ_URL` | clickstream-service, gis-sync-worker | `amqp://guest:guest@rabbitmq:5672/%2F` | Yes |
| `KEYCLOAK_URL` | keycloak, admin-service, driver-service | `http://keycloak:8080` | Yes |
| `KEYCLOAK_REALM` | keycloak | `borne-map` | Yes |
| `KEYCLOAK_CLIENT_ID` | admin-service, driver-service | `backend-service` | Yes |
| `TRAEFIK_DOMAIN` | traefik | `localhost` | Yes |
| `RUST_LOG` | backend services | `info` | Yes |
| `ENVIRONMENT` | all services | `local` | Yes |

## Forbidden Patterns

- No secrets in `docker-compose.yml` (use `${VARIABLE}` substitution)
- No secrets in Dockerfiles (use build args only for non-sensitive values)
- No hardcoded credentials in source code
- No `.env` committed to version control (add to `.gitignore`)
