# Feature Specification: MVP 3 — Async Foundation

**Feature Branch**: `003-mvp3-async-foundation`
**Created**: 2026-05-23
**Status**: Draft
**Input**: User description: "Introduce background tasks: avatar uploads via MinIO + presigned URLs, and an append-only event log fed by RabbitMQ workers. No business-logic regression."

> Predecessors: MVP 1 and 2 merged and validated.
> Read first: [`docs/plan.md`](../../docs/plan.md) §MVP 3.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Driver uploads an avatar (Priority: P1)

A driver opens their profile, taps "Change avatar", picks a photo, watches it upload in the background, and sees the new avatar without blocking the request thread.

**Independent Test**: A driver can upload a 2 MB JPEG on 4G and see the new avatar within 5 s end-to-end.

**Acceptance Scenarios**:

1. **Given** an authenticated driver, **When** they request `/api/v1/profile/avatar/presign`, **Then** they receive `{ upload_url, object_key, expires_at }`.
2. **Given** the presigned URL, **When** the client PUTs the file directly to MinIO, **Then** the object is stored under `avatars/{user_id}/{uuid}.jpg`.
3. **Given** a successful upload, **When** the client PATCHes `/api/v1/profile/me { avatar_object_key }`, **Then** the avatar is associated and a derived public URL is returned.
4. **Given** an expired presigned URL, **When** the client PUTs to it, **Then** MinIO returns `403`.

---

### User Story 2 — System captures audit events asynchronously (Priority: P1)

Every business mutation (station created/updated/deleted, review created/hidden, profile updated) publishes an event. A worker consumes and persists to `event_domain.events`. Admins can browse the log.

**Independent Test**: An admin creating a station produces exactly one `station.created` event visible in `GET /api/v1/admin/events` within 5 s.

**Acceptance Scenarios**:

1. **Given** an admin creates a station, **When** the transaction commits, **Then** an outbox row is inserted in the same transaction.
2. **Given** the worker runs, **When** it drains the outbox, **Then** the event is published to RabbitMQ exchange `bornemap.events`.
3. **Given** the consumer reads from the exchange, **When** an event arrives, **Then** it is persisted in `event_domain.events` with idempotency on `event_id`.

---

### User Story 3 — Graceful broker outage (Priority: P2)

When RabbitMQ is unavailable, business mutations still succeed; events accumulate in the outbox and flush when the broker recovers.

**Independent Test**: Stop the broker, perform 10 admin mutations, restart the broker, and observe all 10 events appear in `event_domain.events` within 60 s.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST expose `POST /api/v1/profile/avatar/presign` returning `{ upload_url, object_key, expires_at }` (15-min TTL).
- **FR-002**: System MUST accept `PATCH /api/v1/profile/me { avatar_object_key }` and resolve the public URL via MinIO/CDN config.
- **FR-003**: Backend MUST implement an **outbox pattern**: every business mutation inserts into `event_domain.outbox` in the same transaction.
- **FR-004**: A separate `bornemap-worker` binary MUST consume `event_domain.outbox` and publish to RabbitMQ.
- **FR-005**: A consumer in `bornemap-worker` MUST subscribe to the exchange and persist into `event_domain.events`, deduplicating on `event_id`.
- **FR-006**: System MUST expose `GET /api/v1/admin/events?since=…&type=…&limit=…` (admin only).
- **FR-007**: Worker lag MUST be exposed as a Prometheus gauge.
- **FR-008**: Request-path latency MUST NOT regress more than 5% P95 vs MVP 2.

### Key Entities

- **OutboxRow**: `event_id`, `event_type`, `payload_json`, `occurred_at`, `published_at NULL`.
- **EventRecord**: Append-only record in `event_domain.events`. Same shape as the canonical event envelope.
- **AvatarObject**: MinIO object with key prefix `avatars/{user_id}/`, content-type validated server-side via presign policy.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: P95 avatar end-to-end upload time ≤ **5 s** on 4G with a 2 MB file.
- **SC-002**: Event publication failure rate < **0.1%** under normal conditions.
- **SC-003**: P95 worker consumer lag ≤ **5 s**.
- **SC-004**: Request-path P95 latency regression vs MVP 2 ≤ **+5%**.
- **SC-005**: Broker-outage drill: 100% of events emitted during outage are persisted after recovery (≤ 60 s).

### Proceed Criteria → MVP 4

All SC-001..SC-005 met; broker-outage drill executed and reported; ≥ 5 driver interviews on avatar UX.

## Assumptions

- MinIO is deployed in the data-storage subnet; presigned URLs are served via the gateway/CDN edge.
- Avatar content-type whitelist: `image/jpeg`, `image/png`, `image/webp`. Max size: 4 MB.
- Event payload schemas live alongside their emitting domain (Rust types serialized via `serde`).

## Constitutional Gates

- [ ] Outbox pattern preserves transactional integrity (no event lost on partial failure).
- [ ] No new public path to the data subnet.
- [ ] Idempotent event ingestion (UNIQUE on `event_id`).
- [ ] OpenAPI updated; worker config documented.
