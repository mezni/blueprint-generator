# Data Model: Clickstream Tracking Pipeline

## AnalyticsEvent

Represents a single app-launch connection event. Produced by the mobile client, consumed by analytics-service.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_id` | String | Yes | Nanouuid: `evt-` + 8 lowercase hex digits (e.g. `evt-f3a219b1`) |
| `client_platform` | Enum | Yes | One of: `web`, `ios`, `android` |
| `app_version` | String | Yes | Semantic version string (e.g. `1.14.0`) |
| `connected_at` | DateTime | Yes | ISO 8601 UTC timestamp (e.g. `2026-05-28T21:30:00Z`) |

**Validation rules**:
- `event_id` MUST match regex `^evt-[a-f0-9]{8}$`
- `client_platform` MUST be one of `web`, `ios`, `android`
- `connected_at` MUST be a valid ISO 8601 UTC date-time

## ConnectionAggregate

A per-platform summary record stored in MongoDB. One document per platform.

| Field | Type | Description |
|-------|------|-------------|
| `_id` | ObjectId | Auto-generated MongoDB document ID |
| `platform` | String | Platform identifier (`web`, `ios`, `android`) |
| `total_connections_count` | Integer | Cumulative count of all connection events for this platform |
| `last_handshake_at` | String | ISO 8601 timestamp of the most recent event |
| `engine_version` | String | App version from the most recent event |

**Constraints**:
- `platform` is the shard key and upsert filter target
- `total_connections_count` is incremented via `$inc` (atomic, no read-before-write)
- Document is created on first event (upsert), updated on subsequent events

## Entity Relationships

```
App Launch → AnalyticsEvent → (via RabbitMQ) → analytics-service → $inc upsert → ConnectionAggregate
```

- One `AnalyticsEvent` creates/updates exactly one `ConnectionAggregate`
- Each `ConnectionAggregate` represents the sum of all events for one platform
- No foreign key relationships (MongoDB is schema-free for this workload)
