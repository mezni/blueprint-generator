# Readiness Endpoint Contract

**Version**: 1.0 | **Date**: 2026-06-01 | **Services**: All with dependencies

## GET /ready

Dependency-aware readiness probe. Returns success only when all required dependencies are reachable.

### Success Response (HTTP 200)

```json
{
  "success": true,
  "data": {
    "status": "ready",
    "dependencies": [
      {
        "name": "postgres",
        "status": "connected"
      }
    ]
  },
  "meta": {}
}
```

### Dependency Unavailable (HTTP 503)

```json
{
  "success": false,
  "error": {
    "code": "SERVICE_UNAVAILABLE",
    "message": "1 of 1 dependencies unavailable"
  },
  "data": {
    "status": "not_ready",
    "dependencies": [
      {
        "name": "postgres",
        "status": "unreachable",
        "error": "connection refused"
      }
    ]
  }
}
```

### Dependencies Per Service

| Service | Dependencies |
|---------|-------------|
| driver-service | postgres |
| admin-service | postgres |
| clickstream-service | rabbitmq |
| gis-worker | postgres, rabbitmq |
| analytics-writer | postgres, rabbitmq |

### Fields

| Field | Type | Description |
|-------|------|-------------|
| status | string | "ready" or "not_ready" |
| dependencies[].name | string | Dependency identifier |
| dependencies[].status | string | "connected" or "unreachable" |
| dependencies[].error | string | Error detail (only on unreachable) |

### Behavior

- Dependencies are checked with a 10-second timeout
- Results are cached for 5 seconds to avoid hammering dependencies on frequent polling
- First check runs synchronously on startup (blocking ready state until complete)
