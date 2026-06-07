# Tasks: Sprint 10 — Partner Dashboard

**Input**: Design documents from `specs/010-partner-dashboard/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency configuration

- [X] T001 Add missing dependencies to `apps/partner-dashboard/package.json` (`@bornemap/api-client`, `@bornemap/auth-client`, `@tanstack/react-query`, `react-router`, `keycloak-js`, `class-variance-authority`)
- [X] T002 Update `apps/partner-dashboard/vite.config.ts` with proxy to localhost:80 and path alias
- [X] T003 Update `apps/partner-dashboard/index.html` with correct title
- [X] T004 Replace `apps/partner-dashboard/src/index.css` with clean Tailwind + design token CSS variables
- [X] T005 Add `ItemEnvelope` type to `packages/api-contracts/src/envelope.ts`
- [X] T006 Add optional `headers` parameter to `packages/api-client/src/index.ts` request methods

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared infrastructure that ALL user stories depend on

- [X] T007 Create `apps/partner-dashboard/src/lib/types.ts` — domain types (Station, Charger, Profile, envelopes)
- [X] T008 [P] Create `apps/partner-dashboard/src/lib/api.ts` — ApiClient singleton pointed at `/api/v1/partner`
- [X] T009 [P] Create `apps/partner-dashboard/src/lib/clickstream.ts` — event emission on `partner_dashboard` channel
- [X] T010 Create `apps/partner-dashboard/src/hooks/useAuth.tsx` — Keycloak auth context provider (initAuth, login, logout, getToken)
- [X] T011 Create `apps/partner-dashboard/src/components/ErrorBoundary.tsx` — React error boundary
- [X] T012 Create `apps/partner-dashboard/src/components/Modal.tsx` — Portal modal with overlay + Escape key
- [X] T013 Create `apps/partner-dashboard/src/main.tsx` — Root with QueryClient + BrowserRouter + AuthProvider + App
- [X] T014 Create `apps/partner-dashboard/src/components/Header.tsx` — Top nav with route links + user info + logout
- [X] T015 Create `apps/partner-dashboard/src/components/AuthGate.tsx` — Auth guard with login prompt
- [X] T016 Create `apps/partner-dashboard/src/App.tsx` — ErrorBoundary > AuthGate > Header > Routes

**Checkpoint**: Foundation ready — all user stories can now be implemented independently

---

## Phase 3: User Story 1 — Station Management (Priority: P1) 🎯 MVP

**Goal**: Partner can list, create, update, delete stations and toggle availability

**Independent Test**: `curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/partner/stations` returns paginated station list

### Backend

- [X] T017 Add `partner` router to `infra/compose/traefik/dynamic/routes.yml` — `PathPrefix(/api/v1/partner)` → admin-service (no stripPrefix)
- [X] T018 Create `infra/env/admin-service.env` with `DATABASE_URL=postgres://platform_user:change-me@postgres:5432/platform_db?sslmode=disable`
- [X] T019 Update `infra/compose/docker-compose.yml` admin-service `env_file` to `.env` (not `.example`); add `ADMIN_SERVICE_MIGRATIONS_DIR` env var; mount migrations volume
- [X] T020 Fix `services/admin-service/migrations/0016_seed_data.up.sql` — change `ON CONFLICT (id)` to `ON CONFLICT (keycloak_user_id)` for user insert
- [X] T021 Modify `services/admin-service/src/main.rs` — make `run_migrations` non-fatal (warn instead of `.expect`)
- [X] T022 Build admin-service debug binary and deploy into container

### Frontend — React Query Hooks

- [X] T023 [P] [US1] Create `apps/partner-dashboard/src/hooks/usePartnerStations.ts` — list/create/update/delete station queries and mutations
- [X] T024 [P] [US1] Create `apps/partner-dashboard/src/hooks/usePartnerAvailability.ts` — availability toggle mutation

### Frontend — Components

- [X] T025 [P] [US1] Create `apps/partner-dashboard/src/components/StationForm.tsx` — create/edit form with name, address, lat, lng, status fields
- [X] T026 [US1] Create `apps/partner-dashboard/src/pages/StationsPage.tsx` — list with pagination, create/edit/delete modals, inline availability dots, expandable chargers

**Checkpoint**: Partner can manage stations and toggle availability independently

---

## Phase 4: User Story 2 — Charger Management (Priority: P2)

**Goal**: Partner can list, create, update chargers on their stations

**Independent Test**: `curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/partner/chargers?station_id=STN-...` returns charger list for that station

### Frontend — React Query Hooks

- [X] T027 [P] [US2] Create `apps/partner-dashboard/src/hooks/usePartnerChargers.ts` — list/create/update charger queries and mutations

### Frontend — Components

- [X] T028 [P] [US2] Create `apps/partner-dashboard/src/components/ChargerForm.tsx` — create/edit form with charger type, power_kw fields
- [X] T029 [US2] Create `apps/partner-dashboard/src/pages/ChargersPage.tsx` — table view of all chargers with edit modal

**Checkpoint**: Partner can manage chargers across all stations independently

---

## Phase 5: User Story 3 — Profile (Priority: P3)

**Goal**: Partner can view their profile information

**Independent Test**: `curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/partner/me` returns partner profile JSON

### Frontend — React Query Hook

- [X] T030 [P] [US3] Create `apps/partner-dashboard/src/hooks/usePartnerProfile.ts` — profile query

### Frontend — Page

- [X] T031 [US3] Create `apps/partner-dashboard/src/pages/ProfilePage.tsx` — display partner name, email, role, IDs

**Checkpoint**: Partner can view profile independently

---

## Phase 6: Polish & Verification

**Purpose**: Build validation, endpoint verification, linting

- [X] T032 Run `npx tsc -b` in `apps/partner-dashboard` and fix type errors
- [X] T033 Run `npm run build` in `apps/partner-dashboard` and verify production build
- [X] T034 Verify all partner endpoints through Traefik (401 for unauthenticated, proper responses with token)
- [X] T035 Commit all changes and create PR against `main`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational + backend changes (T017-T022)
- **US2 (Phase 4)**: Depends on Foundational + US1 stations page (chargers appear in station detail)
- **US3 (Phase 5)**: Depends on Foundational only — no dependencies on other stories
- **Polish (Phase 6)**: Depends on all phases complete

### User Story Dependencies

- **US1 (P1)**: Foundational + backend Traefix/env fix
- **US2 (P2)**: Foundational + US1 (chargers rendered inside StationsPage)
- **US3 (P3)**: Foundational only — fully independent

### Parallel Opportunities

- T008, T009 — API client and clickstream (different files, no deps)
- T023, T024 — Station hooks and availability hooks
- T025, T026 — Station form component can be developed before StationsPage
- T027, T028 — Charger hooks and form can be developed together
- T030, T031 — Profile hook and page can be developed together

## Implementation Strategy

### MVP First (US1 Only)

1. Complete Setup + Foundational
2. Complete backend Traefik/env fix (T017-T022)
3. Complete US1 (Station Management) — MVP stations CRUD
4. STOP and validate with curl

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add US1 (Stations) → Test → MVP deploy
3. Add US2 (Chargers) → Test → Deploy
4. Add US3 (Profile) → Test → Deploy
