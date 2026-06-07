# Feature Specification: Core Database Schema

**Feature Branch**: `004-core-db-schema`

**Created**: 2026-06-02

**Status**: Draft

**Input**: User description: "Sprint 4 — Core Database Schema: write migrations for all schemas/tables/indexes in §4 (inventory, users, gis, analytics stub). Enforce soft delete columns, station visibility rule helpers, GIST indexing on geom. Seed data + spatial query smoke test."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Schema Provisioning for Business Data (Priority: P1)

A platform operator needs all business data structures (partners, stations, chargers, availability) to exist in `platform_db` under the `inventory` schema so that backend services can read and write EV charging inventory. Without these tables, no business logic can function.

**Why this priority**: Every subsequent sprint (admin service, driver service, GIS sync) depends on these tables existing. This is the data foundation.

**Independent Test**: Can be fully tested by running migrations against a fresh `platform_db` and verifying all `inventory.*` tables exist with correct columns, types, constraints, and indexes. Delivers a queryable data model for station management.

**Acceptance Scenarios**:

1. **Given** a fresh `platform_db` with PostGIS enabled, **When** migrations are executed, **Then** the `inventory` schema contains `partner`, `station`, `charger`, and `station_availability` tables with all specified columns, types, and constraints
2. **Given** the `inventory.station` table exists, **When** a station row is inserted with `latitude` and `longitude`, **Then** the `geom` column is automatically populated with `ST_SetSRID(ST_MakePoint(lng,lat),4326)` and a GIST index exists on `geom`
3. **Given** a station references a partner, **When** the partner is queried for deletion, **Then** the system prevents deletion if active stations exist (enforced by a database trigger as data-layer safety net, plus application logic for user-friendly error)

---

### User Story 2 - Identity & User Data Structures (Priority: P1)

A registered driver or partner user needs their identity bridged from Keycloak to `platform_db` via the `users` schema, so that the system can associate actions (favorites, reviews, partner membership) with a real user account. The `partner_membership` table ensures a user belongs to at most one partner.

**Why this priority**: Sprint 3 (Identity & RBAC) already implemented JWT validation; this schema provides the persistent storage for the identity bridge and user-scoped data that Sprint 7 (Driver Service) will query.

**Independent Test**: Can be tested by inserting a `user_account` linked to a Keycloak subject, creating a `partner_membership`, and verifying the 1:1 uniqueness constraint prevents duplicate memberships. Delivers a working user identity layer.

**Acceptance Scenarios**:

1. **Given** a fresh `platform_db`, **When** migrations run, **Then** the `users` schema contains `user_account`, `user_profile`, `partner_membership`, `favorite_station`, and `station_review` tables
2. **Given** a `user_account` row, **When** a second `partner_membership` is inserted for the same `user_id`, **Then** the UNIQUE constraint rejects it
3. **Given** a `station_review` exists for a (user, station) pair, **When** a second review is submitted for the same pair, **Then** the UNIQUE constraint rejects it

---

### User Story 3 - GIS Outbox & Spatial Indexing (Priority: P1)

The GIS worker needs an outbox table (`gis.sync_queue`) in `platform_db` to track pending station geometry syncs, and spatial indexes must be in place so that bbox/radius queries on stations perform efficiently. This enables Sprint 6 (GIS Sync) to consume and process geometry updates.

**Why this priority**: Without the outbox and GIST indexes, GIS sync has no source to consume and spatial queries will be unacceptably slow (full-table scans).

**Independent Test**: Can be tested by inserting outbox rows, verifying status transitions are possible, and running a bbox spatial query that returns results using the GIST index (confirmed via EXPLAIN).

**Acceptance Scenarios**:

1. **Given** the `gis` schema and `inventory.visible_stations` view exist, **When** a row is inserted into `gis.sync_queue`, **Then** it has fields for `entity_type`, `entity_id`, `operation`, `payload`, and `status` with valid states: pending, processing, done, failed, dead_letter
2. **Given** 1000 stations with `geom` populated, **When** a bbox spatial query is executed, **Then** EXPLAIN shows a GIST index scan (not a sequential scan)
3. **Given** stations with mixed visibility (some is_live=false, some deleted, some inactive, some not public), **When** querying `inventory.visible_stations`, **Then** only stations satisfying all four visibility conditions are returned

---

### User Story 4 - Analytics Schema Stub (Priority: P2)

The analytics pipeline needs the `analytics_db` schema created with the `raw_event` partitioned table and `event_dead_letter` table, so that Sprint 14 (Analytics Writer) has a target to write to. This is a structural stub — no application writes to it yet.

**Why this priority**: Needed before analytics writer sprint, but not blocking any immediate sprint (Sprint 5-7). Lower priority than business data structures.

**Independent Test**: Can be tested by verifying `analytics_db` exists with the `analytics` schema, `raw_event` table is partitioned by month, and `event_dead_letter` table exists.

**Acceptance Scenarios**:

1. **Given** a fresh `analytics_db`, **When** migrations run, **Then** the `analytics` schema contains `raw_event` (partitioned monthly with 12 pre-created partitions) and `event_dead_letter` tables
2. **Given** the `raw_event` table, **When** an event is inserted for a given month, **Then** it routes to the correct monthly partition

---

### User Story 5 - Seed Data & Smoke Testing (Priority: P2)

Developers need repeatable seed data (sample partners, stations, chargers, users, reviews) so that subsequent sprints can develop and test against realistic data without manual setup. A spatial query smoke test confirms the GIS layer and PostGIS integration work end-to-end.

**Why this priority**: Accelerates all subsequent development but is not a hard dependency for Sprint 5 (admin service can create its own test data).

**Independent Test**: Can be tested by running the seed script and verifying that sample data is queryable, bbox queries return expected stations, and all relationships are valid.

**Acceptance Scenarios**:

1. **Given** migrations have run, **When** the seed script executes, **Then** sample partners, stations, chargers, users, favorites, and reviews exist with valid relationships
2. **Given** seeded stations in Tunisia, **When** a bbox query covering Tunis is executed, **Then** stations within that bbox are returned with correct `distance_km` values

---

### Edge Cases

- What happens when migrations are re-run on an already-migrated database? (Must be idempotent — no errors, no data loss.)
- What happens when a station is inserted with latitude/longitude outside valid ranges (e.g., lat > 90 or lng > 180)? (Should be rejected by a CHECK constraint.)
- What happens when a `gis.sync_queue` row is inserted with an invalid `operation` value? (Should be rejected by a CHECK constraint on the `operation` column.)
- What happens when a partner soft-delete is attempted but the partner has active stations? (Must be blocked by a database trigger — `ACTIVE_STATIONS_EXIST` error raised at data layer.)
- What happens when a review is soft-deleted? (Status transitions to `deleted`; the row is preserved but excluded from public queries. No `deleted_at` column is used.)
- What happens when a `station.geom` is queried but `latitude`/`longitude` are NULL? (The trigger must handle NULLs gracefully — no geometry generated.)
- What happens if two migrations run concurrently? (Migration locking must prevent race conditions.)

## Clarifications

### Session 2026-06-02

- Q: Should the ACTIVE_STATIONS_EXIST guard be a database trigger or application logic? → A: Both — trigger as safety net plus application-level check with user-friendly error
- Q: How should analytics monthly partitions be created? → A: Pre-create 12 months of monthly partitions (e.g., 2026-01 through 2026-12) in the migration

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST create the `inventory` schema with tables `partner`, `station`, `charger`, and `station_availability` containing all columns, types, and constraints specified in the data model
- **FR-002**: System MUST create the `users` schema with tables `user_account`, `user_profile`, `partner_membership`, `favorite_station`, and `station_review` containing all columns, types, and constraints specified in the data model
- **FR-003**: System MUST create the `gis` schema with the `sync_queue` outbox table containing columns for entity tracking, operation status, and payload
- **FR-004**: System MUST create the `analytics` schema in `analytics_db` with `raw_event` (monthly partitioned) and `event_dead_letter` tables. Migrations MUST pre-create 12 monthly partitions for `raw_event` (e.g., `raw_event_2026_01` through `raw_event_2026_12`). Future partition maintenance is deferred to Sprint 14.
- **FR-005**: Mutable business entities subject to soft-delete (partner, station, charger) MUST include full audit fields: `created_at`, `updated_at`, `created_by`, `updated_by`, and `deleted_at` (nullable). Other tables use minimal timestamps appropriate to their role: operational projections (station_availability) use only `updated_at`; identity/auth tables (user_account) use `created_at` and `last_login_at`; lightweight join tables (user_profile, partner_membership, favorite_station) use only the fields needed for their lifecycle. See data-model.md for the per-table audit field specification.
- **FR-006**: All primary keys MUST use the ULID+prefix strategy: `USR-`, `PRT-`, `STN-`, `CHG-`, `REV-` (and `EVT-` for analytics)
- **FR-007**: Station `geom` column MUST be automatically maintained from `latitude`/`longitude` via a database trigger (`ST_SetSRID(ST_MakePoint(lng,lat),4326)`)
- **FR-008**: GIST index MUST exist on `inventory.station.geom` for efficient spatial queries
- **FR-009**: B-tree indexes MUST exist on all foreign keys and common query filters: `status`, `partner_id`, `is_live`, `station_id`, `is_public`, `city`, `user_id`, `keycloak_user_id`, `session_id`
- **FR-009a**: A SQL VIEW `visible_stations` MUST be created in the `inventory` schema encoding the visibility rule: `is_live = true AND deleted_at IS NULL AND status = 'active' AND is_public = true`. All services querying live public stations SHOULD use this view rather than duplicating the filter logic.
- **FR-010**: Soft delete MUST be enforced for `partner`, `station`, and `review` tables; hard deletes MUST NOT be possible in production. Partner and station use a `deleted_at` column (nullable TIMESTAMPTZ). Review uses a status-based logical delete (`status = 'deleted'`) rather than a `deleted_at` column, per the moderation lifecycle (submitted → published → flagged → hidden → deleted).
- **FR-011**: Partner soft-deletion MUST be blocked when active stations exist. A database trigger on `inventory.partner` MUST prevent setting `deleted_at` if any related station has `is_live = true AND deleted_at IS NULL` (raises `ACTIVE_STATIONS_EXIST` error). Application logic in Sprint 5 will provide user-friendly error handling; the trigger is the data-layer safety net.
- **FR-012**: `partner_membership` MUST enforce a strict 1:1 relationship — a user can belong to at most one partner (UNIQUE constraint on `user_id`)
- **FR-013**: `station_review` MUST enforce one review per user per station (UNIQUE constraint on `user_id, station_id`)
- **FR-014**: `favorite_station` MUST use composite primary key `(user_id, station_id)`
- **FR-015**: CHECK constraints MUST validate: station `status` in allowed values, charger `status` in allowed values, charger `type` in allowed values, review `rating` between 1-5, review `status` in allowed values, `gis.sync_queue` `operation` and `status` in allowed values, `partner.type` and `partner.status` in allowed values
- **FR-016**: Latitude MUST be between -90 and 90; longitude MUST be between -180 and 180 (CHECK constraint)
- **FR-017**: Migrations MUST be idempotent — re-running on an already-migrated database MUST NOT cause errors or data loss
- **FR-018**: Migrations MUST run before service startup and MUST NOT be auto-executed at runtime by services
- **FR-019**: Seed data MUST include sample partners, stations (in Tunisia), chargers, user accounts, partner memberships, favorites, and reviews with valid relationships
- **FR-020**: A spatial query smoke test MUST confirm that a bbox query on seeded stations returns correct results using the GIST index

### Key Entities

- **Partner**: An organization or individual that owns EV charging stations. Identified by `PRT-` prefixed ULID. Has a type (business/private) and status (active/suspended). Can be soft-deleted only when no active stations exist.
- **Station**: A charging location with geographic coordinates, status lifecycle (draft/active/inactive/maintenance), and visibility flags (is_live, is_public). Identified by `STN-` prefixed ULID. Owned by exactly one partner. Has a PostGIS geometry column derived from lat/lng.
- **Charger**: A physical charging unit at a station. Identified by `CHG-` prefixed ULID. Has a connector type (CCS/Type2/CHAdeMO), power rating, and status (available/offline/fault).
- **Station Availability**: An operational projection of station status (available/limited/unavailable) with a source indicator. NOT authoritative — derived/manual.
- **User Account**: The identity bridge linking a Keycloak user to platform data. Identified by `USR-` prefixed ULID. The only bridge is `keycloak_user_id = JWT.sub`.
- **Partner Membership**: Links a user to exactly one partner with a role (owner/manager/operator/viewer). Strict 1:1 — a user cannot belong to multiple partners.
- **Favorite Station**: A user's saved station. Composite key (user_id, station_id). Only registered drivers can create.
- **Station Review**: A user's rating (1-5) and comment for a station. One per user per station. Has a moderation lifecycle (submitted/published/flagged/hidden/deleted). Soft-deleted via `status='deleted'` (not a `deleted_at` column).
- **GIS Sync Queue (Outbox)**: Tracks pending GIS geometry updates for stations/chargers. Has processing states (pending/processing/done/failed/dead_letter).
- **Raw Event**: An immutable analytics event partitioned by month. Deduplicated by `event_id`.
- **Event Dead Letter**: Stores invalid or unprocessable analytics events.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All migrations run cleanly on a fresh database and complete without errors in under 30 seconds
- **SC-002**: Re-running migrations on an already-migrated database completes without errors (idempotency)
- **SC-003**: All foreign key relationships are enforced — inserting a station without a valid partner is rejected
- **SC-004**: A bbox spatial query on 1000 seeded stations returns results in under 50ms using a GIST index scan (verified via EXPLAIN)
- **SC-005**: Soft delete works correctly — setting `deleted_at` on a station excludes it from visibility queries while preserving data
- **SC-006**: All CHECK constraints reject invalid data (invalid statuses, out-of-range ratings, invalid coordinates)
- **SC-007**: The `raw_event` table correctly routes inserts to monthly partitions based on `occurred_at`
- **SC-008**: Seed data produces a consistent, queryable dataset where all relationships are valid and a Tunis bbox query returns expected stations

## Assumptions

- PostgreSQL with PostGIS extension is already available from Sprint 2 (Docker Compose infrastructure)
- `platform_db` and `analytics_db` databases already exist from Sprint 2 Docker Compose setup
- Keycloak realm and roles already exist from Sprint 3, but user provisioning rows will be created by Sprint 3's first-login logic (not by these migrations)
- Migration tooling is SQL-based (raw SQL migration files or a Rust migration framework like `sqlx` or `refinery`)
- The `analytics_db` schema is a structural stub — no application writes to it until Sprint 14
- Seed data is for development/testing only and is not run in production
- The geom trigger fires on INSERT and UPDATE of station rows; it does not handle DELETE (soft delete sets `deleted_at` but retains the row)
- Station lifecycle state transitions (draft → active → inactive → deleted) are enforced at the application level, not by database triggers
