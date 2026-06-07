# Tasks: Driver Web App

**Input**: Design documents from `/specs/009-driver-web-app/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not included — the spec does not explicitly request test file generation. Independent test criteria are defined per story for manual verification.

**Organization**: Tasks grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **App code**: `apps/driver-web/src/`
- **Packages**: `packages/{api-client,auth-client,event-taxonomy}/src/`
- **Design tokens**: `packages/design-tokens/src/`

---

## Phase 1: Setup

**Purpose**: Install missing dependencies, wire up routing, state management providers, and app layout.

- [X] T001 [P] Add dependencies to `apps/driver-web/package.json`: `@tanstack/react-query`, `react-router`, `@bornemap/api-client`, `@bornemap/auth-client`, `@bornemap/event-taxonomy`, `leaflet.markercluster`, `keycloak-js`; add `@types/leaflet.markercluster` as devDependency
- [X] T002 [P] Create app shell in `apps/driver-web/src/main.tsx` — wrap `<App />` with `<QueryClientProvider>`, `<BrowserRouter>`
- [X] T003 Build layout in `apps/driver-web/src/App.tsx` — replace placeholder with `<Header />` + flex container for map area (full-screen, resizable when side panel opens)
- [X] T004 Create `apps/driver-web/src/lib/api.ts` — instantiate `ApiClient` with `baseUrl: "http://localhost/api/v1/driver"` and `getToken` callback from auth-client
- [X] T005 Update `apps/driver-web/src/components/ui/map-container.tsx` — add `onViewportChange` callback that fires with `L.LatLngBounds` and zoom level on `moveend` event

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared React Query hooks that block US1 and US2. No UI components yet.

- [X] T006 [P] Create `apps/driver-web/src/hooks/useStationMarkers.ts` — React Query hook `useStationMarkers(params: StationListParams)` that calls `GET /api/v1/driver/stations` with lat/lng/radius_km; returns `StationListItem[]`
- [X] T007 [P] Create `apps/driver-web/src/hooks/useStationDetail.ts` — React Query hook `useStationDetail(stationId: string | null)` that calls `GET /api/v1/driver/stations/{id}`; disabled when stationId is null
- [X] T008 [P] Create `apps/driver-web/src/hooks/useSearch.ts` — React Query hook `useSearch(query: SearchQuery)` that calls `GET /api/v1/driver/stations/search` with debounced text query; implements 300ms debounce internally; returns `StationListItem[]`

**Checkpoint**: Foundation ready — React Query hooks are functional and can be consumed by UI components.

---

## Phase 3: User Story 1 — Map-first Station Discovery (Priority: P1) 🎯 MVP

**Goal**: Interactive map with clustered station markers, viewport-driven data fetching, and three map states (idle, active, station selected).

**Independent Test**: Open the app. The map renders with clustered markers around Tunisia. Pan/zoom triggers a data refetch after 500ms. When zoomed out, markers cluster. When no stations in viewport, an empty-state message displays. Markers show name and availability on hover.

- [X] T009 [P] [US1] Install `leaflet.markercluster` and integrate clustering into `apps/driver-web/src/components/ui/map-container.tsx` — import `leaflet.markercluster` CSS, create `L.MarkerClusterGroup`, expose clustered layer via `onMount` callback
- [X] T010 [P] [US1] Create `apps/driver-web/src/hooks/useViewport.ts` — custom hook that listens to map `moveend` event, computes center + appropriate radius from zoom level, debounces by 500ms, and cancels in-flight requests via AbortController
- [X] T011 [P] [US1] Create `apps/driver-web/src/components/StationMarkers.tsx` — accepts `L.Map` instance and `StationListItem[]`; creates `L.Marker` per station inside an `L.MarkerClusterGroup`; shows popup on hover with name and availability; emits `onMarkerClick(stationId)` callback
- [X] T012 [P] [US1] Create `apps/driver-web/src/components/MapStateOverlay.tsx` — renders full-screen skeleton during idle state, spinner overlay on data layer during active state, "No stations in this area" empty-state message when no stations
- [X] T013 [US1] Wire `apps/driver-web/src/components/MapView.tsx` — compose MapContainer + StationMarkers + MapStateOverlay; connect useStationMarkers with useViewport; manage map state machine (idle → active → station-selected)

**Checkpoint**: US1 complete — map renders with clustered markers, viewport refetch works, empty-state displays.

---

## Phase 4: User Story 2 — Station Details & Search (Priority: P1)

**Goal**: Side panel with full station detail on marker click; collapsible search overlay with text and filter inputs.
**Note**: Backend `city`, `connector_type`, `availability` filter params are accepted by the route but NOT yet wired in SQL. The app passes them for future-proofing; client-side post-filtering can be added if needed.

**Independent Test**: Click a marker — a side panel slides in showing name, description, charger types with power, availability, and distance. Click outside the panel to close. Click the search icon — a search overlay appears. Type a query — results appear after 300ms debounce. Filters connector type and availability.

- [X] T014 [P] [US2] Build `apps/driver-web/src/components/StationDetailPanel.tsx` — side panel (260–400px) that slides in from the left; resizes the map container via flex layout; shows loading skeleton while useStationDetail is pending; shows inline error with retry button on API failure; closes on outside click or close button
- [X] T015 [P] [US2] Build `apps/driver-web/src/components/StationInfo.tsx` — displays station name, description, city/country address, distance in km; uses Card component from Sprint 8
- [X] T016 [P] [US2] Build `apps/driver-web/src/components/ChargerList.tsx` — renders list of chargers with connector type badge (CCS/Type2/CHAdeMO), power_kw, and availability status indicator; shows "No chargers available" when chargers array is empty
- [X] T017 [P] [US2] Build `apps/driver-web/src/components/SearchOverlay.tsx` — collapsible overlay triggered by search icon in header; contains text input, connector type dropdown, availability dropdown; appears as an overlay on the map area
- [X] T018 [P] [US2] Build `apps/driver-web/src/components/SearchResults.tsx` — scrollable list of `StationListItem` results; each item shows name, city, distance; clicking an item closes search and selects the station on the map; shows "No stations found" empty-state
- [X] T019 [US2] Wire StationDetailPanel and SearchOverlay into `apps/driver-web/src/components/MapView.tsx` — on marker click, open detail panel and select station; on search result click, select station and open detail; manage side panel open/close state

**Checkpoint**: US2 complete — detail panel opens on marker click with all fields, search returns results within 500ms, panel closes on outside click.

---

## Phase 5: User Story 3 — Progressive Authentication (Priority: P2)

**Goal**: Anonymous browsing for read-only actions; login modal appears only for gated actions (favorite, review) and completes them after login.

**Independent Test**: Open app without logging in — map, detail, and search work without auth prompt. Click the favorite button on a station — a login modal appears. Log in (or use mock auth) — the modal closes and the station is favorited. Logged-in users see filled heart icons on already-favorited stations. Login failure shows error with retry/dismiss options.

- [X] T020 [P] [US3] Implement `packages/auth-client/src/index.ts` — create `Keycloak` instance with `url: "http://localhost/auth"`, `realm: "bornemap"`, `clientId: "bornemap-api"`; init with `onLoad: "check-sso"`, `pkceMethod: "S256"`; implement `getToken()` (calls `updateToken(5)` then returns token), `login(provider?)`, `logout()`; create `public/silent-check-sso.html` in `apps/driver-web/public/`
- [X] T021 [P] [US3] Create `apps/driver-web/src/hooks/useAuth.ts` — React context provider `AuthProvider` and hook `useAuth()` wrapping auth-client; exposes `isAuthenticated`, `isInitialized`, `user`, `login`, `logout`, `getToken`; provides `executeGatedAction<T>(action)` that checks auth, shows modal if needed, executes action after login
- [X] T022 [P] [US3] Build `apps/driver-web/src/components/AuthModal.tsx` — reuses Modal primitive from Sprint 8; shows login button that calls `login()`; shows loading state during auth; shows error message on failure with retry and dismiss buttons; triggers `onSuccess` callback after successful auth
- [X] T023 [US3] Wire auth into `apps/driver-web/src/lib/api.ts` — use `getToken` from AuthProvider as the `getToken` callback for ApiClient; handle 401 responses by triggering re-auth; implement `executeGatedAction` in FavoriteButton and ReviewForm

**Checkpoint**: US3 complete — anonymous browsing works end-to-end; auth modal appears on gated action; action completes after login without repeating trigger.

---

## Phase 6: User Story 4 — Favorites & Reviews (Priority: P2)

**Goal**: Registered drivers can toggle favorites and submit/edit/delete reviews. Favorites accessible via inline map filter toggle.

**Independent Test**: Log in. Click a heart icon on a station — heart fills and station appears in favorites. Click again — heart empties. Submit a review with rating + comment — it appears on the station detail panel. Edit the review — content updates. Delete the review — it disappears. Attempting a second review shows an error.

- [X] T024 [P] [US4] Build `apps/driver-web/src/components/FavoriteButton.tsx` — heart toggle icon; filled if favorited, outline if not; uses `executeGatedAction` to trigger auth if anonymous; shows loading state during mutation
- [X] T025 [P] [US4] Create hooks in `apps/driver-web/src/hooks/useFavorites.ts` — `useFavorites()` calls `GET /api/v1/driver/favorites` returning `string[]` of station IDs; `useFavoriteToggle()` uses optimistic mutation (POST to add, DELETE to remove) with rollback on error
- [X] T026 [P] [US4] Build `apps/driver-web/src/components/ReviewForm.tsx` — rating selector (1–5 stars) + comment textarea; validates rating is 1–5; shows error if user already reviewed this station (handles ALREADY_EXISTS from backend)
- [X] T027 [P] [US4] Build `apps/driver-web/src/components/ReviewList.tsx` — displays reviews for a station; shows user's own review with edit/delete buttons; shows "No reviews yet" when none exist
- [X] T028 [P] [US4] Create hooks in `apps/driver-web/src/hooks/useReviews.ts` — `useReviews(stationId)` calls `GET /api/v1/driver/reviews` and filters for the given station; `useReviewMutation()` provides `create`, `update`, `remove` (POST/PATCH/DELETE)
- [X] T029 [US4] Wire favorites filter toggle into `apps/driver-web/src/components/MapView.tsx` — toggle button in header that filters map markers to only favorited stations; wire ReviewForm + ReviewList into StationDetailPanel

**Checkpoint**: US4 complete — favorites toggle with optimistic update, review CRUD, inline favorites filter.

---

## Phase 7: User Story 5 — Clickstream Events (Priority: P3)

**Goal**: Every meaningful user interaction emits a fire-and-forget clickstream event to the analytics pipeline.

**Independent Test**: Open the app and perform each user interaction (page load, map load, viewport change, marker click, search, favorite toggle, review submit). Verify each produces an event with the correct `EventName` and payload without blocking the user experience.

- [X] T030 [P] [US5] Create `apps/driver-web/src/hooks/useClickstream.ts` — hook providing `emit(eventName, payload?)`; generates `EventEnvelope` with `event_id` (ULID), `occurred_at`/`ingested_at`, `channel: "driver_web"`, `session_id`, auth state; POSTs to `/api/v1/clickstream/events`; silently ignores failures
- [X] T031 [US5] Instrument map events in `apps/driver-web/src/components/MapView.tsx` — emit `page.viewed` on mount, `map.loaded` when tiles first load, `map.viewport_changed` on debounced viewport change, `station.marker_clicked` on marker click
- [X] T032 [US5] Instrument search events in `apps/driver-web/src/components/SearchOverlay.tsx` — emit `search.performed` with query and result count when search returns results; emit `search.failed` on search error
- [X] T033 [US5] Instrument favorite events in `apps/driver-web/src/hooks/useFavorites.ts` — emit `favorite_station.added` or `favorite_station.removed` after mutation succeeds
- [X] T034 [US5] Instrument review events in `apps/driver-web/src/hooks/useReviews.ts` — emit `review.submitted` on create, `review.updated` on update
- [X] T035 [US5] Instrument auth events in `apps/driver-web/src/hooks/useAuth.ts` — emit `auth.started` when login modal opens, `auth.succeeded` on success, `auth.failed` on failure

**Checkpoint**: US5 complete — all interactions emit events with correct names and payloads; failures silently ignored.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Edge case hardening, RTL verification, performance benchmarks, error boundaries, build validation.

- [X] T036 [SC-007] RTL audit — verify all new components (StationDetailPanel, SearchOverlay, StationMarkers, ReviewForm, ReviewList) use CSS logical properties via Tailwind (`ms-*`/`me-*`, `ps-*`/`pe-*`); test with `dir="rtl"` on `<html>`
- [X] T037 Build `apps/driver-web/src/components/ErrorBoundary.tsx` — catches render errors, shows fallback UI with retry button
- [X] T038 [SC-001, SC-002, SC-008] Performance verification — measure map mount time (<500ms), viewport update latency (<1s), skeleton render time (<200ms)
- [X] T039 Edge case hardening — rapid panning cancels in-flight requests (verify AbortController works); empty-state shows for no stations/no chargers/no reviews; error state shows inline on API failure with retry
- [X] T040 Build `apps/driver-web` — run `npm run build` and fix any compilation errors; run existing tests to verify no regressions
- [X] T041 Verify AGENTS.md references `specs/009-driver-web-app/plan.md` (should already be set from earlier commit)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — starts immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS US1 and US2
- **US1 (Phase 3)**: Depends on Phase 1 + Phase 2 — needs hooks + deps
- **US2 (Phase 4)**: Depends on Phase 1 + Phase 2 — needs hooks + map from US1 for integration
- **US3 (Phase 5)**: Depends on Phase 1 — auth-client independent; gated action wiring depends on US2/US4 components
- **US4 (Phase 6)**: Depends on Phase 2 + Phase 5 — needs auth for gated actions; detail panel from US2 for review display
- **US5 (Phase 7)**: Depends on Phases 3–6 — instrumentation touches all features
- **Polish (Phase 8)**: Depends on all phases — final hardening

### Within Each User Story

- Components marked [P] can be built in parallel
- Hooks must be created before they can be consumed by UI components
- Test each component independently before integration

### Parallel Opportunities

| Phase | Parallel tasks |
|-------|---------------|
| Phase 1 | T001 (deps) + T002 (app shell) can run in parallel |
| Phase 2 | T006 + T007 + T008 (all hooks) can run in parallel |
| Phase 3 | T009 (clustering) + T010 (viewport) + T011 (markers) + T012 (overlay) |
| Phase 4 | T014 + T015 + T016 + T017 + T018 (all components) |
| Phase 5 | T020 (auth-client) + T021 (auth hook) + T022 (auth modal) |
| Phase 6 | T024 + T025 + T026 + T027 + T028 (all components + hooks) |
| Phase 7 | T030 (hook) can start first; T031–T035 (instrumentation) are per-feature |

---

## Parallel Example: User Story 1

```bash
# Launch all US1 tasks in parallel:
Task: "Install leaflet.markercluster and integrate clustering in map-container.tsx"
Task: "Create useViewport hook in hooks/useViewport.ts"
Task: "Create StationMarkers component"
Task: "Create MapStateOverlay component"

# Then wire them together:
Task: "Wire MapView - compose all map components"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational hooks
3. Complete Phase 3: US1 (Map with clustered markers, viewport query, states)
4. **STOP and VALIDATE**: Test US1 independently — map loads, markers render/cluster, viewport refetches, empty-state shows
5. Demo ready — interactive map with station discovery

### Incremental Delivery

1. Phase 1 + Phase 2 + Phase 3 → Interactive map with station discovery (MVP)
2. Add Phase 4 → Station detail panel + search overlay
3. Add Phase 5 → Progressive authentication (login for gated actions)
4. Add Phase 6 → Favorites + reviews (requires auth)
5. Add Phase 7 → Clickstream instrumentation
6. Add Phase 8 → Polish, RTL, perf, error handling

---

## Notes

- All new components must use design-system tokens (no inline colors/spacing)
- The app is a single-page map — no route changes for detail/search (overlays only)
- React Query is the single source of truth for all server state
- Auth is handled at the UI layer (modal trigger) and data layer (JWT in api-client)
- Clickstream events are fire-and-forget — never block the user experience
- Favorites use React Query optimistic updates for immediate UI feedback
- Review one-per-user enforcement is handled by the backend (ALREADY_EXISTS); frontend shows the error inline
- RTL support via CSS logical properties only — no separate RTL stylesheets
