# Tasks: Sprint 11 — Admin Dashboard

**Input**: Design documents from `specs/011-admin-dashboard/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency configuration

- [X] T001 Add missing dependencies to `apps/admin-dashboard/package.json` (`@bornemap/api-client`, `@bornemap/auth-client`, `@bornemap/api-contracts`, `@bornemap/design-tokens`, `@bornemap/event-taxonomy`, `@tanstack/react-query`, `react-router`, `keycloak-js`, `class-variance-authority`)
- [X] T002 Update `apps/admin-dashboard/vite.config.ts` with proxy to localhost:80 and path alias; add `@tailwindcss/vite` plugin and migrate `tailwind.config.ts` to Tailwind v4 CSS syntax
- [X] T003 Update `apps/admin-dashboard/index.html` with correct title
- [X] T004 Replace `apps/admin-dashboard/src/index.css` with clean Tailwind + design token CSS variables

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared infrastructure that ALL user stories depend on

- [X] T005 Create `apps/admin-dashboard/src/lib/types.ts` — domain types (Partner, Station, Charger, Review, User, envelopes)
- [X] T006 [P] Create `apps/admin-dashboard/src/lib/api.ts` — ApiClient singleton pointed at `/api/v1/admin`
- [X] T007 [P] Create `apps/admin-dashboard/src/lib/clickstream.ts` — event emission on `admin_dashboard` channel
- [X] T008 Create `apps/admin-dashboard/src/hooks/useAuth.tsx` — Keycloak auth context provider (initAuth, login, logout, getToken)
- [X] T009 Create `apps/admin-dashboard/src/components/ErrorBoundary.tsx` — React error boundary
- [X] T010 Create `apps/admin-dashboard/src/components/Modal.tsx` — Portal modal with overlay + Escape key
- [X] T011 [P] Create `apps/admin-dashboard/src/components/Header.tsx` — Top nav with sidebar toggle + user info + logout
- [X] T012 [P] Create `apps/admin-dashboard/src/components/Sidebar.tsx` — 260px sidebar with nav items (Dashboard, Partners, Stations, Reviews, Users)
- [X] T013 Create `apps/admin-dashboard/src/components/Layout.tsx` — Sidebar + Header + main content wrapper
- [X] T014 Create `apps/admin-dashboard/src/components/AuthGate.tsx` — Auth guard with login prompt
- [X] T015 Create `apps/admin-dashboard/src/main.tsx` — Root with QueryClient + BrowserRouter + AuthProvider + App
- [X] T016 Create `apps/admin-dashboard/src/App.tsx` — ErrorBoundary > AuthGate > Layout > Routes

**Checkpoint**: Foundation ready — all user stories can now be implemented independently

---

## Phase 3: MVP (User Stories 1 & 2 — Priority: P1) 🎯

### User Story 1 — System Overview Dashboard (Priority: P1)

**Goal**: Admin sees a summary dashboard with key platform metrics at login

**Independent Test**: Authenticate as admin; navigate to `/`; verify metric cards render with correct counts

- [X] T017 [P] [US1] Create `apps/admin-dashboard/src/hooks/useAdminOverview.ts` — dashboard metrics query
- [X] T018 [P] [US1] Create `apps/admin-dashboard/src/components/DataCard.tsx` — metric card component
- [X] T019 [US1] Create `apps/admin-dashboard/src/pages/DashboardPage.tsx` — overview with DataCard grid, loading skeletons, error/retry, empty state

**Checkpoint**: Dashboard overview renders with platform counts

---

### User Story 2 — Partner Management (Priority: P1)

**Goal**: Admin can list, create, update, and soft-delete partners

**Independent Test**: `curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/admin/partners` returns paginated partner list; UI add/edit/delete workflows work

- [X] T020 [P] [US2] Create `apps/admin-dashboard/src/hooks/useAdminPartners.ts` — partner list/create/edit/delete queries and mutations
- [X] T021 [P] [US2] Create `apps/admin-dashboard/src/components/PartnerForm.tsx` — create/edit form with name, type, status fields
- [X] T022 [US2] Create `apps/admin-dashboard/src/pages/PartnersPage.tsx` — paginated list with create/edit/delete modals, status badges, blocked deletion error

**Checkpoint**: Partner CRUD works — MVP is deliverable with Dashboard + Partners

---

## Phase 4: User Story 3 — Station Management (Priority: P2)

**Goal**: Admin can view all stations across all partners, edit station details, soft-delete stations, and see inline chargers

**Independent Test**: `curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/admin/stations` returns paginated station list; UI edit/delete and charger expansion work

- [ ] T023 [P] [US3] Create `apps/admin-dashboard/src/hooks/useAdminStations.ts` — station list/edit/delete queries and mutations
- [ ] T024 [P] [US3] Create `apps/admin-dashboard/src/components/StationForm.tsx` — edit form with status, is_live, is_public, name, description, lat/lng (with confirmation dialog)
- [ ] T025 [US3] Create `apps/admin-dashboard/src/pages/StationsPage.tsx` — paginated list with partner name, status, city, charger count; expandable row for charger details; edit/delete modals; show-deleted toggle

**Checkpoint**: Station management works with charger visibility

---

## Phase 5: User Story 4 — Review Moderation (Priority: P2)

**Goal**: Admin can view all reviews and moderate their status through the lifecycle

**Independent Test**: `curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/admin/reviews` returns paginated review list; UI status transition works

- [ ] T026 [P] [US4] Create `apps/admin-dashboard/src/hooks/useAdminReviews.ts` — review list/moderate queries and mutations
- [ ] T027 [P] [US4] Create `apps/admin-dashboard/src/components/ReviewModeration.tsx` — status transition controls with lifecycle validation
- [ ] T028 [US4] Create `apps/admin-dashboard/src/pages/ReviewsPage.tsx` — paginated list with station, user, rating, comment preview, status badges, moderation actions

**Checkpoint**: Review moderation follows the correct lifecycle

---

## Phase 6: User Story 5 — User Management (Priority: P3)

**Goal**: Admin can view all registered users in a paginated table

**Independent Test**: `curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/admin/users` returns paginated user list

- [ ] T029 [P] [US5] Create `apps/admin-dashboard/src/hooks/useAdminUsers.ts` — user list query
- [ ] T030 [US5] Create `apps/admin-dashboard/src/pages/UsersPage.tsx` — paginated table with email, status, role, last login; search by email

**Checkpoint**: User list renders with search

---

## Phase 7: Polish & Verification

**Purpose**: Build validation, endpoint verification, linting

- [ ] T031 Run `npx tsc -b` in `apps/admin-dashboard` and fix type errors
- [ ] T032 Run `npm run build` in `apps/admin-dashboard` and verify production build
- [ ] T033 Verify all admin endpoints through Traefik (401 for unauthenticated, proper responses with token)
- [ ] T034 Commit all changes and create PR against `main`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 + US2 (Phase 3)**: Depends on Foundational — MVP (Dashboard + Partners)
- **US3 (Phase 4)**: Depends on Foundational — independent of US1/US2
- **US4 (Phase 5)**: Depends on Foundational — independent of US1/US2/US3
- **US5 (Phase 6)**: Depends on Foundational — independent of all other stories
- **Polish (Phase 7)**: Depends on all phases complete

### User Story Dependencies

- **US1 (P1) — Dashboard**: Foundational only — fully independent
- **US2 (P1) — Partners**: Foundational only — fully independent (both P1 stories are independent)
- **US3 (P2) — Stations**: Foundational only — fully independent
- **US4 (P2) — Reviews**: Foundational only — fully independent
- **US5 (P3) — Users**: Foundational only — fully independent

### Parallel Opportunities

- T006, T007 — API client and clickstream (different files, no deps)
- T011, T012 — Header and Sidebar (different files, no deps)
- T017, T018 — Dashboard hook and DataCard can be developed together
- T020, T021 — Partner hook and form can be developed together
- T023, T024 — Station hook and form can be developed together
- T026, T027 — Review hook and moderation component can be developed together
- T029, T030 — User hook and page can be developed together

## Implementation Strategy

### MVP First (US1 + US2 Only)

1. Complete Setup + Foundational
2. Complete US1 (Dashboard Overview) — metric cards
3. Complete US2 (Partner Management) — CRUD
4. STOP and validate with curl + browser

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add US1 + US2 (Dashboard + Partners) → Test → MVP deploy
3. Add US3 (Stations) → Test → Deploy
4. Add US4 (Reviews) → Test → Deploy
5. Add US5 (Users) → Test → Deploy

---

## Notes

- All admin API endpoints (`/api/v1/admin/*`) already exist from Sprint 5 — no backend changes needed
- Follow same coding patterns as the partner dashboard (`apps/partner-dashboard/`)
- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and independently testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently