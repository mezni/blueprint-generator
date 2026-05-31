---

description: "Task list for Database Persistence & Spatial Query Engine"
---

# Tasks: Database Persistence & Spatial Query Engine

**Input**: Design documents from `specs/003-database-persistence/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not explicitly requested — backend integration tests run against live PostGIS in CI; no dedicated test tasks generated.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `backend/api-service/src/`, `backend/infra/src/`, `backend/db/`
- **Frontend**: `apps/mobile-driver/src/`
- **Infrastructure**: `deployments/`, `.github/workflows/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for the database layer

- [X] T001 Create `backend/db/migrations/` directory structure
- [X] T002 [P] Create `backend/db/seeds/` directory structure
- [X] T003 [P] Create `backend/db/osm/` directory structure
- [X] T004 [P] Create `backend/infra/` crate with `Cargo.toml` and `src/lib.rs`
- [X] T005 [P] Create `deployments/docker-compose.yml` with PostGIS 15-3.3 service

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 Create `backend/db/migrations/20260528000000_init_spatial_schema.sql` with partners, stations, chargers tables, enums, indexes, and PostGIS extension
- [X] T007 [P] Update `backend/api-service/Cargo.toml` to add `sqlx` (postgres, runtime-tokio, tls-rustls), `actix-cors`, `env_logger` dependencies
- [X] T008 [P] Update `backend/Cargo.toml` workspace members to include `infra`
- [X] T009 Implement `backend/infra/src/lib.rs` with `join_database_pool()` async function using sqlx::PgPool
- [X] T010 Rewrite `backend/api-service/src/main.rs` to initialize PgPool from `DATABASE_URL`, add CORS middleware, add health check endpoint (`GET /health`) (FR-013), add request logging via env_logger (FR-014), pass `AppState { db: PgPool }` to routes
- [X] T011 Update `backend/api-service/src/domains/locate/model.rs` with `PartnerSnapshot`, `Station` (with `partner`, `is_live`, `updated_at` fields), and `Charger` structs matching the data contract
- [X] T012 Rewrite `backend/api-service/src/domains/locate/routes.rs` with sqlx spatial query (`ST_DWithin` on `GEOGRAPHY`), `show_staged` parameter, partner JOIN, and nested charger fetching
- [X] T013 Update `backend/api-service/src/domains/locate/mod.rs` to expose `init_routes` function (used by main.rs)
- [X] T026 Implement `PATCH /api/v1/stations/{id}/status` endpoint in `backend/api-service/src/domains/locate/routes.rs` that updates `is_live` flag and returns updated station (FR-012)

**Checkpoint**: Foundation ready — `cargo check` passes, backend compiles with sqlx, connects to PostGIS, serves stations from database, supports status updates

---

## Phase 3: User Story 1 - Mobile Driver Sees Nearby Charging Stations (Priority: P1) 🎯 MVP

**Goal**: A driver opens the app and sees stations plotted on the map with correct status colors, `is_live` filtering, and partner ownership in the detail card

**Independent Test**: Deploy PostGIS, run migrations, seed data, start backend, open app — verify markers appear with correct colors and StationCard shows partner info + STAGED badge for non-live stations

### Implementation for User Story 1

- [X] T014 [P] [US1] Create `backend/db/seeds/demo_data.sql` with 5 partners and 50 stations across 5 Tunisian regions
- [X] T015 [P] [US1] Update `apps/mobile-driver/src/services/api.js` to pass `lat`, `lng`, and `show_staged` parameters to the API endpoint. For development mode, pass `show_staged=true` so all 50 seeded stations (all `is_live=false`) are visible on the map; in production the caller omits `show_staged` (defaults to `false`, hiding non-live stations per acceptance scenario 3)
- [X] T016 [P] [US1] Update `apps/mobile-driver/src/components/StationCard.js` to display `station.partner.name`, `station.partner.type`, and a "STAGED TESTING" badge when `station.is_live` is false
- [X] T017 [P] [US1] Update `apps/mobile-driver/src/components/MapView.web.js` to color markers orange when `station.is_live` is false (Leaflet divIcon)
- [X] T018 [P] [US1] Update `apps/mobile-driver/src/components/MapView.native.js` to color markers orange when `station.is_live` is false (pinColor)

**Checkpoint**: At this point, User Story 1 should be fully functional — open app, see 50 stations with correct colors and detail cards

---

## Phase 4: User Story 2 - Backend Operator Seeds Test Data (Priority: P2)

**Goal**: An operator can run a seeding script to populate the database with representative test data

**Independent Test**: Run migration + seed against empty database, then `curl /api/v1/stations/nearby?lat=36.8065&lng=10.1815&show_staged=true` returns exactly 50 stations

### Implementation for User Story 2

- [X] T019 [P] [US2] Verify `backend/db/seeds/demo_data.sql` produces 5 partners, 50 stations, 50 chargers with correct ID patterns (`prt-`, `stn-`, `chg-` + 8 hex chars)
- [X] T020 [P] [US2] Write ad-hoc SQL validation query for seed data correctness (counts, ID patterns, coordinate ranges)

**Checkpoint**: Seed script produces deterministic, validatable data for CI and local dev

---

## Phase 5: User Story 3 - OSM Importer Enriches Station Data (Priority: P3)

**Goal**: An operator runs an OSM import script that extracts real-world charging station nodes from Tunisia's OSM data

**Independent Test**: Run the import against a fresh database, then verify charging stations from OSM appear in API results

### Implementation for User Story 3

- [X] T021 [P] [US3] Create `backend/db/osm/ev_filter.lua` with osm2pgsql Lua filter that inserts nodes tagged `amenity=charging_station` into the stations table
- [X] T022 [P] [US3] Create `backend/db/osm/import_tunisia.sh` that downloads Tunisia OSM PBF and runs osm2pgsql with the Lua filter

**Checkpoint**: OSM importer runs successfully and supplements the seed data with real-world stations

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T023 [P] Update `.github/workflows/ci.yml` to add PostGIS service container, set `DATABASE_URL` env, run migrations + seed in CI
- [X] T027 [P] Add ad-hoc response time verification for SC-002: curl the nearby endpoint with a known location and verify response <500ms (e.g. `time curl ... | jq '.timing'` or a simple bash timer)
- [X] T024 [P] Update `AGENTS.md` SPECKIT reference to point to `specs/003-database-persistence/plan.md`
- [X] T025 Run quickstart.md validation steps: start DB, migrate, seed, start backend, curl endpoint

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational — CAN start immediately after
- **User Story 2 (Phase 4)**: Depends on Foundational — seed SQL can be created independently
- **User Story 3 (Phase 5)**: Depends on Foundational — OSM scripts are independent
- **Polish (Phase 6)**: Depends on all prior phases — CI update needs all code in place

### User Story Dependencies

- **User Story 1 (P1)**: After Foundational — frontend depends on backend API being functional
- **User Story 2 (P2)**: After Foundational — seed is independent of frontend changes
- **User Story 3 (P3)**: After Foundational — OSM scripts are independent of US1/US2

### Within Each User Story

- Models before services
- Backend before frontend integration
- Story complete before moving to next priority

### Parallel Opportunities

- T002, T003, T004, T005 in Phase 1 can run in parallel
- T007, T008 in Phase 2 can run in parallel
- T015, T016, T017, T018 can run in parallel (different frontend files)
- T019, T020 can run in parallel
- T021, T022 can run in parallel
- T023, T024 in Phase 6 can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all frontend updates together:
Task: "Update api.js to pass lat/lng/show_staged params"
Task: "Update StationCard.js with partner info and staged badge"
Task: "Update MapView.web.js with orange markers"
Task: "Update MapView.native.js with orange markers"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Start PostGIS, migrate, seed, start backend, open app — verify 50 stations on map
5. Deploy/demo if ready

### Incremental Delivery

1. Setup + Foundational → Backend serves stations from PostGIS
2. Add User Story 1 → Frontend shows stations from DB → **MVP!**
3. Add User Story 2 → Seed data quality verified
4. Add User Story 3 → OSM real-world data import
5. Each story adds value without breaking previous stories
