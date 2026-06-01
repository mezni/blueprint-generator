# Tasks: Monorepo Bootstrap

**Input**: Design documents from `/specs/002-monorepo-bootstrap/`
**Branch**: `002-monorepo-bootstrap`
**Date**: 2026-05-31

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Phase 1: User Story 1 — Monorepo Directory Structure (Priority: P1) 🎯 MVP

**Goal**: Create the complete directory tree matching EPIC 1 specification exactly — all top-level dirs, service dirs, crate dirs, app dirs, package dirs, and infrastructure scaffolding.

**Independent Test**: `ls -d apps/ services/ crates/ packages/ infra/ scripts/ docs/ .github/` from repo root returns all 8 dirs; each has the expected subdirectories per FR-002 through FR-005 and FR-015/FR-016.

- [X] T001 [P] Create top-level directories: `apps/`, `services/`, `crates/`, `packages/`, `infra/`, `scripts/`, `docs/`, `.github/`
- [X] T002 [P] Create `apps/driver-web/`, `apps/partner-dashboard/`, `apps/admin-dashboard/`, `apps/driver-mobile/`
- [X] T003 [P] Create `services/admin-service/`, `services/driver-service/`, `services/clickstream-service/`, `services/gis-sync-worker/`
- [X] T004 [P] Create `crates/contracts/`, `crates/common-auth/`, `crates/common-config/`, `crates/common-db/`, `crates/common-errors/`, `crates/common-types/`
- [X] T005 [P] Create `packages/design-system/`, `packages/api-client/`, `packages/analytics-client/`, `packages/auth-client/`
- [X] T006 [P] Create `infra/docker/` and `infra/compose/`
- [X] T007 [P] Create placeholder `infra/docker/Dockerfile` for each service (admin-service, driver-service, clickstream-service, gis-sync-worker) — no build logic
- [X] T008 Create placeholder `infra/compose/docker-compose.dev.yml` for local dev scaffolding

**Checkpoint**: Directory structure matches spec — all acceptance scenarios for US1 pass.

---

## Phase 2: User Story 2 — Rust Workspace with Shared Crates (Priority: P1)

**Goal**: Initialize root Cargo workspace, scaffold all 4 service stubs (main.rs + Cargo.toml) and all 6 shared library crates (lib.rs + Cargo.toml) so that `cargo build --workspace` compiles with zero errors.

**Independent Test**: `cargo build --workspace` from repo root completes with zero errors; each crate compiles independently.

- [X] T009 Create root `Cargo.toml` with `[workspace]` declaring members = `services/*`, `crates/*`, resolver = "2", edition = "2021"
- [X] T010 [P] [US2] Scaffold `services/admin-service/Cargo.toml` (binary) with `src/main.rs` stub — println!("admin-service")
- [X] T011 [P] [US2] Scaffold `services/driver-service/Cargo.toml` (binary) with `src/main.rs` stub — println!("driver-service")
- [X] T012 [P] [US2] Scaffold `services/clickstream-service/Cargo.toml` (binary) with `src/main.rs` stub — println!("clickstream-service")
- [X] T013 [P] [US2] Scaffold `services/gis-sync-worker/Cargo.toml` (binary) with `src/main.rs` stub — println!("gis-sync-worker")
- [X] T014 [P] [US2] Scaffold `crates/contracts/Cargo.toml` (library) with `src/lib.rs` stub
- [X] T015 [P] [US2] Scaffold `crates/common-auth/Cargo.toml` (library) with `src/lib.rs` stub
- [X] T016 [P] [US2] Scaffold `crates/common-config/Cargo.toml` (library) with `src/lib.rs` stub
- [X] T017 [P] [US2] Scaffold `crates/common-db/Cargo.toml` (library) with `src/lib.rs` stub
- [X] T018 [P] [US2] Scaffold `crates/common-errors/Cargo.toml` (library) with `src/lib.rs` stub
- [X] T019 [P] [US2] Scaffold `crates/common-types/Cargo.toml` (library) with `src/lib.rs` stub
- [X] T020 [US2] Verify `cargo build --workspace` compiles with zero errors

**Checkpoint**: Rust workspace fully functional — all 4 services and 6 crates compile.

---

## Phase 3: User Story 3 — Frontend Apps and Shared Packages (Priority: P1)

**Goal**: Scaffold 3 Vite + React web apps, 1 Expo mobile app, all 4 shared TypeScript packages, and root npm workspace so that `npm run build` succeeds for all apps and `tsc --noEmit` passes for all packages.

**Independent Test**: `npm run build` succeeds in each web app; `expo doctor` passes in driver-mobile; `tsc --noEmit` passes for all packages.

- [X] T021 Create root `package.json` with `workspaces: ["apps/*", "packages/*"]` and scripts for `build`, `lint`, `format`, `test`
- [X] T022 Create root `tsconfig.base.json` with strict mode, ES2022 target, and shared compiler options
- [X] T023 [P] [US3] Scaffold `apps/driver-web/` — `npm create vite` with React + TypeScript template, add build script
- [X] T024 [P] [US3] Scaffold `apps/partner-dashboard/` — `npm create vite` with React + TypeScript template, add build script
- [X] T025 [P] [US3] Scaffold `apps/admin-dashboard/` — `npm create vite` with React + TypeScript template, add build script
- [X] T026 [P] [US3] Scaffold `apps/driver-mobile/` — `npx create-expo-app` with TypeScript template
- [X] T027 [P] [US3] Initialize `packages/design-system/` with `package.json`, `tsconfig.json`, `src/index.ts` export stub
- [X] T028 [P] [US3] Initialize `packages/api-client/` with `package.json`, `tsconfig.json`, `src/index.ts` export stub
- [X] T029 [P] [US3] Initialize `packages/analytics-client/` with `package.json`, `tsconfig.json`, `src/index.ts` export stub
- [X] T030 [P] [US3] Initialize `packages/auth-client/` with `package.json`, `tsconfig.json`, `src/index.ts` export stub
- [X] T031 [US3] Verify `npm run build` succeeds for all 3 web apps
- [X] T032 [US3] Verify `expo doctor` passes for driver-mobile
- [X] T033 [US3] Verify `npx tsc --noEmit --project tsconfig.base.json` passes for all packages

**Checkpoint**: All frontend apps and packages compile successfully.

---

## Phase 4: User Story 4 — Shared Contract System (Priority: P2)

**Goal**: Populate `crates/contracts/` with DTOs (StationDTO, UserDTO, PartnerDTO, ReviewDTO), event schema (ClickstreamEventEnvelope + EventType enum with 9 variants), RBAC (Role enum with 3 variants), and NanoID ID types. Wire `packages/api-client/` as a typed TypeScript consumer.

**Independent Test**: A reviewer can verify no struct or DTO outside `crates/contracts/` duplicates a type defined there.

- [X] T034 [US4] Add `serde`, `chrono`, `uuid`, `nanoid` dependencies to `crates/contracts/Cargo.toml`
- [X] T035 [P] [US4] Define `Role` enum (`RegisteredDriver`, `Partner`, `Admin`) with `Serialize`/`Deserialize` in `crates/contracts/src/rbac.rs`
- [X] T036 [P] [US4] Define `StationDTO`, `UserDTO`, `PartnerDTO`, `ReviewDTO` with NanoID-prefixed `String` IDs in `crates/contracts/src/dto.rs`
- [X] T037 [P] [US4] Define `EventType` enum with 9 v1 variants (StationSearched, StationViewed, ChargingStarted, ChargingCompleted, ReviewSubmitted, PartnerStationCreated, PartnerStationUpdated, UserRegistered, ErrorOccurred) in `crates/contracts/src/events.rs`
- [X] T038 [P] [US4] Define `ClickstreamEventEnvelope` struct with `event_id`, `event_type`, `user_id`, `session_id`, `payload`, `timestamp`, `source`, `trace_id` in `crates/contracts/src/events.rs`
- [X] T039 [US4] Create `crates/contracts/src/lib.rs` re-exporting all public types from dto, events, rbac modules
- [X] T040 [US4] Verify `cargo build --workspace` still passes with contracts types populated
- [X] T041 [US4] Update `packages/api-client/src/index.ts` with TypeScript type stubs mirroring the Rust DTOs (StationDTO, UserDTO, PartnerDTO, ReviewDTO, EventType, ClickstreamEventEnvelope, Role)
- [X] T042 [US4] Verify no duplicate DTOs exist outside `crates/contracts/` — audit all service/crate `*.rs` files

**Checkpoint**: Contracts crate defines all cross-service types; api-client mirrors them in TypeScript; `cargo build` passes.

---

## Phase 5: User Story 5 — Tooling and Makefile (Priority: P2)

**Goal**: Create root Makefile with `build-all`, `test-all`, `lint-all`, `format-all` targets that delegate to Cargo and npm tooling. Verify all targets succeed from repo root.

**Independent Test**: `make lint-all && make build-all && make test-all` from repo root succeeds with zero errors.

- [X] T043 [US5] Create root `Makefile` with `.PHONY` declarations and `format-all` target (cargo fmt + prettier)
- [X] T044 [US5] Add `lint-all` target to Makefile (cargo clippy -- -D warnings + eslint)
- [X] T045 [US5] Add `build-all` target to Makefile (cargo build --workspace + vite build for all web apps)
- [X] T046 [US5] Add `test-all` target to Makefile (cargo test --workspace)
- [X] T047 [US5] Verify `make format-all` runs cargo fmt and prettier without errors
- [X] T048 [US5] Verify `make lint-all` passes cargo clippy and eslint without errors
- [X] T049 [US5] Verify `make build-all` completes cargo build + vite build sequentially
- [X] T050 [US5] Verify `make test-all` runs cargo test --workspace

**Checkpoint**: All 4 Makefile targets pass from repo root.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, documentation, and agent context updates.

- [X] T051 [P] Verify all 6 success criteria (SC-001 through SC-006) pass end-to-end
- [X] T052 Update `README.md` at repo root with monorepo overview, directory map, and build instructions
- [X] T053 Create `.gitignore` at repo root with Rust (`target/`), Node (`node_modules/`), and IDE entries
- [X] T054 Create `.editorconfig` at repo root for consistent formatting across editors

**Checkpoint**: Monorepo bootstrap fully complete and documented.

---

## Dependencies & Execution Order

### Phase Dependencies

- **US1 (Phase 1)**: No dependencies — start immediately. **MVP scope.**
- **US2 (Phase 2)**: Depends on US1 (directories must exist for Cargo packages)
- **US3 (Phase 3)**: Depends on US1 (directories must exist for apps/packages)
- **US4 (Phase 4)**: Depends on US1 + US2 (contracts crate must be a workspace member)
- **US5 (Phase 5)**: Depends on US2 + US3 (needs cargo + npm tooling to be present and working)
- **Polish (Phase 6)**: Depends on US1–US5 being complete

### Within Each User Story

- [P] tasks within a phase can run in parallel (different files, no dependencies)
- Sequence: project config > file creation > stub code > verification

### Parallel Opportunities

| Phase | [P] tasks | Can run together |
|-------|-----------|-----------------|
| US1 | T001–T008 | All directory creation tasks |
| US2 | T010–T019 | All service+crate scaffolding |
| US3 | T023–T030 | All app+package scaffolding |
| US4 | T035–T038 | All contract type definitions |
| US5 | — | Sequential (add target per target) |
| Polish | T051, T053, T054 | Independent tasks |

---

## Parallel Example: User Story 1

```bash
Task: "Create top-level directories in repo root"
Task: "Create app subdirectories in apps/"
Task: "Create service subdirectories in services/"
Task: "Create crate subdirectories in crates/"
Task: "Create package subdirectories in packages/"
Task: "Create infra subdirectories in infra/"
Task: "Create Dockerfile placeholders in infra/docker/"
```

## Parallel Example: User Story 2

```bash
Task: "Scaffold admin-service Cargo.toml + main.rs"
Task: "Scaffold driver-service Cargo.toml + main.rs"
Task: "Scaffold clickstream-service Cargo.toml + main.rs"
Task: "Scaffold gis-sync-worker Cargo.toml + main.rs"
Task: "Scaffold contracts crate Cargo.toml + lib.rs"
Task: "Scaffold common-auth crate Cargo.toml + lib.rs"
# ... plus remaining 4 crates
```

## Parallel Example: User Story 3

```bash
Task: "Scaffold driver-web with Vite"
Task: "Scaffold partner-dashboard with Vite"
Task: "Scaffold admin-dashboard with Vite"
Task: "Scaffold driver-mobile with Expo"
Task: "Initialize design-system package"
Task: "Initialize api-client package"
Task: "Initialize analytics-client package"
Task: "Initialize auth-client package"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: US1 — Directory structure (**this is the MVP**)
2. **STOP and VALIDATE**: Verify all 8 top-level dirs + subdirs exist
3. Deploy/demo: Reviewer confirms directory tree matches spec

### Incremental Delivery

1. US1 → Directory structure ready → MVP delivered
2. US2 → Rust workspace compiles → backend teams can start
3. US3 → Frontend apps scaffolded → frontend teams can start
4. US4 → Contract system in place → cross-service type safety
5. US5 → Makefile automation → CI-ready foundation
6. Polish → Final validation, docs, gitignore

### Parallel Team Strategy

With multiple developers:

1. Team: US1 together (directory structure — minutes of work)
2. Split:
   - Developer A: US2 (Rust workspace)
   - Developer B: US3 (Frontend apps + packages)
3. Both done → Merge → Developer C: US4 (Contracts) + US5 (Makefile)
4. Developer A/B: Polish tasks

---

## Notes

- [P] tasks = different files, no dependencies
- [USx] label maps task to specific user story for traceability
- No test tasks generated — spec does not request TDD approach
- Stop at any checkpoint to validate story independently
- All tasks are scaffolding/config only — zero runtime business logic
