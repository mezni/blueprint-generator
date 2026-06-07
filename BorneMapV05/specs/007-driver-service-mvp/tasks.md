# Tasks: Driver Service MVP

**Input**: Design documents from `/specs/007-driver-service-mvp/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Tests**: Error type unit tests (T022); auth enforcement, contract format, and soft-delete tests (T026–T028, 16 total); performance seed script (T029); EXPLAIN ANALYZE script (T030).

**Organization**: Tasks grouped by user story for independent implementation.

## Format: `[ID] [P] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Service binary**: `services/driver-service/src/`
- **Tests**: inline `#[cfg(test)]` modules in `services/driver-service/src/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Cargo project scaffolding and dependencies

- [X] T001 Update `services/driver-service/Cargo.toml` with dependencies: sqlx (postgres + chrono), serde, chrono, tower-http, thiserror
- [X] T002 Create directory structure: `services/driver-service/src/{models,repository,routes}/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core service infrastructure — MUST complete before ANY user story

- [X] T003 [P] Implement `services/driver-service/src/config.rs` with env-driven AppConfig (port, auth, DB, map defaults)
- [X] T004 [P] Implement `services/driver-service/src/db.rs` with PgPool initialization (max_connections=20)
- [X] T005 [P] Implement `services/driver-service/src/error.rs` with ServiceError enum, IntoResponse, error mapping (DB constraint → ALREADY_EXISTS, FORBIDDEN, NOT_FOUND)
- [X] T006 [P] Implement `services/driver-service/src/extractors.rs` with PaginationParams (page, size, offset, limit)

**Checkpoint**: Foundation ready — user stories can now be implemented in parallel

---

## Phase 3: User Story 1 - Public Station Discovery via Map (Priority: P1) 🎯 MVP

**Goal**: Drivers can discover visible charging stations on a map using bbox/radius queries, view station details, and search by text.

**Independent Test**: Query `/api/v1/driver/stations?lat=36.8&lng=10.18&radius_km=10` returns only visible stations with `distance_km` and `geom` fields.

### Implementation for User Story 1

- [X] T007 [P] [US1] Create station models in `services/driver-service/src/models/station.rs` (StationListItem, StationDetail, GeoPoint, ReviewSummary, ChargerTypeInfo, StationListQuery, StationSearchQuery)
- [X] T008 [P] [US1] Create charger model in `services/driver-service/src/models/charger.rs` (Charger with id, station_id, type, power_kw, status)
- [X] T009 [P] [US1] Implement station repository in `services/driver-service/src/repository/station_repo.rs` with spatial queries (ST_DWithin + GIST), visibility filter, search (ILIKE), detail with chargers + availability + review summary
- [X] T010 [P] [US1] Implement discovery routes in `services/driver-service/src/routes/discovery.rs` (GET /stations, GET /stations/{id}, GET /stations/search)
- [X] T011 [P] [US1] Implement public routes in `services/driver-service/src/routes/public.rs` (/health)

**Checkpoint**: US1 complete — station discovery functional with geo queries, search, and detail

---

## Phase 4: User Story 2 - Registered Driver Favorites (Priority: P1)

**Goal**: Logged-in drivers can favorite stations for quick access.

**Independent Test**: Authenticate as `registered_driver`, POST `/api/v1/driver/favorites/STN-123`, then GET `/api/v1/driver/favorites` returns `["STN-123"]`.

### Implementation for User Story 2

- [X] T012 [P] [US2] Create favorite model in `services/driver-service/src/models/favorite.rs` (FavoriteStation with user_id, station_id, created_at)
- [X] T013 [P] [US2] Implement favorite repository in `services/driver-service/src/repository/favorite_repo.rs` (add with ON CONFLICT DO NOTHING, remove, list, is_favorite)
- [X] T014 [US2] Implement favorites routes in `services/driver-service/src/routes/favorites.rs` (POST/DELETE /favorites/{id}, GET /favorites)

**Checkpoint**: US2 complete — favorites CRUD works with auth enforcement

---

## Phase 5: User Story 3 - Driver Reviews (Priority: P2)

**Goal**: Logged-in drivers can submit, update, and delete reviews for stations they visited.

**Independent Test**: Authenticate as `registered_driver`, POST `/api/v1/driver/reviews` with `{station_id, rating: 4, comment: "good"}` succeeds; second POST with same station_id fails with `ALREADY_EXISTS`.

### Implementation for User Story 3

- [X] T015 [P] [US3] Create review models in `services/driver-service/src/models/review.rs` (Review, ReviewCreate with rating validation 1-5, ReviewUpdate)
- [X] T016 [P] [US3] Implement review repository in `services/driver-service/src/repository/review_repo.rs` (create with UNIQUE constraint handling, update with ownership check, soft-delete, list by user)
- [X] T017 [US3] Implement reviews routes in `services/driver-service/src/routes/reviews.rs` (POST/PATCH/DELETE /reviews, GET /reviews)

**Checkpoint**: US3 complete — reviews CRUD works with ownership enforcement and duplicate rejection

---

## Phase 6: User Story 4 - Driver Profile (Priority: P2)

**Goal**: Logged-in drivers can view and update their profile.

**Independent Test**: Authenticate as `registered_driver`, GET `/api/v1/driver/me` returns profile; PATCH `/api/v1/driver/me` with `{display_name: "John"}` updates profile.

### Implementation for User Story 4

- [X] T018 [P] [US4] Create user profile models in `services/driver-service/src/models/user.rs` (UserProfile, DriverProfile, ProfileUpdate)
- [X] T019 [P] [US4] Implement user repository in `services/driver-service/src/repository/user_repo.rs` (get_profile with user_account + user_profile join, upsert_profile)
- [X] T020 [US4] Implement profile routes in `services/driver-service/src/routes/profile.rs` (GET/PATCH /me)

**Checkpoint**: US4 complete — driver profile read/update functional

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Wire everything together, add tests, verify

- [X] T021 Rewrite `services/driver-service/src/main.rs` with route wiring: public routes (discovery + health via optional_auth) + authenticated routes (favorites, reviews, profile via require_role(RegisteredDriver))
- [X] T022 [P] Add error type unit tests in `services/driver-service/src/error.rs` (validation, not_found, forbidden, already_exists)
- [X] T023 Run `cargo build -p driver-service` and fix compilation errors
- [X] T024 Run `cargo test -p driver-service` and verify all tests pass
- [X] T025 Run `cargo build --workspace` and verify no workspace regressions
- [X] T026 [P] Add auth enforcement test verifying unauthenticated requests return `UNAUTHENTICATED` in `services/driver-service/src/main.rs`
- [X] T027 [P] Add contract test verifying standard envelope format `{success, data, meta}` on station list response in `services/driver-service/src/routes/discovery.rs`
- [X] T028 [P] Add soft-delete behavior test verifying review `status` transitions to `deleted` on DELETE in `services/driver-service/src/repository/review_repo.rs`
- [X] T029 Create seed dataset SQL script for 10,000 stations across Tunisia (`specs/007-driver-service-mvp/seed-10000-stations.sql`)
- [X] T030 Create `EXPLAIN ANALYZE` SQL script for bbox and radius spatial queries (`specs/007-driver-service-mvp/explain-analyze-spatial.sql`)
- [X] T031 Review and finalize spec docs at `specs/007-driver-service-mvp/spec.md`
- [X] T032 Create quickstart doc at `specs/007-driver-service-mvp/quickstart.md` with API reference
- [X] T033 Update `AGENTS.md` to reference sprint 7 plan

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase
  - US2 (favorites), US3 (reviews), US4 (profile) depend on US1 only for route wiring in main.rs
  - Models and repos for each story are fully independent
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — no other story dependencies
- **US2 (P1)**: Can start after Phase 2 — independently testable
- **US3 (P2)**: Can start after Phase 2 — independently testable
- **US4 (P2)**: Can start after Phase 2 — independently testable

### Within Each User Story

- Models before repositories
- Repositories before routes

### Parallel Opportunities

| Story | Parallel tasks |
|-------|---------------|
| US1 | T007 (station model), T008 (charger model), T009 (station repo), T010 (discovery routes) |
| US2 | T012 (favorite model), T013 (favorite repo), T014 (favorites routes) |
| US3 | T015 (review model), T016 (review repo), T017 (reviews routes) |
| US4 | T018 (user model), T019 (user repo), T020 (profile routes) |
| Phase 2 | T003–T006 (config, db, error, extractors) |
| Phase 7 tests | T026–T028 (auth, contract, soft-delete tests) |

---

## Parallel Example: User Story 1

```bash
# Launch all models for User Story 1 together:
Task: "Create station/charger models in services/driver-service/src/models/station.rs"
Task: "Create charger model in services/driver-service/src/models/charger.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (Station Discovery)
4. **STOP and VALIDATE**: Test station discovery independently
5. Demo ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add US1 (Station Discovery) → Test → **MVP**
3. Add US2 (Favorites) → Test → Add retention feature
4. Add US3 (Reviews) → Test → Add community feature
5. Add US4 (Profile) → Test → Complete driver journey

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
