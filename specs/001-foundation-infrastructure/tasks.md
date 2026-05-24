---

description: "Task list for Phase 0 — Foundation & Infrastructure Loop"
---

# Tasks: Foundation & Infrastructure Loop

**Input**: Design documents from `specs/001-foundation-infrastructure/`

**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Tests are NOT requested in this feature spec. Test files listed in the project structure (conftest.py, test_health.py) are created as implementation tasks.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `backend/app/`, `backend/tests/`
- **Web**: `web/src/`, `web/` root config files
- **Mobile**: `mobile/app/`, `mobile/components/`
- **Infra**: `docker-compose.yml`, `.github/workflows/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create project directory structure and initialize language environments

- [ ] T001 Create root-level infra files: `docker-compose.yml`, `.gitignore`, `README.md`
- [ ] T002 [P] Initialize backend Python project: `backend/pyproject.toml`, `backend/app/__init__.py`
- [ ] T003 [P] Initialize web frontend: `web/package.json`, `web/vite.config.ts`, `web/tsconfig.json`, `web/index.html`, `web/public/favicon.svg`
- [ ] T004 [P] Initialize mobile Expo project: `mobile/package.json`, `mobile/app.json`, `mobile/tsconfig.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core backend infrastructure — config, database engine, dependency injection, migration framework

**⚠️ CRITICAL**: No user story can begin until this phase is complete

- [ ] T005 Create `backend/app/core/config.py` with pydantic-settings `Settings` class (DATABASE_URL, APP_NAME, CORS_ORIGINS, etc.)
- [ ] T005b Create `backend/.env.example` documenting all required env vars with defaults for local dev (DATABASE_URL=postgresql+asyncpg://borne:map@localhost:5432/borne, APP_NAME=BorneMap, CORS_ORIGINS=["http://localhost:5173"])
- [ ] T006 [P] Create `backend/app/core/database.py` — async SQLAlchemy engine + `AsyncSession` factory for PostGIS
- [ ] T007 [P] Create `backend/app/core/dependencies.py` — `get_db` session dependency, health-check helpers
- [ ] T008 Setup Alembic: `backend/alembic.ini`, `backend/migrations/env.py`, baseline revision `001_create_stations.py`

**Checkpoint**: Foundation ready — user story implementation can begin

---

## Phase 3: User Story 1 — Local Development Environment (Priority: P1) 🎯 MVP

**Goal**: FastAPI backend with health endpoints, hot-reload, PostGIS connectivity

**Independent Test**: Clone repo, run `docker compose up -d`, `uvicorn` dev server, verify `GET /health/live` returns 200 and `GET /health/ready` confirms DB connected

- [ ] T009 [US1] Create FastAPI app entry point in `backend/app/main.py` — app factory, CORS middleware, include health router
- [ ] T010 [P] [US1] Implement health liveness router in `backend/app/health/router.py` — `GET /health/live` → `{"status": "ok"}`
- [ ] T011 [US1] Implement health readiness router — `GET /health/ready` → verifies DB with `SELECT 1`, returns `{"status": "ok", "database": "connected"}` or 503
- [ ] T012 [US1] Define all backend dependencies in `backend/pyproject.toml` — main deps (fastapi, uvicorn, sqlalchemy[asyncio], asyncpg, geoalchemy2, alembic, pydantic-settings) under `[project]`; test deps (httpx, pytest, pytest-asyncio) under `[project.optional-dependencies] dev`
- [ ] T013 [US1] Create `backend/Dockerfile` — Python 3.12-slim, install deps, copy app, run uvicorn with hot-reload
- [ ] T014 [US1] Create `backend/tests/conftest.py` (async test client fixture) and `backend/tests/test_health.py` (live + ready endpoint tests)
- [ ] T015 [P] [US1] Create feature-scoped station stubs: `backend/app/stations/{models.py,schemas.py,repository.py,service.py,router.py}` — each file must have correct module-level declarations per constitution §6 (router.py gets `APIRouter()` instance, models.py gets `Base` import, etc.) so imports across the layer stack resolve

**Checkpoint**: US1 functional — backend serves health endpoints, DB connected, hot-reload works

---

## Phase 4: User Story 2 — Automated Quality Gates (Priority: P1)

**Goal**: PR-triggered CI workflows enforcing formatting, linting, type safety, and build correctness

**Independent Test**: Open a PR with a deliberate formatting violation; the CI pipeline fails and reports the specific issue

- [ ] T016 [US2] Create `.github/workflows/backend.yml` — Ruff lint, Black check, pytest with PostGIS docker service, timeout-minutes: 10
- [ ] T017 [US2] Create `.github/workflows/frontend.yml` — `tsc --noEmit`, web build, mobile typecheck, timeout-minutes: 10

**Checkpoint**: US2 functional — CI blocks bad PRs, passes clean ones, reports within 10 min

---

## Phase 5: User Story 3 — Admin Portal Scaffold (Priority: P2)

**Goal**: React/Vite admin with full-screen Leaflet map, CartoDB Positron tiles, floating panel placeholders, design tokens

**Independent Test**: Navigate to admin portal URL, see full-screen map with CartoDB Positron tiles and floating panel containers with design tokens

- [ ] T018 [P] [US3] Configure `web/tailwind.config.ts` and `web/postcss.config.js` with shadcn/ui theme
- [ ] T019 [P] [US3] Create shadcn/ui config in `web/components.json` and scaffold `web/src/components/ui/button.tsx` and `web/src/components/ui/card.tsx` with design token class bindings
- [ ] T020 [US3] Create `web/src/styles/globals.css` — Tailwind directives, design token CSS custom properties (accent green #22c55e, border-radius 2xl, backdrop blur, shadow), font imports
- [ ] T021 [US3] Create `web/src/main.tsx` (React DOM render) and `web/src/App.tsx` (full-screen layout, h-screen, map mount point, panel slots)
- [ ] T022 [P] [US3] Create Leaflet map wrapper in `web/src/components/map/MapView.tsx` — react-leaflet `MapContainer`, `TileLayer` with CartoDB Positron, `useMapEvents` for pan/zoom
- [ ] T023 [US3] Implement tile fallback: CartoDB Positron primary → OSM on error → warning banner at top of map viewport with red background, white text "Map tiles unavailable — using fallback tiles" (`web/src/components/map/TileFallback.tsx`)
- [ ] T024 [US3] Create floating panel containers in `web/src/components/panels/SearchPanel.tsx`, `StationListPanel.tsx`, `StationFormPanel.tsx` — design tokens applied (green accent border, rounded-2xl, backdrop-blur, shadow-lg)
- [ ] T025 [P] [US3] Create API client stub in `web/src/lib/api.ts` — base fetch wrapper for `http://localhost:8000`
- [ ] T026 [US3] Create stations feature placeholder: `web/src/features/stations/` with empty component stubs

**Checkpoint**: US3 functional — admin portal renders map, panels visible with design tokens, tile fallback works

---

## Phase 6: User Story 4 — Mobile App Placeholder (Priority: P3)

**Goal**: Expo Go app loading a responsive placeholder with tab navigation, safe area, design tokens

**Independent Test**: Scan Expo QR code, see responsive placeholder app shell with safe area margins and accent-colored elements

- [ ] T027 [US4] Create `mobile/app/_layout.tsx` — Expo Router tab layout with SafeAreaView, design tokens (green accent), status bar styling
- [ ] T027b [US4] Create `mobile/babel.config.js` with Expo preset and reanimated plugin (required for @gorhom/bottom-sheet in Phase 2)
- [ ] T028 [P] [US4] Create tab group layout in `mobile/app/(tabs)/_layout.tsx` with Expo Router TabNavigator before individual tab screens
- [ ] T029 [P] [US4] Create map tab in `mobile/app/(tabs)/index.tsx` — placeholder text, map container area, accent-colored header
- [ ] T030 [P] [US4] Create favorites tab in `mobile/app/(tabs)/favorites.tsx` — placeholder list, safe area, design tokens
- [ ] T031 [US4] Create placeholder map component in `mobile/components/map/MapView.tsx` — empty map container (react-native-maps ready for Phase 2), green accent border. Must forward a ref prop for snap-sheet compatibility (Phase 2 needs `bottomSheetRef`)
- [ ] T032 [P] [US4] Create API client stub in `mobile/lib/api.ts` — base fetch wrapper for backend

**Checkpoint**: US4 functional — mobile app loads in Expo Go, tabs render, hot-reload works

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Verify everything works together end-to-end

- [ ] T033 Update `specs/001-foundation-infrastructure/quickstart.md` — add verification checklist items: health endpoints respond, web map loads, mobile renders, hot-reload confirmed

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational — can then proceed in priority order (P1 → P1 → P2 → P3)
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1) Backend health**: No dependencies on other stories — starts after Foundational
- **US2 (P1) CI pipelines**: No dependencies on other stories — after Foundational
- **US3 (P2) Admin portal**: Depends on US1 (needs backend API for health check at runtime) but can be implemented independently
- **US4 (P3) Mobile app**: No dependencies on other stories — fully independent

### Parallel Opportunities

- Setup tasks T002, T003, T004 can run in parallel
- Foundational tasks T006, T007 can run in parallel
- Within US1: T010 and T015 can run in parallel with T009
- Within US3: T018, T019, T022, T025 can run in parallel
- Within US4: T028, T029, T030, T032 can run in parallel (T031 depends on react-native-maps import, not on tab layout)
- US2 and US3 can run in parallel after US1 completes

### Parallel Example: User Story 1

```bash
# Launch simultaneously:
Task: "Create FastAPI entry point in backend/app/main.py"
Task: "Implement health liveness router in backend/app/health/router.py"
Task: "Create stations feature placeholders in backend/app/stations/"
```

### Parallel Example: User Story 3

```bash
# Launch simultaneously:
Task: "Configure Tailwind CSS with design tokens"
Task: "Create shadcn/ui config in web/components.json"
Task: "Create Leaflet map wrapper in web/src/components/map/MapView.tsx"
Task: "Create API client stub in web/src/lib/api.ts"
```

---

## Implementation Strategy

### MVP First (US1 + US2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (backend health)
4. Complete Phase 4: User Story 2 (CI pipelines)
5. **STOP and VALIDATE**: Backend health + CI quality gates
6. Deploy/demo foundation

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add US1 + US2 → Health endpoints + CI (MVP!)
3. Add US3 → Admin portal scaffold (map + panels)
4. Add US4 → Mobile placeholder (Expo Go validation)
5. Each story adds value without breaking previous stories
