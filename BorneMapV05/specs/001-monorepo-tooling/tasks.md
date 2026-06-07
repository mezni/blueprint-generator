---

description: "Task list for Sprint 1: Monorepo + Tooling Foundation"

---

# Tasks: Monorepo + Tooling Foundation

**Input**: Design documents from `/specs/001-monorepo-tooling/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not applicable — this is a build-tooling sprint. Acceptance is compilation-based
(no runtime business logic exists yet). See spec "Testing: Not applicable" and
plan.md "build verification is the acceptance gate."

**Organization**: Tasks are grouped by user story to enable independent implementation
and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Rust workspace**: `services/<name>/`, `crates/<name>/`
- **Frontend workspace**: `apps/<name>/`
- **TS packages**: `packages/<name>/`
- **Infrastructure**: `infra/compose/`, `infra/env/`
- **Configs**: Repository root (`./`)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Root workspace configuration and directory scaffolding

- [x] T001 Create top-level directories: `services/`, `crates/`, `apps/`, `packages/`, `infra/compose/traefik/dynamic/`, `infra/env/`

- [x] T002 Create `.nvmrc` at repository root with content `22`

- [x] T003 Create root `Cargo.toml` as Rust workspace with `[workspace]` and `members = [
  "services/driver-service",
  "services/admin-service",
  "services/clickstream-service",
  "services/gis-worker",
  "services/analytics-writer",
  "crates/common-types",
  "crates/common-errors",
  "crates/common-auth",
  "crates/common-db",
]` and `resolver = "2"`, `edition = "2024"`

- [x] T004 [P] Create root `package.json` with `"private": true`, `"workspaces": [
  "apps/driver-web",
  "apps/partner-dashboard",
  "apps/admin-dashboard",
  "apps/driver-mobile",
  "packages/shared-types",
  "packages/api-client",
  "packages/auth-client",
  "packages/design-tokens",
  "packages/event-taxonomy",
  "packages/api-contracts"
]` and scripts `"build": "npm run build --workspaces"`

- [x] T005 [P] Create `tsconfig.base.json` at repository root with strict TypeScript config (`strict: true`, `esModuleInterop: true`, `moduleResolution: "bundler"`, `target: "ES2022"`, `module: "ESNext"`, JSX settings for React)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared Rust library crates and shared TypeScript packages — these MUST be
complete before any user story can use them

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Rust Shared Library Crates

- [x] T006 [P] Create `crates/common-types/Cargo.toml` with lib target and `crates/common-types/src/lib.rs` defining:
  - `EntityPrefix` enum (`Usr`, `Prt`, `Stn`, `Chg`, `Rev`, `Evt`, `Clk`, `Sess`, `Anon`)
  - `Role` enum (`RegisteredDriver`, `Partner`, `Admin`)
  - `StationStatus`, `StationAvailabilityStatus`, `PartnerStatus`, `ChargerStatus`, `ChargerType`, `ReviewStatus`, `PartnerRole` enums
  - All with `Debug`, `Clone`, `Copy`, `PartialEq`, `Serialize`, `Deserialize` derives

- [x] T007 [P] Create `crates/common-errors/Cargo.toml` with dependency on `common-types` and `crates/common-errors/src/lib.rs` defining:
  - `ErrorCode` enum with all canonical codes (`Unauthenticated`, `Forbidden`, `TokenExpired`, `PartnerScopeViolation`, `InsufficientRole`, `NotFound`, `AlreadyExists`, `SoftDeleted`, `ValidationFailed`, `InvalidCoordinates`, `InvalidStateTransition`, `ActiveStationsExist`, `ReviewStateInvalid`)
  - Standard `Error` impl and `Display` impl

- [x] T008 [P] Create `crates/common-auth/Cargo.toml` (stub) and `crates/common-auth/src/lib.rs` with:
  - Placeholder `validate_token` function signature returning `Result<String, String>` (will be filled in Sprint 3)

- [x] T009 [P] Create `crates/common-db/Cargo.toml` (stub) and `crates/common-db/src/lib.rs` with:
  - Placeholder `get_pool` function signature returning `Result<(), String>` (will be filled in Sprint 4)

### TypeScript Shared Packages

- [x] T010 [P] Create `packages/shared-types/package.json` (name `@bornemap/shared-types`, `"main": "src/index.ts"`, `"types": "src/index.ts"`) and `packages/shared-types/tsconfig.json` extending `../../tsconfig.base.json`
  - `src/index.ts` exporting all type definitions from `data-model.md` (Role, StationStatus, ChargerStatus, GISQueueStatus, etc.)
  - `src/enums.ts` with enum constants
  - `src/ids.ts` with ID prefix type and utility function

- [x] T011 [P] Create `packages/api-client/package.json` (name `@bornemap/api-client`) with:
  - `src/index.ts` exporting a stub `ApiClient` class with method signatures for: `get`, `post`, `patch`, `delete` accepting a path and optional body, returning `Promise<unknown>`

- [x] T012 [P] Create `packages/auth-client/package.json` (name `@bornemap/auth-client`, stub) with:
  - `src/index.ts` exporting a `getToken()` function that returns a `Promise<string | null>` (implementation stubbed for Sprint 3)

- [x] T013 [P] Create `packages/design-tokens/package.json` (name `@bornemap/design-tokens`, empty shell) with:
  - `src/tokens.ts` exporting an empty `tokens` object
  - `src/colors.ts` exporting empty color palette placeholder
  - `src/typography.ts` exporting empty typography placeholder

- [x] T014 [P] Create `packages/event-taxonomy/package.json` (name `@bornemap/event-taxonomy`) with:
  - `src/envelope.ts` — canonical event envelope interface (event_id, event_version, schema_namespace, event_name, occurred_at, ingested_at, channel, session_id, correlation_id?, anonymous_id?, user_id?, actor_role?, path?, payload, metadata) matching `contracts/event-taxonomy.json`
  - `src/channels.ts` — `Channel` type: `"driver_web" | "driver_mobile" | "partner_dashboard" | "admin_dashboard"`
  - `src/events.ts` — enum/union of all 23 event names from the taxonomy catalog
  - `src/index.ts` — re-exports

- [x] T015 [P] Create `packages/api-contracts/package.json` (name `@bornemap/api-contracts`) with:
  - `src/envelope.ts` — generic `SuccessEnvelope<T>` and `ErrorEnvelope` interfaces, `PaginationMeta` interface
  - `src/errors.ts` — `ErrorCode` union type with all canonical codes
  - `src/index.ts` — re-exports

**Checkpoint**: Foundation ready — all shared crates and packages compile. User story
implementation can now begin in parallel.

---

## Phase 3: User Story 1 — Backend Engineers Set Up the Rust Workspace (Priority: P1)

**Goal**: All 5 Rust service binaries compile and run successfully with empty main functions.

**Independent Test**: Run `cargo build --workspace` from repo root — all 9 crates compile
with zero errors.

### Implementation for User Story 1

- [x] T016 [P] [US1] Create `services/driver-service/Cargo.toml` with binary target, dependency on `common-types`, `common-errors`, `common-auth`, `common-db`, and `services/driver-service/src/main.rs` with `fn main() { println!("driver-service ready"); }`

- [x] T017 [P] [US1] Create `services/admin-service/Cargo.toml` with binary target, same crate dependencies, and `services/admin-service/src/main.rs` with `fn main() { println!("admin-service ready"); }`

- [x] T018 [P] [US1] Create `services/clickstream-service/Cargo.toml` with binary target, same crate dependencies, and `services/clickstream-service/src/main.rs` with `fn main() { println!("clickstream-service ready"); }`

- [x] T019 [P] [US1] Create `services/gis-worker/Cargo.toml` with binary target, same crate dependencies, and `services/gis-worker/src/main.rs` with `fn main() { println!("gis-worker ready"); }`

- [x] T020 [P] [US1] Create `services/analytics-writer/Cargo.toml` with binary target, same crate dependencies, and `services/analytics-writer/src/main.rs` with `fn main() { println!("analytics-writer ready"); }`

**Checkpoint**: At this point, `cargo build --workspace` succeeds. Each service binary
runs and prints its name.

---

## Phase 4: User Story 2 — Frontend Engineers Bootstrap Application Shells (Priority: P1)

**Goal**: 3 React+Vite web apps render shell pages. 1 Expo mobile app launches with a
shell screen. All apps import from shared packages.

**Independent Test**: Each app starts with `npm run dev` and renders its name. Mobile
app launches in Expo Go showing "Driver Mobile."

### Implementation for User Story 2

- [x] T021 [P] [US2] Scaffold `apps/driver-web` — run Vite React-TS template, add `package.json` with name `@bornemap/driver-web`, dependencies on `@bornemap/shared-types`, `@bornemap/api-client`, `@bornemap/auth-client`, `@bornemap/design-tokens`. Update `App.tsx` to render `<h1>Driver Web App</h1>`. Update `tsconfig.json` to extend `../../tsconfig.base.json`.

- [x] T022 [P] [US2] Scaffold `apps/partner-dashboard` — same Vite React-TS template, same dependencies, update `App.tsx` to render `<h1>Partner Dashboard</h1>`. Extend base tsconfig.

- [x] T023 [P] [US2] Scaffold `apps/admin-dashboard` — same Vite React-TS template, same dependencies, update `App.tsx` to render `<h1>Admin Dashboard</h1>`. Extend base tsconfig.

- [x] T024 [P] [US2] Scaffold `apps/driver-mobile` — `npx create-expo-app`, rename directory to `driver-mobile`, update `package.json` name to `@bornemap/driver-mobile`, add dependencies on `@bornemap/shared-types`, `@bornemap/api-client`, `@bornemap/auth-client` and `@bornemap/design-tokens`. Update `App.tsx` to render `<Text>Driver Mobile</Text>`.

- [x] T025 [US2] Import a symbol from each shared package (`shared-types`, `api-client`, `auth-client`, `design-tokens`, `event-taxonomy`, `api-contracts`) in each app's entry point to verify workspace references resolve. This can be a single import line like `import {} from '@bornemap/shared-types'` — the import itself proves the reference works.

**Checkpoint**: Each app's dev server renders its shell. `npm install && npm run build --workspaces` succeeds.

---

## Phase 5: User Story 3 — Teams Use Shared Contracts Across the Stack (Priority: P2)

**Goal**: Event taxonomy and API contract types are mirrored in Rust and verified
cross-stack. A type change in one place propagates.

**Independent Test**: A type from `event-taxonomy` is imported in a Rust crate AND in a
web app AND in the mobile app — all compile with matching types.

### Implementation for User Story 3

- [x] T026 [US3] Mirror event taxonomy types in `crates/common-types/src/events.rs`:
  - `EventEnvelope` struct with fields matching `contracts/event-taxonomy.json`
  - `Channel` enum (`DriverWeb`, `DriverMobile`, `PartnerDashboard`, `AdminDashboard`)
  - `EventName` enum with all 23 variants
  - Re-export from `lib.rs`

- [x] T027 [US3] Mirror API contract types in `crates/common-types/src/api.rs`:
  - Generic `SuccessEnvelope<T>` struct
  - `ErrorEnvelope` struct (success=false variant)
  - `PaginationMeta` struct
  - Re-export from `lib.rs`

- [x] T028 [US3] In `services/driver-service/src/main.rs`, add a compile-time import verification: import `EventEnvelope` and `Channel` from `common-types`, create a `const CHANNEL: Channel = Channel::DriverWeb;` and `const _: () = assert!(matches!(CHANNEL, Channel::DriverWeb));` — this proves the Rust side compiles correctly.

- [x] T029 [US3] In `apps/driver-web/src/App.tsx`, add an import of `EventEnvelope` type from `@bornemap/event-taxonomy` and declare a typed constant — proves TS import works.

- [x] T030 [US3] In `apps/driver-mobile/App.tsx`, add an import of `SuccessEnvelope` type from `@bornemap/api-contracts` and declare a typed constant — proves mobile app import works.

**Checkpoint**: `cargo build --workspace` and `npm run build --workspaces` both pass.
The shared types are verified cross-stack.

---

## Phase 6: User Story 4 — Developers Preview Infrastructure Configuration (Priority: P3)

**Goal**: Docker Compose skeleton with all 9 services, internal networking, Traefik routing,
and per-service env templates.

**Independent Test**: `docker compose config` outputs valid YAML with all 9 services.

### Implementation for User Story 4

- [x] T031 [US4] Create `infra/compose/docker-compose.yml` with:
  - 5 backend services (driver-service, admin-service, clickstream-service, gis-worker, analytics-writer) — each with `build` context, internal network, no port exposure, health check placeholder that always passes
  - 4 infrastructure services: traefik (port 80, 443 exposed), keycloak (internal only, env vars from env file), postgres (internal only, 3 DBs), rabbitmq (internal only)
  - All services on `internal` network
  - `internal` network: `driver: bridge`, `internal: true`
  - DNS naming: `driver-service.internal`, `admin-service.internal`, etc.

- [x] T032 [P] [US4] Create `infra/compose/docker-compose.override.yml` for local development with volume mounts and port mappings for keycloak and databases

- [x] T033 [P] [US4] Create `infra/compose/traefik/traefik.yml` static config with entrypoints (web :80, websecure :443), providers (docker, file), and TLS configuration placeholder

- [x] T034 [P] [US4] Create `infra/compose/traefik/dynamic/routes.yml` with routing rules: `driver.*` → driver-service:8081, `admin.*` → admin-service:8082, `api.*` → backend, `auth.*` → keycloak:8080, `partner.*` → partner-dashboard

- [x] T035 [P] [US4] Create `infra/env/traefik.env.example` with `TRAEFIK_HTTP_PORT=80`, `TRAEFIK_HTTPS_PORT=443`, `TRAEFIK_TLS_ENABLED=false`, `TRAEFIK_DOMAIN_DRIVER=driver.example.tn`, `TRAEFIK_DOMAIN_PARTNER=partner.example.tn`, `TRAEFIK_DOMAIN_ADMIN=admin.example.tn`, `TRAEFIK_DOMAIN_API=api.example.tn`, `TRAEFIK_DOMAIN_AUTH=auth.example.tn`

- [x] T036 [P] [US4] Create `infra/env/keycloak.env.example` with `KEYCLOAK_HTTP_PORT=8080`, `KEYCLOAK_REALM=bornemap`, `KEYCLOAK_PUBLIC_URL=https://auth.example.tn`, admin bootstrap vars, DB connection vars

- [x] T037 [P] [US4] Create `infra/env/platform-db.env.example` with `PLATFORM_DB_HOST=postgres.internal`, `PLATFORM_DB_NAME=platform_db`, `PLATFORM_DB_USER=platform_user`, `PLATFORM_DB_PASSWORD=change-me`, `PLATFORM_DB_SSL_MODE=disable`, `PLATFORM_DB_MAX_CONNECTIONS=20`

- [x] T038 [P] [US4] Create `infra/env/analytics-db.env.example` with `ANALYTICS_DB_HOST=postgres.internal`, `ANALYTICS_DB_NAME=analytics_db`, `ANALYTICS_DB_USER=analytics_user`, `ANALYTICS_DB_PASSWORD=change-me`

- [x] T039 [P] [US4] Create `infra/env/rabbitmq.env.example` with `RABBITMQ_HOST=rabbitmq.internal`, `RABBITMQ_PORT=5672`, `RABBITMQ_USER=analytics`, `RABBITMQ_PASSWORD=change-me`, `RABBITMQ_VHOST=/bornemap`, exchange and queue names

- [x] T040 [P] [US4] Create `infra/env/driver-service.env.example` with `DRIVER_SERVICE_PORT=8081`, `APP_ENV=local`, `AUTH_ISSUER=...`, `AUTH_JWKS_URL=...`, `AUTH_AUDIENCE=bornemap-api`, `MAP_DEFAULT_LAT=36.8065`, `MAP_DEFAULT_LNG=10.1815`, `MAP_DEFAULT_RADIUS_KM=10`, `MAP_MAX_RADIUS_KM=50`

- [x] T041 [P] [US4] Create `infra/env/admin-service.env.example` with `ADMIN_SERVICE_PORT=8082`, `PARTNER_DELETE_BLOCK_ACTIVE_STATIONS=true`, `REPORTING_DEFAULT_WINDOW_DAYS=30`

- [x] T042 [P] [US4] Create `infra/env/analytics.env.example` (used by analytics-writer + clickstream-service) with `ANALYTICS_BATCH_SIZE=200`, `ANALYTICS_FLUSH_INTERVAL_MS=2000`, `ANALYTICS_RETENTION_DAYS=90`, `CLICKSTREAM_PORT=8083`, `CLICKSTREAM_BATCH_SIZE=100`, `CLICKSTREAM_ACCEPT_ANONYMOUS=true`, `CLICKSTREAM_ENFORCE_EVENT_ID=true`

**Checkpoint**: `docker compose config` validates the complete topology.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final build verification and validation against acceptance criteria

- [x] T043 Verify `docs/WORKSPACE_CONVENTIONS.md` exists with naming rules (kebab-case, service/crate/app/package conventions) and ownership boundaries

- [x] T044 Verify `AGENTS.md` references `specs/001-monorepo-tooling/plan.md` between SPECKIT markers

- [x] T045 Run `cargo build --workspace` — verify 0 errors, all 9 crates compile, measure ≤5 minutes

- [x] T046 Run `npm install && npm run build --workspaces` — verify 0 errors across all 4 apps + 6 packages

- [x] T047 Run `docker compose -f infra/compose/docker-compose.yml config` — verify valid YAML output with all 9 services listed

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - US1 (Phase 3) and US2 (Phase 4) are independent of each other — can run in parallel
  - US3 (Phase 5) depends on US1 (at least one Rust crate) and US2 (at least one frontend app)
  - US4 (Phase 6) is independent of all other stories — can run in parallel with US1/US2/US3
- **Polish (Phase 7)**: Depends on all phases being complete

### User Story Dependencies

- **US1 (P1) — Rust workspace**: Depends on Phase 1 + Phase 2 (shared crates). No dependency on other stories.
- **US2 (P1) — Frontend apps**: Depends on Phase 1 + Phase 2 (shared packages). No dependency on other stories.
- **US3 (P2) — Cross-stack contracts**: Depends on US1 (for Rust import) and US2 (for TS import).
- **US4 (P3) — Infrastructure**: No dependency on any user story. Can start after Phase 1.

### Within Each Phase

- Models/services before verification tasks
- [P] tasks within a phase can run in parallel
- Core implementation before integration

### Parallel Opportunities

- **Phase 1 (Setup)**: T004, T005 can run after T001
- **Phase 2 (Foundational)**: All 10 tasks (T006–T015) are fully parallel — they create independent files
- **Phase 3 (US1)**: All 5 tasks (T016–T020) are fully parallel — each is an independent crate
- **Phase 4 (US2)**: T021–T024 are parallel (independent apps); T025 depends on all 4 being scaffolded
- **Phase 5 (US3)**: T026–T027 are parallel (different modules); T028–T030 depend on those
- **Phase 6 (US4)**: T031 alone first, then T032–T042 are parallel
- **Phase 7 (Polish)**: T045, T046, T047 can run in parallel

---

## Parallel Example: Phase 3 (User Story 1)

```bash
# Launch all Rust service crates in parallel:
Task: T016 create services/driver-service/src/main.rs
Task: T017 create services/admin-service/src/main.rs
Task: T018 create services/clickstream-service/src/main.rs
Task: T019 create services/gis-worker/src/main.rs
Task: T020 create services/analytics-writer/src/main.rs
```

## Parallel Example: Phase 4 (User Story 2)

```bash
# Launch all Vite + Expo scaffolding in parallel:
Task: T021 scaffold apps/driver-web
Task: T022 scaffold apps/partner-dashboard
Task: T023 scaffold apps/admin-dashboard
Task: T024 scaffold apps/driver-mobile

# Then wire workspace references:
Task: T025 import shared packages in all apps
```

---

## Implementation Strategy

### MVP First (User Story 1 + 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (shared crates + shared packages)
3. Complete Phase 3 (US1): All 5 Rust service binaries → `cargo build`
4. Complete Phase 4 (US2): All 4 frontend app shells → `npm run build`
5. **STOP and VALIDATE**: Run T045 + T046 — both build systems compile
6. Basic monorepo is operational. US3 and US4 can be deferred without blocking.

### Incremental Delivery

1. Setup + Foundational → shared contracts ready
2. Add US1 (Rust workspace) → backend skeleton deployable → MVP checkpoint
3. Add US2 (Frontend apps) → full frontend+backend skeleton → Deployable
4. Add US3 (Cross-stack validation) → contract integrity verified
5. Add US4 (Infrastructure) → Docker Compose skeleton ready for Sprint 2

### Parallel Team Strategy

With multiple developers/agents:

1. Complete Phase 1 + Phase 2 together (or assign fully parallel)
2. Once Foundational is done:
   - Agent A: Phase 3 (US1) — Rust service binaries
   - Agent B: Phase 4 (US2) — Frontend app shells
   - Agent C: Phase 6 (US4) — Infrastructure scaffolding
3. Phase 5 (US3) — Cross-stack validation (done after US1 + US2)
4. Phase 7 (Polish) — Final verification

---

## Notes

- [P] tasks = different files, no dependencies
- [US1]–[US4] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- No runtime test tasks because the spec explicitly excludes testing for Sprint 1
  (build verification is the acceptance gate)
- Commit after each logical task group
- Stop at any checkpoint to validate story independently
- All file paths are relative to the repository root
