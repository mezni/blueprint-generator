# Tasks: Sprint 1 — OSM Data & Station Discovery

**Date**: 2026-06-05 | **Status**: Ready for Implementation

**Input**: Design documents from `/specs/004-sprint-1-foundation/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Overview

Sprint 1 implements OpenStreetMap data ingestion and geospatial station discovery for BorneMap. Four user stories organized by priority:

- **US1** (P1): Public Driver Discovers Nearby Charging Stations → MVP-blocking
- **US2** (P1): OSM Data Imported & Station Locations Synced → MVP-blocking, prerequisite for US1 testing
- **US3** (P2): Registered Driver Creates Favorites & Views Reviews → Post-MVP, independent
- **US4** (P2): Partner Views Their Own Stations → Post-MVP, independent

**MVP Scope**: US1 + US2 (public discovery with real OSM data)

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

---

## Dependency Graph

```
Phase 1: Setup (shared)
    ↓
Phase 2: Foundational (blocks all user stories)
    ├─ DB Schema + Migrations
    ├─ Authentication Middleware
    ├─ Rate Limiting Middleware
    ├─ Error Handling & Logging
    └─ Environment Configuration
    ↓
Phase 3: US2 - OSM Data & GIS Sync (prerequisite for US1)
    ├─ GIS Schema Migrations
    ├─ Outbox Pattern Implementation
    ├─ OSM Import Script
    ├─ GIS Sync Worker
    └─ Test Data Population
    ↓
Phase 4: US1 - Public Discovery (depends on US2 for test data)
    ├─ Station Repository (GIS queries)
    ├─ Nearby Use Case
    ├─ Nearby API Handler
    ├─ Distance Validation
    └─ Integration Tests
    ↓ (PARALLEL)
Phase 5: US3 - Favorites (independent of US1/US2)
Phase 5: US4 - Partner Stations (independent of US1/US2)
    ↓
Phase 6: Polish & Cross-Cutting Concerns
```

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and structure per implementation plan

**Est. Duration**: 2-4 hours

### Project Structure & Dependencies

- [X] T001 Create Rust project structure per plan.md; verify crate layout (driver-service, partner-service, gis-worker, ev-geo, ev-auth, ev-domain) at `ev-platform/crates/`
- [X] T002 [P] Initialize Cargo.toml workspace with shared dependencies: actix-web, sqlx, tokio, serde, tracing, uuid (ev-platform/Cargo.toml)
- [X] T003 [P] Configure Rust tooling: rustfmt, clippy, and linting in `ev-platform/.cargo/config.toml`
- [X] T004 [P] Setup Docker Compose for local development with PostgreSQL 15+ and Keycloak (ev-platform/docker-compose.yml)
- [X] T005 Create environment template file (.env.example) with all required variables (DATABASE_URL, KEYCLOAK_*, RUST_LOG)

**Checkpoint**: Project skeleton ready, dependencies installed, Docker infrastructure available

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST complete before ANY user story implementation

**⚠️ CRITICAL**: No user story work can begin until this phase completes

**Est. Duration**: 12-16 hours

### Database Migrations Framework

- [X] T006 Initialize SQLx CLI and migration system; create `crates/driver-service/migrations/` directory
- [ ] T007 Create migration: `001_create_inventory_schema.sql` with inventory, users, gis schemas (crates/driver-service/migrations/)
- [ ] T008 [P] Create migration: `002_create_partner_table.sql` in `crates/driver-service/migrations/`
- [ ] T009 [P] Create migration: `003_create_station_table.sql` with soft-delete support in `crates/driver-service/migrations/`
- [ ] T010 [P] Create migration: `004_create_charger_table.sql` in `crates/driver-service/migrations/`
- [ ] T011 [P] Create migration: `005_create_users_schema_and_tables.sql` (user, favorite, review) in `crates/driver-service/migrations/`
- [ ] T012 [P] Create migration: `006_create_gis_schema_and_indexes.sql` with GIST spatial indexes in `crates/driver-service/migrations/`
- [ ] T013 [P] Create migration: `007_create_station_outbox.sql` for GIS sync pattern in `crates/driver-service/migrations/`
- [X] T014 Run migrations and verify schema schema creation in local PostgreSQL (test via psql \dt inventory.*, users.*, gis.*)

### Authentication & Authorization

- [X] T015 Implement JWT validation middleware in `crates/ev-auth/src/jwt_validator.rs` (decode, validate expiry, extract claims)
- [X] T016 Create Claims struct in `crates/ev-auth/src/claims.rs` with user_id, partner_id, role, scopes
- [X] T017 Create AuthError enum in `crates/ev-auth/src/lib.rs` for 401/403 responses
- [X] T018 Implement auth middleware for driver-service in `crates/driver-service/src/interface/middleware/auth.rs` (attach claims to request extensions)
- [X] T019 Create PartnerScope extractor in `crates/driver-service/src/interface/middleware/partner_scope.rs` (validates partner_id in JWT, fails if missing)

### Rate Limiting

- [X] T020 Implement IP-based rate limiter in `crates/driver-service/src/interface/middleware/rate_limiter.rs` (100 req/min per IP, 429 response)
- [X] T021 Add rate limiter middleware to driver-service routing (crates/driver-service/src/main.rs)

### Error Handling & Logging

- [X] T022 Create error hierarchy in `crates/driver-service/src/error.rs` (ValidationError, NotFound, Unauthorized, RateLimited, InternalError)
- [X] T023 Implement HTTP response mapping for errors in `crates/driver-service/src/interface/handlers/error_handler.rs` (400, 401, 403, 404, 429, 500)
- [X] T024 Setup structured logging with tracing crate in `crates/driver-service/src/lib.rs` (JSON output, correlation IDs)

### Environment Configuration

- [ ] T025 Create config module in `crates/driver-service/src/config.rs` (DATABASE_URL, KEYCLOAK_*, RUST_LOG from environment)
- [ ] T026 [P] Create config module in `crates/gis-worker/src/config.rs` (DATABASE_URL, GIS_WORKER_INTERVAL_SECS)
- [ ] T027 [P] Create config module in `crates/partner-service/src/config.rs` (DATABASE_URL, KEYCLOAK_*)

### Shared Domain Models

- [ ] T028 Create NanoID generation in `crates/ev-domain/src/ids.rs` with prefixes (STN-, CHG-, PRT-, USR-, FAV-, REV-)
- [ ] T029 Create Station entity skeleton in `crates/ev-domain/src/station.rs` (fields, validation rules, but no DB logic yet)
- [ ] T030 Create Charger entity skeleton in `crates/ev-domain/src/charger.rs`
- [ ] T031 Create Partner entity skeleton in `crates/ev-domain/src/partner.rs`
- [ ] T032 Create User entity skeleton in `crates/ev-domain/src/user.rs`
- [ ] T033 Create Favorite entity skeleton in `crates/ev-domain/src/favorite.rs`
- [ ] T034 Create Review entity skeleton in `crates/ev-domain/src/review.rs`

**Checkpoint**: All foundational infrastructure ready; migrations complete; auth, logging, config working; user story implementation can begin in parallel

---

## Phase 3: User Story 2 — OSM Data Imported & Station Locations Synced (Priority: P1)

**Goal**: Import OpenStreetMap data for Tunisia; implement GIS projection layer and async sync worker to enable public discovery queries

**Independent Test**: DBA runs osm-import.sh script; verifies `gis.osm_ways` has 10k+ roads, `gis.station_locations` syncs from inventory.station, GIS queries return correct proximity results

**Est. Duration**: 16-20 hours

**Blocking Dependency**: US1 cannot test without OS data and working GIS sync

### OSM Data Import Infrastructure

- [x] T035 Create osm2pgsql style configuration in `scripts/osm2pgsql-style.lua` (ways, nodes, relations for Tunisia)
- [x] T036 Create OSM import script in `scripts/osm-import.sh` (wrapper for osm2pgsql; validates params, handles errors, creates indexes)
- [x] T037 Create OSM import Dockerfile in `Dockerfile.osm` for containerized import process
- [x] T038 [P] Add test data seed script in `scripts/seed-test-data.sql` (50+ stations, 5+ partners, test users for manual testing)

### GIS Sync Worker - Domain Layer

- [x] T039 Create GIS projection domain model in `crates/gis-worker/src/domain/gis_projection.rs` (StationLocationProjection entity, validation, mapping from inventory.station)
- [x] T040 Create outbox event reader in `crates/gis-worker/src/domain/outbox_event.rs` (OutboxEvent struct, event types: created, updated, deleted)
- [x] T041 Create GIS sync use case in `crates/gis-worker/src/application/sync_usecase.rs` (orchestrates: read outbox, fetch station, project to GIS, mark processed)

### GIS Sync Worker - Infrastructure Layer

- [x] T042 Create outbox reader in `crates/gis-worker/src/infrastructure/outbox_reader.rs` (polls outbox, fetches unprocessed events with exponential backoff)
- [x] T043 Create GIS projector in `crates/gis-worker/src/infrastructure/gis_projector.rs` (upserts gis.station_locations, calculates geom from coordinates, handles soft deletes)
- [x] T044 Create database pool manager in `crates/gis-worker/src/infrastructure/db_pool.rs` (SQLx connection pooling, health checks)
- [x] T045 Create migration runner in `crates/gis-worker/src/infrastructure/migrations.rs` (runs migrations on startup)

### GIS Sync Worker - Worker Binary

- [x] T046 Create GIS worker main in `crates/gis-worker/src/main.rs` (startup, migration, polling loop with configurable interval)
- [x] T047 [P] Add signal handlers in `crates/gis-worker/src/main.rs` (graceful shutdown on SIGTERM)
- [x] T048 Create worker tests in `crates/gis-worker/tests/integration/sync_worker.rs` (mock outbox, verify GIS sync behavior, soft delete handling)

### Driver Service - GIS Repository Layer

- [x] T049 Create station repository trait in `crates/driver-service/src/infrastructure/repository/mod.rs` (interface for station queries)
- [x] T050 Create GIS station repository implementation in `crates/driver-service/src/infrastructure/repository/gis_repository.rs` (ST_DWithin queries, sorting by distance, soft-delete filtering)
- [x] T051 Create database pool in `crates/driver-service/src/infrastructure/db.rs` (SQLx connection pooling)

### Trigger for Outbox Events

- [x] T052 Create database trigger in `scripts/create_outbox_trigger.sql` (on INSERT/UPDATE/DELETE of inventory.station, insert to station_outbox)
- [x] T053 Execute trigger creation during schema migration (add to migration SQL or separate initialization script)

### GIS Data Validation

- [x] T054 Create coordinate validation in `crates/ev-domain/src/geo.rs` (validate lat -90..90, lng -180..180)
- [x] T055 Create Haversine distance function in `crates/ev-geo/src/distance.rs` (for application-level sorting if needed)

**Checkpoint**: OSM import script working; GIS sync worker polling and syncing; test data populated; ready for US1 discovery queries

---

## Phase 4: User Story 1 — Public Driver Discovers Nearby Charging Stations (Priority: P1)

**Goal**: Implement `/api/v1/stations/nearby` public endpoint enabling users to find charging stations by geographic proximity without authentication

**Independent Test**: `GET /api/v1/stations/nearby?lat=36.8&lng=10.1&radius=5000` returns 10+ stations sorted by distance with names, addresses, availability, capacity

**Est. Duration**: 12-16 hours

### Domain Layer (Pure Logic)

- [x] T056 Create nearby query domain model in `crates/driver-service/src/domain/nearby_query.rs` (lat, lng, radius validation, distance calculation)
- [x] T057 Create station distance struct in `crates/driver-service/src/domain/station_with_distance.rs` (station + distance_m field)
- [x] T058 Create validation rules in `crates/driver-service/src/domain/validation.rs` (coordinate bounds, radius bounds 100-50000m)

### Application Layer (Use Cases)

- [x] T059 Create nearby stations use case in `crates/driver-service/src/application/nearby_stations_usecase.rs` (validates input, calls repo, returns sorted results)
- [x] T060 Implement pagination logic in use case if needed (offset/limit for future large result sets)

### Infrastructure Layer (Data Access)

- [x] T061 [P] Extend GIS repository in `crates/driver-service/src/infrastructure/repository/gis_repository.rs` with nearby_stations method (ST_DWithin + ORDER BY distance)
- [x] T062 [P] Create query builder for nearby endpoint in `crates/driver-service/src/infrastructure/queries/nearby.sql` (raw SQL for code review)

### Interface Layer (HTTP Handlers)

- [x] T063 Create request/response types in `crates/driver-service/src/interface/dto/nearby_query.rs` (NearbyRequest, NearbyResponse, Station DTO)
- [x] T064 Create nearby handler in `crates/driver-service/src/interface/handlers/nearby_handler.rs` (parse query params, validate, call use case, format response)
- [x] T065 Register nearby route in `crates/driver-service/src/main.rs` (GET /api/v1/stations/nearby, wrap with rate limiter)

### Error Handling & Validation

- [x] T066 Add validation error messages in `crates/driver-service/src/interface/handlers/error_handler.rs` (invalid_latitude, invalid_longitude, invalid_radius)
- [x] T067 Implement constraint validation in request types (lat bounds, lng bounds, radius bounds)

### Contract Tests (Independent Test Criteria)

- [x] T068 [P] Create contract test US1-AC1 in `crates/driver-service/tests/contract/nearby_stations.rs` — stations within radius sorted by distance
- [x] T069 [P] Create contract test US1-AC2 in `crates/driver-service/tests/contract/nearby_stations.rs` — empty radius returns empty list (not error)
- [x] T070 [P] Create contract test US1-AC3 in `crates/driver-service/tests/contract/nearby_stations.rs` — invalid coordinates rejected with clear error message

### Integration Tests

- [ ] T071 Create integration test in `crates/driver-service/tests/integration/nearby_integration.rs` — setup test DB, populate 50+ stations, verify query latency <500ms p95, verify 100 concurrent searches
- [ ] T072 Test soft-delete filtering in `crates/driver-service/tests/integration/nearby_integration.rs` — deleted stations excluded from results
- [ ] T073 Test rate limiting in `crates/driver-service/tests/integration/rate_limiting.rs` — IP rate limit enforced, 429 response after 100 req/min

**Checkpoint**: Public discovery endpoint fully functional with real OSM data; contract tests passing; integration tests passing; MVP feature delivery ready

---

## Phase 5A: User Story 3 — Registered Driver Creates Favorites & Views Reviews (Priority: P2)

**Goal**: Implement authenticated endpoints for registered drivers to save/view/remove station favorites; enable viewing reviews from other users

**Independent Test**: Authenticate as driver; POST /api/v1/favorites with station_id; GET /api/v1/favorites returns saved station; DELETE removes it

**Est. Duration**: 12-14 hours

**Can Run In Parallel With**: Phase 5B (US4), as they don't share implementation

### Domain Layer

- [ ] T074 Create favorite domain model in `crates/driver-service/src/domain/favorite.rs` (user_id, station_id, created_at, validation)
- [ ] T075 Create review domain model in `crates/driver-service/src/domain/review.rs` (rating 1-5, comment, user_id, station_id)

### Application Layer

- [x] T071 Create favorites use case in `crates/driver-service/src/application/favorites_usecase.rs` (add, remove, list favorites)
- [x] T072 Implement favorites pagination logic in use case if needed
- [x] T073 Create review use case in `crates/driver-service/src/application/review_usecase.rs` (add, update, list reviews)

### Infrastructure Layer

- [x] T074 Create favorite repository in `crates/driver-service/src/infrastructure/repository/favorite_repository.rs` (add, remove, list, check favorite status)
- [x] T075 Create review repository in `crates/driver-service/src/infrastructure/repository/review_repository.rs` (insert, update, list_by_station, soft delete)
- [x] T076 Create database pool manager in `crates/driver-service/src/infrastructure/db_pool.rs` (SQLx connection pooling, health checks)

### Interface Layer

- [x] T077 Create favorite DTO in `crates/driver-service/src/interface/dto/favorite_dto.rs` (AddFavoriteRequest, FavoriteResponse with embedded station)
- [x] T078 Create add favorite handler in `crates/driver-service/src/interface/handlers/favorites_handler.rs` — POST /api/v1/favorites
- [x] T079 [P] Create list favorites handler in `crates/driver-service/src/interface/handlers/favorites_handler.rs` — GET /api/v1/favorites with pagination
- [x] T080 [P] Create remove favorite handler in `crates/driver-service/src/interface/handlers/favorites_handler.rs` — DELETE /api/v1/favorites/{favorite_id}
- [x] T081 Register favorite routes in `crates/driver-service/src/main.rs` (POST, GET, DELETE /api/v1/favorites)

### Contract Tests

- [x] T086 [P] Create contract test US3-AC1 in `crates/driver-service/tests/contract/favorites.rs` — add favorite creates record
- [x] T087 [P] Create contract test US3-AC2 in `crates/driver-service/tests/contract/favorites.rs` — list favorites shows station data with availability
- [x] T088 [P] Create contract test US3-AC3 in `crates/driver-service/tests/contract/favorites.rs` — remove favorite deletes record

### Integration Tests

- [x] T089 Create integration test in `crates/driver-service/tests/integration/favorites_integration.rs` — authenticate, CRUD favorites, verify data isolation (user can only see own favorites)
- [x] T090 Test pagination in `crates/driver-service/tests/integration/favorites_integration.rs` — list endpoint respects limit/offset
- [x] T091 Test soft-deleted station handling in `crates/driver-service/tests/integration/favorites_integration.rs` — favorite can point to deleted station (UI responsibility to hide)

**Checkpoint**: Favorites feature complete; user data isolation verified; integration tests passing

---

## Phase 5B: User Story 4 — Partner Views Their Own Stations (Priority: P2)

**Goal**: Implement partner dashboard endpoints enabling business users to view ONLY their own stations with operational metrics (charger status, availability)

**Independent Test**: Partner A authenticates; GET /api/v1/partner/stations returns only Partner A's stations (no Partner B stations); 403 if attempting cross-partner access

**Est. Duration**: 14-16 hours

**Can Run In Parallel With**: Phase 5A (US3), as they don't share implementation

### Domain Layer

- [x] T092 Create partner scope domain model in `crates/driver-service/src/domain/partner_scope.rs` (user_id, partner_id, role; validates partner_id from JWT)
- [x] T093 Create charger status domain model in `crates/driver-service/src/domain/charger_status.rs` (count by status: available, in_use, maintenance, offline)

### Application Layer

- [x] T094 Create list partner stations use case in `crates/driver-service/src/application/list_partner_stations_usecase.rs` (filters by partner_scope, includes charger summary)
- [x] T095 Create get partner station use case in `crates/driver-service/src/application/get_partner_station_usecase.rs` (detail view with all chargers)
- [x] T096 Create update station use case in `crates/driver-service/src/application/update_station_usecase.rs` (name, address, availability_status, capacity; triggers outbox)
- [x] T097 Create create station use case in `crates/driver-service/src/application/create_station_usecase.rs` (validates coords, creates with partner_id from scope, triggers outbox)

### Infrastructure Layer

- [x] T098 Create partner station repository in `crates/partner-service/src/infrastructure/repository/partner_station_repository.rs` (scoped queries: list_by_partner, get_with_chargers, create, update)
- [x] T099 [P] Create charger repository for partner service in `crates/partner-service/src/infrastructure/repository/charger_repository.rs` (list_by_station, update_status)
- [x] T100 Create database pool in `crates/partner-service/src/infrastructure/db.rs` (SQLx connection pooling)

### Interface Layer

- [ ] T101 Create partner station DTO in `crates/partner-service/src/interface/dto/station_dto.rs` (StationResponse with charger summary, CreateStationRequest, UpdateStationRequest)
- [ ] T102 Create list partner stations handler in `crates/partner-service/src/interface/handlers/partner_handler.rs` — GET /api/v1/partner/stations with pagination and optional status filter
- [ ] T103 [P] Create get station detail handler in `crates/partner-service/src/interface/handlers/partner_handler.rs` — GET /api/v1/partner/stations/{station_id}
- [ ] T104 [P] Create update station handler in `crates/partner-service/src/interface/handlers/partner_handler.rs` — PATCH /api/v1/partner/stations/{station_id}
- [ ] T105 [P] Create create station handler in `crates/partner-service/src/interface/handlers/partner_handler.rs` — POST /api/v1/partner/stations
- [ ] T106 Register partner routes in `crates/partner-service/src/main.rs` (GET, POST, PATCH /api/v1/partner/stations)
- [ ] T107 Implement partner scope middleware in `crates/partner-service/src/interface/middleware/partner_scope.rs` (enforces partner_id from JWT on all requests)

### Contract Tests

- [ ] T108 [P] Create contract test US4-AC1 in `crates/partner-service/tests/contract/partner_stations.rs` — partner sees only own stations
- [ ] T109 [P] Create contract test US4-AC2 in `crates/partner-service/tests/contract/partner_stations.rs` — station detail includes charger summary (available, in_use, etc.)
- [ ] T110 [P] Create contract test US4-AC3 in `crates/partner-service/tests/contract/partner_stations.rs` — 403 Forbidden when attempting to access another partner's station

### Integration Tests

- [ ] T111 Create integration test in `crates/partner-service/tests/integration/partner_stations_integration.rs` — setup 2 partners with stations, verify isolation (no cross-partner access)
- [ ] T112 Test charger summary calculation in `crates/partner-service/tests/integration/partner_stations_integration.rs` — verify counts match charger status
- [ ] T113 Test station creation triggers outbox in `crates/partner-service/tests/integration/partner_stations_integration.rs` — create station, verify outbox event created
- [ ] T114 Test update triggers GIS sync in `crates/partner-service/tests/integration/partner_stations_integration.rs` — update coordinates, verify GIS sync within 5 min SLA

**Checkpoint**: Partner dashboard complete; station management working; partner isolation enforced at API layer; integration tests passing

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final quality checks, observability, documentation, and deployment preparation

**Est. Duration**: 8-10 hours

### Documentation & Examples

- [ ] T115 Create API documentation in `/docs/api/v1/` (endpoints, request/response examples from contracts/)
- [ ] T116 Update quickstart.md with actual task IDs and timelines (crates/driver-service/quickstart.md)
- [ ] T117 Create database schema documentation in `/docs/database/schema.md` (entity relationships, indexes, migrations)
- [ ] T118 Create architecture decision records (ADRs) in `/docs/adr/` for major choices (GIS as derived, outbox pattern, rate limiting)

### Observability & Monitoring

- [ ] T119 Add metrics collection for critical paths in `crates/driver-service/src/metrics.rs` (nearby query latency, favorite CRUD counts, GIS sync latency)
- [ ] T120 Add structured logging for user journey events (logged: nearby query, favorite create/delete, station create/update, auth failures)
- [ ] T121 Create alerts configuration (Keycloak down, GIS sync worker down, rate limit threshold exceeded, slow queries)

### Performance Validation

- [ ] T122 Load test nearby endpoint with 100 concurrent users (verify p95 <500ms, see quickstart.md for test script)
- [ ] T123 Load test authenticated endpoints with 1000 concurrent users (verify no degradation)
- [ ] T124 Verify GIST index effectiveness on gis.station_locations (EXPLAIN ANALYZE on ST_DWithin query)
- [ ] T125 Profile GIS sync worker with 1000+ stations in outbox (verify 5-min SLA achievable)

### Security Review

- [ ] T126 Verify auth middleware rejects unauthenticated requests to protected endpoints (test 401 responses)
- [ ] T127 Verify partner scope isolation (partner A cannot access partner B data; 403 enforced)
- [ ] T128 Verify rate limiter blocks abuse (test 429 responses after threshold)
- [ ] T129 Verify input validation rejects all malformed requests (invalid coords, invalid IDs, injection attempts)

### Docker & Deployment Prep

- [ ] T130 Create Dockerfile for driver-service in `crates/driver-service/Dockerfile`
- [ ] T131 [P] Create Dockerfile for partner-service in `crates/partner-service/Dockerfile`
- [ ] T132 [P] Create Dockerfile for gis-worker in `crates/gis-worker/Dockerfile`
- [ ] T133 Update docker-compose.yml to include all three services (driver, partner, gis-worker)
- [ ] T134 Create deployment guide in `/docs/deployment/sprint1-deployment.md` (manual steps, post-launch validation)

### Code Quality & Testing

- [ ] T135 Run cargo test --all to verify all unit tests pass
- [ ] T136 Run cargo clippy --all to fix all warnings
- [ ] T137 Run rustfmt --all to format code
- [ ] T138 Generate test coverage report (coverage >80% for domain/application layers)

### Final Integration

- [ ] T139 Create end-to-end test scenario in `tests/e2e/sprint1_full_flow.rs` (populate OSM data, create stations, discover nearby, create favorite, view as partner)
- [ ] T140 Document test data setup in quickstart.md (scripts/seed-test-data.sql with expected counts)
- [ ] T141 Create rollback procedure documentation if any migrations fail (manual recovery steps)

**Checkpoint**: All code reviewed, tested, documented, containerized, and ready for production deployment

---

## Summary

### Task Counts by Phase

| Phase | User Story | Task Count | Est. Hours |
|-------|-----------|-----------|-----------|
| Phase 1 | Setup | 5 | 4 |
| Phase 2 | Foundational | 27 | 16 |
| Phase 3 | US2 (OSM & GIS) | 19 | 20 |
| Phase 4 | US1 (Discovery) | 18 | 16 |
| Phase 5A | US3 (Favorites) | 18 | 12 |
| Phase 5B | US4 (Partner) | 24 | 16 |
| Phase 6 | Polish | 27 | 10 |
| **TOTAL** | | **141** | **104** |

### Task Counts by Story (Excluding Setup, Foundational, Polish)

- **US1 (Public Discovery)**: 18 tasks (0 contracts, 0 tests provided; all from plan.md)
- **US2 (OSM & GIS Sync)**: 19 tasks (async worker, import, projection)
- **US3 (Favorites)**: 18 tasks (CRUD endpoints, contract tests, integration tests)
- **US4 (Partner Stations)**: 24 tasks (dashboard, station mgmt, scope isolation)

### Parallelization Opportunities

**Phase 2 (Foundational)**: 6 tasks marked [P] (all migrations, configs can run in parallel)

**Phase 3 (US2)**: 3 tasks marked [P] (outbox reader and projector can develop in parallel after domain/use case)

**Phase 4 (US1)**: 2 tasks marked [P] (repo and query builder parallel after domain)

**Phase 5A (US3)**: 4 tasks marked [P] (handlers for GET, DELETE can develop in parallel; repo tasks parallel)

**Phase 5B (US4)**: 9 tasks marked [P] (handlers for GET, PATCH, POST parallel; repos parallel; middleware parallel)

**Phase 6**: 3 tasks marked [P] (Dockerfiles can build in parallel)

### MVP Scope

**Minimum Viable Product**: Phase 1 + Phase 2 + Phase 3 + Phase 4 = ~60 tasks, ~56 hours

- ✅ Public discovery endpoint working
- ✅ OSM data imported for Tunisia
- ✅ GIS sync worker operational
- ✅ All contract tests for US1 passing
- ✅ Performance targets met (<500ms p95 for queries)

**Post-MVP Additions**:
- Phase 5A: Favorites feature (US3)
- Phase 5B: Partner dashboard (US4)
- Phase 6: Polish, observability, deployment

### Independent Test Criteria (Ready-for-Demo)

**US1 (Public Discovery)**:
```bash
curl "http://localhost:3001/api/v1/stations/nearby?lat=36.8&lng=10.1&radius=5000" | jq '.stations | length'
# Expected: 10+ stations sorted by distance_m
```

**US2 (OSM & GIS Sync)**:
```sql
SELECT COUNT(*) FROM gis.osm_ways;  -- 10,000+
SELECT COUNT(*) FROM gis.station_locations WHERE deleted_at IS NULL;  -- 50+
```

**US3 (Favorites)**:
```bash
# Authenticate, create favorite, list, delete
TOKEN=$(curl ... get jwt)
curl -X POST http://localhost:3001/api/v1/favorites \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"station_id": "STN-..."}'
# Expected: 201 with favorite record
```

**US4 (Partner Stations)**:
```bash
# Authenticate as partner
curl -X GET http://localhost:3002/api/v1/partner/stations \
  -H "Authorization: Bearer $TOKEN"
# Expected: 200 with partner's stations only; no cross-partner access
```

### Dependency Notes

- **US2 blocks US1**: Cannot test public discovery without OSM data and working GIS sync
- **Phase 2 blocks all stories**: Auth, logging, DB schema required
- **US3 & US4 independent**: Can implement in parallel; don't depend on each other
- **No frontend work in Sprint 1**: Tasks focus on backend API; frontend consumes these endpoints

---

## Next Steps

1. **Sprint Planning**: Use task estimates to schedule sprints (typically 2-week sprints at 40 hrs/week = ~2 complete phases)
2. **Team Assignment**: Assign Phase 1 to lead; Phase 2 to shared team; Phase 3-4 to backend team; Phase 5A-5B to separate teams (parallel); Phase 6 to all
3. **CI/CD Setup**: Before starting Phase 1, ensure GitHub Actions configured to run `cargo test`, `cargo clippy`, Docker build
4. **Tracking**: Use tasks.md as source of truth in project management tool (Jira, Linear, etc.)
5. **Definition of Done**: Each task requires code review, passing tests, and documentation update

---

**Status**: ✅ **READY FOR IMPLEMENTATION**

**Tasks Generated**: 2026-06-05 | **Feature Branch**: `004-sprint-1-foundation`

Proceed to Phase 1 setup or ask clarifying questions about estimates, dependencies, or task scope.
