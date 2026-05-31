# Aggregates Query API Contract

**Endpoint**: `GET /api/v1/analytics/connections`

**Producer**: admin / monitoring tools

**Consumer**: api-service (Actix-web gateway)

## Request

### Headers

| Header | Value | Required |
|--------|-------|----------|
| `Accept` | `application/json` | No (default) |

No query parameters.

## Response

### Success (200 OK)

```json
{
  "aggregates": [
    {
      "platform": "web",
      "total_connections_count": 42,
      "last_handshake_at": "2026-05-28T21:30:00Z",
      "engine_version": "1.14.0"
    },
    {
      "platform": "ios",
      "total_connections_count": 17,
      "last_handshake_at": "2026-05-28T20:15:00Z",
      "engine_version": "1.14.0"
    }
  ]
}
```

### Empty (200 OK)

```json
{
  "aggregates": []
}
```

## Error Handling

- Returns empty array if no aggregates exist
- On store connection failure, returns 500 with error message
