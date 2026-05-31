# Tasks: Cross-Platform UI Synchronization

**Input**: Design documents from `specs/006-cross-platform-ui/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Mobile/Web app**: `apps/mobile-driver/`
- **Backend API**: `backend/api-service/`
- Paths shown are relative to repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Install new npm dependencies and configure test infrastructure

- [x] T001 Install navigation dependencies in `apps/mobile-driver/`: `npm install @react-navigation/native @react-navigation/bottom-tabs react-native-screens react-native-safe-area-context`
- [x] T002 [P] Install test dependencies in `apps/mobile-driver/`: `npm install --save-dev @testing-library/react-native jest-expo`
- [x] T003 Create Jest config at `apps/mobile-driver/jest.config.js` with `jest-expo` preset

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared infrastructure that ALL user stories depend on

- [x] T004 [P] Create shared theme tokens in `apps/mobile-driver/src/styles/theme.js` (colors, spacing, breakpoints, typography)
- [x] T005 [P] Create platform detection utility in `apps/mobile-driver/src/utils/platform.js` (isDesktop, isMobile helpers)
- [x] T006 [P] Create session management in `apps/mobile-driver/src/services/session.js` (generate/persist session_id in localStorage or AsyncStorage)
- [x] T007 [P] Create shared app state context in `apps/mobile-driver/src/context/AppContext.js` (navigation tab, filter state, viewport state, session_id)
- [x] T008 [P] Extend API client in `apps/mobile-driver/src/services/api.js` with methods: `search(query, filters)`, `getStationDetail(stationId)`, `getFilters(sessionId)`, `setFilters(sessionId, filters)`
- [x] T009 [P] Create analytics service in `apps/mobile-driver/src/services/analytics.js` (send clickstream events to `POST /api/v1/analytics/connect`)
- [x] T010 [P] Create `useAppState` hook in `apps/mobile-driver/src/hooks/useAppState.js` (foreground/resume detection for filter poll trigger)
- [x] T011 [P] Create new `filters` domain module in `backend/api-service/src/domains/filters/mod.rs` with `GET /api/v1/filters` and `PUT /api/v1/filters` routes backed by `web::Data<Mutex<HashMap<String, FilterState>>>`
- [x] T012 [P] Create `backend/api-service/src/domains/filters/routes.rs` with in-memory HashMap implementation, session-keyed, server-timestamped, GET returns current filters (or empty defaults), PUT upserts per session_id
- [x] T013 [P] Register `filters` module in `backend/api-service/src/domains/mod.rs` and wire routes in `backend/api-service/src/main.rs`

**Checkpoint**: Foundation ready — shared state, session, API client, analytics service, backend filter endpoints all functional

---

## Phase 3: User Story 1 — Unified Map Layout Parity (P1) 🎯 MVP

**Goal**: Desktop web MapPortal and mobile MapScreen expose identical six-zone layout structure

**Independent Test**:
1. Desktop: Open MapPortal at 1920×1080 → verify zones: header/nav, map viewport, search/filter panel, zoom controls (bottom-right), detail panel (bottom), FAB (bottom-center)
2. Mobile: Open MapScreen at 390×844 → verify same six zones in mobile-adapted form
3. Resize from 1440px → 375px → verify layout adapts without losing zones

### Implementation for User Story 1

- [x] T014 [US1] Create `apps/mobile-driver/src/context/NavigationProvider.js` wrapping `NavigationContainer` with a bottom tab navigator (Map, Explore, Saved, Profile tabs); Map tab renders the existing MapScreen
- [x] T015 [US1] Refactor `apps/mobile-driver/App.js` to wrap the app tree with `AppContext.Provider` and `NavigationProvider`
- [x] T016 [US1] Enhance `apps/mobile-driver/src/screens/MapScreen.js` to integrate: compact header, search/filter bar overlay, floating zoom controls, bottom sheet, FAB — component slots that will be filled by later stories
- [x] T017 [P] [US1] Create desktop web layout in `apps/mobile-driver/src/components/MapPortal.js` with: full-height viewport, overlaid top search/filter panel, bottom detail panel, floating zoom controls (inline bottom-right), FAB (bottom-center)
- [x] T018 [US1] Ensure both MapPortal and MapScreen import and use the same `MapView.web.js` / `MapView.native.js` for tile and marker rendering

**Checkpoint**: Map layout parity achieved — desktop and mobile render all six zones

---

## Phase 4: User Story 2 — Cross-Platform Navigation Consistency (P1)

**Goal**: Desktop NavBar and mobile BottomTabBar expose identical four-item navigation

**Independent Test**:
1. Desktop: NavBar renders Map, Explore, Saved, Profile — each click changes view, active item shows underline
2. Mobile: BottomTabBar renders same four items — each tap changes view, active item shows filled icon

### Implementation for User Story 2

- [x] T019 [P] [US2] Create desktop NavBar component in `apps/mobile-driver/src/components/NavBar.js` with four inline SVG icons, underline active indicator, click handler updates AppContext navigation tab state
- [x] T020 [P] [US2] Create placeholder screens: `apps/mobile-driver/src/screens/ExploreScreen.js`, `apps/mobile-driver/src/screens/SavedScreen.js`, `apps/mobile-driver/src/screens/ProfileScreen.js` — each renders a centered title and placeholder text
- [x] T021 [P] [US2] Create mobile BottomTabBar integration in the navigation provider (uses `@react-navigation/bottom-tabs` with filled icon active indicator for Map, Explore, Saved, Profile tabs)
- [x] T022 [US2] Wire NavBar into MapPortal (desktop) and BottomTabBar into App.js navigator (mobile) — both read/write the same `activeTab` state from AppContext

**Checkpoint**: Navigation parity achieved — both platforms navigate four destinations

---

## Phase 5: User Story 3 — Map Component & Interaction Parity (P1)

**Goal**: Map tiles, charger markers, and cluster behavior are identical on both platforms

**Note**: The existing `MapView.web.js` (react-leaflet) and `MapView.native.js` (react-native-maps) already use the same tile layer URL and marker rendering. This phase is verification only — no code changes required if parity is confirmed.

- [x] T023 [US3] Verify `MapView.web.js` and `MapView.native.js` use identical tile layer URL and tile sizing configuration
- [x] T024 [US3] Verify both platforms use the same marker icon for charging stations and the same cluster threshold (≤200 stations per viewport)
- [ ] T025 [US3] Manual cross-platform visual comparison: load the same map region (center: Tunis 36.8065, 10.1815) on desktop and mobile at zoom levels 10, 13, 16 — confirm tile appearance and marker density match

**Checkpoint**: Map rendering parity verified on both platforms

---

## Phase 6: User Story 4 — Search & Filter Parity (P2)

**Goal**: Search bar and filter controls work identically on desktop and mobile

**Independent Test**:
1. Submit same query on both platforms → results match exactly
2. Set filters on one platform → other platform reflects them within 60s (poll interval)

### Implementation for User Story 4

- [x] T026 [P] [US4] Create `useSearch` hook in `apps/mobile-driver/src/hooks/useSearch.js` with debounced input (300ms), calls `api.search()`, manages loading/error/empty states
- [x] T027 [P] [US4] Create `useFilters` hook in `apps/mobile-driver/src/hooks/useFilters.js` with poll-based sync (on foreground/resume + 60s interval), calls `api.getFilters()` / `api.setFilters()`, last-writer-wins
- [x] T028 [P] [US4] Create shared `SearchBar` component in `apps/mobile-driver/src/components/SearchBar.js` with text input, clear button, loading spinner, debounced submit
- [x] T029 [P] [US4] Create shared `FilterControls` component in `apps/mobile-driver/src/components/FilterControls.js` with connector type chips (Type 2, CCS, CHAdeMO, Tesla), status chips, min-available stepper
- [x] T030 [US4] Integrate SearchBar + FilterControls into MapPortal (desktop top panel) and MapScreen (mobile compact header area)
- [x] T031 [P] [US4] Add `GET /api/v1/search` route in `backend/api-service/src/domains/locate/routes.rs` (query stations by name/address with optional filter params passed to PostGIS spatial query)
- [x] T032 [US4] Add error/empty/network-failure states to SearchBar and FilterControls per spec edge cases

**Checkpoint**: Search and filter parity achieved — identical queries return identical results; filters sync cross-platform

---

## Phase 7: User Story 5 — Zoom Control Parity (P2)

**Goal**: Zoom in/out and locate-me controls accessible on both platforms

**Independent Test**:
1. Desktop: inline zoom group in bottom-right — clicking ± changes zoom level
2. Mobile: floating zoom buttons in bottom-right — tapping ± changes zoom level
3. Both: locate-me button centers map on user location (or shows disabled tooltip if GPS denied)

### Implementation for User Story 5

- [x] T033 [P] [US5] Create shared `ZoomControls` component in `apps/mobile-driver/src/components/ZoomControls.js` — platform-aware: renders inline group on desktop (CSS flex), floating buttons on mobile (absolute positioned, 44×44pt touch targets)
- [x] T034 [P] [US5] Create shared `FAB` component in `apps/mobile-driver/src/components/FAB.js` — floating action button, bottom-center, platform-styled
- [x] T035 [US5] Integrate ZoomControls + FAB into MapPortal and MapScreen
- [x] T036 [US5] Add locate-me functionality with geolocation API: GPS available → center map on user; GPS denied → disable button with tooltip per spec edge cases

**Checkpoint**: Zoom control parity achieved — both platforms zoom, locate, and float consistently

---

## Phase 8: User Story 6 — Station Detail Sheet Parity (P2)

**Goal**: Marker tap opens detail view with identical information on both platforms

**Independent Test**:
1. Desktop: tap marker → bottom panel opens with six fields (name, address, available/total, connector types, status, Navigate CTA) — dismiss with X or outside click
2. Mobile: tap marker → bottom sheet peeks at 120px (name + status), drag up to 70% expanded (all six fields), swipe down past peek threshold to dismiss

### Implementation for User Story 6

- [x] T037 [P] [US6] Create `useStationDetail` hook in `apps/mobile-driver/src/hooks/useStationDetail.js` — fetches `GET /api/v1/stations/{id}`, manages loading (300ms skeleton → 2s timeout) / error / retry states, 500ms debounce on rapid marker taps
- [x] T038 [P] [US6] Create desktop `StationDetailPanel` component in `apps/mobile-driver/src/components/StationDetailPanel.js` — fixed-height bottom panel, non-draggable, dismiss via X button or outside click, collapses to minimized bar below 500px viewport height
- [x] T039 [P] [US6] Create mobile `StationDetailSheet` component in `apps/mobile-driver/src/components/StationDetailSheet.js` — draggable bottom sheet, peek 120px (name + status), expanded 70% (all six fields + Navigate CTA), swipe-down to dismiss, no map pan conflict within sheet bounds
- [x] T040 [US6] Wire station marker tap in MapScreen/MapPortal to open detail via StationDetailPanel (desktop) or StationDetailSheet (mobile) — both call `useStationDetail.fetch(stationId)` and render the same six fields

**Checkpoint**: Station detail parity achieved — both platforms show identical station information

---

## Phase 9: User Story 7 — Shared Analytics Events (P2)

**Goal**: Both platforms emit identical clickstream event payloads for all map interactions

**Independent Test**:
1. Marker tap on desktop → analytics event matches marker tap on mobile payload (same event_name, same required fields)
2. Zoom action on either platform → event includes `zoom_level` and `viewport_center`

### Implementation for User Story 7

- [x] T041 [P] [US7] Create `useAnalytics` hook in `apps/mobile-driver/src/hooks/useAnalytics.js` — provides `track(eventName, properties)` that builds `ClickstreamEvent` payload per `contracts/api.yaml` and sends via `analytics.send()`
- [ ] T042 [US7] Integrate `useAnalytics` into `SearchBar` — emit `search_submit` event on each search submission with `{query}` property
- [ ] T043 [US7] Integrate `useAnalytics` into `FilterControls` — emit `filter_change` event on each filter modification with `{filters}` property
- [ ] T044 [US7] Integrate `useAnalytics` into `ZoomControls` — emit `zoom_in` / `zoom_out` events on zoom changes with `{zoom_level, viewport_center}` properties
- [ ] T045 [US7] Integrate `useAnalytics` into map marker tap handler — emit `marker_tap` event with `{station_id}` property
- [ ] T046 [US7] Integrate `useAnalytics` into `FAB` locate-me — emit `locate_me` event on button press

**Checkpoint**: Analytics parity achieved — all six event types fire from both platforms with identical schemas

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Accessibility compliance, edge case hardening, documentation

- [ ] T047 [P] Add `aria-label` attributes to all interactive elements (NavBar items, search input, filter chips, zoom buttons, FAB, panel close button, marker cluster groups)
- [ ] T048 [P] Add `aria-live="polite"` to search results region and station detail loading area
- [ ] T049 [P] Add keyboard navigation: Tab order across all controls on desktop, Enter/Space activation, Escape closes detail panel, arrow keys for expanded cluster navigation
- [ ] T050 [P] Verify and fix 44×44pt minimum touch target on all mobile interactive elements (zoom ± buttons, FAB, filter chips, tab bar items)
- [ ] T051 [P] Implement narrow viewport handling: desktop panel collapses to minimized bar below 500px viewport height
- [ ] T052 [P] Run project documentation: update `apps/mobile-driver/README.md` with new component descriptions and cross-platform testing instructions
- [ ] T053 Run quickstart.md validation: start dev server, open desktop web and mobile simultaneously, verify all six user stories

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 — Map Layout Parity (Phase 3)**: Depends on Foundational — BLOCKS US2, US4, US5, US6 (these stories add components into US1's layout slots)
- **US2 — Navigation (Phase 4)**: Depends on US1 — BLOCKS none (can proceed after US1)
- **US3 — Map Parity (Phase 5)**: No code dependencies — verification-only, can run in parallel with any phase
- **US4 — Search/Filter (Phase 6)**: Depends on Foundational + US1 — can run in parallel with US2
- **US5 — Zoom Controls (Phase 7)**: Depends on US1 — can run in parallel with US2, US4
- **US6 — Station Detail (Phase 8)**: Depends on US1 — can run in parallel with US2, US4, US5
- **US7 — Analytics (Phase 9)**: Depends on US4, US5, US6 (needs interactive components to wire events into)
- **Polish (Phase 10)**: Depends on all desired user stories

### User Story Dependencies

- **US1 (P1)**: Blocks all other stories (they slot into US1's layout)
- **US2 (P1)**: Independent after US1
- **US3 (P1)**: Verification only — independent
- **US4 (P2)**: Independent after US1
- **US5 (P2)**: Independent after US1
- **US6 (P2)**: Independent after US1
- **US7 (P2)**: Depends on US4 + US5 + US6 (event sources)

### Within Each User Story

- Foundational tasks before component tasks
- Model/service/hooks before UI components
- UI components before integration into layout
- Story complete before moving to next priority

---

## Parallel Opportunities

- All Phase 1 tasks are [P] — install npm deps and create Jest config in parallel
- All Phase 2 foundational tasks are [P] — theme, platform utils, session, context, API extensions, analytics service, backend filters module can all be built independently
- US4, US5, US6 can be developed in parallel once US1 layout is ready
- All tasks within a story marked [P] can run in parallel (same files, no dependencies)
- US3 verification can run alongside any implementation phase

---

## Parallel Example: User Story 4 (Search & Filter)

```bash
# Launch hooks and API service together:
Task: "Create useSearch hook in apps/mobile-driver/src/hooks/useSearch.js"
Task: "Create useFilters hook in apps/mobile-driver/src/hooks/useFilters.js"
Task: "Create SearchBar component in apps/mobile-driver/src/components/SearchBar.js"
Task: "Create FilterControls component in apps/mobile-driver/src/components/FilterControls.js"
```

---

## Implementation Strategy

### MVP First (User Stories 1–3, P1 only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (BLOCKS all stories)
3. Complete Phase 3: US1 — Map Layout Parity
4. Complete Phase 4: US2 — Navigation Consistency
5. Complete Phase 5: US3 — Map Parity Verification
6. **STOP and VALIDATE**: Test all P1 stories independently
7. Deploy/demo if ready

### Full Delivery (P1 + P2)

1. Complete Setup + Foundational → Foundation ready
2. US1–3 complete → P1 MVP (deployable)
3. Add US4 (Search/Filter) → independently testable
4. Add US5 (Zoom Controls) → independently testable
5. Add US6 (Station Detail) → independently testable
6. Add US7 (Analytics) → depends on US4-6 wired components
7. Final Polish → WCAG, edge cases, documentation

### Parallel Team Strategy

With multiple developers:
1. Team completes Setup + Foundational together
2. Once Foundational done and US1 layout is complete:
   - Developer A: US2 (Navigation)
   - Developer B: US4 (Search/Filter)
   - Developer C: US5 (Zoom Controls)
   - Developer D: US6 (Station Detail)
3. All four stories integrate into the same US1 layout independently
4. After US4+US5+US6 complete → single developer handles US7 (Analytics wiring)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same-file conflicts, cross-story dependencies that break independence
- WCAG 2.1 AA compliance (T047–T050) is gathered in Phase 10 but should be minded during earlier component implementation
