# Tasks: GIS Sync System v1

**Input**: Design documents from `/specs/006-gis-sync-v1/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: No explicit test tasks requested — tests are included inline within implementation tasks where appropriate.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust service**: `services/gis-worker/src/`
- **Migrations**: `services/gis-worker/migrations/`
- **Env config**: `infra/env/gis-worker.env.example`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency setup for the gis-worker

- [x] T001 Add `sqlx` with postgres/chrono features and `chrono` to `services/gis-worker/Cargo.toml`
- [x] T002 [P] Create `services/gis-worker/src/config.rs` with env-var config struct (poll interval, batch size, retry params, feature flags)
- [x] T003 [P] Create `services/gis-worker/src/db.rs` with PgPool factory and migration runner using `common-db`
- [x] T004 [P] Create `services/gis-worker/src/error.rs` with WorkerError enum (InvalidCoordinates, StationNotFound, DbError, Unknown)
- [x] T005 [P] Create `services/gis-worker/src/models.rs` with GisQueueEntry struct and OSM table row types
- [x] T006 Extend `infra/env/gis-worker.env.example` with all new env vars (GIS_WORKER_MAX_RETRIES, GIS_WORKER_RETRY_BASE_DELAY_MS, GIS_WORKER_STALE_PROCESSING_TIMEOUT_MS, FF_ENABLE_GIS_SYNC)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core worker infrastructure that MUST be complete before any user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 Create migration `services/gis-worker/migrations/0001_create_gis_osm_tables.up.sql` — create `gis.osm_roads`, `gis.osm_admin_boundaries`, `gis.osm_pois` tables with GIST indexes
- [x] T008 Create migration `services/gis-worker/migrations/0001_create_gis_osm_tables.down.sql`
- [x] T009 [P] Implement `services/gis-worker/src/health.rs` — move existing `/health` handler into its own module
- [x] T010 Refactor `services/gis-worker/src/main.rs` — bootstrap DB pool from config, register health route, implement graceful shutdown with tokio signal

**Checkpoint**: Foundation ready — worker compiles, starts, connects to DB, serves `/health`

---

## Phase 3: User Story 1 - Station Geometry Auto-Syncs on Mutation (Priority: P1) 🎯 MVP

**Goal**: The gis-worker polls `gis.sync_queue` for pending rows, computes station geometry from lat/lng, and transitions rows to `done`

**Independent Test**: Create a station via the partner API, wait for the GIS worker poll cycle, then verify the station's `geom` column is populated with a valid Point(4326) geometry

### Implementation for User Story 1

- [x] T011 [US1] Implement `services/gis-worker/src/geometry.rs` — function to compute `geom = ST_SetSRID(ST_MakePoint(lng, lat), 4326)` via sqlx query
- [x] T012 [US1] Implement `services/gis-worker/src/worker.rs` — main poll loop: fetch pending batch, claim rows (atomically set `processing`), process in parallel via `tokio::join_all`, transition to `done` on success
- [x] T013 [US1] Integrate worker loop into `services/gis-worker/src/main.rs` — spawn worker task on main thread alongside health endpoint, respect `FF_ENABLE_GIS_SYNC` flag

**Checkpoint**: At this point, a station created via the partner API gets its `geom` populated within one poll cycle

---

## Phase 4: User Story 2 - Idempotent and Replay-Safe Processing (Priority: P1)

**Goal**: Replaying the same outbox row produces identical station geometry; stale `processing` rows are recovered on startup

**Independent Test**: Process an outbox row, reset it to `pending`, process again — station geometry is identical after both runs. Crash mid-processing, restart worker — stale `processing` rows are recovered.

### Implementation for User Story 2

- [x] T014 [US2] Add stale row recovery to `services/gis-worker/src/worker.rs` — on startup, reset `processing` rows older than `GIS_WORKER_STALE_PROCESSING_TIMEOUT_MS` back to `pending`
- [x] T015 [US2] Ensure `services/gis-worker/src/geometry.rs` idempotency — UPDATE with same lat/lng always produces identical `ST_AsGeoJSON(geom)` output; delete sets `geom = NULL` (naturally idempotent)
- [x] T016 [US2] Add state transition logging to `services/gis-worker/src/worker.rs` — log row_id, entity_id, operation, old_status→new_status for every transition per FR-012

**Checkpoint**: Replaying the same outbox row twice yields identical geometry; worker survives crash recovery

---

## Phase 5: User Story 3 - Failed Processing with Retry and Dead-Letter (Priority: P2)

**Goal**: Transient errors trigger retry with exponential backoff; max retries sends row to `dead_letter` for manual inspection

**Independent Test**: Submit an outbox row with invalid lat/lng, observe the worker retry cycle, then verify the row lands in `dead_letter` after exhausting retries

### Implementation for User Story 3

- [x] T017 [P] [US3] Implement `services/gis-worker/src/retry.rs` — exponential backoff with jitter: `base_delay * 2^attempt + random(0, base_delay)`, capped at `GIS_WORKER_MAX_RETRIES`
- [x] T018 [US3] Add coordinate validation to `services/gis-worker/src/geometry.rs` — validate lat ∈ [-90, 90], lng ∈ [-180, 180]; return `InvalidCoordinates` error if out of range
- [x] T019 [US3] Integrate retry/backoff into `services/gis-worker/src/worker.rs` — on transient error (DbError), transition to `failed` and schedule retry; after max retries, transition to `dead_letter`; on fatal error (InvalidCoordinates, StationNotFound), transition directly to `dead_letter`

**Checkpoint**: Invalid-coordinate rows reach `dead_letter` after max retries; transient DB errors are retried with backoff

---

## Phase 6: User Story 4 - Basic OSM Tunisia Base Layer (Priority: P3)

**Goal**: A one-time CLI script downloads Tunisia OSM data from Geofabrik and imports into `gis.osm_*` tables

**Independent Test**: After OSM import, run a bbox query over Tunis and verify that roads and administrative boundaries are returned from the `gis` schema

### Implementation for User Story 4

- [x] T020 [US4] Implement `services/gis-worker/src/osm_import.rs` — CLI binary that downloads Geofabrik PBF (`https://download.geofabrik.de/africa/tunisia-latest.osm.pbf`), invokes `osm2pgsql` or `ogr2ogr` to import into `gis.osm_roads`, `gis.osm_admin_boundaries`, `gis.osm_pois` tables
- [x] T021 [US4] Add `[[bin]]` entry to `services/gis-worker/Cargo.toml` for the OSM import CLI binary
- [ ] T022 [US4] Add Dockerfile or docker-compose step to run OSM import on first deploy (manual — run `cargo run --bin osm-import`)

**Checkpoint**: OSM import runs successfully and Tunisia spatial data is queryable

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T023 [P] Add `GIS_WORKER_CONCURRENCY` env var support to `services/gis-worker/src/config.rs` for future configurable parallelism
- [ ] T024 Add roundtrip integration test — insert outbox row directly, run worker poll, verify station.geom is populated (requires PostGIS DB)
- [x] T025 Update Docker Compose healthcheck for gis-worker in `infra/compose/docker-compose.yml` if needed (already correct)
- [x] T026 Run `cargo build` and verify workspace compiles cleanly
- [x] T027 Add `RABBITMQ_QUEUE_GIS_SYNC` env var to `services/gis-worker/src/config.rs` with no-op consumer stub at `services/gis-worker/src/rabbitmq.rs` (logs a warning if configured in v1)
- [x] T028 [P] Add unit tests for geometry computation in `services/gis-worker/src/geometry.rs` (valid coords, NULL coords, boundary values)
- [x] T029 [P] Add unit tests for retry/backoff logic in `services/gis-worker/src/retry.rs` (backoff delay calculation, max retries, jitter)
- [ ] T030 [P] Add unit tests for state transitions in `services/gis-worker/src/worker.rs` (pending→processing→done, processing→failed→dead_letter) (requires DB — deferred)
- [ ] T031 [P] Add unit test for stale processing row recovery in `services/gis-worker/src/worker.rs` (requires DB — deferred)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational — poll loop, geometry computation
- **User Story 2 (Phase 4)**: Depends on US1 (Phase 3) — builds on worker loop for idempotency and stale recovery
- **User Story 3 (Phase 5)**: Depends on US1 (Phase 3) — builds on worker loop for retry/backoff
- **User Story 4 (Phase 6)**: Independent of US1-3 — one-time import script, separate binary
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational — No dependencies on other stories
- **User Story 2 (P1)**: Depends on US1 — extends worker loop with recovery logic
- **User Story 3 (P2)**: Depends on US1 — extends worker loop with retry logic
- **User Story 4 (P3)**: Can start after Foundational — independent of all other stories

### Within Each User Story

- Models before services
- Core logic before integration
- Story complete before moving to next priority
- US1 (MVP) must be fully functional before US2 or US3

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel
- US4 (OSM) can run in parallel with US1-3
- US2 and US3 can share the worker loop but extend different parts

---

## Parallel Example: User Story 1

```bash
# Launch geometry module + worker module in parallel:
Task: "Implement services/gis-worker/src/geometry.rs"
Task: "Implement services/gis-worker/src/worker.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Create a station, verify geom is synced within one poll cycle
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Worker starts, connects to DB, serves health
2. Add User Story 1 → Station geometry syncs → Deploy/Demo (MVP!)
3. Add User Story 2 → Idempotent replay and crash recovery → Deploy/Demo
4. Add User Story 3 → Retry/backoff and dead-letter → Deploy/Demo
5. Add User Story 4 → OSM base layer → Deploy/Demo
