# Clickstream Event Contract v1

## Purpose

Define the event envelope structure, supported event types, and delivery
guarantees for the BorneMap clickstream analytics pipeline.

## Version

1.0.0 (v1 event schema)

## Event Envelope

```json
{
  "event_id": "string (NanoID, EVT- prefix)",
  "event_type": "string (from approved type list)",
  "timestamp": "ISO-8601 datetime",
  "session_id": "string (client-generated session UUID)",
  "actor_id": "string|null (USR- prefix if authenticated, null if anonymous)",
  "platform": "string (one of: web, mobile, admin)",
  "payload": {}
}
```

## Event Types (v1)

| Type | Trigger | Payload Fields |
|------|---------|----------------|
| `station_viewed` | User opens station details | `station_id`, `source` (map/search/direct) |
| `station_searched` | User submits search | `query`, `result_count`, `filters` |
| `map_moved` | User pans/zooms map | `bounds` (N,S,E,W), `zoom_level` |
| `favorite_added` | User favorites a station | `station_id` |
| `favorite_removed` | User unfavorites a station | `station_id` |
| `review_created` | User posts a review | `station_id`, `rating` |
| `review_deleted` | User deletes a review | `station_id`, `review_id` |
| `auth_login_success` | User logs in successfully | `method` (keycloak/google/facebook) |
| `auth_login_failed` | User login fails | `method`, `reason` |

## Delivery Rules

- **Schema**: Versioned (v1). Breaking changes require a new version (v2).
- **Secrets**: No secrets, PII, or credentials in payloads.
- **Delivery**: At-least-once. Consumers must handle duplicates via
  `event_id` deduplication.
- **Payload**: JSONB only, accept any valid JSON object.

## Enforcement

Ingestion API validates envelope structure and event_type against the approved
list. Invalid events are rejected with 422 and logged for monitoring.
