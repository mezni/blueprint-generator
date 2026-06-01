# Feature Specification: Architecture Contracts

**Feature Branch**: `001-architecture-contracts`

**Created**: 2026-05-31

**Status**: Draft

**Input**: User description: "read from docs/epic00.md"

## Clarifications

### Session 2026-05-31

- Q: What is the expected initial data volume and scale? → A: Tunisia-wide initial deployment (<500 stations, <50K users, <100K events/day) with moderate growth.
- Q: What are the target response times for discovery APIs? → A: <500ms p95 for discovery listings, <2s p99 for geo-queries (nearby search, map markers).
- Q: What regulatory or compliance framework applies? → A: Tunisia's Law 2004-63 on data protection; current soft-delete and retention model is sufficient for MVP.
- Q: What are the station lifecycle states? → A: Active / Inactive (manual partner toggle) + soft-deleted (admin action). No maintenance or other states for MVP.
- Q: Should services be designed for horizontal scaling? → A: Yes — design stateless (no in-memory session state, no sticky sessions) but deploy single-instance for MVP. Scale out via Docker Compose replica config later.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Define Service Architecture Boundaries (Priority: P1)

As a platform architect, I want service boundaries, communication rules,
and data ownership finalized so that all teams build against the same
contract and avoid cross-service coupling.

**Why this priority**: Every downstream decision (DB schema, API design,
deployment) depends on knowing which service owns what. Without this,
teams make incompatible assumptions.

**Independent Test**: A reviewer can verify that each service has a
clearly documented responsibility, a list of owned tables, and that no
two services claim ownership of the same data write path.

**Acceptance Scenarios**:

1. **Given** the service boundary document, **When** inspected,
   **Then** each service lists a unique set of owned DB tables with no
   overlaps across services.
2. **Given** the communication rules, **When** checked,
   **Then** all inter-service interactions are labeled as REST (sync)
   or RabbitMQ (async), and cross-service DB access is explicitly
   forbidden.
3. **Given** the architectural invariant, **When** reviewed,
   **Then** it states that `inventory.station` is the single source of
   truth and all other systems are derived projections.

---

### User Story 2 — Define PostgreSQL Schema Contracts (Priority: P1)

As a data architect, I want all four schemas (`inventory`, `users`,
`gis`, `analytics`) fully specified so that migrations can be written
unambiguously.

**Why this priority**: The database is the system of record. Schema
definitions must be locked before any service code is written.

**Independent Test**: Each schema spec can be reviewed independently
for table lists, column rules, constraint definitions, and ownership.

**Acceptance Scenarios**:

1. **Given** the `inventory` schema spec, **When** reviewed,
   **Then** it defines `partner`, `station` (with PostGIS POINT),
   `charger`, and `station_availability` tables with Admin Service as
   sole writer.
2. **Given** the `users` schema spec, **When** reviewed,
   **Then** it defines `user_account`, `user_profile`,
   `partner_membership`, `favorite_station`, and `station_review`
   with composite PK rules for favorites.
3. **Given** the `gis` schema spec, **When** reviewed,
   **Then** it contains only derived views (`roads`, `boundaries`,
   `station_geospatial_view`) that are fully rebuildable from
   `inventory.station`.
4. **Given** the `analytics` schema spec, **When** reviewed,
   **Then** it defines `raw_event` (time-partitioned),
   `daily_event_count`, `station_daily_metric`, and
   `search_daily_metric` with append-only JSONB payload rules.

---

### User Story 3 — Define Clickstream Event Contract (Priority: P2)

As a platform architect, I want the event envelope and event type list
finalized so that frontend teams and analytics consumers build against
the same schema.

**Why this priority**: The clickstream pipeline touches every frontend
app. The event contract must be stable before frontend instrumentation
begins.

**Independent Test**: A mock event producer can send a valid envelope
and a mock consumer can parse it without schema negotiation.

**Acceptance Scenarios**:

1. **Given** the event envelope, **When** validated against the spec,
   **Then** it contains `event_id`, `event_type`, `timestamp`
   (ISO-8601), `session_id`, `actor_id`, `platform`, and `payload`.
2. **Given** the event type list, **When** inspected,
   **Then** it includes at minimum: `station_viewed`,
   `station_searched`, `map_moved`, `favorite_added`,
   `favorite_removed`, `review_created`, `review_deleted`,
   `auth_login_success`, `auth_login_failed`.
3. **Given** the delivery rules, **When** reviewed,
   **Then** they specify at-least-once delivery, JSONB payloads only,
   and no secrets in payloads.

---

### User Story 4 — Define RBAC Model in Keycloak (Priority: P2)

As a security architect, I want the role model and enforcement layers
defined so that authentication and authorization are consistent across
all services.

**Why this priority**: Every API endpoint depends on role checks.
Delaying RBAC definition causes rework across all services.

**Independent Test**: A test can validate that each of the three roles
exists in the model and that enforcement is specified at three layers:
Keycloak, service layer, and DB constraints.

**Acceptance Scenarios**:

1. **Given** the RBAC model, **When** reviewed,
   **Then** it defines exactly three roles: `registered_driver`,
   `partner`, `admin`.
2. **Given** the partner isolation rule, **When** checked,
   **Then** it states that partner queries MUST enforce `partner_id` at
   the repository level with no API-layer exceptions.

---

### User Story 5 — Define CI/CD and Observability Contracts (Priority: P3)

As a platform engineer, I want the CI/CD pipeline, observability
standards, caching strategy, and security rules finalized so that the
operational foundation is locked.

**Why this priority**: CI/CD and observability are cross-cutting
concerns that must be consistent but can be refined in parallel with
service implementation.

**Independent Test**: A reviewer can verify that the pipeline stages,
logging format, metric list, and cache invalidation rules are
documented and unambiguous.

**Acceptance Scenarios**:

1. **Given** the CI/CD contract, **When** reviewed,
   **Then** it specifies lint → test → build → contract validation →
   Docker build → GHCR publish stages with no auto-deployment.
2. **Given** the observability contract, **When** checked,
   **Then** it requires structured JSON logs with `service_name`,
   `request_id`, `trace_id`, and `event_type`; and metrics for
   latency, error rates, GIS sync lag, and ingestion lag.

---

### Edge Cases

- What happens when a new schema or service is proposed? The contract
  specifies that no new schemas or roles may be added without explicit
  architectural approval.
- How does the system handle conflicting ownership claims? The data
  ownership matrix is the single source of truth — any overlap is a
  contract violation.
- How are breaking changes to events or APIs managed? The contract
  requires versioned endpoints (`/v1/`) and backward-compatible
  migrations only; destructive changes require explicit versioning.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST define exactly five runtime services:
  Keycloak, Admin Service, Driver Service, Clickstream Service, and
  GIS Sync Worker, plus Traefik as the edge proxy.
- **FR-002**: System MUST enforce that only Traefik exposes public
  ports; all other services remain on internal networks.
- **FR-003**: System MUST define a data ownership matrix that assigns
  each PostgreSQL schema to exactly one owning service, with `users`
  co-owned by Driver and Admin.
- **FR-004**: System MUST enforce that cross-service DB access is
  forbidden; all inter-service communication is REST or RabbitMQ.
- **FR-005**: System MUST define exactly four PostgreSQL schemas:
  `inventory`, `users`, `gis`, `analytics`.
- **FR-006**: The `inventory` schema MUST contain `partner`, `station`
  (with PostGIS POINT geometry), `charger`, and `station_availability`
  tables, with Admin Service as sole writer.
- **FR-007**: The `users` schema MUST contain `user_account`,
  `user_profile`, `partner_membership`, `favorite_station`, and
  `station_review` tables.
- **FR-008**: The `gis` schema MUST contain only derived artifacts
  (`roads`, `boundaries`, `station_geospatial_view`) that are fully
  rebuildable from `inventory.station`.
- **FR-009**: The `analytics` schema MUST contain `raw_event`
  (time-partitioned), `daily_event_count`, `station_daily_metric`, and
  `search_daily_metric` tables with append-only JSONB payloads.
- **FR-010**: System MUST define a clickstream event envelope with
  `event_id`, `event_type`, `timestamp` (ISO-8601), `session_id`,
  `actor_id`, `platform`, and `payload`.
- **FR-011**: System MUST support at minimum these clickstream event
  types: `station_viewed`, `station_searched`, `map_moved`,
  `favorite_added`, `favorite_removed`, `review_created`,
  `review_deleted`, `auth_login_success`, `auth_login_failed`.
- **FR-012**: System MUST enforce at-least-once delivery for
  clickstream events with no secrets in payloads.
- **FR-013**: System MUST define exactly three roles:
  `registered_driver`, `partner`, `admin`.
- **FR-014**: System MUST enforce partner isolation at the repository
  level — all partner queries MUST filter by `partner_id` with no
  API-layer exceptions.
- **FR-015**: System MUST use REST APIs with JSON payloads, versioned
  under `/v1/`, with cursor-based pagination only.
- **FR-016**: System MUST return errors in a standard format including
  `error_code`, `message`, and `trace_id`.
- **FR-017**: System MUST have a CI pipeline with stages: lint →
  test → build → contract validation → Docker build → GHCR publish.
- **FR-018**: System MUST NOT auto-deploy to production; deployment is
  manual only, pulling pre-built images from GHCR.
- **FR-019**: System MUST emit structured JSON logs with fields:
  `service_name`, `request_id`, `trace_id`, `user_id`, `event_type`.
- **FR-020**: System MUST propagate `trace_id` across service
  boundaries, at minimum for Driver → Clickstream → Analytics flows.
- **FR-021**: System MUST cache nearby stations, station details, map
  markers, and search results, with invalidation on station update,
  availability change, or GIS sync.
- **FR-022**: System MUST use Keycloak as the single identity provider
  with enforcement at three layers: Keycloak (claims), service layer
  (authorization), DB (constraints).
- **FR-023**: System MUST use only two environments — local (Docker
  Compose) and production (bare metal) — with no staging environment.
- **FR-024**: System MUST implement soft delete for users, stations,
  and reviews.
- **FR-025**: System MUST retain `raw_event` data for 30–90 days hot
  retention, logs for 7–14 days, and aggregates permanently.
- **FR-026**: System MUST target <500ms p95 response time for
  discovery listings (station list, search results) and <2s p99 for
  geo-spatial queries (nearby search, map markers).
- **FR-027**: All services MUST be designed stateless (no in-memory
  session state, no sticky sessions) to support horizontal scaling.
  MVP deployment uses single instances; scale-out via Docker Compose
  replica count changes.

### Key Entities *(include if feature involves data)*

- **Partner**: An organization that owns charging stations. Identified
  by `PRT-` prefix NanoID. Has a type (`business` or `private`) that
  is metadata only, not authorization.
- **Station**: A physical EV charging location with a PostGIS POINT
  geometry. Canonical spatial entity. Identified by `STN-` prefix
  NanoID. Owned by exactly one partner. Has lifecycle states: Active
  (visible, partner toggled), Inactive (hidden, partner toggled), or
  soft-deleted (admin action).
- **Charger**: An individual charging unit at a station. Identified by
  `CHG-` prefix NanoID. Linked to a parent station via foreign key.
- **User Account**: A registered user (driver or partner). Identified
  by `USR-` prefix NanoID. Linked to Keycloak via `keycloak_user_id`.
- **Station Review**: User-generated rating and comment for a station.
  Identified by `REV-` prefix NanoID. Composite uniqueness per
  user + station.
- **Raw Event**: An append-only analytics event with JSONB payload.
  Partitioned by time. Part of the clickstream pipeline.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All five service boundaries are documented with explicit
  table ownership and no ownership overlaps between services.
- **SC-002**: All four PostgreSQL schemas are fully specified with
  table lists, column types, constraints, and ownership rules.
- **SC-003**: The clickstream event contract is finalized with an
  envelope schema, 9 event types, and delivery rules — all reviewed
  and approved by frontend and backend stakeholders.
- **SC-004**: The RBAC model defines exactly three roles with
  documented enforcement at Keycloak, service, and DB layers.
- **SC-005**: CI/CD pipeline stages are documented and can be executed
  against a scaffolded repository to produce a GHCR artifact.
- **SC-006**: All contract documents are stored in the repository under
  a `docs/` or `specs/` path and pass a peer review with zero
  unresolved objections.
- **SC-007**: A reviewer can verify that no placeholder tokens or
  ambiguous statements remain in any contract document.

## Assumptions

- The project already has a constitution (`docs/constitution.md`) and
  an EPIC 0 breakdown (`docs/epic00.md`) that define the high-level
  architecture — this spec translates those into a feature-deliverable
  contract set.
- Keycloak will be used as the identity provider and is assumed to be
  deployable via Docker Compose alongside the platform services.
- PostgreSQL with PostGIS extension is the only database; no
  alternative data stores are considered for the MVP.
- RabbitMQ is the only message broker; no Kafka or other event
  streaming platforms are considered for the MVP.
- The existing identifier strategy (NanoID with USR-/PRT-/STN-/CHG-/REV-
  prefixes) is accepted and does not require revalidation.
- Contract documents will be written in Markdown and stored in the
  repository; no formal contract-testing framework is required at this
  stage.
- Expected initial scale: Tunisia-wide deployment with <500 stations,
  <50K registered users, and <100K clickstream events per day, with
  moderate year-over-year growth. This informs partitioning strategy,
  cache sizing, and index design decisions.
- Data protection compliance is governed by Tunisia's Law 2004-63 on
  personal data protection. The soft-delete, retention, and consent
  model defined in this spec aligns with its requirements for MVP.
