# Feature Specification: Clickstream Tracking Pipeline

**Feature Branch**: `005-clickstream-tracking`

**Created**: 2026-05-28

**Status**: Reviewed

**Input**: User description: "Component Specification: Clickstream Tracking Pipeline (v1.14.0-ADDENDUM) ..."

## User Scenarios & Testing

### User Story 1 - Track Application Launches Unobtrusively (Priority: P1)

When a driver opens the mobile app, a connection event is silently recorded in the background. The user notices no delay or interruption — the map loads at full speed regardless of the tracking pipeline's availability.

**Why this priority**: This is the core value — knowing how many users are actively connecting, on which platforms, and with which app version. Without this, the pipeline produces no data.

**Independent Test**: Launch the app on each target platform (web, iOS, Android). Verify the connection event is received by the backend without any visible loading indicator or user-facing delay. Simulate a network failure and confirm the app still loads normally.

**Acceptance Scenarios**:

1. **Given** the mobile app is installed and launched, **When** the home screen finishes loading, **Then** a connection event is dispatched to the backend without blocking the UI.
2. **Given** the tracking backend is unreachable (network down), **When** the app launches, **Then** the user sees the map normally and the failed event is silently dropped without any error message.

---

### User Story 2 - View Platform Connection Aggregates (Priority: P2)

An operations team member checks a dashboard to see how many active connections have been recorded per platform (web, iOS, Android) over time.

**Why this priority**: Aggregated data enables the product team to understand platform adoption and detect usage trends without inspecting individual raw events.

**Independent Test**: Send simulated connection events for two different platforms (e.g. 5 web events, 3 iOS events). Query the aggregate store — verify the counts are 5 and 3 respectively.

**Acceptance Scenarios**:

1. **Given** multiple connection events have been tracked across different platforms, **When** the aggregate store is queried, **Then** total connection counts are grouped by platform with the timestamp of the most recent handshake.
2. **Given** a platform receives its first connection event, **When** the aggregate is queried, **Then** a new platform entry is created (upserted) rather than failing due to a missing record.

---

### User Story 3 - Audit Live Connection Metrics (Priority: P3)

An administrator inspects connection aggregates through a read-only API endpoint or connects to the analytics data store directly for ad-hoc queries.

**Why this priority**: Direct data access enables debugging, custom reporting, and cross-referencing with other operational metrics.

**Independent Test**: Insert known connection data into the aggregate store. Query via the read API endpoint and verify the returned records match the inserted data. Also verify direct store access works for ad-hoc queries.

**Acceptance Scenarios**:

1. **Given** connection aggregates exist in the store, **When** an administrator queries all records, **Then** each record shows the platform identifier, total connection count, last handshake timestamp, and the app version of the most recent connection.

---

### Edge Cases

- What happens when analytics-service is down when events arrive? Events remain in the queue and are picked up when the consumer comes back online.
- What happens if the broker is unreachable? The gateway returns 500 but the client never blocks — it logs and moves on.
- What happens when duplicate event IDs are received? Each event increments the platform's connection counter independently; duplicate IDs do not cause errors.
- How does the system handle a sudden burst of 1000 simultaneous app launches? Events are queued and consumed sequentially — no data is lost as long as the broker has capacity.
- How does an operator know the pipeline is healthy? The consumer exposes a health endpoint with queue depth, last-processed timestamp, and uptime; a growing queue or stale last-processed timestamp signals a problem.

## Requirements

### Functional Requirements

- **FR-001**: The app MUST silently emit a connection event on every launch without blocking the user interface.
- **FR-002**: The gateway MUST accept connection events and return a 202 Accepted response immediately without waiting for downstream processing.
- **FR-003**: Connection events MUST be delivered to a durable message queue that survives broker restarts.
- **FR-004**: The consumer worker MUST read events from the queue and upsert a per-platform aggregate record.
- **FR-005**: Each aggregate record MUST track total connection count, most recent connection timestamp, and most recent app version for its platform.
- **FR-006**: Failed deliveries (broker unavailable, network errors) MUST NOT cause client-side errors — failures are silently logged.
- **FR-007**: Each event MUST carry a unique identifier conforming to the pattern `evt-` followed by 8 lowercase hex digits.
- **FR-008**: The consumer worker MUST expose a health endpoint reporting queue depth, last-processed event timestamp, and consumer uptime.
- **FR-009**: The gateway MUST expose a read-only `GET /api/v1/analytics/connections` endpoint returning all platform aggregate records as JSON.

### Key Entities

- **Connection Event**: Represents a single app launch. Contains an event ID, platform identifier, app version, and timestamp. Dispatched by the client and consumed asynchronously.
- **Connection Aggregate**: A per-platform counter that accumulates connection events. Stores total count, last handshake timestamp, and last known app version for each platform identifier.

## Success Criteria

### Measurable Outcomes

- **SC-001**: A connection event is recorded for every app launch within 5 seconds under normal network conditions.
- **SC-002**: The tracking pipeline adds zero measurable latency to the app launch experience (>99th percentile app launch time is unchanged with tracking enabled vs. disabled).
- **SC-003**: Aggregated platform connection data is queryable within 2 seconds of an event being processed by the consumer.
- **SC-004**: No data loss occurs during a 60-second broker outage — events queued during the outage are consumed once the broker recovers.
- **SC-005**: The pipeline handles 100 simultaneous connection events per second without data loss or backlog growth.

## Clarifications

### Session 2026-05-28

- Q: Pipeline Health Monitoring → A: Include health signals — expose a health endpoint on the consumer reporting queue depth, last-processed timestamp, and consumer uptime
- Q: Analytics Query Interface → A: Add a read-only API endpoint exposing platform aggregates at GET /analytics/connections, supplemented by direct DB access for ad-hoc queries
- Q: Data Retention Policy → A: Keep indefinitely — no automatic purge

## Assumptions

- The message broker and analytics store are deployed as part of the local development stack (docker-compose) and are not externally managed.
- Connection events are informational only — they carry no personally identifiable information (PII).
- Event IDs are generated client-side and are unique-enough for analytics purposes; collisions are tolerated (they result in one extra counter increment).
- The analytics data store retains data indefinitely — no automatic archival or purging policy.
- Existing authentication and authorization on the API gateway applies to the analytics endpoint (same backend service, same network boundary).
