# Research: Clickstream Tracking Pipeline

## Decisions

### Decision 1: Message Broker — RabbitMQ

- **Decision**: RabbitMQ 3.12 with AMQP 0-9-1 protocol
- **Rationale**: Durable message queue with publisher confirms and consumer acknowledgements. Survives broker restarts (durable queues). Supports at-least-once delivery guarantees. The `analytics.connections` queue is declared as durable so events are not lost when the consumer is offline.
- **Alternatives considered**:
  - **In-process channel (tokio::mpsc)**: Events lost on consumer or gateway restart. Violates FR-003.
  - **Postgres LISTEN/NOTIFY**: No durable buffer; channel is lost if no listener is active. Poor backpressure handling.
  - **Redis Streams**: Deprecated by Principle I (Validation Before Optimization) — Redis is not yet validated.

### Decision 2: Analytics Store — MongoDB

- **Decision**: MongoDB 6.0 with `$inc` atomic upsert pattern
- **Rationale**: The aggregate data model is a simple key-value counter (platform → {count, last_seen, version}). MongoDB's `db.collection.update_one(filter, {$inc: ...}, {upsert: true})` maps one-to-one with FR-005. No schema migrations needed for a single-collection workload.
- **Alternatives considered**:
  - **PostgreSQL upsert** (`INSERT ... ON CONFLICT DO UPDATE`): Works but adds schema management overhead (CREATE TABLE, migrations) for two fields. The analytics data is write-heavy, read-rare — PostgreSQL is optimized for relational queries, not simple counters.
  - **In-memory HashMap**: Lost on service restart. Violates durability expectations.

### Decision 3: Health Endpoint Pattern

- **Decision**: Expose `/health` on the consumer worker as a simple Actix-web route returning JSON with queue depth, last-processed timestamp, and uptime
- **Rationale**: Standard health-check pattern for internal services. Container orchestrators (Docker, K8s) can probe this endpoint. Queue depth trend (growing → consumer may be falling behind) and stale last-processed timestamp (>30s → consumer may be stuck) are the primary signals.
- **Alternatives considered**:
  - **Log-based only**: Harder to automate alerting. No standard probe interface.
  - **Prometheus metrics endpoint**: Overkill for a single-consumer pipeline at this scale.

### Decision 4: Gateway Integration Pattern

- **Decision**: The analytics endpoint (`/api/v1/analytics/connect`) lives in a new `analytics` domain module within `api-service`. The AMQP channel is shared via `web::Data<AppState>`.
- **Rationale**: Reuses the existing Actix-web server and connection pool infrastructure. No separate gateway process needed. The `AppState` struct already holds database pool; extending it with the AMQP channel is the established pattern (see `locate` domain).
- **Alternatives considered**:
  - **Standalone gateway for analytics**: Unnecessary process overhead for a single endpoint.

### Decision 5: Event ID Generation

- **Decision**: Client-side generation using `evt-` prefix + 8 hex random digits
- **Rationale**: No server-side round trip for ID generation. The nanouuid pattern (`evt-[a-f0-9]{8}`) matches the constitutional ID standard (Principle IV). Collision probability for 8 hex digits is ~1/4B — acceptable for analytics where a duplicate results in +1 counter increment.
- **Alternatives considered**:
  - **Server-side UUIDv4**: Barred by Principle IV.
  - **Sequential ID**: Barred by Principle IV.

### Decision 6: Consumer Worker Structure

- **Decision**: New `analytics-service` crate in the workspace under `backend/analytics-service/`
- **Rationale**: Keeps the consumer fully decoupled from the API gateway. They share only the `core` crate for types. The consumer can be scaled independently (though single-worker is the initial deployment).
- **Alternatives considered**:
  - **Background thread in api-service**: Ties consumer lifecycle to gateway. Consumer restart would require gateway restart.

## Compliance Review

| Constitution Rule | Status | Notes |
|------------------|--------|-------|
| Principle I: RabbitMQ deprecated | Violation | Justified in Complexity Tracking — durable queue is architecturally required for non-blocking pipeline |
| Principle II: Stack LOCKED | Violation (MongoDB) | Justified in Complexity Tracking — fit-for-purpose for write-heavy counter workload |
| Principle III: Service isolation | Compliant | analytics-service is a discrete domain, no circular deps |
| Principle IV: nanouuid IDs | Compliant | `evt-[a-f0-9]{8}` pattern |
| Principle V: Docker Compose | Compliant | RabbitMQ + MongoDB added to deployments/docker-compose.yml |
| Principle V: /spec and /docs sync | Compliant | All artifacts under specs/005-clickstream-tracking/ |
