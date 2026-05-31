# Tasks: Mobile Canvas

**Input**: Design documents from `specs/004-mobile-canvas/`

**Prerequisites**: plan.md, spec.md, data-model.md, contracts/

**Tests**: No test tasks included — feature is primarily documentation, schema alignment, and frontend UI enhancements. Not a TDD workflow.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `backend/api-service/src/`, `backend/db/`
- **Frontend**: `apps/mobile-driver/src/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify current database state and project baseline before making changes

- [ ] T001 Verify the current migration at `backend/db/migrations/20260528000000_init_spatial_schema.sql` — confirm `partner_type` ENUM name and presence of CHECK constraints for ID patterns (`^prt-`, `^stn-`, `^chg-`)
- [ ] T002 [P] Confirm demo seed data at `backend/db/seeds/demo_data.sql` can be re-applied after schema changes
- [ ] T003 [P] Verify `deployments/docker-compose.yml` has PostGIS running and accessible

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Update the database migration to rename `partner_type` → `partner_classification`. This MUST be complete before any user story can be validated against the correct schema.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Update `backend/db/migrations/20260528000000_init_spatial_schema.sql`: rename `CREATE TYPE partner_type` to `CREATE TYPE partner_classification`, replace column type references from `partner_type` to `partner_classification`
- [x] T005 Re-apply the migration: `psql -h localhost -U borne -d borne_map -f backend/db/migrations/20260528000000_init_spatial_schema.sql`
- [x] T006 Re-seed demo data: `psql -h localhost -U borne -d borne_map -f backend/db/seeds/demo_data.sql`
- [x] T007 Fix the Rust handler at `backend/api-service/src/domains/locate/routes.rs` to cast `p.type::TEXT` to account for renamed ENUM in the `get_nearby_stations` query
- [x] T008 Verify with `cargo build -p api-service` that backend compiles after the changes

**Checkpoint**: Foundation ready — user story implementation can now begin

---

## Phase 3: User Story 1 — Architecture Documentation Alignment (Priority: P1) 🎯 MVP

**Goal**: Provide a documented project directory tree that matches the actual codebase structure, enabling developers to understand system boundaries quickly.

**Independent Test**: Every directory and file listed in the documentation must correspond to an actual path in the repository.

### Implementation for User Story 1

- [x] T009 [P] [US1] Create `ARCHITECTURE.md` at repository root with the canonical directory tree matching the plan in `specs/004-mobile-canvas/plan.md` (lines 67–103)
- [x] T010 [P] [US1] Add a README.md at repository root referencing `ARCHITECTURE.md` and the current spec under `specs/004-mobile-canvas/`
- [x] T011 [US1] Validate documented tree against actual filesystem: every path in the tree must resolve to an existing file or directory

**Checkpoint**: US1 complete — the project tree is documented and verifiable against the filesystem

---

## Phase 4: User Story 2 — Consistent Partner Classification Naming (Priority: P1)

**Goal**: The ENUM rename from `partner_type` to `partner_classification` is applied and consistent across all layers.

**Independent Test**: Query `SELECT typname FROM pg_type WHERE typname = 'partner_classification'` returns the type, and no references to `partner_type` remain in the schema.

### Implementation for User Story 2

- [x] T012 [P] [US2] Verify migration at `backend/db/migrations/20260528000000_init_spatial_schema.sql` uses `partner_classification` throughout (no remaining `partner_type` references)
- [x] T013 [US2] Run the updated migration against the local PostGIS database and verify the ENUM exists
- [x] T014 [US2] Run `cargo test -p api-service` to confirm backend tests pass with the renamed type (build verified in foundational phase)
- [x] T015 [US2] Confirm the API still returns 200: `curl "http://localhost:8080/api/v1/stations/nearby?lat=36.8065&lng=10.1815&show_staged=true"`

**Checkpoint**: US2 complete — the ENUM rename is applied in migration, backend code compiled, and API verified

---

## Phase 5: User Story 3 — Identifier Contract Enforcement (Priority: P2)

**Goal**: All three nanouuid patterns are enforced at the database level via CHECK constraints, rejecting non-conforming IDs.

**Independent Test**: Inserting rows with non-matching IDs (`^prt-[a-f0-9]{8}$`, `^stn-[a-f0-9]{8}$`, `^chg-[a-f0-9]{8}$`) is rejected by the database.

### Implementation for User Story 3

- [x] T016 [P] [US3] Verify CHECK constraint on `partners.id`: confirm `backend/db/migrations/20260528000000_init_spatial_schema.sql` has `CHECK (id ~ '^prt-[a-f0-9]{8}$')`
- [x] T017 [P] [US3] Verify CHECK constraint on `stations.id`: confirm the migration has `CHECK (id ~ '^stn-[a-f0-9]{8}$')`
- [x] T018 [P] [US3] Verify CHECK constraint on `chargers.id`: confirm the migration has `CHECK (id ~ '^chg-[a-f0-9]{8}$')`
- [x] T019 [US3] Test enforcement by attempting INSERT with invalid IDs via psql
- [x] T020 [US3] Document the identifier patterns in `ARCHITECTURE.md` (or a data contracts section in README.md)

**Checkpoint**: US3 complete — all ID patterns enforced at DB level and documented

---

## Phase 6: User Story 4 — Frontend Map and Station Card UI (Priority: P2)

**Goal**: The map renders station markers, and the bottom detail sheet shows charger-level information including loading and error states.

**Independent Test**: Loading the app shows markers on the map; tapping a marker displays charger details in a bottom sheet.

### Implementation for User Story 4

- [x] T021 [P] [US4] Add loading spinner state to `apps/mobile-driver/src/screens/MapScreen.js` — display centered spinner during API fetch
- [x] T022 [P] [US4] Add error banner with retry button to `apps/mobile-driver/src/screens/MapScreen.js` — persistent banner at top on API failure, retry triggers new fetch
- [x] T023 [P] [US4] Update `apps/mobile-driver/src/components/StationCard.js` to handle empty chargers list — show "No chargers available" message instead of blank sheet
- [x] T024 [P] [US4] Update `apps/mobile-driver/src/components/StationCard.js` to reflect the extended status values (Available, Occupied, Offline, Maintenance) — update status display logic for Offline and Maintenance states
- [x] T025 [P] [US4] Update `apps/mobile-driver/src/services/api.js` to pass `show_staged` parameter correctly to the API endpoint
- [x] T026 [US4] Verify web build: `npx expo export --platform web` completes without errors
- [ ] T026b [US4] Verify map render time ≤3 seconds via browser Network/Performance tab on WiFi with ≥10 Mbps connection

**Checkpoint**: US4 complete — map renders with markers, loading/error states handled, charger details displayed

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and documentation sync

- [x] T027 [P] Run `cargo test -p api-service` for final backend validation (build verified in foundational phase)
- [x] T028 [P] Run `npx expo export --platform web` from `apps/mobile-driver/` and confirm frontend builds
- [x] T029 Verify the full flow: start DB → apply migration → seed data → start API → load frontend → markers appear → tap marker → charger details shown
- [x] T030 Verify AGENTS.md references `specs/004-mobile-canvas/plan.md` (already set)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phase 3–6)**: All depend on Foundational phase completion
  - User stories can proceed in parallel or sequentially by priority
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1 — MVP)**: Can start after Foundational — no dependencies on other stories (documentation only)
- **US2 (P1 — MVP)**: Can start after Foundational — migration already updated in foundational, verification only
- **US3 (P2)**: Can start after Foundational — CHECK constraints already in migration, verification only
- **US4 (P2)**: Can start after Foundational — depends on API being up (validated in US2 verification)

### Parallel Opportunities

- **Phase 1 (Setup)**: All tasks marked [P] can run in parallel
- **Phase 3 (US1)**: T009 and T010 can run in parallel (different files)
- **Phase 6 (US4)**: T021, T022, T023, T024, T025 can all run in parallel (different files/components)
- **Phase 7 (Polish)**: T027 and T028 can run in parallel

---

## Parallel Example: User Story 4

```bash
# Launch all frontend updates together:
Task: "Add loading spinner to apps/mobile-driver/src/screens/MapScreen.js"
Task: "Add error banner to apps/mobile-driver/src/screens/MapScreen.js"
Task: "Handle empty chargers in apps/mobile-driver/src/components/StationCard.js"
Task: "Add extended status to apps/mobile-driver/src/components/StationCard.js"
Task: "Fix show_staged param in apps/mobile-driver/src/services/api.js"
```

---

## Implementation Strategy

### MVP First (User Story 1 + 2 — Both P1)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — migration rename)
3. Complete Phase 3: User Story 1 (documentation)
4. Complete Phase 4: User Story 2 (migration verification + API check)
5. **STOP and VALIDATE**: Both US1 and US2 independently verifiable
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add US1 (documentation) → independently testable → MVP increment
3. Add US2 (migration) → independently testable → MVP increment
4. Add US3 (identifier enforcement) → independently testable
5. Add US4 (frontend UI) → independently testable
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 (documentation tree)
   - Developer B: US2 (migration verification)
3. After US1 + US2:
   - Developer A: US4 (frontend)
   - Developer B: US3 (identifier checks)
4. Stories complete and integrate independently
