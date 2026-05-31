---

description: "Task list for Mobile Driver App Scaffold feature implementation"

---

# Tasks: Mobile Driver App Scaffold

**Input**: Design documents from `specs/001-mobile-driver-scaffold/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Not requested in specification — manual visual verification and CI build validation are the primary testing approaches.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Mobile app**: `apps/mobile-driver/` is the project root; `apps/mobile-driver/src/screens/` for screen components
- **CI**: `.github/workflows/ci.yml`
- Paths follow the structure defined in plan.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create project directory structure at .github/workflows/ and apps/mobile-driver/src/screens/
- [x] T002 Create apps/mobile-driver/package.json with Expo SDK 51 dependencies (expo ~51.0.0, react 18.2.0, react-native 0.74.1, react-native-maps 1.14.0, expo-status-bar ~1.12.1)
- [x] T003 [P] Install npm dependencies in apps/mobile-driver with npm install

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Create GitHub Actions CI workflow at .github/workflows/ci.yml to run npx expo export --platform web on push/PR to main or develop

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Project Scaffolding & CI Pipeline (Priority: P1) 🎯 MVP

**Goal**: Application entrypoint exists and CI validates the build automatically

**Independent Test**: Clone the repo on a fresh machine, run npm ci && npx expo export --platform web, and verify the build succeeds with zero errors

### Implementation for User Story 1

- [x] T005 [US1] Create App.js entrypoint at apps/mobile-driver/App.js with SafeAreaView wrapper and StatusBar, importing MapScreen from ./src/screens/MapScreen
- [ ] T006 [US1] Verify CI pipeline build: push to main/develop and confirm GitHub Actions frontend-test job passes

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Offline Map Baseline (Priority: P2)

**Goal**: A driver opens the app and sees a map centered on Tunis with a marker, rendering offline without network requests

**Independent Test**: Launch the app with airplane mode enabled and confirm the map viewport renders centered over Tunis with the debug overlay visible

### Implementation for User Story 2

- [x] T007 [P] [US2] Create MapScreen.js at apps/mobile-driver/src/screens/MapScreen.js with MapView (PROVIDER_DEFAULT) centered on Tunis (latitude 36.8065, longitude 10.1815, latitudeDelta 0.12, longitudeDelta 0.06) with full pan, zoom, and gesture support
- [x] T008 [US2] Add Marker at Tunis center coordinate with title "Tunis Core Baseline" and description "Phase 1 Offline Isolation Landmark Checkpoint"
- [x] T009 [US2] Handle offline tile unavailability (silent grey tile area, no crash, no error UI) and map component initialization failure (error fallback screen with error description, debug overlay remains visible)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Debug Diagnostics Overlay (Priority: P3)

**Goal**: A developer troubleshooting rendering sees a persistent debug overlay confirming sandbox mode

**Independent Test**: Launch the app and visually confirm a white overlay card at the top of the screen shows "BorneMap Sandbox Mode" and "Tunisia Map Layer Rendered Offline"

### Implementation for User Story 3

- [x] T010 [P] [US3] Add debug overlay View to MapScreen.js with bold "BorneMap Sandbox Mode" text and subtitle "Tunisia Map Layer Rendered Offline"
- [x] T011 [US3] Style debug overlay with semi-transparent white background (rgba 255,255,255,0.95), rounded corners (borderRadius 12), drop shadow, positioned at top of screen; ensure overlay does not block map gestures (pointerEvents) and persists when map component errors

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T012 [P] Run quickstart.md validation steps: clean install (rm -rf node_modules .expo && npm install) and tunnel launch (npm run start:tunnel)

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
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) — Depends on US1 for App.js entrypoint
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) — Depends on US2 (debug overlay lives in MapScreen.js)

### Within Each User Story

- Models before services (not applicable — no backend models)
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Map config (T007) and Marker placement (T008) can run in parallel within US2
- Debug overlay content (T010) and styling (T011) can run in parallel within US3

---

## Parallel Example: User Story 2

```bash
# Launch MapView and Marker tasks together:
Task: "Create MapScreen.js at apps/mobile-driver/src/screens/MapScreen.js..."
Task: "Add Marker at Tunis center coordinate..."
```

## Parallel Example: User Story 3

```bash
# Launch debug overlay content and styling tasks together:
Task: "Add debug overlay View to MapScreen.js..."
Task: "Style debug overlay..."
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Run CI build — verify npx expo export --platform web passes
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 (App.js + CI) → Test independently → Deploy/Demo
3. Add User Story 2 (MapScreen offline map) → Test independently → Deploy/Demo
4. Add User Story 3 (Debug overlay) → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (App.js entrypoint)
   - Developer B: User Story 2 (MapScreen map + marker)
   - Developer C: User Story 3 (Debug overlay — once US2 MapScreen exists)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- No test tasks generated — spec does not request automated testing beyond CI build verification
