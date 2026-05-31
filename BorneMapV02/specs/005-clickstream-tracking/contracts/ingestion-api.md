# Ingestion API Contract

**Endpoint**: `POST /api/v1/analytics/connect`

**Producer**: mobile-driver (React Native app)

**Consumer**: api-service (Actix-web gateway)

## Request

### Headers

| Header | Value | Required |
|--------|-------|----------|
| `Content-Type` | `application/json` | Yes |

### Body

```json
{
  "event_id": "evt-f3a219b1",
  "client_platform": "web",
  "app_version": "1.14.0",
  "connected_at": "2026-05-28T21:30:00Z"
}
```

### Validation

- `event_id`: Required, regex `^evt-[a-f0-9]{8}$`
- `client_platform`: Required, one of `web`, `ios`, `android`
- `app_version`: Required, non-empty string
- `connected_at`: Required, valid ISO 8601 UTC timestamp

## Response

### Success (202 Accepted)

```json
{
  "accepted": true
}
```

### Validation Error (400 Bad Request)

```json
{
  "error": "invalid event_id: must match pattern evt-[a-f0-9]{8}"
}
```

### Server Error (500 Internal Server Error)

```json
{
  "error": "failed to enqueue event"
}
```

## Error Handling

- The client MUST NOT block on this request; network failures are silently logged
- The server returns 202 before the event is persisted to the queue
- If the broker is unreachable, the server returns 500
