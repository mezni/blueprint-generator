---

description: "Task list for Phase 1 — Admin Spatial Core Validation"
---

# Tasks: Admin Spatial Core Validation

**Input**: Design documents from `specs/002-admin-spatial-core/`

**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Tests are NOT explicitly requested in the feature spec. Test tasks are omitted from this list.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `bornemap/backend/app/`, `bornemap/backend/tests/`
- **Web**: `bornemap/frontend/web/src/`
- **Infra**: `docker-compose.yml`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add missing dependencies and configure project for spatial auth

- [ ] T001 Add PyJWT dependency to `bornemap/backend/pyproject.toml`
- [ ] T002 Add JWT_SECRET and SPATIAL_CONFIG to `bornemap/backend/app/core/config.py`
- [ ] T003 Reinstall backend deps: `cd bornemap/backend && pip install -e .`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core backend infrastructure — mock auth, CoordinatePoint schema

**⚠️ CRITICAL**: No user story implementation can begin until this phase is complete

- [ ] T004 Create `CoordinatePoint` Pydantic schema with `@model_validator` enforcing longitude ∈ [-180,180] and latitude ∈ [-90,90] in `bornemap/backend/app/stations/schemas.py`
- [ ] T005 Implement mock JWT auth dependency (`get_current_user`) in `bornemap/backend/app/core/dependencies.py` — decode `X-Mock-Token` header, validate `sub`/`username`/`roles`/`iat`/`exp` claims
- [ ] T006 Add `require_admin` dependency wrapper in `bornemap/backend/app/core/dependencies.py` — checks `"admin"` in `roles` list

**Checkpoint**: Foundation ready — user story implementation can begin

---

## Phase 3: User Story 3 — Admin Authentication (Priority: P1) 🎯 MVP

**Goal**: Admins authenticated via mock JWT before modifying station data. Unauthenticated visitors can view the map but cannot add/edit/delete.

**Independent Test**: View map without token (should see stations). Try POST to /api/v1/stations without `X-Mock-Token` (should return 401). Try with valid token (should succeed).

### Implementation

- [ ] T007 [P] [US3] Protect station write endpoints: add `Depends(require_admin)` to POST, PATCH, DELETE in `bornemap/backend/app/stations/router.py`
- [ ] T008 [P] [US3] Protect partner write endpoints: add `Depends(require_admin)` to POST, PATCH, DELETE in `bornemap/backend/app/partners/router.py`
- [ ] T009 [P] [US3] Protect charger write endpoints: add `Depends(require_admin)` to POST, PATCH, DELETE in `bornemap/backend/app/chargers/router.py`
- [ ] T010 [P] [US3] Protect user write endpoints: add `Depends(require_admin)` to POST, PATCH, DELETE in `bornemap/backend/app/users/router.py`
- [ ] T011 [US3] Add `X-Mock-Token` header support to frontend API client in `bornemap/frontend/web/src/lib/api.ts` — accept optional `token` parameter, pass as header

**Checkpoint**: Admin authentication functional — write endpoints reject unauthenticated requests

---

## Phase 4: User Story 1 — Station Management on Map (Priority: P1) 🎯 MVP

**Goal**: Admins can add, edit, and delete stations directly on the interactive map. Click a location → form opens → save → marker appears immediately. Click existing marker → detail panel → edit/delete.

**Independent Test**: Open admin portal, click map at a location, fill station form, save, see green marker appear. Click marker, see details, edit a field, save, see update. Delete with confirmation, see marker removed.

### Implementation

- [ ] T012 [P] [US1] Create `StationMarker` custom green DivIcon component in `bornemap/frontend/web/src/components/map/StationMarker.tsx` — green circle (#22c55e) with white border, drop shadow, no Leaflet blue pin
- [ ] T013 [P] [US1] Create `StationPopup` detail popup component in `bornemap/frontend/web/src/components/map/StationPopup.tsx` — shows station name, operator, plug types, speed, address, edit/delete buttons
- [ ] T014 [US1] Extend `MapView.tsx` in `bornemap/frontend/web/src/components/map/MapView.tsx` — add `onClick` handler that opens station form at clicked coordinates, render `StationMarker` for each station, render `StationPopup` on marker click
- [ ] T015 [US1] Extend `StationFormPanel.tsx` in `bornemap/frontend/web/src/components/panels/StationFormPanel.tsx` — accept create/edit mode, pre-fill when editing, submit to POST/PATCH endpoint, coordinate field for map click
- [ ] T016 [P] [US1] Add React Query hooks for station CRUD in `bornemap/frontend/web/src/features/stations/api.ts` — `useCreateStation`, `useUpdateStation`, `useDeleteStation` with cache invalidation
- [ ] T017 [US1] Implement station POST endpoint with location handling in `bornemap/backend/app/stations/router.py` — accepts `CoordinatePoint`, calls service
- [ ] T018 [US1] Implement station PATCH endpoint in `bornemap/backend/app/stations/router.py`
- [ ] T019 [US1] Implement station DELETE endpoint in `bornemap/backend/app/stations/router.py`
- [ ] T020 [US1] Update `stations/repository.py` in `bornemap/backend/app/stations/repository.py` — handle GEOGRAPHY insert with `ST_GeomFromGeoJSON` or `ST_MakePoint`, handle update with location field
- [ ] T021 [US1] Update `stations/service.py` in `bornemap/backend/app/stations/service.py` — validate unique station name per operator on create/update
- [ ] T022 [US1] Update `stations/schemas.py` in `bornemap/backend/app/stations/schemas.py` — extend `StationCreate` to accept `CoordinatePoint` for location, extend response with location GeoJSON

**Checkpoint**: Station CRUD fully functional on map — markers appear, edit/delete work, no page reload

---

## Phase 5: User Story 2 — Spatial Search & Discovery (Priority: P1)

**Goal**: Admins can search for nearby stations (20 km default radius) and see station list auto-filter by map viewport. Results sorted by distance.

**Independent Test**: Pan map to Tunis area → station list shows only stations in that viewport. Type address in search → map centers → nearby stations listed sorted by distance. Search with zero results → "No stations found" message.

### Implementation

- [ ] T023 [P] [US2] Add `GET /api/v1/stations/nearby` endpoint in `bornemap/backend/app/stations/router.py` — query params: `lat`, `lng`, `radius_m` (default 20000)
- [ ] T024 [US2] Implement `find_nearby` method in `bornemap/backend/app/stations/repository.py` — `ST_DWithin` filter + `ST_Distance` ORDER BY, return distance in response
- [ ] T025 [US2] Add `find_by_viewport` method in `bornemap/backend/app/stations/repository.py` — accept SW/NE bounds, convert to center+diagonal radius, `ST_DWithin` filter
- [ ] T026 [US2] Add viewport query params to `GET /api/v1/stations` in `bornemap/backend/app/stations/router.py` — optional `sw_lat`, `sw_lng`, `ne_lat`, `ne_lng`
- [ ] T027 [US2] Extend `SearchPanel.tsx` in `bornemap/frontend/web/src/components/panels/SearchPanel.tsx` — add address/coordinate search input, on submit center map and fetch nearby stations
- [ ] T028 [US2] Extend `StationListPanel.tsx` in `bornemap/frontend/web/src/components/panels/StationListPanel.tsx` — subscribe to map viewport changes, re-fetch stations with viewport bounds, show distance for nearby results, show empty state
- [ ] T029 [US2] Add `useNearbyStations` and `useStationsByViewport` React Query hooks in `bornemap/frontend/web/src/features/stations/api.ts`

**Checkpoint**: Spatial search and viewport filtering work — station list updates on pan/zoom, nearby search centres map

---

## Phase 6: User Story 4 — Stations Data Table (Priority: P2)

**Goal**: Admins can switch to a sortable, filterable table view of all stations as an alternative to the map view.

**Independent Test**: Click toggle to switch from map to table view → see all stations listed. Type in search box → table filters in real time. Click column header → table sorts ascending/descending.

### Implementation

- [ ] T030 [US4] Create stations table page in `bornemap/frontend/web/src/pages/StationsPage.tsx` — reuses `DataTable` component with columns for name, operator, address, plug types, speed, status
- [ ] T031 [US4] Add client-side search filtering by station name — controlled input above the table
- [ ] T032 [US4] Add column sorting (click header to toggle asc/desc)
- [ ] T033 [US4] Add `useStations` React Query hook in `bornemap/frontend/web/src/features/stations/api.ts` (if not already present)
- [ ] T034 [US4] Wire stations table route `/data/stations` in `bornemap/frontend/web/src/App.tsx` (if not already present)

**Checkpoint**: Stations data table functional — search, sort, and full station list visible

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Verify everything works together end-to-end

- [ ] T035 Verify all backend imports resolve: `cd bornemap/backend && .venv/bin/python -c "from app.main import app; print('OK')"`
- [ ] T036 Run ruff lint on backend: `cd bornemap/backend && .venv/bin/ruff check app/`
- [ ] T037 Run TypeScript type check: `cd bornemap/frontend/web && npx tsc --noEmit`
- [ ] T038 Verify quickstart.md validation steps pass

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational — can then proceed in priority order
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **US3 (P1) Auth**: Independent — can start after Foundational
- **US1 (P1) Station Map**: Depends on US3 (needs auth for write endpoints)
- **US2 (P1) Spatial Search**: Depends on US1 (needs stations in DB to search) but can be implemented independently
- **US4 (P2) Data Table**: Depends on US1 (needs station data to display) but UI can be built independently

### Parallel Opportunities

- T007, T008, T009, T010 (US3 auth guards) can run in parallel
- T012, T013, T016 (US1 frontend components) can run in parallel
- T023, T024 (US2 backend endpoint + query) can run in parallel
- T030 (US4 table) can run in parallel with US2
- US3 and US1 can run sequentially (US3 → US1 auth guard)
- US2 backend can run in parallel with US1 frontend

### Parallel Example: User Story 1

```bash
# Launch simultaneously:
Task: "Create StationMarker component"
Task: "Create StationPopup component"
Task: "Add React Query hooks for station CRUD"
```

### Parallel Example: User Story 2

```bash
# Launch simultaneously:
Task: "Add GET /stations/nearby endpoint"
Task: "Implement find_nearby in repository"
```

---

## Implementation Strategy

### MVP First (US3 + US1 + US2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 3 (Auth)
4. Complete Phase 4: User Story 1 (Station Map)
5. Complete Phase 5: User Story 2 (Spatial Search)
6. **STOP and VALIDATE**: Station CRUD + spatial search + auth
7. Deploy/demo MVP

### Incremental Delivery

1. Setup + Foundational → Auth infrastructure ready
2. Add US3 → Write endpoints protected
3. Add US1 → Station CRUD on map works
4. Add US2 → Spatial search/viewport filter works
5. Add US4 → Data table view (P2, deferred if needed)
6. Each story adds value without breaking previous stories
