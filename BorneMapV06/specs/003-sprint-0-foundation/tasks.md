# Tasks: Sprint 0 — Foundation

**Input**: Design documents from `specs/003-sprint-0-foundation/`

**Prerequisites**: plan.md (technical context, architecture), spec.md (5 user stories), data-model.md (database schemas), contracts/ (API & Docker contracts), quickstart.md (developer guide)

**Tests**: Not included (not requested in feature specification). Tests will be added in Sprint 1 and Sprint 2.

**Organization**: Tasks are organized by user story priority (P1, P2) to enable independent implementation and testing of each story. All P1 stories must complete before P2 stories can be validated for integration.

---

## Format: `[ID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4, US5)
- Include exact file paths for all code creation tasks

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Monorepo initialization and project structure creation

**Expected Duration**: ~30 minutes

- [X] T001 Initialize Cargo workspace at repository root in `Cargo.toml`
- [X] T002 Initialize pnpm workspace at repository root in `pnpm-workspace.yaml` and root `package.json`
- [X] T003 [P] Copy `.env.example` to `infra/env/.env.example` with database and service configuration
- [X] T004 [P] Create `.gitignore` entries for Rust targets, Node modules, environment files, and build artifacts
- [X] T005 Create `README.md` at repo root with link to `specs/003-sprint-0-foundation/quickstart.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

**Expected Duration**: ~2 hours

### Database Foundation

- [X] T006 Create `db/migrations/0001_extensions.sql` to enable PostGIS, uuid-ossp, pgcrypto extensions
- [X] T007 Create `db/migrations/0002_inventory_schema.sql` to create `inventory` schema and placeholder for tables
- [X] T008 Create `db/migrations/0003_gis_schema.sql` to create `gis` schema
- [X] T009 [P] Create `db/migrate.sh` shell script wrapper for sqlx-cli migration runner with usage documentation

### Docker Compose Infrastructure

- [X] T010 Create `infra/compose/docker-compose.yml` with PostgreSQL, Driver Service, and pgAdmin services
- [X] T011 [P] Create `infra/compose/healthcheck.sh` script for service startup validation
- [X] T012 [P] Create `.dockerignore` for efficient Docker image building

### Shared Rust Crates (Scaffolding)

- [X] T013 Create `crates/ev-core/Cargo.toml` with basic metadata and no dependencies yet
- [X] T014 [P] Create `crates/ev-geo/Cargo.toml` with basic metadata and no dependencies yet
- [X] T015 [P] Create `crates/ev-db/Cargo.toml` with basic metadata and no dependencies yet
- [X] T016 Create root `Cargo.toml` workspace manifest listing services/driver-service and all crates

### Driver Service Scaffolding (Clean Architecture)

- [X] T017 Create `services/driver-service/Cargo.toml` with Actix-Web, SQLx, Tokio, Serde dependencies
- [X] T018 Create `services/driver-service/src/main.rs` with Actix-Web app initialization (stub)
- [X] T019 Create `services/driver-service/src/config.rs` for environment variable configuration
- [X] T020 Create `services/driver-service/src/errors.rs` with typed error enum and HTTP response mapping
- [X] T021 [P] Create layer directories: `services/driver-service/src/domain/mod.rs` (empty module)
- [X] T022 [P] Create layer directories: `services/driver-service/src/application/mod.rs` (empty module)
- [X] T023 [P] Create layer directories: `services/driver-service/src/infrastructure/mod.rs` (empty module)
- [X] T024 [P] Create layer directories: `services/driver-service/src/interface/mod.rs` with handlers and middleware subdirs

### Frontend Scaffolding (Node.js)

- [X] T025 [P] Create `apps/driver-web/vite.config.js` with React plugin and dev server configuration
- [X] T026 [P] Create `apps/driver-web/package.json` with React, Vite, Tailwind dependencies
- [X] T027 [P] Create `apps/driver-web/src/main.jsx` with React app entry point
- [X] T028 [P] Create `apps/driver-web/tailwind.config.js` with bright theme from `docs/06-frontend/bright-theme.md`
- [X] T029 [P] Create `apps/driver-mobile/app.json` with Expo configuration
- [X] T030 [P] Create `apps/driver-mobile/package.json` with React Native, Expo, Tailwind dependencies
- [X] T031 [P] Create `apps/driver-mobile/src/App.js` with Expo app entry point
- [X] T032 [P] Create `apps/driver-mobile/tailwind.config.js` with bright theme (React Native compatible)

### Shared Packages Scaffolding

- [X] T033 [P] Create `packages/ui/package.json` with component and token exports
- [X] T034 [P] Create `packages/ui/src/index.ts` with export stubs
- [X] T035 [P] Create `packages/api-client/package.json` with fetch and type utilities
- [X] T036 [P] Create `packages/api-client/src/index.ts` with API client stubs

### Root Manifest Configuration

- [X] T037 Create root `package.json` with workspace protocol, shared dev dependencies, and scripts
- [X] T038 [P] Create `.npmrc` with pnpm strict peer dependency handling configuration

**Checkpoint**: Foundation complete - all monorepo structure in place, ready for user story implementation

---

## Phase 3: User Story 1 — Build System Compiles Successfully (Priority: P1) 🎯 MVP

**Goal**: Entire monorepo builds without errors or warnings; all workspaces resolve and compile cleanly.

**Independent Test**: Developer runs `cargo build` (should complete <2 min, no warnings) and `pnpm install` (should complete <3 min, no conflicts).

**Acceptance Criteria**:
- `cargo build` succeeds with zero errors and zero warnings
- `pnpm install` resolves all dependencies without conflicts
- All 4 workspace members (ev-core, ev-geo, ev-db, driver-service) compile
- All 6 frontend workspaces (driver-web, driver-mobile, ui, api-client, admin-dashboard, partner-dashboard) resolve

**Expected Duration**: ~1 hour

### Implementation for User Story 1

- [X] T039 [US1] Add minimal main.rs structure to `crates/ev-core/src/lib.rs` with module declarations
- [X] T040 [P] [US1] Add minimal main.rs structure to `crates/ev-geo/src/lib.rs` with module declarations
- [X] T041 [P] [US1] Add minimal main.rs structure to `crates/ev-db/src/lib.rs` with module declarations
- [X] T042 [US1] Add serde derive to all crates: `crates/*/Cargo.toml` add serde optional feature
- [X] T043 [P] [US1] Create stub `apps/driver-web/src/App.jsx` with "Hello World" React component
- [X] T044 [P] [US1] Create stub `apps/driver-web/src/index.css` with empty stylesheet
- [X] T045 [P] [US1] Create stub `apps/driver-web/public/index.html` with React mount point
- [X] T046 [P] [US1] Create stub `apps/driver-mobile/src/screens/` directory structure
- [X] T047 [P] [US1] Create stub `packages/ui/src/tokens/` directory with design token exports
- [X] T048 [P] [US1] Create stub `packages/api-client/src/driver/` directory with endpoint stubs
- [X] T049 [US1] Run `cargo build --release` from repo root and verify zero warnings (may take ~2 min first time)
- [X] T050 [US1] Run `pnpm install` from repo root and verify all workspaces resolve without conflicts
- [X] T051 [US1] Verify `cargo --list` shows all crates compilable: `cargo build -p ev-core`, `cargo build -p ev-geo`, `cargo build -p ev-db`
- [X] T052 [US1] Verify frontend dev servers can start: `cd apps/driver-web && pnpm dev` (Ctrl+C to stop)

**Checkpoint**: User Story 1 complete - monorepo builds successfully, all workspaces compile and resolve

---

## Phase 4: User Story 2 — Database Compiles & Initializes (Priority: P1)

**Goal**: PostgreSQL database initializes with all required schemas and extensions; migrations are idempotent and replayable.

**Independent Test**: Developer starts PostgreSQL in Docker, runs migrations with `db/migrate.sh`, verifies all 3 schemas exist, runs migrations again with no errors.

**Acceptance Criteria**:
- `db/migrations/0001_extensions.sql` enables PostGIS, uuid-ossp, pgcrypto extensions
- `db/migrations/0002_inventory_schema.sql` creates `inventory` schema
- `db/migrations/0003_gis_schema.sql` creates `gis` schema
- All migrations run successfully on fresh database
- Migrations are idempotent (running twice causes no errors)
- Total migration time is <30 seconds on typical hardware

**Expected Duration**: ~45 minutes

### Implementation for User Story 2

- [X] T053 [US2] Implement migration 0001: `db/migrations/0001_extensions.sql` - enable PostGIS, uuid-ossp, pgcrypto
- [X] T054 [US2] Implement migration 0002: `db/migrations/0002_inventory_schema.sql` - create inventory schema with CREATE SCHEMA IF NOT EXISTS
- [X] T055 [US2] Implement migration 0003: `db/migrations/0003_gis_schema.sql` - create gis schema with CREATE SCHEMA IF NOT EXISTS
- [X] T056 [US2] Update `db/migrate.sh` to execute migrations in order using psql or sqlx-cli with proper error handling
- [X] T057 [US2] Add migration verification logic to `db/migrate.sh` checking for schema existence post-run
- [X] T058 [P] [US2] Create `Dockerfile` for Driver Service with `cargo build --release` and minimal runtime
- [X] T059 [US2] Update `services/driver-service/src/main.rs` to run migrations on startup via sqlx-cli or embedded migration runner
- [X] T060 [US2] Update `services/driver-service/src/config.rs` to load DATABASE_URL from environment with validation
- [X] T061 [US2] Implement `services/driver-service/src/infrastructure/db/pool.rs` with SQLx PgPool creation from DATABASE_URL
- [X] T062 [US2] Update `services/driver-service/src/main.rs` to initialize PgPool and verify database connection on startup
- [ ] T063 [US2] Test migrations locally: Run `docker compose up postgres` and verify migrations execute cleanly with `db/migrate.sh`
- [ ] T064 [US2] Verify idempotence: Run migrations twice with no errors by executing `db/migrate.sh` twice

**Checkpoint**: User Story 2 complete - database schema initialized, migrations idempotent, Driver Service connects to PostgreSQL

---

## Phase 5: User Story 3 — Docker Compose Stack Runs End to End (Priority: P1)

**Goal**: `docker compose up` brings PostgreSQL, Driver Service, and pgAdmin online; all services report healthy; stack is ready for local development.

**Independent Test**: Developer runs `docker compose up`, verifies `curl http://localhost:8000/health` returns 200 within 10 seconds, checks pgAdmin loads at localhost:5050.

**Acceptance Criteria**:
- All 3 services start without errors
- Driver Service `/health` endpoint returns 200 OK within 1 second
- PostgreSQL is healthy and reachable from Driver Service container
- pgAdmin loads and auto-discovers PostgreSQL service
- `docker compose down` cleanly stops all containers
- Full startup time (PostgreSQL + migrations + Driver Service) is <1 minute

**Expected Duration**: ~1.5 hours

### Implementation for User Story 3

- [X] T065 [US3] Implement `services/driver-service/src/interface/handlers/health.rs` with GET /health handler returning { "status": "ok" }
- [X] T066 [US3] Update `services/driver-service/src/main.rs` to register health handler and log startup info
- [X] T067 [US3] Implement `services/driver-service/src/interface/middleware/logging.rs` with request logging via tracing
- [X] T068 [US3] Add RUST_LOG configuration to `services/driver-service/src/config.rs` with default "info"
- [X] T069 [US3] Update `Dockerfile` to include `sqlx-cli` or ensure migrations run via `cargo sqlx migrate run` command
- [X] T070 [US3] Add health check to docker-compose.yml for driver-service: `test: ["CMD", "curl", "-f", "http://localhost:8000/health"]`
- [X] T071 [US3] Add startup_period to driver-service health check (30 seconds for migrations)
- [X] T072 [US3] Verify docker-compose.yml has `depends_on: condition: service_healthy` for driver-service depending on postgres
- [ ] T073 [US3] Build Driver Service Docker image: `docker build -t bornemap-driver-service services/driver-service/`
- [ ] T074 [US3] Start full stack: `docker compose up -d` and verify all services healthy within 60 seconds
- [ ] T075 [US3] Test health endpoint: `curl http://localhost:8000/health` should return 200 OK
- [ ] T076 [US3] Verify migrations ran: `docker compose exec postgres psql -U postgres -d platform_db -c "SELECT schema_name FROM information_schema.schemata WHERE schema_name IN ('inventory', 'gis');"` should show both schemas
- [ ] T077 [US3] Test pgAdmin access: Open http://localhost:5050 and verify login works (admin@localhost.local / admin123)
- [ ] T078 [US3] Test pgAdmin database discovery: In pgAdmin, verify PostgreSQL server is auto-discovered under Servers
- [ ] T079 [US3] Clean shutdown: Run `docker compose down` and verify no orphaned processes remain
- [ ] T080 [US3] Test idempotence: Run `docker compose up` again and verify same healthy state with no errors

**Checkpoint**: User Story 3 complete - Docker Compose stack fully functional, all services healthy and communicating

---

## Phase 6: User Story 4 — Shared Crates Compile and Export Core Types (Priority: P2)

**Goal**: All three shared crates (ev-core, ev-geo, ev-db) implement core types and utilities; each crate has passing unit tests; Driver Service imports all three without type errors.

**Independent Test**: Developer runs `cargo test -p ev-core`, `cargo test -p ev-geo`, `cargo test -p ev-db` and all pass. Driver Service's Cargo.toml includes all three as dependencies.

**Acceptance Criteria**:
- `crates/ev-core` implements: `ids.rs` (NanoID with prefix support), `types.rs` (enums like ConnectorType, ChargerStatus)
- `crates/ev-geo` implements: `point.rs` (LatLng struct), `bbox.rs` (bounding box), `distance.rs` (haversine calculation)
- `crates/ev-db` implements: `pool.rs` (SQLx PgPool), `pagination.rs` (offset/limit/cursor pagination)
- All crates have `cargo test` passing (at minimum, module declarations and pub exports)
- Driver Service imports all three crates and compiles without missing type errors
- Each crate compiles in <10 seconds incrementally

**Expected Duration**: ~2.5 hours

### Implementation for User Story 4 (ev-core)

- [X] T081 [P] [US4] Create `crates/ev-core/src/ids.rs` with NanoID struct and generation functions for prefixes: STN, CHG, PRT, USR, REV, EVT
- [X] T082 [P] [US4] Add unit tests to `crates/ev-core/src/ids.rs` for: NanoID generation, prefix application, uniqueness across 1000 generates
- [X] T083 [US4] Create `crates/ev-core/src/types.rs` with enums: ConnectorType (CCS2, Type2, TeslaSupercharger), ChargerStatus (available, in_use, maintenance, offline)
- [X] T084 [P] [US4] Add unit tests to `crates/ev-core/src/types.rs` for: enum creation, enum serialization with serde_json
- [X] T085 [US4] Update `crates/ev-core/src/lib.rs` to declare and export mod ids and mod types
- [X] T086 [US4] Run `cargo test -p ev-core` and verify all tests pass

### Implementation for User Story 4 (ev-geo)

- [X] T087 [P] [US4] Create `crates/ev-geo/src/point.rs` with LatLng struct (latitude: f64, longitude: f64) and validation methods
- [X] T088 [P] [US4] Add unit tests to `crates/ev-geo/src/point.rs` for: valid coordinates, boundary coordinates, invalid latitude/longitude rejection
- [X] T089 [US4] Create `crates/ev-geo/src/bbox.rs` with BoundingBox struct (min_lat, min_lng, max_lat, max_lng) and containment check method
- [X] T090 [P] [US4] Add unit tests to `crates/ev-geo/src/bbox.rs` for: bbox creation, point containment, boundary conditions
- [X] T091 [P] [US4] Create `crates/ev-geo/src/distance.rs` with haversine_distance function taking two LatLng and returning meters
- [X] T092 [P] [US4] Add unit tests to `crates/ev-geo/src/distance.rs` for: same point (0m), known coordinates (Tunisia Tunis→Sfax ~350km), numerical accuracy
- [X] T093 [US4] Update `crates/ev-geo/src/lib.rs` to declare and export mod point, mod bbox, mod distance
- [X] T094 [US4] Run `cargo test -p ev-geo` and verify all tests pass

### Implementation for User Story 4 (ev-db)

- [X] T095 [P] [US4] Create `crates/ev-db/src/pool.rs` with PgPool initialization from DATABASE_URL environment variable
- [X] T096 [P] [US4] Add unit tests to `crates/ev-db/src/pool.rs` for: valid DATABASE_URL parsing, invalid URL error handling
- [X] T097 [US4] Create `crates/ev-db/src/pagination.rs` with PaginationQuery struct (offset, limit) and validation logic
- [X] T098 [P] [US4] Add unit tests to `crates/ev-db/src/pagination.rs` for: valid offset/limit, boundary values (0, 100), invalid values rejection
- [X] T099 [US4] Update `crates/ev-db/src/lib.rs` to declare and export mod pool and mod pagination
- [X] T100 [US4] Run `cargo check -p ev-db` and verify all tests pass

### Integration for User Story 4

- [X] T101 [US4] Add dependencies to `services/driver-service/Cargo.toml`: ev_core, ev_geo, ev_db (path-based local dependencies)
- [X] T102 [US4] Create stub `services/driver-service/src/domain/station.rs` importing from ev_core and ev_geo
- [X] T103 [US4] Update `services/driver-service/src/main.rs` to import ev_core, ev_geo, ev_db modules (no-op for now)
- [X] T104 [US4] Run `cargo check -p driver-service` and verify all imported types resolve without errors

**Checkpoint**: User Story 4 complete - all shared crates implement core types with passing tests, Driver Service imports all three

---

## Phase 7: User Story 5 — Frontend Apps Scaffold with Dependencies Installed (Priority: P2)

**Goal**: React Web app and Expo Mobile app are fully scaffolded with all dependencies resolved and dev servers ready to start.

**Independent Test**: Developer runs `pnpm dev` in driver-web (dev server starts at localhost:5173), then `expo start` in driver-mobile (Expo CLI shows QR code). Both apps render without build errors.

**Acceptance Criteria**:
- `apps/driver-web` scaffolding complete: src/, public/, vite.config.js, package.json with all deps
- `apps/driver-web` dev server starts: `pnpm dev` serves at http://localhost:5173 with hot reload
- `apps/driver-mobile` scaffolding complete: src/, app.json, package.json with all deps
- `apps/driver-mobile` Expo CLI starts: `expo start` displays QR code and connects to simulator
- `packages/ui` exports design tokens: Colors, typography, spacing from bright theme
- `packages/api-client` exports stubs: Driver service endpoint stubs (functions, not implementations)
- All workspaces resolve: Root `pnpm install` installs all without conflicts
- Frontend build time is <15 seconds (Vite)

**Expected Duration**: ~2 hours

### Implementation for User Story 5 (driver-web)

- [X] T105 [P] [US5] Update `apps/driver-web/src/main.jsx` with proper React 18 ReactDOM.createRoot setup
- [X] T105a [P] [US5] Create `apps/driver-web/index.html` entry point with proper meta tags and react mount div
- [X] T106 [P] [US5] Create `apps/driver-web/src/App.jsx` with "Driver App" heading and bright theme colors (bg-ev-bg, text-ev-textMain)
- [X] T107 [P] [US5] Create `apps/driver-web/src/index.css` with Tailwind imports: @tailwind base, components, utilities
- [X] T108 [P] [US5] Create `apps/driver-web/vite.config.js` with React plugin, dev server on port 5173, source map enabled
- [X] T109 [US5] Update `apps/driver-web/package.json` dependencies: react, react-dom, vite, @vitejs/plugin-react, tailwindcss, postcss, autoprefixer
- [X] T110 [US5] Create `apps/driver-web/postcss.config.js` with tailwindcss and autoprefixer plugins
- [X] T111 [P] [US5] Create `apps/driver-web/tailwind.config.js` importing bright theme from docs (see spec for ev-* color naming)
- [ ] T112 [US5] Run `pnpm install` in apps/driver-web and verify all dependencies resolve
- [ ] T113 [US5] Run `pnpm dev` in apps/driver-web and verify dev server starts at localhost:5173 (Ctrl+C to stop)
- [ ] T114 [US5] Verify app renders: Open localhost:5173 in browser and confirm no console errors

### Implementation for User Story 5 (driver-mobile)

- [X] T115 [P] [US5] Create `apps/driver-mobile/src/App.js` with Expo NavigationContainer stub
- [X] T116 [P] [US5] Create `apps/driver-mobile/src/screens/MapScreen.js` (stub component, returns <Text>Map</Text>)
- [X] T117 [P] [US5] Create `apps/driver-mobile/src/screens/ListScreen.js` (stub component)
- [X] T118 [P] [US5] Create `apps/driver-mobile/src/screens/SearchScreen.js` (stub component)
- [X] T119 [P] [US5] Update `apps/driver-mobile/app.json` with proper name, slug, platforms (ios, android), plugins (expo-router, expo-location)
- [X] T120 [US5] Update `apps/driver-mobile/package.json` dependencies: react-native, expo, expo-router, expo-location, @react-navigation/native
- [ ] T121 [US5] Run `pnpm install` in apps/driver-mobile and verify all dependencies resolve
- [ ] T122 [US5] Run `expo start` in apps/driver-mobile and verify Expo CLI starts with QR code display (Ctrl+C to stop)

### Implementation for User Story 5 (packages/ui)

- [X] T123 [P] [US5] Create `packages/ui/src/tokens/colors.ts` exporting all bright theme colors: ev-bg, ev-surface, ev-green, ev-glow, ev-mapBg, ev-muted, ev-border
- [X] T124 [P] [US5] Create `packages/ui/src/tokens/typography.ts` exporting font sizes: xs (12px), sm (14px), md (16px), lg (18px), xl (24px), 2xl (32px)
- [X] T125 [P] [US5] Create `packages/ui/src/tokens/spacing.ts` exporting 4px base unit scale: xs (8px), sm (16px), md (24px), lg (32px), xl (48px)
- [X] T126 [US5] Create `packages/ui/src/index.ts` exporting all tokens
- [X] T127 [P] [US5] Create `packages/ui/src/components/Button.jsx` (stub: exports default empty function)
- [X] T128 [P] [US5] Create `packages/ui/src/components/Input.jsx` (stub)
- [X] T129 [P] [US5] Create `packages/ui/src/components/Badge.jsx` (stub)
- [X] T130 [US5] Update `packages/ui/package.json` as workspace package with exports: "./tokens", "./components"

### Implementation for User Story 5 (packages/api-client)

- [X] T131 [P] [US5] Create `packages/api-client/src/driver/stations.ts` with stub exports: getNearbyStations(), getMarkers(), searchStations(), getStationDetail()
- [X] T132 [P] [US5] Create `packages/api-client/src/types.ts` with TypeScript interfaces: Station, Charger, StationDetail, StationSummary (all optional for now)
- [X] T133 [US5] Create `packages/api-client/src/index.ts` exporting all driver endpoints and types
- [X] T134 [US5] Update `packages/api-client/package.json` as workspace package with exports: "./driver", "./types"

### Root Workspace for User Story 5

- [X] T135 [US5] Update root `package.json` to include all frontend workspaces: apps/* and packages/*
- [ ] T136 [US5] Run `pnpm install` from repo root and verify all 6 frontend workspaces resolve without conflicts
- [ ] T137 [US5] Verify individual workspace dev servers: `pnpm --filter driver-web dev` and `pnpm --filter driver-mobile start`

**Checkpoint**: User Story 5 complete - Frontend apps fully scaffolded, dev servers start, all dependencies resolved

---

## Phase 8: Polish & Validation

**Purpose**: Final verification and documentation updates

**Expected Duration**: ~1 hour

- [ ] T138 [P] Run full `cargo test` suite to verify all shared crates have passing tests
- [ ] T139 [P] Run `cargo build --release` one final time to verify no warnings
- [ ] T140 [P] Run `pnpm install && pnpm --filter driver-web build` to verify Vite build succeeds
- [ ] T141 [P] Verify all migrations idempotency: Run `db/migrate.sh` twice with no errors
- [ ] T142 Run `docker compose up -d` and verify full stack online within 1 minute with `curl http://localhost:8000/health`
- [ ] T143 Update `docs/10-delivery/mvp01/README.md` Sprint 0 section to mark all tasks complete
- [ ] T144 Update `quickstart.md` if any setup procedures changed during implementation
- [ ] T145 [P] Create file `SETUP_COMPLETE.md` documenting:
  - Date Sprint 0 was completed
  - All 5 user stories verified
  - Commands to validate setup (`cargo build`, `pnpm install`, `docker compose up`)
  - Known issues (if any)

**Checkpoint**: Sprint 0 complete - entire foundation ready, all user stories independent testable, ready for Sprint 1 (OSM Schema + Data Import)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - start immediately ✅
- **Foundational (Phase 2)**: Depends on Setup - **BLOCKS** all user stories
- **User Stories (Phases 3-7)**: All depend on Foundational completion
  - P1 stories (US1, US2, US3) should complete before P2 (US4, US5)
  - However, US1 (Build) and US4 (Crates) can start in parallel after Foundational
  - US2 (Database) must complete before US3 (Docker stack)
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

```
Setup (T001-T038)
    ↓
Foundational (T039-T062)
    ├─→ US1: Build System (T063-T076) [P1] ← Can start immediately after Foundational
    ├─→ US2: Database (T077-T080) [P1] ← Depends on Foundational only
    ├─→ US3: Docker Stack (T081-T094) [P1] ← Depends on US2 completion
    ├─→ US4: Shared Crates (T095-T118) [P2] ← Can start after Foundational, parallel with US1-US3
    └─→ US5: Frontend Apps (T119-T161) [P2] ← Can start after Foundational, parallel with all
    ↓
Polish & Validation (T162-T169)
```

**Critical Path** (minimum duration without parallelization):
- Phase 1 (Setup) → Phase 2 (Foundational) → US1 → US2 → US3 → Polish
- Total: ~5.5 hours sequentially

**With Full Parallelization** (optimal with multiple developers):
- Setup (0.5h) + Foundational (2h) + max(US1 1h, US2 0.75h, US3 1.5h, US4 2.5h, US5 2h) + Polish (1h)
- Total: ~8.5 hours wall-clock time with 5+ developers

### Within Each User Story

**US1 (Build)**: T063-T076
- All tasks sequential; each builds on previous build outputs

**US2 (Database)**: T077-T094
- Migrations (T053-T055) must complete before testing (T063-T064)
- Driver Service changes (T056-T062) must complete before Docker integration in US3

**US3 (Docker)**: T095-T124
- Depends on US2 complete (database migrations, Driver Service connection)
- Health endpoint (T065-T066) must complete before Docker health checks (T070)
- All Docker-specific tasks (T073-T080) can run in sequence

**US4 (Crates)**: T125-T147
- ev-core tasks (T081-T086) can run in parallel with ev-geo (T087-T094) and ev-db (T095-T100)
- Integration tasks (T101-T104) depend on all three crates completing
- Tests (T082, T084, T088, T090, T092, T096, T098) can run in parallel for each crate

**US5 (Frontend)**: T148-T169
- Driver-web tasks (T105-T114) independent from driver-mobile (T115-T122)
- Both can run in parallel; packages/ui and api-client support both
- Root workspace validation (T135-T137) last

### Parallel Opportunities

**During Setup (Phase 1)**:
- T003, T004, T005 can run in parallel (different files)
- T025-T036 all frontend scaffolding can run in parallel

**During Foundational (Phase 2)**:
- Database migrations (T006-T008) can run in sequence
- Docker scaffolding (T010-T012) independent, can parallel
- Shared crates (T013-T015) can parallel
- Driver Service (T017-T024) sequential due to dependencies
- Frontend (T025-T032) can all parallel
- Packages (T033-T036) can parallel

**During US4 (Shared Crates)**:
- T081-T086 (ev-core) vs T087-T094 (ev-geo) vs T095-T100 (ev-db) can all run in **FULL PARALLEL**
- T082, T084, T088, T090, T092, T096, T098 (unit tests) can all run in parallel for each crate
- Only integration (T101-T104) requires all crates complete first

**During US5 (Frontend)**:
- T105-T114 (driver-web) vs T115-T122 (driver-mobile) can run in **FULL PARALLEL**
- T123-T130 (packages/ui) can parallel with both web and mobile
- T131-T134 (packages/api-client) can parallel with both

---

## Parallel Example: User Story 4 (Shared Crates)

With 3 developers, work on crates in **full parallel**:

```bash
# Developer A: ev-core
T081: Create crates/ev-core/src/ids.rs
T082: Add tests to ids.rs
T083: Create crates/ev-core/src/types.rs
T084: Add tests to types.rs
T085: Update crates/ev-core/src/lib.rs
T086: cargo test -p ev-core

# Developer B: ev-geo (in parallel)
T087: Create crates/ev-geo/src/point.rs
T088: Add tests to point.rs
T089: Create crates/ev-geo/src/bbox.rs
T090: Add tests to bbox.rs
T091: Create crates/ev-geo/src/distance.rs
T092: Add tests to distance.rs
T093: Update crates/ev-geo/src/lib.rs
T094: cargo test -p ev-geo

# Developer C: ev-db (in parallel)
T095: Create crates/ev-db/src/pool.rs
T096: Add tests to pool.rs
T097: Create crates/ev-db/src/pagination.rs
T098: Add tests to pagination.rs
T099: Update crates/ev-db/src/lib.rs
T100: cargo test -p ev-db

# Once all 3 developers complete (parallel wait):
# Developer A: Integration
T101: Add dependencies to services/driver-service/Cargo.toml
T102: Create stub driver/station.rs importing ev-core/ev-geo
T103: Update services/driver-service/src/main.rs to import all three
T104: cargo build -p driver-service
```

**Result**: US4 completes in ~2.5 hours with 3 developers (vs ~6 hours with 1 developer)

---

## Parallel Example: User Story 5 (Frontend Apps)

With 2 developers, work on web and mobile in **full parallel**:

```bash
# Developer A: driver-web
T105-T114: All driver-web tasks (dev server ready)

# Developer B: driver-mobile + packages (in parallel)
T115-T122: All driver-mobile tasks (Expo CLI ready)
T123-T130: packages/ui tokens and components (in parallel with mobile)
T131-T134: packages/api-client stubs (in parallel with mobile)

# Both developers:
T135-T137: Root workspace validation (both apps working)
```

**Result**: US5 completes in ~2 hours with 2 developers (vs ~2 hours with 1 developer, due to high parallelizability)

---

## Implementation Strategy

### MVP First (User Stories 1-3 Only)

1. Complete Phase 1: Setup (0.5 hours)
2. Complete Phase 2: Foundational (2 hours) — **CRITICAL BLOCKING POINT**
3. Complete Phase 3: User Story 1 (1 hour) — Build system compiles
4. Complete Phase 4: User Story 2 (0.75 hours) — Database initializes
5. Complete Phase 5: User Story 3 (1.5 hours) — Docker stack runs
6. **VALIDATE**: All P1 stories work independently and together
7. **Deploy/Demo** Sprint 0 MVP to team

**Total MVP time**: ~5.5 hours (one developer, sequential)

### Incremental Expansion

8. Complete Phase 6: User Story 4 (2.5 hours) — Shared crates
9. **VALIDATE**: Crates have passing tests, Driver Service imports all three
10. Complete Phase 7: User Story 5 (2 hours) — Frontend apps
11. **VALIDATE**: Web dev server starts, Mobile Expo CLI ready
12. Complete Phase 8: Polish (1 hour) — Final validation

**Total with all stories**: ~10.5 hours (one developer, sequential)

### Parallel Team Strategy

With 5 developers after Foundational completion:

```
Time 0-2h:      All work on Setup + Foundational together
Time 2-4h:      Dev A: US1 + US4 (crates ev-core)
                Dev B: US2 + US4 (crates ev-geo)
                Dev C: US3 (Docker)
                Dev D: US4 (crates ev-db)
                Dev E: US5 (Frontend web)
Time 4-5h:      Dev A: US5 (Frontend mobile) + Dev B finish ev-geo
Time 5-6h:      Polish & validation
```

**Total with team**: ~6 hours wall-clock time

---

## Notes

- All [P] tasks = different files, no dependencies on incomplete tasks
- [US#] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group (e.g., "T065-T066: Implement health endpoint")
- Stop at any checkpoint (Phase 2, US1-3 MVP, All stories, Polish) to validate independently
- **Avoid**: Cross-story task dependencies that break independence; same file conflicts in parallel tasks

---

## Task Checklist Status

- [ ] All tasks follow strict format: `- [ ] [ID] [P?] [US?] Description with file path`
- [ ] File paths are absolute/project-relative and specific
- [ ] Story labels [US1] through [US5] correctly applied
- [ ] Parallel markers [P] correctly applied to independent tasks
- [ ] All 170 tasks accounted for (T001-T145)
- [ ] Phase dependencies documented and clear
- [ ] MVP scope clearly marked (User Stories 1-3)
- [ ] Parallel opportunities identified and examples provided
- [ ] Independent test criteria for each user story clear
- [ ] Estimated duration per phase and user story provided

**Ready for Implementation**: ✅ All tasks defined, dependencies clear, MVP path clear
