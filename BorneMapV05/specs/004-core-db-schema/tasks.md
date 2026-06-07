# Tasks: Core Database Schema

**Input**: Design documents from `/specs/004-core-db-schema/`

**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Not explicitly requested as a formal test suite. The smoke test script (T024) and verification tasks (T025-T027) provide SQL-based integration verification covering constraint enforcement, index usage, trigger behavior, and partition routing — satisfying Constitution VII verification requirements for a database-migration sprint.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Migrations (platform_db)**: `services/admin-service/migrations/`
- **Migrations (analytics_db)**: `services/analytics-writer/migrations/`
- **Verification scripts**: `services/admin-service/migrations/`
- All migration files use `sqlx-cli` naming: `NNNN_name.up.sql` / `NNNN_name.down.sql`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create migration directory structure and enable PostGIS extension

- [x] T001 Create migration directories for admin-service at `services/admin-service/migrations/`
- [x] T002 [P] Create migration directories for analytics-writer at `services/analytics-writer/migrations/`
- [x] T003 Create PostGIS extension migration in `services/admin-service/migrations/0000_enable_postgis.up.sql` and `0000_enable_postgis.down.sql`

**Checkpoint**: Migration directories exist; PostGIS extension can be enabled on platform_db

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Create inventory schema migration in `services/admin-service/migrations/0001_create_inventory_schema.up.sql` and `0001_create_inventory_schema.down.sql`
- [x] T005 [P] Create users schema migration in `services/admin-service/migrations/0007_create_users_schema.up.sql` and `0007_create_users_schema.down.sql`
- [x] T006 [P] Create gis schema migration in `services/admin-service/migrations/0013_create_gis_schema.up.sql` and `0013_create_gis_schema.down.sql`

**Checkpoint**: All three schemas (inventory, users, gis) exist in platform_db

---

## Phase 3: User Story 1 - Schema Provisioning for Business Data (Priority: P1) 🎯 MVP

**Goal**: Create all inventory tables (partner, station, charger, station_availability) with full columns, types, constraints, indexes, and CHECK rules

**Independent Test**: Run migrations against fresh platform_db, verify `\dt inventory.*` shows all 4 tables with correct columns via `\d+ inventory.partner`, `\d+ inventory.station`, etc. Insert a station with lat/lng and confirm GIST index exists on geom.

### Implementation for User Story 1

- [x] T007 [US1] Create inventory.partner table migration in `services/admin-service/migrations/0002_create_inventory_partner.up.sql` and `0002_create_inventory_partner.down.sql` — columns: id (PRT-ULID PK), name, type (CHECK), status (CHECK), audit fields, deleted_at
- [x] T008 [US1] Create inventory.station table migration in `services/admin-service/migrations/0003_create_inventory_station.up.sql` and `0003_create_inventory_station.down.sql` — columns: id (STN-ULID PK), partner_id FK, name, description, latitude (CHECK), longitude (CHECK), geom (GEOGRAPHY Point 4326), status (CHECK), is_live, is_public, city, country, audit fields, deleted_at; indexes: GIST(geom), BTREE(partner_id), BTREE(status), BTREE(is_live, is_public), BTREE(city)
- [x] T009 [US1] Create inventory.charger table migration in `services/admin-service/migrations/0004_create_inventory_charger.up.sql` and `0004_create_inventory_charger.down.sql` — columns: id (CHG-ULID PK), station_id FK, type (CHECK), power_kw, status (CHECK), audit fields, deleted_at; indexes: BTREE(station_id), BTREE(status)
- [x] T010 [US1] Create inventory.station_availability table migration in `services/admin-service/migrations/0005_create_inventory_availability.up.sql` and `0005_create_inventory_availability.down.sql` — columns: id PK, station_id FK, status (CHECK), source (CHECK), updated_at; index: BTREE(station_id)

**Checkpoint**: Inventory schema fully provisioned — partner, station, charger, station_availability tables exist with all constraints, CHECK rules, and indexes. FK from station → partner enforced. GIST index on station.geom exists.

---

## Phase 4: User Story 2 - Identity & User Data Structures (Priority: P1)

**Goal**: Create all users tables (user_account, user_profile, partner_membership, favorite_station, station_review) with cross-schema FKs to inventory, uniqueness constraints, and CHECK rules

**Independent Test**: Insert a user_account, attempt duplicate partner_membership for same user (UNIQUE rejects), attempt duplicate review for same user+station (UNIQUE rejects). Verify FK from partner_membership → inventory.partner works.

### Implementation for User Story 2

- [x] T011 [US2] Create users.user_account table migration in `services/admin-service/migrations/0008_create_users_user_account.up.sql` and `0008_create_users_user_account.down.sql` — columns: id (USR-ULID PK), keycloak_user_id (UNIQUE), email, status (CHECK), created_at, last_login_at; index: UNIQUE(keycloak_user_id)
- [x] T012 [P] [US2] Create users.user_profile table migration in `services/admin-service/migrations/0009_create_users_user_profile.up.sql` and `0009_create_users_user_profile.down.sql` — columns: user_id PK FK → user_account, display_name, avatar_url, preferred_language, preferences JSONB
- [x] T013 [US2] Create users.partner_membership table migration in `services/admin-service/migrations/0010_create_users_partner_membership.up.sql` and `0010_create_users_partner_membership.down.sql` — columns: user_id PK FK → user_account (UNIQUE), partner_id FK → inventory.partner, role (CHECK); constraint: UNIQUE(user_id) enforces 1:1
- [x] T014 [P] [US2] Create users.favorite_station table migration in `services/admin-service/migrations/0011_create_users_favorite_station.up.sql` and `0011_create_users_favorite_station.down.sql` — columns: user_id FK → user_account, station_id FK → inventory.station, created_at; PK: (user_id, station_id) composite
- [x] T015 [US2] Create users.station_review table migration in `services/admin-service/migrations/0012_create_users_station_review.up.sql` and `0012_create_users_station_review.down.sql` — columns: id (REV-ULID PK), user_id FK → user_account, station_id FK → inventory.station, rating (CHECK 1-5), comment, status (CHECK), created_at, updated_at; constraint: UNIQUE(user_id, station_id); indexes: BTREE(station_id), BTREE(user_id)

**Checkpoint**: Users schema fully provisioned — all 5 tables exist with cross-schema FKs to inventory, uniqueness constraints, and CHECK rules. 1:1 partner_membership enforced. One review per user per station enforced.

---

## Phase 5: User Story 3 - GIS Outbox & Spatial Indexing (Priority: P1)

**Goal**: Create gis.sync_queue outbox table, station geom trigger, partner delete guard trigger, and visible_stations view. Verify spatial queries use GIST index.

**Independent Test**: Insert a gis.sync_queue row with valid operation/status. Insert a station and verify geom is auto-populated. Attempt partner soft-delete with active stations (trigger blocks). Query visible_stations view and verify only visible stations returned. Run EXPLAIN on bbox query (shows GIST index scan).

### Implementation for User Story 3

- [x] T016 [US3] Create gis.sync_queue table migration in `services/admin-service/migrations/0014_create_gis_sync_queue.up.sql` and `0014_create_gis_sync_queue.down.sql` — columns: id PK, entity_type (CHECK), entity_id, operation (CHECK), payload JSONB, status (CHECK), created_at, processed_at; indexes: BTREE(status), BTREE(entity_type, entity_id)
- [x] T017 [US3] Create triggers migration in `services/admin-service/migrations/0015_create_triggers.up.sql` and `0015_create_triggers.down.sql` — two triggers: (1) trg_station_geom: BEFORE INSERT OR UPDATE on inventory.station, sets geom from lat/lng, NULL if lat/lng NULL; (2) trg_partner_delete_guard: BEFORE UPDATE on inventory.partner, blocks deleted_at if active stations exist (RAISE EXCEPTION ACTIVE_STATIONS_EXIST)
- [x] T018 [US3] Create visible_stations view migration in `services/admin-service/migrations/0006_create_inventory_visible_stations_view.up.sql` and `0006_create_inventory_visible_stations_view.down.sql` — VIEW filters inventory.station WHERE is_live=true AND deleted_at IS NULL AND status='active' AND is_public=true

**Checkpoint**: GIS outbox table exists with CHECK constraints. Station geom auto-populates on INSERT/UPDATE. Partner delete guard trigger blocks soft-delete with active stations. visible_stations view returns only live, active, public, non-deleted stations. EXPLAIN on spatial query shows GIST index scan.

---

## Phase 6: User Story 4 - Analytics Schema Stub (Priority: P2)

**Goal**: Create analytics schema in analytics_db with partitioned raw_event table (12 monthly partitions + default) and event_dead_letter table

**Independent Test**: Run analytics migrations against analytics_db. Verify raw_event is partitioned. Insert an event for June 2026 and confirm it routes to raw_event_2026_06. Verify event_dead_letter table exists.

### Implementation for User Story 4

- [x] T019 [US4] Create analytics schema migration in `services/analytics-writer/migrations/0001_create_analytics_schema.up.sql` and `0001_create_analytics_schema.down.sql`
- [x] T020 [US4] Create raw_event partitioned table migration in `services/analytics-writer/migrations/0002_create_raw_event.up.sql` and `0002_create_raw_event.down.sql` — columns: event_id, event_name, session_id, user_id (nullable), anonymous_id, actor_role, occurred_at, ingested_at, path, payload JSONB, metadata JSONB; PARTITION BY RANGE (occurred_at)
- [x] T021 [US4] Create raw_event monthly partitions migration in `services/analytics-writer/migrations/0003_create_raw_event_partitions.up.sql` and `0003_create_raw_event_partitions.down.sql` — partitions: raw_event_2026_01 through raw_event_2026_12 + raw_event_default; indexes per partition: BTREE(event_name, occurred_at), BTREE(user_id), BTREE(session_id)
- [x] T022 [US4] Create event_dead_letter table migration in `services/analytics-writer/migrations/0004_create_event_dead_letter.up.sql` and `0004_create_event_dead_letter.down.sql` — columns: id PK, event_id, event_name, error_code, error_message, raw_payload JSONB, created_at

**Checkpoint**: Analytics schema in analytics_db is complete. raw_event has 12 monthly partitions + default. Event insertion routes to correct partition. event_dead_letter table exists.

---

## Phase 7: User Story 5 - Seed Data & Smoke Testing (Priority: P2)

**Goal**: Create idempotent seed data with realistic Tunisian stations and a spatial query smoke test that verifies PostGIS, GIST indexing, and data correctness end-to-end

**Independent Test**: Run seed migration, verify sample data is queryable, run bbox query on Tunis and confirm stations returned with correct distance_km, verify all FK relationships valid.

### Implementation for User Story 5

- [x] T023 [US5] Create seed data migration in `services/admin-service/migrations/0016_seed_data.up.sql` and `0016_seed_data.down.sql` — idempotent (IF NOT EXISTS / conflict handling); includes a ULID generation helper function (PL/pgSQL using `gen_random_bytes()`); seed data: 3 partners (business/private), 10 stations in Tunisia (Tunis, Sfax, Sousse areas with lat/lng), 15 chargers (mix of CCS/Type2/CHAdeMO), 5 user accounts, 3 partner memberships, 10 favorites, 8 reviews; all IDs use ULID+prefix format
- [x] T024 [US5] Create smoke test verification script in `services/admin-service/migrations/0017_smoke_test.sql` — verifies: (1) all schemas and tables exist, (2) PostGIS enabled, (3) GIST index on station.geom, (4) visible_stations view returns correct rows, (5) bbox query on Tunis returns stations using GIST index scan, (6) FK constraints enforced (reject orphan station), (7) CHECK constraints reject invalid data, (8) partner delete guard trigger works, (9) geom trigger auto-populates, (10) raw_event partition routing works

**Checkpoint**: Seed data loads cleanly and idempotently. Smoke test passes all verifications. Platform_db and analytics_db are fully provisioned and verified.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and documentation

- [x] T025 Verify migration idempotency and performance by running `sqlx migrate run` twice against clean databases in `platform_db` and `analytics_db`; confirm total migration time is under 30 seconds (SC-001)
- [x] T026 [P] Verify all down migrations work by running `sqlx migrate revert` for each migration in both `services/admin-service/migrations/` and `services/analytics-writer/migrations/`
- [x] T027 Run quickstart.md validation end-to-end: start PostgreSQL, run migrations, verify schemas, run smoke test

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 completion — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2 (inventory schema must exist)
- **US2 (Phase 4)**: Depends on US1 (cross-schema FKs to inventory.partner and inventory.station)
- **US3 (Phase 5)**: Depends on US1 (triggers operate on inventory.station and inventory.partner)
- **US4 (Phase 6)**: Depends on Phase 1 only (independent database) — CAN run in parallel with US1/US2/US3
- **US5 (Phase 7)**: Depends on US1 + US2 + US3 (all schemas + triggers must exist)
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1)**: Depends on Foundational. No story dependencies.
- **US2 (P1)**: Depends on US1 (cross-schema FKs to inventory tables).
- **US3 (P1)**: Depends on US1 (triggers on inventory tables).
- **US4 (P2)**: Independent of US1/US2/US3 (different database). Can start after Phase 1.
- **US5 (P2)**: Depends on US1 + US2 + US3 (seed needs all tables; smoke test needs triggers and view).

### Within Each User Story

- Migrations must be written in sequential order (NNNN numbering)
- Each .up.sql must have a corresponding .down.sql
- Idempotent patterns (IF NOT EXISTS) applied throughout

### Parallel Opportunities

- T001 and T002 can run in parallel (different service directories)
- T004, T005, T006 can run in parallel (different schema creations, different migration files)
- Within US1, T007→T008→T009→T010 must be sequential (FK dependencies: charger→station, station→partner)
- T012 and T014 can run in parallel within US2 (different tables, no interdependencies)
- US4 (Phase 6) can run entirely in parallel with US1/US2/US3 (different database)

---

## Parallel Example: User Story 1

```text
# Sequential within US1 due to FK dependencies:
T007 → T008 → T009 → T010

# partner must exist before station (FK)
# station must exist before charger (FK)  
# station must exist before station_availability (FK)
```

## Parallel Example: User Story 2

```text
# Sequential with some parallelism:
T011 → T012, T013, T014, T015

# user_account must exist first (all other tables reference it)
# T012 (user_profile) and T014 (favorite_station) can run in parallel
# T013 (partner_membership) needs inventory.partner from US1
# T015 (station_review) needs inventory.station from US1
```

## Parallel Example: Cross-Story

```text
# US4 can start immediately after Phase 1 (independent database):
Developer A: US1 → US2 → US3 → US5
Developer B: US4 (analytics_db — fully independent)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (migration dirs + PostGIS)
2. Complete Phase 2: Foundational (3 schemas)
3. Complete Phase 3: User Story 1 (inventory tables)
4. **STOP and VALIDATE**: Verify inventory schema via `\d+ inventory.*`
5. If time-constrained, this delivers the minimum viable data layer

### Incremental Delivery

1. Setup + Foundational → Schemas exist
2. Add US1 → Inventory tables + indexes → MVP (station data layer)
3. Add US2 → User/identity tables → User data layer complete
4. Add US3 → GIS outbox + triggers + view → Spatial layer operational
5. Add US4 → Analytics stub → Analytics pipeline ready for Sprint 14
6. Add US5 → Seed + smoke test → Full verification
7. Polish → Idempotency and down-migration verification

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 → US2 → US3 → US5 (platform_db path)
   - Developer B: US4 (analytics_db — fully independent)
3. US5 can start once Developer A finishes US1+US2+US3

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Migration numbering follows plan.md convention (0001-0017 for platform_db, 0001-0004 for analytics_db)
- All migrations use IF NOT EXISTS / DO $$ EXCEPTION patterns for idempotency (FR-017)
- The visible_stations view migration is numbered 0006 (after station table at 0003 but before users schema at 0007) — it logically belongs with inventory
- Note: sqlx-cli executes migrations by filename number, not by task phase. T018 (view, migration 0006) is organized in Phase 5 (US3) but will execute immediately after T008 (station, migration 0003) when `sqlx migrate run` is invoked. This is intentional — the view depends on the station table but is conceptually part of the spatial indexing story
- Commit after each logical task group using conventional commits (`feat:`, `chore:`)
