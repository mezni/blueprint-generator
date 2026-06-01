---
description: "Task list for Database & Schema Foundation implementation"
---

# Tasks: Database & Schema Foundation

**Input**: Design documents from `specs/006-database-schema-foundation/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included where the spec's success criteria (SC-001 through SC-007) require verifiable outcomes.

**Organization**: Tasks are grouped by user story execution order (US3 must precede US1 as migration system is a prerequisite for schema creation).

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic dependency setup

- [X] T001 Add sqlx dependencies to `crates/common-db/Cargo.toml` (runtime-tokio, postgres features)
- [X] T002 [P] Add sqlx migrate feature to `services/admin-service/Cargo.toml`
- [X] T003 [P] Add ID generation module (`generate_id` with prefixes USR-/PRT-/STN-/CHG-/REV-) to `contracts/src/lib.rs`
- [X] T004 Create PostGIS init script at `infra/compose/db/init/01-postgis-init.sql`
- [X] T005 Add DB init volume mount to `infra/compose/docker-compose.yml` for `services/admin-service/migrations/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story work

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 Implement `PgPool` initialization and `run_migrations()` in `crates/common-db/src/lib.rs`
- [X] T007 Integrate migration runner (`common_db::run_migrations()`) into Admin Service startup in `services/admin-service/src/main.rs`
- [X] T008 [P] Configure `DATABASE_URL` environment variable usage in Admin Service with fallback to localhost default

**Checkpoint**: Foundation ready - migration infrastructure operational

---

## Phase 3: User Story 3 - Migration System (Priority: P2)

**Goal**: A versioned, transactional migration system that is auditable, reversible, and consistent across environments.

**Independent Test**: Apply migration 001 to create extensions; verify PostGIS and uuid-ossp exist. Roll back; confirm extensions removed.

- [X] T009 Create migration `services/admin-service/migrations/20260531000001_create_extensions.sql` (CREATE EXTENSION postgis, uuid-ossp)
- [X] T010 Create migration `services/admin-service/migrations/20260531000002_create_schemas.sql` (CREATE SCHEMA inventory, users, gis, analytics)
- [X] T011 Create migration `services/admin-service/migrations/20260531000003_create_roles.sql` (CREATE ROLE admin_service, driver_service, gis_worker, clickstream_service)
- [X] T012 Write migration integration test in `services/admin-service/tests/integration/migration_test.rs` (apply + rollback cycle)

**Checkpoint**: Migration system functional - can create extensions, schemas, and roles via sqlx

---

## Phase 4: User Story 1 - Database Schema with Schema Ownership (Priority: P1)

**Goal**: Four isolated PostgreSQL schemas with strict ownership boundaries enforced at the database level.

**Independent Test**: Verify that only the Admin Service role can INSERT into `inventory` tables, the GIS Worker can only READ `inventory` but WRITE `gis`, and unauthorized writes are rejected.

- [X] T013 [P] [US1] Create migration `services/admin-service/migrations/20260531000004_create_users_tables.sql` (user_account, favorite)
- [X] T014 [P] [US1] Create migration `services/admin-service/migrations/20260531000005_create_inventory_tables.sql` (partner, station, charger, review)
- [X] T015 [P] [US1] Create migration `services/admin-service/migrations/20260531000006_create_gis_tables.sql` (station_enrichment, geo_boundary)
- [X] T016 [P] [US1] Create migration `services/admin-service/migrations/20260531000007_create_analytics_tables.sql` (raw_event with daily partitioning)
- [X] T017 [US1] Create migration `services/admin-service/migrations/20260531000008_grant_privileges.sql` (GRANT statements per schema-ownership contract)
- [X] T018 [US1] Write schema permissions integration test in `services/admin-service/tests/integration/schema_permissions_test.rs`

**Checkpoint**: Four schemas exist with correct table definitions, indexes, and ownership enforced

---

## Phase 5: User Story 2 - Keycloak Identity Integration (Priority: P1)

**Goal**: Every Keycloak user maps to exactly one `user_account` record; provisioning happens automatically on first authentication.

**Independent Test**: Present a valid JWT with a new `sub` claim; verify a `user_account` record is created with matching `keycloak_id`. Present the same JWT again; verify the existing record is reused (no duplicate).

- [X] T019 [P] [US2] Implement identity provisioning query (UPSERT on keycloak_id) in `crates/common-db/src/identity.rs`
- [X] T020 [US2] Integrate identity auto-provisioning into auth middleware in `crates/common-auth/src/middleware.rs`
- [X] T021 [US2] Write identity mapping integration test in `services/admin-service/tests/integration/identity_mapping_test.rs`

**Checkpoint**: New Keycloak users auto-provision; returning users reuse existing records

---

## Phase 6: User Story 4 - Spatial and Analytical Query Performance (Priority: P3)

**Goal**: Station proximity searches under 300ms; analytics partitioned to prevent query interference.

**Independent Test**: Insert 10,000 stations with random coordinates. Query nearby stations with ST_DWithin; verify response time <300ms. Run concurrent analytics INSERT; verify search remains under 300ms.

- [X] T022 [P] [US4] Add spatial index migration (GIST on inventory.station.location) in existing migration or new migration
- [X] T023 [US4] Implement partition management logic in `crates/common-db/src/partition.rs` (create next day's partition daily)
- [X] T024 [US4] Write spatial query performance test in `services/admin-service/tests/queries/spatial_performance_test.rs`

**Checkpoint**: Spatial queries under 300ms; analytics partitions created automatically

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Tooling, documentation, and final verification

- [X] T025 [P] Create `scripts/db/migrate.sh` wrapper for sqlx migrate run
- [X] T026 [P] Create `scripts/db/rollback.sh` wrapper for sqlx migrate revert
- [X] T027 Run quickstart.md end-to-end validation
- [X] T028 Verify all success criteria (SC-001 through SC-007) are met

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user story phases
- **US3 - Migration System (Phase 3)**: Depends on Foundational — technical prerequisite for US1
- **US1 - Schema & Ownership (Phase 4)**: Depends on US3 (needs migration system to create tables)
- **US2 - Keycloak Identity (Phase 5)**: Depends on US1 (needs `users.user_account` table to exist)
- **US4 - Spatial/Analytics (Phase 6)**: Depends on US1 (needs `inventory.station` and `analytics.raw_event` tables)
- **Polish (Phase 7)**: Depends on all user story phases

### User Story Dependencies

- **US3 (Migration System - P2)**: Foundational → US3 — first story to implement (despite P2 priority, it is a technical prerequisite)
- **US1 (Schema & Ownership - P1)**: US3 → US1 — requires migration system
- **US2 (Keycloak Identity - P1)**: US1 → US2 — requires user_account table
- **US4 (Spatial/Analytics - P3)**: US1 → US4 — requires station and raw_event tables

### Within Each Phase

- Models before services
- Core implementation before integration
- Phase complete before moving to next

### Parallel Opportunities

- All [P] tasks within a phase can run in parallel
- Setup Phase: T002, T003 are parallel
- US1 Phase: T013, T014, T015, T016 are parallel (different SQL files)
- US2 Phase: T019 is parallel (independent DB query module)
- US4 Phase: T022 is parallel (index only)
- Polish Phase: T025, T026 are parallel

---

## Parallel Example: User Story 1

```bash
# Launch all table creation migrations in parallel:
Task: "Create users tables migration"
Task: "Create inventory tables migration"
Task: "Create gis tables migration"
Task: "Create analytics tables migration"
```

---

## Implementation Strategy

### MVP First (User Story 3 + User Story 1)

1. Complete Phase 1: Setup (crate deps, init script)
2. Complete Phase 2: Foundational (common-db pool + migration runner)
3. Complete Phase 3: Migration System (extensions, schemas, roles)
4. Complete Phase 4: Schema & Ownership (tables, grants, permissions)
5. **STOP and VALIDATE**: Four schemas exist, ownership enforced, migration system works
6. Deploy/demo if ready

### Incremental Delivery

1. Setup + Foundational → Database connectivity ready
2. Add US3 (Migration System) → Migration infrastructure operational
3. Add US1 (Schema & Ownership) → **MVP achieved** — four schemas with ownership
4. Add US2 (Keycloak Identity) → User auto-provisioning functional
5. Add US4 (Spatial/Analytics) → Performance optimization complete
6. Polish → Production-ready tooling

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Developer A: US3 (Migration System) + US1 (Schema & Ownership)
3. Developer B: US2 (Keycloak Identity) — starts after US1 tables available
4. Developer C: US4 (Spatial/Analytics) — starts after US1 tables available
5. Polish tasks distributed at end

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability (US1-US4)
- Each user story is independently completable and testable
- US3 is listed first despite P2 priority because it is a technical prerequisite for US1
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
