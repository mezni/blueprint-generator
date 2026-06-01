# Feature Specification: Database & Schema Foundation

**Feature Branch**: `006-database-schema-foundation`

**Created**: 2026-05-31

**Status**: Draft

**Input**: User description from docs/epic05.md

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Database Schema with Schema Ownership (Priority: P1)

As a platform developer, I want four isolated database schemas (inventory, users, gis, analytics) with strict ownership boundaries so that each service owns its data and no cross-service writes can corrupt data integrity.

**Why this priority**: The database foundation blocks all backend service development — no service can function without its schema.

**Independent Test**: A developer can verify that only the Admin Service can write to the inventory schema, the GIS Sync Worker can only read inventory but write to gis, and no service can write outside its owned schema.

**Acceptance Scenarios**:

1. **Given** the PostgreSQL database is initialized, **When** the platform starts, **Then** four schemas exist: `inventory`, `users`, `gis`, and `analytics`.
2. **Given** a service attempts to write to a schema it does not own, **When** the write is executed, **Then** it is rejected at the database level.
3. **Given** the Admin Service writes a new station, **When** the station record is created, **Then** it appears in the `inventory.station` table with a globally unique identifier.

---

### User Story 2 — Keycloak Identity Integration (Priority: P1)

As a platform operator, I want every Keycloak user to map to exactly one application-level user record so that authentication identity is synchronized with application data without duplication.

**Why this priority**: The identity mapping is a hard dependency for all user-facing services — without it, no authenticated user can interact with application data.

**Independent Test**: A new user logs in via Keycloak for the first time; a `user_account` record is created automatically with the Keycloak ID. On subsequent logins, the existing record is reused.

**Acceptance Scenarios**:

1. **Given** a user authenticates for the first time, **When** the login completes, **Then** a `user_account` record is created with the Keycloak `sub` as the `keycloak_id`.
2. **Given** a returning user authenticates, **When** the login completes, **Then** the existing `user_account` record is used (no duplicate created).
3. **Given** a user's role is changed in Keycloak, **When** they next authenticate, **Then** the application reflects the updated role.

---

### User Story 3 — Migration System (Priority: P2)

As a database administrator, I want all schema changes to go through a single versioned migration system so that changes are auditable, reversible, and consistent across environments.

**Why this priority**: Without a migration system, schema changes become ad-hoc and untracked, leading to environment drift and deployment failures.

**Independent Test**: A developer applies migration version 001 to create a new table; the table appears in the correct schema. Rolling back the migration removes the table.

**Acceptance Scenarios**:

1. **Given** a migration is created in the migration directory, **When** the migration tool runs, **Then** the change is applied to the database and recorded in the migration tracking table.
2. **Given** a migration fails partway, **When** the error occurs, **Then** the transaction is rolled back and the database remains in its previous state.
3. **Given** a rollback script is provided, **When** the rollback is executed, **Then** the change is reverted cleanly.

---

### User Story 4 — Spatial and Analytical Query Performance (Priority: P3)

As a driver using the platform, I want nearby station searches to return results quickly so that I can find charging stations without noticeable delay.

**Why this priority**: Performance directly impacts user satisfaction but the foundational schema must exist before optimization has value.

**Independent Test**: A driver searches for nearby stations; results appear within 300ms on a standard broadband connection.

**Acceptance Scenarios**:

1. **Given** a driver searches for nearby stations, **When** the query executes, **Then** results appear in under 300ms for a dataset of up to 10,000 stations.
2. **Given** an analytics query runs against the event data, **When** the query executes, **Then** it does not block or degrade station search performance.

---

### Edge Cases

- What happens when the migration directory contains conflicting version numbers? The migration system detects the conflict and rejects the run with a clear error.
- What happens when a Keycloak user is deleted but their `user_account` record still exists? The `user_account` record enters an "orphaned" state and an admin can manually trigger cleanup; no automatic cleanup occurs.
- What happens when a service attempts to write to a schema it does not own? The database connection credentials are scoped per service so the write is rejected at the PostgreSQL level.
- What happens when PostGIS is not available during initialization? The database setup fails with a clear error indicating the missing extension.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide four isolated PostgreSQL schemas: `inventory`, `users`, `gis`, and `analytics`.
- **FR-002**: The `inventory` schema MUST be writable only by the Admin Service.
- **FR-003**: The `users` schema MUST be writable by the Driver Service and Admin Service.
- **FR-004**: The `gis` schema MUST be writable only by the GIS Sync Worker.
- **FR-005**: The `analytics` schema MUST be writable only by the Clickstream Service and analytics workers.
- **FR-006**: Every Keycloak user MUST map to exactly one `user_account` record via a 1:1 mapping of `keycloak_id` to `sub`.
- **FR-007**: On first authentication, the system MUST automatically provision a `user_account` record if none exists.
- **FR-008**: All user roles MUST be sourced from Keycloak only; backend services MUST NOT override roles independently.
- **FR-009**: The system MUST provide a versioned migration system under `/services/admin-service/migrations/` for all schema changes.
- **FR-010**: All schema changes MUST go through the migration system; ad-hoc SQL in services is forbidden.
- **FR-011**: All spatial data MUST use SRID 4326 with the GEOGRAPHY(Point) geometry type.
- **FR-012**: The `inventory.station.location` column MUST be the authoritative source of station geometry.
- **FR-013**: All entity identifiers MUST follow the deterministic ID format: `USR-xxx` (users), `PRT-xxx` (partners), `STN-xxx` (stations), `CHG-xxx` (chargers), `REV-xxx` (reviews).
- **FR-014**: The `analytics.raw_event` table MUST be append-only and partitioned by time.

### Key Entities

- **Schema**: An isolated PostgreSQL namespace (`inventory`, `users`, `gis`, `analytics`) with strict ownership boundaries.
- **Migration**: A versioned SQL script that applies or rolls back a schema change, tracked in a migration table.
- **Identity Mapping**: The 1:1 contract between a Keycloak user `sub` and the application `user_account.keycloak_id`.
- **Spatial Index**: A PostGIS GIST index on geospatial columns to enable fast proximity queries.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Four PostgreSQL schemas exist with correct ownership enforced — verified by automated schema audit.
- **SC-002**: A new Keycloak user automatically receives a matching `user_account` record on first login — verified by end-to-end auth test.
- **SC-003**: A returning Keycloak user reuses their existing `user_account` record without duplication — verified by repeated login test.
- **SC-004**: A migration can be applied and rolled back without data loss — verified by migration test suite.
- **SC-005**: Station search queries return results in under 300ms for up to 10,000 stations — verified by performance benchmark.
- **SC-006**: Unauthorized cross-schema writes are rejected at the database level — verified by permission test.
- **SC-007**: All entity identifiers follow the correct format and are globally unique — verified by ID generation test.

---

## Clarifications

### Session 2026-05-31

- Q: Which tool will execute database migrations? → A: sqlx migrations (Rust-native, file-based migrations)
- Q: How many analytics events per day expected? → A: Medium volume (~100K events/day), supporting daily table partitions
- Q: What default grace period for orphaned user cleanup? → A: No automatic cleanup; cleanup is manual/admin-triggered only

## Assumptions

- PostgreSQL 16 with PostGIS 3.4 is available as the database platform (from EPIC 2 runtime).
- Keycloak 25.0 is operational as the identity provider (from EPIC 2 runtime).
- Migration scripts will be written in plain SQL using sqlx migrations and executed automatically on Admin Service startup.
- The migration runner will be integrated into the Admin Service startup process.
- Database connection credentials will be scoped per service to enforce schema ownership at the connection level.
- The GIS Sync Worker has read-only access to `inventory` schema and write access to `gis` schema.
- ID generation happens in the application layer (not database sequences) to remain portable.
- The analytics schema will use daily table partitioning for the `raw_event` table, sized for ~100K events per day.
- No cross-schema joins in write paths — reads may join across schemas when necessary.
