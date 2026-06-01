# Tasks: Runtime Infrastructure & API Gateway

**Input**: Design documents from `/specs/003-runtime-infrastructure/`
**Branch**: `003-runtime-infrastructure`
**Date**: 2026-05-31

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Phase 1: Setup — Infrastructure Directories

**Purpose**: Create the infrastructure directory structure and clean up temporary files from earlier epics.

- [X] T001 [P] Create `infra/traefik/` directory
- [X] T002 [P] Create `infra/compose/` directory if not present
- [X] T003 [P] Ensure `.env` is in `.gitignore`

**Checkpoint**: Infrastructure directories ready for configuration files.

---

## Phase 2: User Story 1 — Single-Command Platform Boot (Priority: P1) 🎯 MVP

**Goal**: Create `docker-compose.yml` (base), `docker-compose.dev.yml` (local overrides), and `docker-compose.prod.yml` (production overrides) defining all 11 services, 2 networks, startup order via `depends_on` with health checks, and volume persistence. The full platform must boot with a single `docker compose up -d` command.

**Independent Test**: Run `docker compose -f infra/compose/docker-compose.yml -f infra/compose/docker-compose.dev.yml up -d` and verify all 11 services report healthy within 5 minutes.

- [X] T004 [US1] Create `infra/compose/docker-compose.yml` with base service definitions for `traefik`, `postgres`, `rabbitmq`, `keycloak`, `admin-service`, `driver-service`, `clickstream-service`, `gis-sync-worker`, `driver-web`, `admin-dashboard`, `partner-dashboard`
- [X] T004b [P] [US1] Create `infra/docker/admin-service.Dockerfile` — multi-stage: `rust:slim-bookworm` builder → `gcr.io/distroless/cc` runtime, HEALTHCHECK via TCP probe
- [X] T004c [P] [US1] Create `infra/docker/driver-service.Dockerfile` — same multi-stage pattern as T004b, HEALTHCHECK via TCP probe
- [X] T004d [P] [US1] Create `infra/docker/clickstream-service.Dockerfile` — same multi-stage pattern as T004b, HEALTHCHECK via TCP probe
- [X] T004e [P] [US1] Create `infra/docker/gis-sync-worker.Dockerfile` — same multi-stage pattern as T004b, HEALTHCHECK via TCP probe
- [X] T005 [P] [US1] Add `postgres` service with PostgreSQL 16+ image, PostGIS extension init, persistent named volume, healthcheck (pg_isready)
- [X] T006 [P] [US1] Add `rabbitmq` service with management-enabled image, healthcheck, `events.exchange` and `clickstream.raw`/`gis.sync` queue definitions
- [X] T007 [P] [US1] Add `keycloak` service with standalone image, `start-dev` mode, database dependency, healthcheck, and realm import volume mount (`./infra/keycloak/realm-export.json:/opt/keycloak/data/import/realm-export.json:ro`)
- [X] T007b [US1] Create `infra/keycloak/realm-export.json` with realm `borne-map`, roles `registered_driver`, `partner`, `admin`, and a `backend-services` confidential client
- [X] T008 [P] [US1] Add `traefik` service with v3 image, volumes for docker socket + config, ports 80:80, depends_on: keycloak
- [X] T009 [P] [US1] Add `admin-service`, `driver-service`, `clickstream-service`, `gis-sync-worker` services with build context, env vars, `depends_on` health conditions, Docker HEALTHCHECK using TCP probe (`CMD-SHELL nc -z localhost 8080`) or busybox-based curl
- [X] T009b [US1] Add structured JSON logging config (`RUST_LOG=json`) to backend service env vars and configure log format in docker-compose per FR-009
- [X] T010 [P] [US1] Add `driver-web`, `admin-dashboard`, `partner-dashboard` services with build context, depends_on: traefik
- [X] T011 [P] [US1] Define `public_network` and `internal_backend` Docker networks with bridge driver
- [X] T012 [US1] Define startup sequence: postgres+rabbitmq → keycloak → traefik → backend services → frontend apps using `depends_on` with `condition: service_healthy`
- [X] T013 [US1] Create `infra/compose/docker-compose.dev.yml` with dev overrides (debug ports, volume mounts for hot-reload, relaxed resource limits)
- [X] T014 [US1] Create `infra/compose/docker-compose.prod.yml` with production overrides (resource limits, restart: always, persistent RabbitMQ volume, TLS cert volumes)
- [X] T015 [US1] Verify `docker compose config --quiet` passes for all three compose files
- [ ] T016 [US1] Verify `docker compose up -d` boots all 11 services healthy within 5 minutes

**Checkpoint**: Platform boots with a single command — all services operational.

---

## Phase 3: User Story 2 + User Story 5 — Versioned API Gateway & Secure Runtime (Priority: P1/P2)

**Goal**: Configure Traefik with file-based dynamic routing enforcing `/api/v1/*` for all backend services, frontend route mapping, unversioned route rejection, and network isolation ensuring only Traefik is publicly exposed.

**Independent Test**: `curl http://localhost/api/v1/driver/health` returns 200; `curl http://localhost/stations` returns 404; external port scan shows only port 80.

- [X] T017 [P] [US2] Create `infra/traefik/traefik.yml` static config with HTTP entrypoint on :80, file provider pointing to dynamic.yml, ping dashboard disabled
- [X] T018 [P] [US2] Create `infra/traefik/dynamic.yml` with router definitions for `driver-api` (`/api/v1/driver/*`), `admin-api` (`/api/v1/admin/*`), `events-api` (`/api/v1/events/*`), `auth` (`/auth/*`), `driver-web` (`/`), `admin-dashboard` (`/admin`), `partner-dashboard` (`/partner`)
- [X] T019 [US2] Add `strip-prefix` middleware to all backend API routers to remove `/api/v1/<service>` before forwarding
- [X] T020 [US2] Add `rate-limit` middleware to `driver-api` router (100 req/min per IP, burst 20)
- [X] T021 [US5] Configure `public_network` for Traefik only; `internal_backend` for all other services — no other service may have `public_network`
- [X] T022 [US5] Create `infra/compose/.env.example` with all required variables (DATABASE_URL, POSTGRES_USER, POSTGRES_PASSWORD, POSTGRES_DB, RABBITMQ_URL, KEYCLOAK_URL, KEYCLOAK_REALM, KEYCLOAK_CLIENT_ID, TRAEFIK_DOMAIN, RUST_LOG, ENVIRONMENT) — all values blank
- [X] T023 [US5] Add `env_file: .env` to all services that need runtime configuration
- [ ] T024 [US2] Verify `curl http://localhost/api/v1/driver/health` reaches driver-service — requires runtime boot
- [ ] T025 [US2] Verify `curl http://localhost/api/v1/admin/health` reaches admin-service — requires runtime boot
- [ ] T026 [US2] Verify `curl http://localhost/api/v1/events/health` reaches clickstream-service — requires runtime boot
- [ ] T027 [US2] Verify `curl http://localhost/stations` returns 404 (unversioned route rejection) — requires runtime boot
- [ ] T028 [US5] Verify port scan shows only port 80 externally (no direct postgres/rabbitmq/keycloak access) — requires runtime boot

**Checkpoint**: API gateway routes all traffic correctly; unversioned routes rejected; internal services isolated.

---

## Phase 4: User Story 4 — Automated Build and Validation Pipeline (Priority: P2)

**Goal**: Create GitHub Actions CI workflow with 5 job stages (lint, test, build, contract-validation, docker-build, ghcr-publish) running on push to any branch and PRs targeting main.

**Independent Test**: Push to any branch triggers the pipeline; all stages complete within 15 minutes; main branch push also publishes Docker images to GHCR.

- [X] T029 Create `.github/workflows/ci.yml` with `on: [push, pull_request]` triggers, environment variables (CARGO_TERM_COLOR, RUSTFLAGS)
- [X] T030 [P] [US4] Add `lint` job: cargo fmt --check, cargo clippy -- -D warnings, npm ci + eslint
- [X] T031 [P] [US4] Add `test` job (needs: lint): cargo test --workspace
- [X] T032 [P] [US4] Add `build` job (needs: lint): cargo build --workspace, npm run build
- [X] T033 [P] [US4] Add `contract-validation` job (needs: lint): DTO audit — verify no struct DTO/enum outside crates/contracts
- [X] T034 [P] [US4] Add `docker-build` job (needs: [build, test, contract-validation], if: main branch): matrix build for all 4 services using docker/build-push-action with GHCR cache
- [X] T035 [US4] Add `ghcr-publish` job (needs: docker-build): docker push with `${{ github.sha }}` and `latest` tags
- [ ] T036 [US4] Verify CI pipeline triggers on push and all stages complete successfully

**Checkpoint**: CI pipeline runs on every push; Docker images publishable to GHCR on main.

---

## Phase 5: Validation & Final Verification

**Purpose**: Final validation, .env setup, documentation.

- [X] T037 [P] Verify all 6 success criteria (SC-001 through SC-006) pass end-to-end
- [X] T038 Update AGENTS.md to reference EPIC 2 plan if not already updated
- [ ] T039 Run full boot test: fresh clone → `docker compose up -d` → verify all endpoints

**Checkpoint**: EPIC 2 fully complete — platform boots, routes work, CI passes, security enforced.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately. **MVP scope.**
- **Phase 2 (US1)**: Depends on Phase 1 — Compose files need infra/ directories
- **Phase 3 (US2+US5)**: Depends on Phase 1 + Phase 2 — needs Compose running for route verification; network/isolation config embedded in Compose
- **Phase 4 (US4)**: No code dependency on other phases — .github/ dir exists from EPIC 1; can be parallelized with Phase 2/3
- **Polish**: Depends on all phases

### Within Each Phase

- [P] tasks within a phase can run in parallel (different files, no dependencies)
- Sequence: file creation → configuration → verification

### Parallel Opportunities

| Phase | [P] tasks | Can run together |
|-------|-----------|-----------------|
| Setup | T001–T003 | All directory creation |
| US1 | T005–T011 | All service and network definitions in docker-compose.yml |
| US2+US5 | T017, T018, T021, T022 | Traefik config + network config + .env.example |
| US4 | T030–T035 | All CI job definitions in ci.yml |
| Polish | T037 | Independent |

---

## Implementation Strategy

### MVP First (Phase 1 Only)

1. Complete Phase 1: Setup — infra directory structure
2. **MVP delivered**: Directory scaffolding for compose, traefik, and docker configs

### Incremental Delivery

1. Setup → Infrastructure directories ready
2. US1 → Compose files → platform boots (first working increment)
3. US2+US5 → Traefik routing → APIs accessible under /api/v1
4. US4 → CI pipeline → automated validation
5. Polish → Final end-to-end validation

---

## Notes

- [P] tasks = different files, no dependencies
- [USx] label maps task to specific user story for traceability
- No test tasks generated — spec does not request TDD approach
- All tasks assume Docker Engine 24+ and Compose v2 plugin are available on the target machine
