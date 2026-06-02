---

description: "Task list for Monorepo Foundation Sprint 1"

---

# Tasks: Monorepo Foundation

**Input**: Design documents from `/specs/001-monorepo-foundation/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No test tasks included — tests not requested in this feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Backend services**: `services/<name>/`
- **Rust shared crates**: `crates/<name>/`
- **Web apps**: `apps/<name>/`
- **Mobile app**: `apps/driver-mobile/`
- **TypeScript packages**: `packages/<name>/`
- **Infrastructure**: `infra/compose/` and `infra/env/`
- **CI**: `.github/workflows/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and directory structure

- [ ] T001 Create repository directory structure per plan.md (apps/, services/, crates/, packages/, infra/, docs/, .github/)
- [ ] T002 [P] Create .gitignore with Rust, Node, Expo, and Docker patterns at repo root
- [ ] T003 Create WORKSPACE_CONVENTIONS.md in docs/ defining naming rules, ownership boundaries, and commit conventions

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build workspace infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Create Rust workspace root Cargo.toml with members for all services/ and crates/ directories
- [ ] T005 [P] Create npm workspace root package.json with workspaces for all apps/ and packages/ directories
- [ ] T006 [P] Create root tsconfig.json with strict mode base configuration for all TypeScript projects
- [ ] T007 [P] Create root .editorconfig for consistent formatting across languages

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel

---

## Phase 3: User Story 1 — Backend Service Compilation (Priority: P1) 🎯 MVP

**Goal**: Developer can clone and build all Rust backend services with a single command.

**Independent Test**: Clone the repo to a clean directory and run `cargo build` from root — all crates compile without errors.

### Implementation for User Story 1

- [ ] T008 [P] [US1] Create driver-service crate at services/driver-service/Cargo.toml with src/main.rs skeleton
- [ ] T009 [P] [US1] Create admin-service crate at services/admin-service/Cargo.toml with src/main.rs skeleton
- [ ] T010 [P] [US1] Create clickstream-service crate at services/clickstream-service/Cargo.toml with src/main.rs skeleton
- [ ] T011 [P] [US1] Create gis-worker crate at services/gis-worker/Cargo.toml with src/main.rs skeleton
- [ ] T012 [P] [US1] Create analytics-writer crate at services/analytics-writer/Cargo.toml with src/main.rs skeleton
- [ ] T013 [P] [US1] Create common-types crate at crates/common-types/Cargo.toml with src/lib.rs
- [ ] T014 [P] [US1] Create common-errors crate at crates/common-errors/Cargo.toml with src/lib.rs
- [ ] T015 [P] [US1] Create common-auth crate at crates/common-auth/Cargo.toml with src/lib.rs
- [ ] T016 [P] [US1] Create common-db crate at crates/common-db/Cargo.toml with src/lib.rs
- [ ] T017 [P] [US1] Create common-observability crate at crates/common-observability/Cargo.toml with src/lib.rs
- [ ] T018 [US1] Verify cargo build succeeds across workspace — fix any compilation errors

**Checkpoint**: At this point, User Story 1 should be fully functional — `cargo build` succeeds, all 10 crates compile

---

## Phase 4: User Story 2 — Frontend Application Boot (Priority: P1)

**Goal**: Developer can start each web app dev server and see a rendered page.

**Independent Test**: Start each app's dev server and open in browser — all three render a landing page with no console errors.

### Implementation for User Story 2

- [ ] T019 [P] [US2] Scaffold driver-web app at apps/driver-web/ using Vite + React + TypeScript template
- [ ] T020 [P] [US2] Scaffold partner-dashboard app at apps/partner-dashboard/ using Vite + React + TypeScript template
- [ ] T021 [P] [US2] Scaffold admin-dashboard app at apps/admin-dashboard/ using Vite + React + TypeScript template
- [ ] T022 [P] [US2] Configure React Router with basic layout shell in each app at apps/*/src/App.tsx
- [ ] T023 [US2] Verify all three apps start via npm run dev and render a functional page

**Checkpoint**: At this point, User Story 2 should be fully functional — all 3 web apps boot and render

---

## Phase 5: User Story 3 — Cross-Stack Shared Contracts (Priority: P2)

**Goal**: Developer can import shared types, API contracts, and event taxonomy across frontend and backend.

**Independent Test**: Create a test file in a frontend app that imports and uses a type from shared-types — compilation succeeds.

### Implementation for User Story 3

- [ ] T024 [P] [US3] Create shared-types package at packages/shared-types/ with tsconfig.json and src/index.ts
- [ ] T025 [P] [US3] Create api-client package at packages/api-client/ with standard response envelope types from contracts/api-envelope.md
- [ ] T026 [P] [US3] Create auth-client package at packages/auth-client/ with stub auth types
- [ ] T027 [P] [US3] Create event-taxonomy package at packages/event-taxonomy/ with event envelope interface matching data-model.md fields
- [ ] T028 [P] [US3] Create design-tokens package at packages/design-tokens/ with colors.ts, spacing.ts, typography.ts stubs
- [ ] T029 [US3] Import shared-types into driver-web at apps/driver-web/ and verify tsc --noEmit passes
- [ ] T030 [US3] Import event-taxonomy into at least one Rust service crate and verify cargo check passes

**Checkpoint**: At this point, User Story 3 should be fully functional — shared packages compile and import across stacks

---

## Phase 6: User Story 4 — Infrastructure Validation (Priority: P2)

**Goal**: Developer can validate Docker Compose configuration and environment variables.

**Independent Test**: Run `docker compose config` — passes with all services listed.

### Implementation for User Story 4

- [ ] T031 [P] [US4] Create base docker-compose.yml at infra/compose/docker-compose.yml with services: traefik, postgres, keycloak, rabbitmq, and backend service placeholders
- [ ] T032 [P] [US4] Define internal Docker network in docker-compose.yml (internal network only, no public ports beyond Traefik)
- [ ] T033 [P] [US4] Create infra/env/shared.env with global env vars (APP_ENV, LOG_LEVEL, etc.)
- [ ] T034 [P] [US4] Create infra/env/driver-service.env with service-specific env vars
- [ ] T035 [P] [US4] Create infra/env/admin-service.env with service-specific env vars
- [ ] T036 [P] [US4] Create infra/env/clickstream-service.env with service-specific env vars
- [ ] T037 [P] [US4] Create infra/env/gis-worker.env with service-specific env vars
- [ ] T038 [P] [US4] Create infra/env/analytics-writer.env with service-specific env vars
- [ ] T039 [US4] Verify docker compose config passes with no errors
- [ ] T040 [US4] Verify each service env file has no hardcoded values — all values reference environment variables

**Checkpoint**: At this point, User Story 4 should be fully functional — Docker Compose validates, env system ready

---

## Phase 7: User Story 5 — Health Check Confirmation (Priority: P3)

**Goal**: Operator can query each service's /health endpoint and receive consistent JSON.

**Independent Test**: Send HTTP GET to each service's /health endpoint — 200 status with consistent JSON envelope.

### Implementation for User Story 5

- [ ] T041 [P] [US5] Implement /health endpoint in driver-service at services/driver-service/src/main.rs returning standard health envelope
- [ ] T042 [P] [US5] Implement /health endpoint in admin-service at services/admin-service/src/main.rs returning standard health envelope
- [ ] T043 [P] [US5] Implement /health endpoint in clickstream-service at services/clickstream-service/src/main.rs returning standard health envelope
- [ ] T044 [P] [US5] Add HTTP framework dependency (axum or actix-web) to all three service Cargo.toml files
- [ ] T045 [US5] Verify cargo build succeeds after health endpoint additions

**Checkpoint**: At this point, User Story 5 should be fully functional — 3 services respond to /health

---

## Phase 8: User Story 6 — Mobile App Launch (Priority: P3)

**Goal**: Developer can start the Expo mobile app on device or emulator without crash.

**Independent Test**: Start Expo dev server and open on emulator — app renders a default screen with no crash.

### Implementation for User Story 6

- [ ] T046 [P] [US6] Initialize Expo app at apps/driver-mobile/ with TypeScript template
- [ ] T047 [P] [US6] Create basic navigation shell at apps/driver-mobile/src/App.tsx with a default screen
- [ ] T048 [US6] Verify app launches on iOS simulator without crash
- [ ] T049 [US6] Verify app launches on Android emulator without crash
- [ ] T050 [US6] Verify tsc --noEmit passes in apps/driver-mobile/

**Checkpoint**: At this point, User Story 6 should be fully functional — mobile app boots on both platforms

---

## Phase 9: User Story 7 — CI Pipeline Verification (Priority: P3)

**Goal**: Developer pushes changes and CI automatically runs build and typecheck jobs.

**Independent Test**: Push a commit to a branch — CI triggers and all jobs pass.

### Implementation for User Story 7

- [ ] T051 [P] [US7] Create GitHub Actions workflow at .github/workflows/rust-build.yml for Rust workspace cargo build
- [ ] T052 [P] [US7] Create GitHub Actions workflow at .github/workflows/frontend-build.yml for frontend npm run build
- [ ] T053 [P] [US7] Create GitHub Actions workflow at .github/workflows/typecheck.yml for tsc --noEmit across all TypeScript projects
- [ ] T054 [P] [US7] Create placeholder Docker build workflow at .github/workflows/docker-build.yml
- [ ] T055 [US7] Verify all CI workflows parse correctly via GitHub Actions dry-run or push to branch

**Checkpoint**: At this point, User Story 7 should be fully functional — CI runs on push

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T056 [P] Update docs/README.md with project overview, architecture summary, and link to quickstart
- [ ] T057 [P] Run full build validation: cargo build + npm run build (all apps) + tsc --noEmit — fix all issues
- [ ] T058 [P] Remove any placeholder comments or TODO markers from scaffolded code
- [ ] T059 Run quickstart.md validation — verify every command in quickstart works on a clean clone

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed) or sequentially in priority order
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational — No dependencies on other stories 🎯 MVP
- **User Story 2 (P1)**: Can start after Foundational — No dependencies on other stories
- **User Story 3 (P2)**: Can start after Foundational — Depends on US1 for Rust-side import validation
- **User Story 4 (P2)**: Can start after Foundational — No dependencies on other stories
- **User Story 5 (P3)**: Depends on US1 (needs service crates) and US3 (needs api-envelope types)
- **User Story 6 (P3)**: Can start after Foundational — No dependencies on other stories
- **User Story 7 (P3)**: Depends on US1, US2, US3 (needs working builds to validate CI)

### Within Each User Story

- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, US1, US2, US4, and US6 can all start in parallel
- All tasks within a user story marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all service crates together:
Task: "T008 [P] [US1] Create driver-service crate at services/driver-service/Cargo.toml"
Task: "T009 [P] [US1] Create admin-service crate at services/admin-service/Cargo.toml"
Task: "T010 [P] [US1] Create clickstream-service crate at services/clickstream-service/Cargo.toml"
Task: "T011 [P] [US1] Create gis-worker crate at services/gis-worker/Cargo.toml"
Task: "T012 [P] [US1] Create analytics-writer crate at services/analytics-writer/Cargo.toml"

# Launch all shared crates together:
Task: "T013 [P] [US1] Create common-types crate at crates/common-types/Cargo.toml"
Task: "T014 [P] [US1] Create common-errors crate at crates/common-errors/Cargo.toml"
Task: "T015 [P] [US1] Create common-auth crate at crates/common-auth/Cargo.toml"
Task: "T016 [P] [US1] Create common-db crate at crates/common-db/Cargo.toml"
Task: "T017 [P] [US1] Create common-observability crate at crates/common-observability/Cargo.toml"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: User Story 1 (Backend Service Compilation)
4. **STOP and VALIDATE**: Verify `cargo build` succeeds
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1: Backend compiles — RUN `cargo build` to validate
3. Add User Story 2: Frontend boots — RUN `npm run dev` in each app
4. Add User Story 3: Shared contracts importable — RUN `tsc --noEmit`
5. Add User Story 4: Infrastructure validates — RUN `docker compose config`
6. Add User Story 5: Health endpoints — RUN `curl /health` on each service
7. Add User Story 6: Mobile app launches — RUN `npx expo start`
8. Add User Story 7: CI passes — PUSH to trigger workflows

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 (Backend) + US5 (Health) sequentially
   - Developer B: US2 (Frontend) + US6 (Mobile) sequentially
   - Developer C: US3 (Contracts) + US4 (Infra) + US7 (CI) sequentially
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
