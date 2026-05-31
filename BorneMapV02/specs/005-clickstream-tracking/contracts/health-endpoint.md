# Consumer Health Endpoint Contract

**Endpoint**: `GET /health`

**Producer**: monitoring / container orchestrator

**Consumer**: analytics-service

## Response

### Healthy (200 OK)

```json
{
  "status": "healthy",
  "queue_depth": 3,
  "last_processed_at": "2026-05-28T21:30:05Z",
  "uptime_seconds": 86400
}
```

### Degraded (200 OK with warning signal)

```json
{
  "status": "degraded",
  "queue_depth": 500,
  "last_processed_at": "2026-05-28T20:00:00Z",
  "uptime_seconds": 86400,
  "warnings": ["queue depth growing", "last processed > 30s ago"]
}
```

## Fields

| Field | Type | Description |
|-------|------|-------------|
| `status` | String | `healthy` or `degraded` |
| `queue_depth` | Integer | Number of unprocessed messages in the queue |
| `last_processed_at` | String | ISO 8601 timestamp of the last successfully processed event |
| `uptime_seconds` | Integer | Seconds since the consumer started |
| `warnings` | String[] | Present only when `degraded`; describes specific concerns |
