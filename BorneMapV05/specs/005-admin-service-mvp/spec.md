# Feature Specification: Admin Service MVP

**Feature Branch**: `005-admin-service-mvp`

**Created**: 2026-06-02

**Status**: Draft

**Input**: User description: "Sprint 5 — Admin Service MVP (Inventory Write API): implement /api/v1/admin/* and /api/v1/partner/* — partner CRUD, station CRUD, charger CRUD, availability update. Full partner isolation (every query scoped by membership partner_id). On station change: insert gis.sync_queue outbox row and emit analytics event."

## Clarifications

### Session 2026-06-02

- Q: How should the `Idempotency-Key` be stored and for how long should it be retained? → A: Dedicated `inventory.idempotency_key` table in `platform_db` with TTL-based cleanup (24h retention via `created_at`)
- Q: How should concurrent edits to the same station be handled? → A: Optimistic locking — compare `updated_at` in PATCH payload/ETag; reject with `CONCURRENT_MODIFICATION` if stale
- Q: Should admin station updates also trigger GIS outbox events? → A: Yes — admin station mutations also insert `gis.sync_queue` outbox rows, consistent with FR-018's "any API path" scope
- Q: What latency target should admin/partner write endpoints meet? → A: ≤500ms p95

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Partner Manages Own Stations (Priority: P1)

A partner user needs to create, update, and delete their own charging stations through the partner API, with full confidence that they can never accidentally access or modify another partner's data. Every query is automatically scoped to their partner membership.

**Why this priority**: Partner station management is the core business operation. Without it, no stations exist in the system for drivers to discover.

**Independent Test**: A partner creates a station, retrieves it, updates it, and soft-deletes it. Another partner cannot see or modify that station.

**Acceptance Scenarios**:

1. **Given** a partner with an active membership, **When** they create a station via the partner API, **Then** the station is created under their partner_id (derived from membership, never from the request body)
2. **Given** two partners with separate stations, **When** partner A lists their stations, **Then** only partner A's stations are returned (partner B's stations are invisible)
3. **Given** a partner with active stations, **When** they attempt to soft-delete their partner account, **Then** the system blocks deletion with `ACTIVE_STATIONS_EXIST`

---

### User Story 2 - Partner Manages Chargers and Availability (Priority: P1)

A partner user needs to manage chargers at their stations and update station availability status, ensuring operational data stays current for driver discovery.

**Why this priority**: Chargers and availability are essential for drivers to find working charging points. Closely coupled with station management.

**Independent Test**: A partner adds a charger to their station, updates its status, and modifies station availability. Another partner cannot access these resources.

**Acceptance Scenarios**:

1. **Given** a partner owns a station, **When** they create a charger for that station, **Then** the charger is linked to the station and inherits partner scoping
2. **Given** a partner owns a station, **When** they update the station availability, **Then** the availability record is updated with the appropriate source indicator
3. **Given** a partner tries to add a charger to another partner's station, **Then** the request is rejected with `PARTNER_SCOPE_VIOLATION`

---

### User Story 3 - Admin Manages Partners and Stations Globally (Priority: P1)

An admin user needs to perform global CRUD operations on partners, stations, and reviews across the entire platform, with the ability to moderate content and enforce platform policies while respecting soft-delete and audit rules.

**Why this priority**: Admin oversight is required for platform governance, partner onboarding, and content moderation. Without it, there is no way to manage partners or moderate reviews.

**Independent Test**: An admin creates a partner, lists all partners, updates a partner, and attempts deletion blocked by active stations. Admin moderates a review's status.

**Acceptance Scenarios**:

1. **Given** an admin user, **When** they list all partners, **Then** all partners across the platform are returned with pagination
2. **Given** an admin user, **When** they delete a partner that has active stations, **Then** the deletion is blocked with `ACTIVE_STATIONS_EXIST`
3. **Given** an admin user, **When** they update a review's status to `hidden`, **Then** the review status transitions to `hidden` following the moderation lifecycle
4. **Given** an admin user, **When** they update or soft-delete a station, **Then** a `gis.sync_queue` outbox row is inserted (same as partner API path, per FR-018)

---

### User Story 4 - Partner Views Own Profile (Priority: P2)

A partner user needs to view their own membership information (partner_id, role) to understand their access scope and organizational affiliation.

**Why this priority**: Convenience endpoint for partner self-service. Not blocking other functionality.

**Independent Test**: A partner calls the profile endpoint and receives their membership details.

**Acceptance Scenarios**:

1. **Given** a partner with an active membership, **When** they call the partner profile endpoint, **Then** they receive their partner_id, role, and membership info

---

### User Story 5 - Station Changes Trigger GIS Outbox Events (Priority: P1)

When a station is created, updated, or deleted through any API path (partner or admin), the system must insert a corresponding row into the `gis.sync_queue` outbox table so the GIS worker can process geometry updates asynchronously.

**Why this priority**: Without outbox events, GIS state never updates and spatial queries become stale. This is critical for map-based discovery.

**Independent Test**: After creating a station, verify a `gis.sync_queue` row exists with the correct entity_id, operation, and status `pending`.

**Acceptance Scenarios**:

1. **Given** a station is created via the partner API, **When** the creation completes, **Then** a `gis.sync_queue` row is inserted with `entity_type='station'`, `entity_id` matching the new station, and `operation='insert'`
2. **Given** a station is updated, **When** the update completes, **Then** a `gis.sync_queue` row is inserted with `operation='update'`
3. **Given** a station is soft-deleted, **When** the deletion completes, **Then** a `gis.sync_queue` row is inserted with `operation='delete'`

---

### Edge Cases

- What happens when a partner tries to create a station with coordinates outside valid ranges? (Must return `INVALID_COORDINATES` error)
- What happens when a partner submits a station creation with an `Idempotency-Key` that was already used? (Must return the existing station without creating a duplicate)
- What happens when an admin tries to hard-delete a station? (Must only support soft delete; hard delete must not be possible)
- What happens when a partner tries to modify a station they don't own? (Must return `PARTNER_SCOPE_VIOLATION`)
- What happens when a user without the partner role calls partner endpoints? (Must return `INSUFFICIENT_ROLE`)
- What happens when a partner tries to create a charger for a station they don't own? (Must return `PARTNER_SCOPE_VIOLATION`)
- What happens when station status transitions violate the lifecycle (e.g., jumping from `draft` to `inactive`)? (Must return `INVALID_STATE_TRANSITION`)
- What happens when a partner creates a station but the partner is suspended? (Must return `FORBIDDEN`)
- What happens when two users concurrently update the same station with stale `updated_at`? (Must return `CONCURRENT_MODIFICATION`)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide `GET /api/v1/partner/me` returning the authenticated partner's `partner_id`, `role`, and membership info derived from `users.partner_membership`
- **FR-002**: System MUST provide `GET /api/v1/partner/stations` returning only stations owned by the authenticated partner (scoped by `partner_id` from membership), with optional `include_deleted` query parameter and pagination
- **FR-003**: System MUST provide `POST /api/v1/partner/stations` to create a station under the authenticated partner's ownership, requiring an `Idempotency-Key` header to prevent duplicate creation
- **FR-004**: System MUST provide `PATCH /api/v1/partner/stations/{id}` to update a station owned by the authenticated partner only, triggering GIS outbox event on change
- **FR-005**: System MUST provide `DELETE /api/v1/partner/stations/{id}` to soft-delete a station owned by the authenticated partner only (sets `deleted_at`, never hard deletes)
- **FR-006**: System MUST provide `GET /api/v1/partner/chargers` returning only chargers at stations owned by the authenticated partner
- **FR-007**: System MUST provide `POST /api/v1/partner/chargers` to create a charger at a station owned by the authenticated partner only
- **FR-008**: System MUST provide `PATCH /api/v1/partner/chargers/{id}` to update a charger at a station owned by the authenticated partner only
- **FR-009**: System MUST provide `PATCH /api/v1/partner/stations/{id}/availability` to update station availability for a station owned by the authenticated partner only
- **FR-010**: System MUST derive `partner_id` exclusively from `users.partner_membership` — NEVER accept `partner_id` from the request body or query parameters
- **FR-011**: System MUST reject any partner API request where the target resource belongs to a different partner, returning `PARTNER_SCOPE_VIOLATION`
- **FR-012**: System MUST provide `GET /api/v1/admin/users` returning all users with pagination (admin role required)
- **FR-013**: System MUST provide `GET /api/v1/admin/partners` and `POST /api/v1/admin/partners` for partner listing and creation (admin role required)
- **FR-014**: System MUST provide `PATCH /api/v1/admin/partners/{id}` to update a partner and `DELETE /api/v1/admin/partners/{id}` to soft-delete a partner, blocked if active stations exist (`ACTIVE_STATIONS_EXIST`)
- **FR-015**: System MUST provide `GET /api/v1/admin/stations` returning all stations with pagination, and `PATCH /api/v1/admin/stations/{id}` to update any station
- **FR-016**: System MUST provide `DELETE /api/v1/admin/stations/{id}` to soft-delete any station (admin role required)
- **FR-017**: System MUST provide `GET /api/v1/admin/reviews` returning all reviews with pagination, and `PATCH /api/v1/admin/reviews/{id}/status` to moderate review status (published/hidden/flagged/deleted)
- **FR-018**: System MUST insert a row into `gis.sync_queue` with appropriate operation (`insert`/`update`/`delete`) after every station create, update, or soft-delete through any API path
- **FR-019**: System MUST enforce authentication on all endpoints — unauthenticated requests return `UNAUTHENTICATED`; expired tokens return `TOKEN_EXPIRED`
- **FR-020**: System MUST enforce role-based authorization — partner endpoints require the `partner` role; admin endpoints require the `admin` role; wrong role returns `INSUFFICIENT_ROLE`
- **FR-021**: System MUST use the standard response envelope: success `{ "success": true, "data": {}, "meta": {} }` and error `{ "success": false, "error": { "code": "STRING", "message": "STRING" } }`
- **FR-022**: System MUST paginate all list endpoints with meta `{ "page", "size", "total", "total_pages", "has_next", "has_prev" }`
- **FR-023**: System MUST validate station coordinates against valid ranges (latitude -90 to 90, longitude -180 to 180), returning `INVALID_COORDINATES` on violation
- **FR-024**: System MUST validate station status transitions follow the lifecycle (draft → active → inactive → maintenance → active), returning `INVALID_STATE_TRANSITION` on invalid transitions
- **FR-025**: System MUST use ULID+prefix IDs for all created entities (STN-, CHG-, PRT-)
- **FR-026**: System MUST populate audit fields (`created_by`, `updated_by`) from the authenticated user's identity on all mutations
- **FR-027**: System MUST store `Idempotency-Key` values in a dedicated `inventory.idempotency_key` table within the same transaction as station creation, mapping each key to the created station ID, with expired keys (older than 24 hours based on `created_at`) eligible for cleanup
- **FR-028**: System MUST enforce optimistic concurrency on station and partner updates — the client MUST supply the current `updated_at` value (via `If-Match` ETag or request body field), and if it does not match the stored value, the request MUST be rejected with `CONCURRENT_MODIFICATION`

### Key Entities

- **Partner Station**: A station owned by a specific partner, manageable only through partner-scoped APIs. All CRUD operations are scoped by membership-derived partner_id.
- **Partner Charger**: A charger at a partner-owned station, inheriting partner scoping from its parent station.
- **Station Availability**: An operational status projection for a station, updatable by the owning partner.
- **Admin Station**: A station visible to admins across all partners for global oversight and modification.
- **Admin Partner**: A partner entity manageable by admins for onboarding, status changes, and deletion (blocked by active stations).
- **Admin Review**: A review visible to admins for content moderation (status transitions).
- **GIS Outbox Event**: A row in `gis.sync_queue` created on station mutations for asynchronous geometry processing.
- **Idempotency Key**: A client-provided header stored in `inventory.idempotency_key` that prevents duplicate station creation when retried. Keys expire after 24 hours.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A partner can complete the full station lifecycle (create → read → update → delete) end-to-end in under 5 seconds total across all four operations
- **SC-002**: Partner isolation is enforced — a partner cannot read, modify, or infer another partner's data (verified by integration test attempting cross-partner access)
- **SC-003**: All partner-scoped endpoints reject requests with wrong ownership, returning `PARTNER_SCOPE_VIOLATION` within the standard error envelope
- **SC-004**: Every station mutation (create, update, soft-delete) produces a corresponding `gis.sync_queue` outbox row within the same transaction
- **SC-005**: All list endpoints return paginated results with correct metadata
- **SC-006**: Soft delete works correctly — deleted stations have `deleted_at` set and are excluded from default queries but visible with the soft-deleted flag
- **SC-007**: Authentication and role-based authorization are enforced on every endpoint — unauthenticated or wrong-role requests are rejected with appropriate error codes
- **SC-008**: Idempotent station creation — resubmitting with the same `Idempotency-Key` returns the original station without duplication
- **SC-009**: All admin and partner write endpoints respond within ≤500ms at p95 under normal load

## Assumptions

- Sprint 4 (Core DB Schema) migrations are complete — all tables, constraints, indexes, and triggers exist in `platform_db`
- Sprint 3 (Identity & RBAC) JWT validation middleware is available in `common-auth` crate
- The `admin-service` binary already exists from Sprint 1 with a `/health` endpoint from Sprint 2
- GIS outbox insertion happens synchronously within the same database transaction as the station mutation
- Analytics event emission for station changes is deferred to a future sprint (the outbox row is the minimum viable event mechanism)
- Rate limiting is configured at the infrastructure level (Traefik) rather than in application code
- The `Idempotency-Key` is stored in a dedicated `inventory.idempotency_key` table with a 24-hour TTL (expired keys cleaned up based on `created_at`)
- Station status lifecycle transitions are enforced at the application level (not via database triggers)
- Partner suspension blocks all partner API operations (a suspended partner cannot manage stations or chargers)
