# API Response Envelope Contract

**Version**: 1.0.0

**Applies to**: All backend services (driver-service, admin-service,
clickstream-service, gis-worker, analytics-writer)

## Success Envelope

```json
{
  "success": true,
  "data": {},
  "meta": {}
}
```

- `success` MUST always be `true`
- `data` contains the response payload
- `meta` contains pagination, timing, or other metadata
- Both `data` and `meta` MAY be empty objects `{}`

## Error Envelope

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable description"
  }
}
```

- `success` MUST always be `false`
- `error.code` is a machine-readable string (SCREAMING_SNAKE_CASE)
- `error.message` is a human-readable description

## Health Endpoint Contract

**Path**: `GET /health`

**Response**:

```json
{
  "success": true,
  "data": {
    "status": "ok",
    "service": "<service-name>",
    "version": "<semver>"
  }
}
```

- `status` is `"ok"` when the service is healthy
- `service` identifies the responding service (e.g., `driver-service`)
- `version` follows semantic versioning (e.g., `0.1.0`)

## Standard Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INTERNAL_ERROR` | 500 | Unexpected server error |
| `NOT_FOUND` | 404 | Requested resource not found |
| `BAD_REQUEST` | 400 | Malformed request |
| `SERVICE_UNAVAILABLE` | 503 | Service dependency unavailable |

## Content Type

All responses MUST use `Content-Type: application/json`.

## Versioning

URL-based versioning only (`/api/v1/`). Headers MUST NOT be used for
API version selection.
