# Tasks: Runtime Foundation

**Input**: Design documents from `specs/002-runtime-foundation/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No tests requested in the feature specification — only implementation tasks.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Infrastructure**: `infra/compose/`, `infra/env/`, `infra/postgres/`, `infra/rabbitmq/`, `infra/keycloak/`
- **Rust services**: `services/{name}/src/main.rs`, `services/{name}/Cargo.toml`
- **Shared crates**: `crates/common-*/`
- **Scripts**: `scripts/smoke-test.sh`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create infrastructure directories, environment profile stubs, and init script directories

- [ ] T001 Create postgres init directory at `infra/postgres/init/`
- [ ] T002 Create rabbitmq init directory at `infra/rabbitmq/init/`
- [ ] T003 Create keycloak realm directory at `infra/keycloak/realm-export/`
- [ ] T004 Create environment profile directories at `infra/env/local/` and `infra/env/docker/`
- [ ] T005 Create scripts directory at `scripts/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared infrastructure code that all services depend on — config loader trait, observability initialization, Cargo dependency updates

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T006 Update workspace `Cargo.toml` to add `sqlx`, `lapin`, `tracing`, `tracing-subscriber` to workspace dependencies
- [ ] T007 [P] Implement `ConfigLoader` trait in `crates/common-config/src/lib.rs` with required env var validation
- [ ] T008 [P] Implement `ConfigSource` enum (env_vars, file) in `crates/common-config/src/source.rs`
- [ ] T009 [P] Add `tracing` + `tracing-subscriber` JSON initialization to `crates/common-observability/src/lib.rs`
- [ ] T010 [P] Add `SERVICE_VERSION` and `SERVICE_NAME` compile-time constants to `crates/common-observability/src/lib.rs`
- [ ] T011 Add `common-config` and `common-observability` as workspace members in root `Cargo.toml`

**Checkpoint**: Foundation ready — shared config and logging crates exist and compile

---

## Phase 3: User Story 1 — Developer boots the full platform locally (Priority: P1) 🎯 MVP

**Goal**: Single `docker compose up` starts all 9 containers (PostgreSQL, RabbitMQ, Keycloak, Traefik, 5 Rust services) with health checks, volumes, and startup dependencies.

**Independent Test**: Run `docker compose up` from project root — all 9 containers show `healthy` within 120 seconds, `docker compose ps` shows restart count 0 for all.

### Docker Compose & Infrastructure

- [ ] T012 [P] [US1] Write PostgreSQL `Dockerfile` and init script `01-create-databases.sh` in `infra/postgres/init/` to create `keycloak_db`, `users_db`, `inventory_db`, `analytics_db`
- [ ] T013 [P] [US1] Write RabbitMQ `Dockerfile` and definitions file `definitions.json` in `infra/rabbitmq/init/` with queues `clickstream.raw`, `gis.sync`, `analytics.ingest`
- [ ] T014 [P] [US1] Export Keycloak realm JSON for `ev-platform` realm with placeholder clients (`driver-web`, `partner-dashboard`, `admin-dashboard`, `driver-mobile`) and save to `infra/keycloak/realm-export/ev-platform-realm.json`
- [ ] T015 [US1] Write complete `docker-compose.yml` at `infra/compose/docker-compose.yml` with all 9 service definitions, health checks, volumes, container names, networks, restart policies, and startup dependencies (`depends_on` with condition `service_healthy`)
- [ ] T016 [P] [US1] Write Traefik configuration labels in `docker-compose.yml` for `/api/driver`, `/api/admin`, `/auth` routing, with entrypoints and middleware structure
- [ ] T017 [US1] Write per-service `Dockerfile.build` (multi-stage workspace-aware) at each `services/*/Dockerfile` that compiles the whole workspace but runs only the target binary

### Environment Files

- [ ] T018 [P] [US1] Create `infra/env/local/postgres.env` with local PostgreSQL credentials
- [ ] T019 [P] [US1] Create `infra/env/local/rabbitmq.env` with local RabbitMQ credentials
- [ ] T020 [P] [US1] Create `infra/env/local/keycloak.env` with local Keycloak bootstrap credentials
- [ ] T020b [P] [US1] Add `KC_DB=postgres`, `KC_DB_URL=jdbc:postgresql://postgres.internal:5432/keycloak_db`, `KC_DB_USERNAME=keycloak`, `KC_DB_PASSWORD=change-me` to `infra/env/local/keycloak.env`
- [ ] T021 [P] [US1] Create `infra/env/local/traefik.env` with local Traefik config
- [ ] T022 [P] [US1] Create `infra/env/local/driver-service.env` with local driver-service config
- [ ] T023 [P] [US1] Create `infra/env/local/admin-service.env` with local admin-service config
- [ ] T024 [P] [US1] Create `infra/env/local/clickstream-service.env` with local clickstream-service config
- [ ] T025 [P] [US1] Create `infra/env/local/gis-worker.env` with local gis-worker config
- [ ] T026 [P] [US1] Create `infra/env/local/analytics-writer.env` with local analytics-writer config
- [ ] T027 [P] [US1] Create corresponding env files under `infra/env/docker/` — identical to local except internal DNS hostnames (e.g., `postgres.internal`) and no host port exposure for management UIs
- [ ] T028 [US1] Update root `.env.example` with all documented variables and clear comments

### Verification

- [ ] T029 [US1] Validate `docker compose -f infra/compose/docker-compose.yml config` succeeds
- [ ] T030 [US1] Run `docker compose build` and fix any Dockerfile issues

**Checkpoint**: `docker compose up` boots all 9 containers healthy with no restart loops.

---

## Phase 4: User Story 2 — Operator verifies service health and readiness (Priority: P1)

**Goal**: All 5 Rust services expose `/health` (static liveness) and `/ready` (dependency-aware readiness) endpoints per the contracts in `contracts/health-endpoint.md` and `contracts/ready-endpoint.md`.

**Independent Test**: Curl each service's `/health` → HTTP 200 with service metadata. Curl `/ready` when dependencies are up → HTTP 200 with dependency list. Tear down a dependency → `/ready` returns HTTP 503.

- [ ] T031 [US2] Add `common-config` and `axum` dependencies to `services/driver-service/Cargo.toml` for config-driven port binding
- [ ] T032 [US2] Implement `/health` handler returning `{"status":"ok","service":"<name>","version":"<version>"}` in `services/driver-service/src/main.rs`
- [ ] T033 [US2] Implement `/ready` handler with dependency check for PostgreSQL in `services/driver-service/src/main.rs`
- [ ] T034 [US2] Implement `/health` handler in `services/admin-service/src/main.rs`
- [ ] T035 [US2] Implement `/ready` handler with dependency check for PostgreSQL in `services/admin-service/src/main.rs`
- [ ] T036 [US2] Implement `/health` handler in `services/clickstream-service/src/main.rs`
- [ ] T037 [US2] Implement `/ready` handler with dependency check for RabbitMQ in `services/clickstream-service/src/main.rs`
- [ ] T038 [US2] Implement `/health` handler in `services/gis-worker/src/main.rs`
- [ ] T039 [US2] Implement `/ready` handler with dependency checks for PostgreSQL + RabbitMQ in `services/gis-worker/src/main.rs`
- [ ] T040 [US2] Implement `/health` handler in `services/analytics-writer/src/main.rs`
- [ ] T041 [US2] Implement `/ready` handler with dependency checks for PostgreSQL + RabbitMQ in `services/analytics-writer/src/main.rs`
- [ ] T042 [US2] Verify `cargo build --workspace` succeeds with new handlers

**Checkpoint**: All 5 services serve `/health` and `/ready` endpoints conforming to the contract specs.

---

## Phase 5: User Story 3 — Developer configures a service via environment files (Priority: P2)

**Goal**: Each service reads its port, DB/RabbitMQ credentials, log level, and auth config from environment variables. Missing required vars cause immediate startup failure with clear error messages.

**Independent Test**: Start a service with a missing required env var → crashes with error naming the variable. Start with valid config → logs redacted config summary. Change `SERVICE_PORT` in env file → service listens on new port.

- [ ] T043 [P] [US3] Implement `DriverServiceConfig` struct with serde Deserialize + manual validation in `services/driver-service/src/config.rs`
- [ ] T044 [P] [US3] Implement `AdminServiceConfig` struct in `services/admin-service/src/config.rs`
- [ ] T045 [P] [US3] Implement `ClickstreamServiceConfig` struct in `services/clickstream-service/src/config.rs`
- [ ] T046 [P] [US3] Implement `GisWorkerConfig` struct in `services/gis-worker/src/config.rs`
- [ ] T047 [P] [US3] Implement `AnalyticsWriterConfig` struct in `services/analytics-writer/src/config.rs`
- [ ] T048 [US3] Wire config loading into `services/driver-service/src/main.rs` — fail fast on startup if validation fails, log redacted summary
- [ ] T049 [US3] Wire config loading into `services/admin-service/src/main.rs`
- [ ] T050 [US3] Wire config loading into `services/clickstream-service/src/main.rs`
- [ ] T051 [US3] Wire config loading into `services/gis-worker/src/main.rs`
- [ ] T052 [US3] Wire config loading into `services/analytics-writer/src/main.rs`
- [ ] T053 [US3] Verify `cargo build --workspace` succeeds — all services crash with clear message on missing env var

**Checkpoint**: All 5 services load and validate configuration at startup. Missing vars = immediate crash with actionable error.

---

## Phase 6: User Story 4 — Developer accesses infrastructure UIs (Priority: P2)

**Goal**: Keycloak admin console, RabbitMQ management UI, and Traefik dashboard are accessible via browser in the `local` profile. Internal DNS names resolve correctly.

**Independent Test**: Navigate to `http://localhost:8080` (Keycloak), `http://localhost:15672` (RabbitMQ), `http://localhost:8080/dashboard/` (Traefik) — all load correctly.

- [ ] T054 [P] [US4] Add `local` profile port mappings in `docker-compose.yml` for Keycloak (`8080:8080`), RabbitMQ (`15672:15672`), Traefik dashboard (`8080:8080`)
- [ ] T055 [P] [US4] Configure Docker internal network `bornemap-net` with DNS resolution in `docker-compose.yml`
- [ ] T056 [P] [US4] Add container_name and network aliases for DNS resolution (`postgres.internal`, `rabbitmq.internal`, `keycloak.internal`, `driver.internal`, `admin.internal`, `clickstream.internal`, `gis.internal`, `analytics.internal`)
- [ ] T057 [US4] Add Traefik middleware and route rules for `/api/driver`, `/api/admin`, `/auth` in `docker-compose.yml` labels
- [ ] T058 [US4] Verify Traefik dashboard shows all 3 routes as "UP" after boot
- [ ] T059 [US4] Verify DNS resolution from within containers: `docker compose exec driver-service ping postgres.internal`

**Checkpoint**: All management UIs accessible, internal DNS resolves, Traefik routes forward correctly.

---

## Phase 7: User Story 5 — Operator observes service startup lifecycle (Priority: P3)

**Goal**: Service startup emits structured JSON logs showing lifecycle stages: config load → dependency check → route registration → ready state.

**Independent Test**: Restart any service and observe logs — 4 lifecycle stage entries appear in order with JSON fields.

- [ ] T060 [P] [US5] Initialize `tracing-subscriber` JSON format in `services/driver-service/src/main.rs` with service name and env fields
- [ ] T061 [P] [US5] Initialize `tracing-subscriber` JSON format in `services/admin-service/src/main.rs`
- [ ] T062 [P] [US5] Initialize `tracing-subscriber` JSON format in `services/clickstream-service/src/main.rs`
- [ ] T063 [P] [US5] Initialize `tracing-subscriber` JSON format in `services/gis-worker/src/main.rs`
- [ ] T064 [P] [US5] Initialize `tracing-subscriber` JSON format in `services/analytics-writer/src/main.rs`
- [ ] T065 [US5] Add lifecycle log entries at each boot stage in all services: `"stage":"config_load"`, `"stage":"dependency_check"`, `"stage":"route_registration"`, `"stage":"ready"`
- [ ] T066 [US5] Verify `cargo build --workspace` succeeds with tracing initialization
- [ ] T067 [US5] Start a service and verify JSON log output parses cleanly with `jq`

**Checkpoint**: All service startup logs show structured JSON with lifecycle stages.

---

## Phase 8: DB & RabbitMQ Connectivity Layers

**Goal**: Services that depend on PostgreSQL or RabbitMQ establish connections at startup with retry logic, timeout handling, and clean failure.

**Independent Test**: Start platform normally → service logs show `"dependency_check"` → `"connected"`. Start without PostgreSQL → services crash with clear connection error.

- [ ] T068 [P] Implement PostgreSQL connection bootstrap in `services/driver-service/src/db.rs` using `sqlx::PgPool::connect_with`
- [ ] T069 [P] Implement PostgreSQL connection bootstrap in `services/admin-service/src/db.rs`
- [ ] T070 [P] Implement PostgreSQL connection bootstrap in `services/analytics-writer/src/db.rs`
- [ ] T071 [P] Implement RabbitMQ connection bootstrap in `services/clickstream-service/src/rabbitmq.rs` using `lapin::Connection::connect`
- [ ] T072 [P] Implement RabbitMQ connection bootstrap in `services/gis-worker/src/rabbitmq.rs`
- [ ] T073 [P] Implement RabbitMQ connection bootstrap in `services/analytics-writer/src/rabbitmq.rs`
- [ ] T074 Wire PostgreSQL pool into `/ready` dependency check for driver-service at `services/driver-service/src/main.rs`
- [ ] T075 Wire PostgreSQL pool into `/ready` dependency check for admin-service at `services/admin-service/src/main.rs`
- [ ] T076 Wire PostgreSQL pool into `/ready` dependency check for analytics-writer at `services/analytics-writer/src/main.rs`
- [ ] T077 Wire RabbitMQ connection into `/ready` dependency check for clickstream-service at `services/clickstream-service/src/main.rs`
- [ ] T078 Wire RabbitMQ connection into `/ready` dependency check for gis-worker at `services/gis-worker/src/main.rs`
- [ ] T079 Wire RabbitMQ connection into `/ready` dependency check for analytics-writer at `services/analytics-writer/src/main.rs`
- [ ] T080 Verify `cargo build --workspace` succeeds with all connectivity layers

**Checkpoint**: All services connect to their dependencies at startup, report status via `/ready`, and fail cleanly on connection failure.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Environment profiles, smoke tests, and final verification

### Environment Profiles & Boot Ordering

- [ ] T081 Complete `infra/env/docker/` profile env files — all using internal DNS hostnames, no host port exposure
- [ ] T082 Complete `infra/env/local/` profile env files — host-mapped ports for debugging
- [ ] T083 Create `infra/env/staging/` profile as placeholder (copy of docker profile)
- [ ] T084 Ensure `depends_on` with `condition: service_healthy` enforces boot order: PostgreSQL → RabbitMQ → Keycloak → Traefik → Application services
- [ ] T085 Verify profile switching works: change `APP_ENV` env var, restart, service picks up different env file

### Smoke Test Script

- [ ] T086 Write `scripts/smoke-test.sh` — validates: DB connectivity (`pg_isready`), RabbitMQ queue list, Keycloak realm availability, Traefik route health, and all service `/health` + `/ready` endpoints
- [ ] T087 Add smoke test instructions to `quickstart.md` at `specs/002-runtime-foundation/quickstart.md`

### Final Verification

- [ ] T088 Run `cargo build --workspace` — final clean build
- [ ] T089 Run `cargo clippy --workspace` — no warnings
- [ ] T090 Run `cargo test --workspace` — all tests pass
- [ ] T091 Run `docker compose -f infra/compose/docker-compose.yml config` — valid compose file
- [ ] T092 Run `bash scripts/smoke-test.sh` — exit code 0

**Checkpoint**: All artifacts complete, builds pass, smoke test passes.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 — Docker Compose (Phase 3)**: Depends on Foundational (needs `common-config` crate)
- **US2 — Health/Ready (Phase 4)**: Depends on Foundational (needs config for port binding)
- **US3 — Config Loading (Phase 5)**: Depends on Foundational (uses `common-config` trait)
- **US4 — Networking (Phase 6)**: Depends on US1 (needs compose file with services)
- **US5 — Logging (Phase 7)**: Depends on Foundational (uses `common-observability`)
- **DB/RMQ Connectivity (Phase 8)**: Depends on US2 + US3 (needs config + health endpoint structure)
- **Polish (Phase 9)**: Depends on all user stories

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — No dependencies on other stories
- **US2 (P1)**: Can start after Phase 2 — depends on US1 for Docker compose but can be developed independently
- **US3 (P2)**: Can start after Phase 2 — independent of other stories
- **US4 (P2)**: Depends on US1 (Docker Compose with networking)
- **US5 (P3)**: Can start after Phase 2 — independent of other stories
- **DB/RMQ (Phase 8)**: Depends on US2 (health endpoint structure) + US3 (config)

### Parallel Opportunities

- All env file creation tasks (T018-T028) can run in parallel
- All service config struct tasks (T043-T047) can run in parallel
- All health handler tasks (T031-T041) can run in parallel
- All logging init tasks (T060-T064) can run in parallel
- All DB bootstrap tasks (T068-T070) can run in parallel
- All RMQ bootstrap tasks (T071-T073) can run in parallel

---

## Parallel Example: Phase 3 (US1)

```bash
# Infrastructure init scripts in parallel:
Task: T012 — Write PostgreSQL init script
Task: T013 — Write RabbitMQ definitions
Task: T014 — Export Keycloak realm JSON

# Per-service Dockerfiles in parallel:
Task: T017 — Write Dockerfile for driver-service
   (repeat for all 5 services — same structure)
```

## Parallel Example: Phase 4 (US2) + Phase 5 (US3)

```bash
# Health handlers in parallel (different services):
Task: T032 — /health in driver-service
Task: T034 — /health in admin-service
Task: T036 — /health in clickstream-service
Task: T038 — /health in gis-worker
Task: T040 — /health in analytics-writer

# Config structs in parallel (different files):
Task: T043 — DriverServiceConfig
Task: T044 — AdminServiceConfig
Task: T045 — ClickstreamServiceConfig
Task: T046 — GisWorkerConfig
Task: T047 — AnalyticsWriterConfig
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL)
3. Complete Phase 3: US1 — Docker Compose Infrastructure
4. **STOP and VALIDATE**: `docker compose up` boots all 9 containers healthy
5. Deploy/demo if ready — platform boots!

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. US1 (Docker Compose) → Boot platform → Deploy/Demo (MVP!)
3. US2 (Health endpoints) → Observability → Deploy
4. US3 (Config loading) → Reliability → Deploy
5. US4 (Networking) → Polish → Deploy
6. US5 (Logging) → Observability → Deploy
7. DB/RMQ Connectivity → Full runtime → Final smoke test

### Parallel Team Strategy

With multiple developers:
1. Complete Phases 1-2 together
2. Developer A: US1 (Docker Compose + infras)
3. Developer B: US2 (Health/ready endpoints) + US3 (Config loading)
4. Developer C: US5 (Logging)
5. After US1 done: Developer A continues to US4 (Networking) + Phase 8 (DB/RMQ)
6. Team integrates and runs smoke test together

---

## Notes

- [P] tasks = different files, no dependencies — can run in parallel
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Total: 92 tasks across 9 phases
