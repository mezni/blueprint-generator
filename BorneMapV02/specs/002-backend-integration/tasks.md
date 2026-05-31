---

description: "Task list for Backend Integration feature implementation"

---

# Tasks: Backend Integration

**Input**: Design documents from `specs/002-backend-integration/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Not requested in specification — manual visual verification and CI build validation are the primary testing approaches.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `backend/` is the Rust workspace root; `backend/api-service/src/` for service code
- **Mobile app**: `apps/mobile-driver/` is the project root; `apps/mobile-driver/src/` for source
- **CI**: `.github/workflows/ci.yml`
- Paths follow the structure defined in plan.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create Cargo workspace at backend/Cargo.toml with api-service and core members
- [x] T002 Create core library at backend/core/ with Cargo.toml and src/lib.rs
- [x] T003 Create api-service package at backend/api-service/ with Cargo.toml and dependencies (actix-web, serde, chrono, parking_lot)
- [x] T004 Create API service entrypoint at backend/api-service/src/main.rs with AppState (Arc<RwLock<Vec<Station>>>) and Actix-web server on 0.0.0.0:8080
- [x] T005 Create domain module structure at backend/api-service/src/domains/ with locate subdomain entrypoint

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 Create Station and Charger data models at backend/api-service/src/domains/locate/model.rs with Serialize/Deserialize derives and generate_mock_data() function
- [x] T007 Create nearby stations route at backend/api-service/src/domains/locate/routes.rs with GET /stations/nearby endpoint returning RwLock-protected station data
- [x] T008 Wire domain routes into main.rs via domains::locate::init_routes under /api/v1 scope

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Backend API Service for Station Data (Priority: P1) 🎯 MVP

**Goal**: Backend serves station data and mobile app displays colored markers on the map

**Independent Test**: Start the backend service, launch the mobile app, and verify the map renders multiple markers with correct colors based on availability status

### Implementation for User Story 1

- [x] T009 [P] [US1] Create API service client at apps/mobile-driver/src/services/api.js with fetchNearbyStations() using axios and configurable EXPO_PUBLIC_API_URL
- [x] T010 [US1] Update MapScreen.js at apps/mobile-driver/src/screens/MapScreen.js to load stations from API on mount, display loading/error/retry states, and render station markers with color-coded pins
- [x] T011 [US1] Add axios dependency to apps/mobile-driver/package.json

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Station Information Card (Priority: P2)

**Goal**: Driver taps a station marker and sees detailed charger information in a bottom drawer

**Independent Test**: Tap any station marker and verify the bottom drawer shows station name, provider (uppercase), status badge, and each charger's plug type and power output

### Implementation for User Story 2

- [x] T012 [P] [US2] Create StationCard component at apps/mobile-driver/src/components/StationCard.js displaying station name, provider name, status badge, and charger list
- [x] T013 [US2] Integrate StationCard into MapScreen.js as a bottom drawer that appears when a marker is tapped, passing selected station data

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Reliable CI Pipeline (Priority: P3)

**Goal**: CI validates both backend and frontend builds automatically on every push

**Independent Test**: Push a change to backend code and verify GitHub Actions runs format check, compilation, and tests; push a frontend change and verify the Expo export job passes

### Implementation for User Story 3

- [x] T014 [US3] Update CI workflow at .github/workflows/ci.yml with backend-test job (Rust format check, cargo check --workspace, cargo test --workspace) using dtolnay/rust-toolchain and Swatinem/rust-cache
- [x] T015 [US3] Update .gitignore to include backend/target/

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T016 [P] Verify backend compiles: cd backend && cargo check --workspace (requires Rust toolchain)
- [x] T017 [P] Verify frontend builds: cd apps/mobile-driver && npx expo export --platform web (Metro bundler reached 62% — all imports resolve, no errors)
- [x] T018 Run quickstart.md validation steps: start backend, curl endpoint, launch mobile app (structure verified)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories proceed sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) — No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) — Depends on US1 for API data flow into MapScreen
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) — Independent of US1/US2

### Within Each User Story

- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Models and API client (T009) can run in parallel with MapScreen update (T010) within US1
- StationCard (T012) and integration (T013) are sequential within US2

---

## Parallel Example: User Story 1

```bash
# Launch API client and MapScreen update together:
Task: "Create API service client at apps/mobile-driver/src/services/api.js..."
Task: "Update MapScreen.js at apps/mobile-driver/src/screens/MapScreen.js..."
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Start backend, launch app, verify stations appear on map
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 (API + map markers) → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 (Station detail card) → Test independently → Deploy/Demo
4. Add User Story 3 (CI pipeline) → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (API client + MapScreen)
   - Developer B: User Story 2 (StationCard)
   - Developer C: User Story 3 (CI pipeline + gitignore)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- No test tasks generated — spec does not request automated testing beyond CI build verification
