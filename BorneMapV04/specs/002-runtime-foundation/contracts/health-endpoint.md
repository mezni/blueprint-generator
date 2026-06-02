# Health Endpoint Contract

**Version**: 1.0 | **Date**: 2026-06-01 | **Services**: All

## GET /health

Static liveness probe — responds immediately with no dependency checks.

### Success Response (HTTP 200)

```json
{
  "success": true,
  "data": {
    "status": "ok",
    "service": "driver-service",
    "version": "0.1.0"
  },
  "meta": {}
}
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| status | string | Always "ok" |
| service | string | Service name from APP_NAME |
| version | string | Cargo package version |

### Error Response

This endpoint never returns an error — unavailable = container is down (Docker handles restart).
