# Event Queue Contract

**Exchange Type**: Direct (default exchange, routing on queue name)

**Queue Name**: `analytics.connections`

**Producer**: api-service (via lapin Channel)

**Consumer**: analytics-service

## Delivery Semantics

| Property | Value |
|----------|-------|
| Queue durability | `true` (survives broker restart) |
| Auto-delete | `false` (queue persists until explicitly deleted) |
| Consumer acknowledgement | Manual (`basic.ack`) |
| Prefetch count | 1 (process one message at a time) |
| Redelivery | At-least-once (unacked messages are redelivered on consumer disconnect) |

## Message Payload

```json
{
  "event_id": "evt-f3a219b1",
  "client_platform": "web",
  "app_version": "1.14.0",
  "connected_at": "2026-05-28T21:30:00Z"
}
```

## Error Handling

- If `analytics-service` crashes mid-processing, the unacknowledged message is redelivered
- Invalid/poison messages are NOT explicitly requeued (consumer should ack and log)
- There is no dead-letter exchange configured for v1
