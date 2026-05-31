# Tasks: Admin Dashboard

**Input**: Design documents from `/specs/007-admin-dashboard/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No automated test framework required for sandbox mode — manual visual verification per acceptance scenarios.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Admin Dashboard**: `apps/admin-dashboard/src/`
- **Web Driver**: `apps/web-driver/src/` (existing)
- **Mobile Driver**: `apps/mobile-driver/src/` (existing)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for the new admin-dashboard app

- [x] T001 Scaffold admin-dashboard Vite + React project in `apps/admin-dashboard/` with package.json, vite.config.js, index.html
- [x] T002 [P] Install Leaflet/react-leaflet dependencies (already present in mobile-driver — web platform runs from mobile-driver)
- [x] T003 [P] Verify all three apps (`admin-dashboard`, `mobile-driver` web, `mobile-driver` native) can start independently

**Checkpoint**: All three apps start without errors

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared infrastructure that MUST be complete before ANY user story can be implemented

- [x] T004 Create mock data module with Partner and Station arrays in `apps/admin-dashboard/src/data/mockData.js`
- [x] T005 Create admin theme/styles module in `apps/admin-dashboard/src/styles/theme.js`
- [x] T006 Create App shell with sidebar + top bar + content canvas layout in `apps/admin-dashboard/src/App.jsx`

---

## Phase 3: User Story 1 - Admin Console Overview Dashboard (Priority: P1) 🎯 MVP

**Goal**: Admin dashboard loads with a navigation sidebar, top status bar, and overview tab showing vitals KPI cards

**Independent Test**: Load the admin dashboard URL and verify three metric cards (PARTNERS, STATIONS, MOCK TELEMETRY HITS) render in the overview tab

### Implementation for User Story 1

- [x] T007 [P] [US1] Create TopBar component with sandbox title and MOCK ENGINE ACTIVE badge in `apps/admin-dashboard/src/components/TopBar.jsx`
- [x] T008 [P] [US1] Create Sidebar component with collapsible ENTITIES menu (PARTNERS, STATIONS) and flat links (USERS, ANALYTICS, SETTINGS, LOGS) in `apps/admin-dashboard/src/components/Sidebar.jsx`
- [x] T009 [P] [US1] Create OverviewMetrics component with three KPI cards in `apps/admin-dashboard/src/components/OverviewMetrics.jsx`
- [x] T010 [US1] Create FallbackView component for non-implemented tabs in `apps/admin-dashboard/src/components/FallbackView.jsx`
- [x] T011 [US1] Create DashboardOverview screen wiring Sidebar, TopBar, and tab routing (overview, partners, stations, fallback) in `apps/admin-dashboard/src/screens/DashboardOverview.jsx`
- [x] T012 [US1] Wire DashboardOverview into App shell in `apps/admin-dashboard/src/App.jsx`

**Checkpoint**: Admin dashboard renders overview tab with all three metric cards visible

---

## Phase 4: User Story 2 - Partners and Stations Data Tables (Priority: P1)

**Goal**: Admin navigates to PARTNERS or STATIONS tabs and sees high-density data tables with text search filtering

**Independent Test**: Click PARTNERS in the sidebar and verify the data table shows rows with ID, BRAND ENTITY NAME, HUBS, and STATUS columns

### Implementation for User Story 2

- [x] T013 [P] [US2] Create PartnersTable component with inline text search in `apps/admin-dashboard/src/components/PartnersTable.jsx`
- [x] T014 [P] [US2] Create StationsTable component with inline text search in `apps/admin-dashboard/src/components/StationsTable.jsx`
- [x] T015 [US2] Wire partners/stations tabs into DashboardOverview in `apps/admin-dashboard/src/screens/DashboardOverview.jsx`

**Checkpoint**: Both tables render with correct columns and search filters rows in real time

---

## Phase 5: User Story 3 - Desktop Web Map Portal (Priority: P2)

**Goal**: Desktop user sees full-screen Leaflet map with navbar, search overlay, zoom controls, and popover detail card

**Independent Test**: Open web-driver URL and verify navbar (ABOUT, APP, MAP, CONTACT), map, and zoom buttons render

### Implementation for User Story 3

- [x] T016 [US3] Rewrite MapPortal component per blueprint spec in `apps/mobile-driver/src/components/MapPortal.js`

**Checkpoint**: Web-driver renders Leaflet map with top navbar, centered search overlay, zoom column, and popover detail card on marker click

---

## Phase 6: User Story 4 - Mobile Native Map Screen (Priority: P2)

**Goal**: Mobile user sees full-screen MapView with floating header, zoom controls, and draggable bottom sheet

**Independent Test**: Launch mobile app and verify map, header (+ brand + Reg), zoom controls, and bottom sheet render

### Implementation for User Story 4

- [x] T017 [US4] Rewrite MapScreen component per blueprint spec in `apps/mobile-driver/src/screens/MapScreen.js`
  - Updated StationDetailSheet to open at 35% screen height per spec

**Checkpoint**: Mobile-driver renders native MapView with floating header, search row, zoom controls, and bottom sheet on marker tap

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Build verification, documentation

- [x] T018 Verify all three apps build without errors (`npm run build` for admin-dashboard, `expo export` for web/mobile-driver)
- [x] T019 Polish: removed unused NavBar import from App.js, removed duplicate NavBar wrapper

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 — Overview Dashboard (Phase 3)**: Depends on Foundational — BLOCKS US2 (US2 uses DashboardOverview shell)
- **US2 — Data Tables (Phase 4)**: Depends on US1 (DashboardOverview tab routing must exist)
- **US3 — Web Map (Phase 5)**: Depends on Foundational only — independent of admin-dashboard stories
- **US4 — Mobile Map (Phase 6)**: Depends on Foundational only — independent of admin-dashboard stories
- **Polish (Phase 7)**: Depends on all user stories

### User Story Dependencies

- **US1 (P1)**: Blocks US2 (DashboardOverview shell hosts the data tables)
- **US2 (P1)**: Depends on US1 (needs DashboardOverview tab routing)
- **US3 (P2)**: Independent — can proceed alongside US1/US2
- **US4 (P2)**: Independent — can proceed alongside US1/US2

### Within Each User Story

- Models before services
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- Phase 1 Setup tasks T001-T003 can run in parallel ([P])
- Phase 2 Foundational tasks T004-T006 can run in parallel ([P])
- US1 tasks T007-T010 can run in parallel ([P])
- US2 tasks T013-T014 can run in parallel ([P])
- US3 (T016) and US4 (T017) are independent of admin-dashboard stories — can run alongside US1/US2

---

## Parallel Example: User Story 1

```bash
# Launch all US1 components together:
Task: "Create TopBar component in apps/admin-dashboard/src/components/TopBar.jsx"
Task: "Create Sidebar component in apps/admin-dashboard/src/components/Sidebar.jsx"
Task: "Create OverviewMetrics component in apps/admin-dashboard/src/components/OverviewMetrics.jsx"
Task: "Create FallbackView component in apps/admin-dashboard/src/components/FallbackView.jsx"
```

---

## Implementation Strategy

### MVP First (Admin Dashboard Core)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (mock data, theme, App shell)
3. Complete Phase 3: US1 — Admin Overview Dashboard
4. **STOP and VALIDATE**: Test admin dashboard overview independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. US1 + US2 (Admin Dashboard) → independently testable
3. US3 (Web Map Portal) → independently testable
4. US4 (Mobile Map) → independently testable
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:
1. Team completes Setup + Foundational together
2. Developer A: US1 + US2 (Admin Dashboard — sequential)
3. Developer B: US3 (Web Map Portal — independent)
4. Developer C: US4 (Mobile Map — independent)
5. Stories integrate independently into the blueprint matrix

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same-file conflicts, cross-story dependencies that break independence
