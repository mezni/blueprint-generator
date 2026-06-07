# Clickstream Service API

## `POST /events`

Ingest a clickstream event.

**Request:**
```json
{
  "event_type": "page_view",
  "payload": {
    "page": "/stations/abc123",
    "referrer": "/search"
  },
  "session_id": "sess_abc123"
}
```

**Response:** `202 Accepted`

## Event Types

See [Event Taxonomy](../06-frontend/event-taxonomy.md) for complete list of
event types and their payload schemas.

## Rate Limiting

- Max 100 events per second per session
- Events exceeding the limit are dropped silently
