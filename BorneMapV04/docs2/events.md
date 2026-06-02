Event Taxonomy Specification (v1.0) — Bornemap
1. Purpose

This document defines the canonical event system for all behavioral analytics across the Bornemap platform.

It ensures:

consistent event naming across all clients
strict validation in Clickstream Service
reliable ingestion into analytics pipeline
no UI-driven or implementation-specific events
stable evolution over time without drift
2. Scope

This taxonomy applies ONLY to:

Driver Web App
Driver Mobile App
Partner Dashboard
Admin Dashboard

It excludes:

system logs
infrastructure logs
domain events (DB triggers, GIS sync events)
backend internal telemetry
3. Core Principles
3.1 Event = user behavior only

Events MUST represent:

user action
system-visible state transition
interaction outcome
3.2 No UI leakage

Forbidden:

button.clicked
modal.opened
page.button_pressed

Allowed:

station.favorited
station.opened
search.performed
3.3 Event immutability

Once emitted:

event is immutable
never updated
never deleted
3.4 At-least-once delivery

All events are:

duplicated-safe
idempotent via event_id
4. Event Envelope (Canonical)

Every event MUST follow:

{
  "event_id": "CLK-01ABCDEF",
  "event_version": 1,
  "schema_namespace": "clickstream",
  "event_name": "station.opened",
  "occurred_at": "2026-06-01T12:00:00Z",
  "ingested_at": "2026-06-01T12:00:01Z",

  "channel": "driver_web",

  "session_id": "sess-123",
  "correlation_id": "flow-xyz",

  "anonymous_id": "anon-123",
  "user_id": "usr-123",

  "actor_role": "registered_driver",

  "path": "/stations/STN-123",

  "payload": {},
  "metadata": {}
}
5. Field Definitions
5.1 Required Fields
Field	Description
event_id	unique id (dedup key)
event_name	canonical event type
occurred_at	client timestamp
ingested_at	server timestamp
channel	source app
session_id	session tracking
5.2 Identity Fields
Field	Rule
user_id	nullable (anonymous allowed)
anonymous_id	required if no user
actor_role	derived from JWT or anonymous
5.3 Channels
driver_web
driver_mobile
partner_dashboard
admin_dashboard
6. Event Naming Convention
Format
<domain>.<action>
Rules
lowercase only
dot-separated
no verbs in UI style
must represent business meaning
7. Domain Taxonomy
7.1 Navigation Events
page.viewed
{
  "path": "/stations/STN-123",
  "referrer": "/search"
}
map.loaded
{
  "initial_center": "user_location"
}
map.viewport_changed
{
  "bbox": "10.1,36.7,10.4,36.9",
  "zoom": 13
}
7.2 Discovery Events (CORE)
search.performed
{
  "query": "tunis",
  "result_count": 12
}
stations.nearby.viewed
{
  "radius_km": 10,
  "result_count": 20,
  "center_source": "user_location"
}
filter.applied
{
  "filters": {
    "connector_type": ["CCS"],
    "availability": "available"
  }
}
7.3 Station Interaction Events
station.marker_clicked
{
  "station_id": "STN-123"
}
station.opened
{
  "station_id": "STN-123",
  "source": "marker"
}
charger.opened
{
  "station_id": "STN-123",
  "charger_id": "CHG-456"
}
7.4 Favorites Events
favorite_station.added
{
  "station_id": "STN-123"
}
favorite_station.removed
{
  "station_id": "STN-123"
}
7.5 Review Events
review.submitted
{
  "station_id": "STN-123",
  "rating": 5
}
review.updated
{
  "review_id": "REV-123",
  "rating": 4
}
7.6 Authentication Events
auth.started
{
  "method": "google"
}
auth.succeeded
{
  "method": "google",
  "first_login": true
}
auth.failed
{
  "method": "google",
  "reason": "provider_cancelled"
}
7.7 Partner Events
partner_station.created
{
  "station_id": "STN-123",
  "is_live": false
}
partner_station.updated
{
  "station_id": "STN-123",
  "changed_fields": ["location"]
}
partner_availability.updated
{
  "station_id": "STN-123",
  "availability_status": "limited"
}
7.8 Admin Events
admin_station.created
{
  "station_id": "STN-123",
  "partner_id": "PRT-456"
}
admin_review.moderated
{
  "review_id": "REV-123",
  "status": "hidden"
}
7.9 Failure Events (CRITICAL)
search.failed
{
  "query": "tunis",
  "reason": "timeout"
}
station.load_failed
{
  "station_id": "STN-123",
  "reason": "not_found"
}
8. Validation Rules (STRICT)
8.1 Clickstream Service MUST enforce:
event_name must exist in taxonomy
event_id must be unique
payload must be JSON valid
session_id required
actor_role must match JWT
8.2 Payload safety

Forbidden:

passwords
tokens
emails
phone numbers
raw authentication data
9. Identity Resolution Rules
Context	user_id
logged-in user	required
anonymous user	null
partner/admin	required
10. Event Versioning
Rule
breaking change → increment event_version
additive fields allowed without version bump
11. Delivery Semantics
at-least-once delivery
dedup via event_id
ordering NOT guaranteed
12. Storage Mapping (analytics_db)

Indexed fields:

event_id
event_name
occurred_at
session_id
user_id
channel

Payload:

JSONB flexible schema
13. Partitioning Strategy
monthly partitions:
raw_event_2026_06
raw_event_2026_07
14. Performance Requirements
ingestion: > 100 events/sec baseline
write latency: < 50ms avg
dedup lookup: indexed by event_id
15. Anti-Patterns (FORBIDDEN)
UI click tracking events
frontend-specific event names
duplicate semantic events
embedding sensitive data in payload
mixing system logs with analytics
16. Summary

This taxonomy guarantees:

clean behavioral analytics layer
strict event validation pipeline
scalable ingestion via RabbitMQ
consistent cross-app tracking
safe analytics without PII leakage
long-term evolution without drift
