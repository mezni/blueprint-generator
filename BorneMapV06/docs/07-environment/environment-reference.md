# Environment Reference

## Common Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | Platform DB connection string | `postgres://...` |
| `ANALYTICS_DATABASE_URL` | Analytics DB connection string | `postgres://...` |
| `KEYCLOAK_URL` | Keycloak server URL | `http://keycloak:8080` |
| `RABBITMQ_URL` | RabbitMQ connection string | `amqp://guest:pass@rabbitmq:5672` |
| `JWT_ISSUER` | JWT issuer URL for validation | `http://keycloak:8080/realms/bornemap` |
| `JWT_AUDIENCE` | JWT audience | `bornemap-api` |

## Service-Specific Variables

### Driver Service
| Variable | Description |
|----------|-------------|
| `DRIVER_SERVICE_PORT` | HTTP port (default: 8001) |

### Admin Service
| Variable | Description |
|----------|-------------|
| `ADMIN_SERVICE_PORT` | HTTP port (default: 8002) |

### Clickstream Service
| Variable | Description |
|----------|-------------|
| `CLICKSTREAM_SERVICE_PORT` | HTTP port (default: 8003) |

### GIS Sync Worker
| Variable | Description |
|----------|-------------|
| `POLL_INTERVAL_SECONDS` | Outbox poll interval (default: 30) |
