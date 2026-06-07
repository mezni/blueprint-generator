# Tasks: Admin Service MVP

**Input**: Design documents from `/specs/005-admin-service-mvp/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: No explicit test tasks — the spec defines acceptance scenarios and independent test criteria per story, implemented as integration tests within each story phase.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to
- Include exact file paths in descriptions

## Path Conventions

Paths under `services/admin-service/` and `crates/` at monorepo root.

---

## Phase 1: Setup — Shared Crate Infrastructure + Migration

**Purpose**: Update shared crates with new dependencies and capabilities needed by all user stories. Create the new migration for the idempotency key table.

- [x] T001 [P] Add `sqlx` (features: runtime-tokio, postgres, migrate, chrono) to `crates/common-db/Cargo.toml`; add `ulid = "1"` to `crates/common-types/Cargo.toml`; add `axum` to `crates/common-errors/Cargo.toml`
- [x] T002 [P] Implement `common-db` PgPool factory + migration runner in `crates/common-db/src/lib.rs` — `init_pool(database_url) -> PgPool`, `run_migrations(pool) -> Result`
- [x] T003 [P] Add `ConcurrentModification` variant to `common_errors::ErrorCode` in `crates/common-errors/src/lib.rs`; implement `IntoResponse` for `ApiError` returning standard error envelope with correct HTTP status per error code
- [x] T004 [P] Add `generate_id(prefix: EntityPrefix) -> String` ULID generator to `crates/common-types/src/lib.rs`; add `ItemEnvelope<T>` (single-item success with empty meta) + `impl IntoResponse` for `SuccessEnvelope<T>`, `ItemEnvelope<T>`, `ErrorEnvelope` in `crates/common-types/src/api.rs`
- [x] T005 [P] Create migration `services/admin-service/migrations/0018_create_inventory_idempotency_key.up.sql` — table `inventory.idempotency_key` (id TEXT PK, key TEXT UNIQUE NOT NULL, station_id TEXT NOT NULL FK references station, created_at TIMESTAMPTZ NOT NULL DEFAULT now()); create `0018_create_inventory_idempotency_key.down.sql`
- [x] T006 Update `services/admin-service/Cargo.toml` — add `sqlx` (with runtime-tokio, postgres, chrono, migrate features), `serde` (with derive), `ulid = "1"`, `chrono = "0.4"` (with serde), `tower-http = "0.5"` (with cors feature)
- [x] T007 [P] Create module files — `services/admin-service/src/models/mod.rs`, `services/admin-service/src/repository/mod.rs`, `services/admin-service/src/routes/mod.rs` — each declaring submodules that will be created in later phases

**Checkpoint**: Setup complete — shared crates compile, migration file exists, deps declared.

---

## Phase 2: Foundational — App Bootstrap + Error Handling + Auth Provisioning

**Purpose**: Core admin-service infrastructure that MUST be complete before any user story. Config, service error type, custom extractors, outbox repository, provisioning upgrade.

- [x] T008 Implement `services/admin-service/src/config.rs` — struct `AppConfig` reading env vars: `ADMIN_SERVICE_PORT`, `AUTH_ISSUER`, `AUTH_JWKS_URL`, `AUTH_AUDIENCE`, `PLATFORM_DB_*` connection params, `PARTNER_DELETE_BLOCK_ACTIVE_STATIONS`
- [x] T009 Implement `services/admin-service/src/error.rs` — enum `ServiceError` with variants `Auth(AuthError)`, `Api(ApiError)`, `Db(sqlx::Error)`, `Validation(String)`, `Internal(String)`; `impl From` for each source type; `impl IntoResponse` mapping to correct HTTP statuses (409 for ConcurrentModification/ActiveStationsExist, 404 for RowNotFound, 400 for Validation, etc.)
- [x] T010 Implement `services/admin-service/src/extractors.rs` — axum extractors: `PaginationParams` (page, size with defaults), `IdempotencyKey` (from header), `IfMatch` (from header)
- [x] T011 Implement `services/admin-service/src/repository/outbox_repo.rs` — function `insert_outbox_entry(tx: &mut Transaction<'_, Postgres>, entity_type: &str, entity_id: &str, operation: &str)` inserting into `gis.sync_queue` with generated ID, status `pending`
- [x] T012 [P] Rewrite `crates/common-auth/src/provisioning.rs` — replace FNV-1a stub with real DB lookup: query `users.user_account` by `keycloak_user_id`, auto-provision if not found (INSERT with generated USR- ULID), return `ProvisionedUser` with user_id and email
- [x] T013 [P] Update `crates/common-auth/src/guards.rs` — after provisioning, query `users.partner_membership` by `user_id` and populate `CurrentUser.partner_id` and membership role; accept `PgPool` via request extensions or `State`
- [x] T014 Implement `services/admin-service/src/db.rs` — `init_db_pool()` reading config and calling `common_db::init_pool()`; `run_migrations()` calling `common_db::run_migrations()`
- [x] T015 Rewrite `services/admin-service/src/main.rs` — extend app bootstrap: init tracing, read config, init DB pool + run migrations, set auth config + init JWKS cache, build router with `/health` (public) + all route modules mounted at `/api/v1/partner` and `/api/v1/admin` behind auth/role middleware, bind and serve

**Checkpoint**: Foundation ready — service boots, DB connects, auth middleware populates `partner_id`, error types work. User story implementation can begin.

---

## Phase 3: User Story 1 + User Story 5 — Partner Station CRUD with GIS Outbox (Priority: P1) 🎯 MVP

**Goal**: Partner can create, list, update, and soft-delete their own stations. Station mutations (create/update/delete) automatically insert `gis.sync_queue` outbox rows. Partner isolation prevents cross-partner access.

**Independent Test**: Partner A creates → reads → updates → soft-deletes a station. Partner B cannot read or modify Partner A's station. After creation, a `gis.sync_queue` row exists with matching `entity_id` and `operation='insert'`.

**FRs**: FR-002, FR-003, FR-004, FR-005, FR-010, FR-011, FR-018, FR-023, FR-024, FR-025, FR-026, FR-027, FR-028

### Implementation for User Story 1

- [x] T016 [P] [US1] Create Partner model structs in `services/admin-service/src/models/partner.rs` — `PartnerRow` (DB row with all columns), `CreatePartnerRequest` DTO, `UpdatePartnerRequest` DTO, `PartnerResponse` DTO with serde derives; impl `From<PartnerRow> for PartnerResponse`
- [x] T017 [P] [US1] Create Station model structs in `services/admin-service/src/models/station.rs` — `StationRow`, `CreateStationRequest`, `UpdateStationRequest`, `StationResponse`; include validation helper `validate_coordinates(lat, lng) -> Result` and `validate_status_transition(from, to) -> Result`
- [x] T018 Implement `services/admin-service/src/repository/idempotency_repo.rs` — `lookup_key(pool, key) -> Option<String>` returns station_id if found, `insert_key(tx, key, station_id)` within transaction
- [x] T019 Implement `services/admin-service/src/repository/station_repo.rs` — `list_partner_stations(pool, partner_id, params) -> (Vec<StationRow>, PaginationMeta)`, `get_station(pool, id) -> Result<StationRow>`, `create_station(tx, partner_id, req) -> StationRow` (generates STN- ULID, sets audit fields), `update_station(tx, id, req, updated_by, expected_updated_at) -> Result` (WHERE updated_at = expected, returns CONCURRENT_MODIFICATION if 0 rows), `soft_delete_station(tx, id, updated_by) -> Result` (SET deleted_at = now()); all partner-scoped queries include `WHERE partner_id = $1`
- [x] T020 Implement `services/admin-service/src/routes/partner.rs` — 4 station CRUD handlers: `list_stations`, `create_station` (check Idempotency-Key → lookup → insert station + idempotency_key + outbox in transaction), `update_station` (update station + insert outbox in transaction), `delete_station` (soft-delete + insert outbox); all handlers extract `CurrentUser` with `partner_id`, return standard envelopes
- [x] T021 [US1] Implement validation in `services/admin-service/src/models/station.rs` — coordinate bounds check, status lifecycle enforcement (`draft → active → inactive → maintenance → active`), ensure `partner_id` never accepted from request body

**Checkpoint**: US1 + US5 complete — partner can manage own stations, GIS outbox rows created, isolation enforced.

---

## Phase 4: User Story 2 — Partner Manages Chargers and Availability (Priority: P1)

**Goal**: Partner can manage chargers at their stations and update station availability. Inherits partner scoping from parent station.

**Independent Test**: Partner A adds a charger to their station → updates its status → updates station availability. Partner B cannot access these resources (`PARTNER_SCOPE_VIOLATION`).

**FRs**: FR-006, FR-007, FR-008, FR-009, FR-010, FR-011, FR-025, FR-026

### Implementation for User Story 2

- [ ] T022 [P] [US2] Create Charger model structs in `services/admin-service/src/models/charger.rs` — `ChargerRow`, `CreateChargerRequest`, `UpdateChargerRequest`, `ChargerResponse`
- [ ] T023 [P] [US2] Create Availability model structs in `services/admin-service/src/models/availability.rs` — `StationAvailabilityRow`, `UpdateAvailabilityRequest`, `AvailabilityResponse`
- [ ] T024 Implement `services/admin-service/src/repository/charger_repo.rs` — `list_partner_chargers(pool, partner_id, params) -> (Vec<ChargerRow>, PaginationMeta)` (scoped via `station.partner_id`), `create_charger(tx, req, created_by)` (generates CHG- ULID, sets `created_by`/`updated_by`), `update_charger(tx, id, req, updated_by, expected_updated_at)` (sets `updated_by`, uses optimistic locking); all queries verify charger belongs to partner's station via JOIN or subquery
- [ ] T025 Implement `services/admin-service/src/repository/availability_repo.rs` — `upsert_availability(pool, station_id, status, source)` — INSERT or UPDATE on conflict by station_id; verify station belongs to partner before upsert
- [ ] T026 Add charger and availability handlers to `services/admin-service/src/routes/partner.rs` — `list_chargers`, `create_charger`, `update_charger`, `update_availability`; all enforce partner scoping via repository layer

**Checkpoint**: US2 complete — partner can manage chargers and availability with full isolation.

---

## Phase 5: User Story 3 — Admin Global Management (Priority: P1)

**Goal**: Admin can perform global CRUD on partners, stations, users, and reviews. Respects soft-delete and audit rules. Station mutations through admin API also trigger GIS outbox.

**Independent Test**: Admin creates a partner → lists partners → attempts delete blocked by active stations → moderates a review's status.

**FRs**: FR-012, FR-013, FR-014, FR-015, FR-016, FR-017, FR-018, FR-025, FR-026

### Implementation for User Story 3

- [x] T027 [P] [US3] Create Review model structs in `services/admin-service/src/models/review.rs` — `ReviewRow`, `ModerateReviewRequest`, `ReviewResponse`; include valid status transitions for moderation
- [x] T028 [P] [US3] Create User model structs in `services/admin-service/src/models/user.rs` — `UserAccountRow`, `UserResponse`; include `PartnerMembershipRow` (for admin visibility)
- [x] T029 Implement `services/admin-service/src/repository/partner_repo.rs` — `list_admin_partners(pool, params)`, `create_partner(tx, req, created_by)` (generates PRT- ULID, sets `created_by`/`updated_by`), `update_partner(tx, id, req, updated_by, expected_updated_at)` (sets `updated_by`, uses optimistic locking), `soft_delete_partner(tx, id, updated_by)` (sets `deleted_at`; check active stations first via trigger or explicit query)
- [x] T030 Implement `services/admin-service/src/repository/user_repo.rs` — `list_users(pool, params) -> (Vec<UserResponse>, PaginationMeta)` — simple SELECT with pagination
- [x] T031 Implement `services/admin-service/src/repository/review_repo.rs` — `list_reviews(pool, params)`, `update_review_status(pool, id, new_status, moderated_by)` — validate status transition is allowed
- [x] T032 Implement `services/admin-service/src/routes/admin.rs` — handlers: `list_users`, `list_partners`, `create_partner`, `update_partner`, `delete_partner`, `list_stations`, `update_station` (with GIS outbox insertion), `delete_station` (with GIS outbox insertion), `list_reviews`, `moderate_review`; reuse station_repo from Phase 3 for station operations (no partner_id filter for admin)
- [x] T033 [US3] Add admin station handlers that reuse `services/admin-service/src/repository/station_repo.rs` (admin variants: no partner_id filter) and call `outbox_repo::insert_outbox_entry` on mutation

**Checkpoint**: US3 complete — admin can manage all platform entities globally.

---

## Phase 6: User Story 4 — Partner Profile (Priority: P2)

**Goal**: Partner can view their own membership information.

**Independent Test**: A partner calls `GET /api/v1/partner/me` and receives their `partner_id`, `role`, and membership info.

**FRs**: FR-001, FR-010

### Implementation for User Story 4

- [x] T034 [US4] Implement partner profile handler in `services/admin-service/src/routes/partner.rs` — `GET /api/v1/partner/me` extracts `CurrentUser` (already populated by auth middleware with `partner_id` from membership), returns `partner_id`, `partner_name` (query `inventory.partner`), `membership_role`, `email`, `user_id`

**Checkpoint**: US4 complete — partner sees their profile.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Finalize observability, pagination, contract alignment, and cleanup.

- [x] T035 [P] Add structured JSON logging to all route handlers — `tracing::info!` on create/update/delete with entity type, ID, operation; ensure no PII in logs
- [x] T036 [P] Verify all list endpoints (partner stations, admin partners, admin stations, admin users, admin reviews, partner chargers) return correct `PaginationMeta` with total count
- [x] T037 Verify all response envelopes match contracts — success `{ "success": true, "data": {}, "meta": {} }`, error `{ "success": false, "error": { "code": "...", "message": "..." } }`
- [x] T038 [P] Update `services/admin-service/Dockerfile` if needed — ensure new dependencies (sqlx uses postgres native, no extra libs needed); verify build succeeds
- [x] T039 Run `cargo build -p admin-service` and fix any compilation errors; run `cargo clippy -p admin-service` and fix warnings
- [x] T040 [P] Verify SC-009 — run a basic latency check against each write endpoint (POST/PATCH/DELETE), measuring `time_total` with `curl -w` to confirm ≤500ms p95 under no-load conditions

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — can start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 — BLOCKS all user stories
- **Phase 3 (US1 + US5)**: Depends on Phase 2 — station repo uses outbox_repo (T011) and error types (T009)
- **Phase 4 (US2)**: Depends on Phase 2 and Phase 3 — charger/availability require existing stations
- **Phase 5 (US3)**: Depends on Phase 2 and Phase 3 — admin station operations reuse station_repo and outbox_repo
- **Phase 6 (US4)**: Can start immediately after Phase 2 (only depends on auth middleware provisioning)
- **Phase 7 (Polish)**: Depends on all user story phases

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — no dependencies on other stories. **This is the MVP.**
- **US2 (P1)**: Depends on US1 (needs stations to attach chargers to)
- **US3 (P1)**: Depends on Phase 2 but NOT on other stories for partner/user/review CRUD; station operations reuse US1's station_repo and outbox_repo
- **US4 (P2)**: Can start immediately after Phase 2 — simple extraction from `CurrentUser`
- **US5** (cross-cutting): Embedded in US1 and US3 via outbox_repo

### Parallel Opportunities

- Phase 1: T001, T002, T003, T004, T005, T007 all run in parallel
- Phase 2: T012 and T013 run in parallel
- Phase 3: T016 and T017 (models) run in parallel
- Phase 4: T022 and T023 (models) run in parallel
- Phase 5: T027 and T028 (models) run in parallel
- Phase 6: Can start after Phase 2, in parallel with US1 if desired
- Phase 7: T035, T036, T038, T040 run in parallel

---

## Parallel Example: Phase 3 (US1)

```bash
# Models in parallel:
Task: "Create Partner model in models/partner.rs"
Task: "Create Station model in models/station.rs"

# Repositories in sequence (T018 → T019):
Task: "Implement idempotency_repo.rs"
Task: "Implement station_repo.rs (uses T018)"

# Routes after repos:
Task: "Implement partner station routes in routes/partner.rs"
```

---

## Implementation Strategy

### MVP First (Phase 1 + Phase 2 + Phase 3 = US1 + US5)

1. Complete Phase 1: Setup (shared crates, migration)
2. Complete Phase 2: Foundational (config, error, extractors, provisioning, outbox)
3. Complete Phase 3: Partner station CRUD + GIS outbox
4. **STOP and VALIDATE**: Full station lifecycle works with partner isolation + outbox
5. Deploy/demo MVP

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add US1 + US5 (partner station CRUD + outbox) → **MVP delivered**
3. Add US2 (charger/availability CRUD) → Partner operational dashboard ready
4. Add US3 (admin global CRUD) → Admin platform governance ready
5. Add US4 (partner profile) → Partner self-service complete
6. Polish → Production-ready
