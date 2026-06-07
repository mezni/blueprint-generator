# Tasks: Runtime Infrastructure (Docker Compose v1)

**Input**: Design documents from `specs/002-runtime-infrastructure/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Not requested — manual verification via `docker compose up` + `curl` per spec

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Infrastructure**: `infra/compose/`, `infra/env/`
- **Backend**: `services/{service-name}/`
- **Shared crates**: `crates/`
- Paths shown reflect the existing monorepo structure from plan.md

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Ensure all project infrastructure prerequisites are in place

- [ ] T001 Verify all 5 Rust services have a functioning HTTP `/health` endpoint in `services/*/src/main.rs` (driver-service, admin-service, clickstream-service, gis-worker, analytics-writer)
- [ ] T002 [P] Verify all 5 Dockerfiles exist and EXPOSE the correct port in `services/*/Dockerfile` (8081-8085)

> ⚠️ T014-T024 all modify `infra/compose/docker-compose.yml` — execute sequentially after T006 (do NOT parallelize within this file).
- [ ] T003 [P] Verify all `.env.example` files exist under `infra/env/` with Docker-compatible variable names

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core Compose and Traefik configuration — MUST be complete before any user story

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Create Traefik static file provider config at `infra/compose/traefik/config.yml` with `localhost` entrypoint and 5 path-based routers (`/api/v1/drivers/*` → driver-service:8081, `/api/v1/admin/*` → admin-service:8082, `/api/v1/clickstream/*` → clickstream-service:8083, `/api/v1/gis/*` → gis-worker:8084, `/api/v1/analytics/*` → analytics-writer:8085) with `StripPrefix` middleware on each
- [ ] T005 [P] Create `infra/compose/traefik/traefik.yml` with entry point `:80`, file provider pointing to `config.yml`, and logging configuration
- [ ] T006 Update `infra/compose/docker-compose.yml` to mount Traefik static config files as volumes, switch to file provider, and remove the old Docker provider labels
- [ ] T007 [P] Create `infra/compose/docker-compose.override.yml` exposing infrastructure ports for local development: postgres:5432, rabbitmq:5672+15672, keycloak:8080
- [ ] T008 [P] Verify `infra/compose/postgres/init-dbs.sh` creates 3 databases (keycloak_db, platform_db with PostGIS, analytics_db) idempotently
- [ ] T009 [P] Verify `infra/compose/keycloak/bornemap-realm.json` imports `bornemap` realm with 3 roles (registered_driver, partner, admin) and `bornemap-api` public client
- [ ] T010 Update `infra/env/postgres.env.example` with Docker-compatible `POSTGRES_*` variable names
- [ ] T011 [P] Update `infra/env/keycloak.env.example` with Keycloak Quarkus `KC_*` variable names
- [ ] T012 [P] Update `infra/env/rabbitmq.env.example` with Docker-compatible variable names (`RABBITMQ_DEFAULT_USER`, `RABBITMQ_DEFAULT_PASS`, etc.)
- [ ] T013 [P] Update all 5 backend `.env.example` files in `infra/env/` with correct internal Docker network hostnames (e.g., `postgres.internal`, `rabbitmq.internal`, `keycloak.internal`)

**Checkpoint**: Foundation ready — `docker compose config` validates all 9 services with correct networking

---

## Phase 3: User Story 1 — Developer Brings Full Stack Online Locally (Priority: P1) 🎯 MVP

**Goal**: Run `docker compose up` and all 9 services start in the correct order with health checks

**Independent Test**: `docker compose up -d && docker compose ps` shows all 9 services as `healthy` within 120 seconds

### Implementation for User Story 1

- [ ] T014 [P] [US1] Add `depends_on: condition: service_healthy` to `infra/compose/docker-compose.yml` for startup ordering: PostgreSQL → RabbitMQ → Keycloak → Traefik → backend services
- [ ] T015 [P] [US1] Add health check configuration to postgres service in `infra/compose/docker-compose.yml` using `pg_isready -U $POSTGRES_USER`
- [ ] T016 [P] [US1] Add health check configuration to rabbitmq service in `infra/compose/docker-compose.yml` using `rabbitmq-diagnostics check_port_connectivity`
- [ ] T017 [P] [US1] Add health check configuration to keycloak service in `infra/compose/docker-compose.yml` using `curl -sf http://localhost:9000/health/ready`
- [ ] T018 [P] [US1] Add health check configuration to traefik service in `infra/compose/docker-compose.yml` using `/dev/tcp/localhost/80` shell probe
- [ ] T019 [P] [US1] Add health check configuration to all 5 backend services in `infra/compose/docker-compose.yml` using `/dev/tcp/localhost:{PORT}` shell probes
- [ ] T020 [P] [US1] Configure `infra/compose/docker-compose.yml` with internal-only bridge network `bornemap_internal` with `internal: true`, and ensure only Traefik has host port mappings
- [ ] T021 [P] [US1] Wire Keycloak service in `infra/compose/docker-compose.yml` to use postgres as its database backend via `KC_DB_URL=jdbc:postgresql://postgres.internal:5432/keycloak_db`
- [ ] T022 [P] [US1] Wire Keycloak service in `infra/compose/docker-compose.yml` with realm import volume mount (`./keycloak/bornemap-realm.json:/opt/keycloak/data/import/bornemap-realm.json`) and `--import-realm` startup argument
- [ ] T023 [P] [US1] Wire postgres service in `infra/compose/docker-compose.yml` with init script volume mount (`./postgres/init-dbs.sh:/docker-entrypoint-initdb.d/init-dbs.sh`)
- [ ] T024 [P] [US1] Set environment variable references in `infra/compose/docker-compose.yml` for all 9 services, pulling from their respective `.env` files via `env_file:` directives
- [ ] T025 [US1] Run `docker compose up -d` from `infra/compose/` and verify all 9 services reach `healthy` status

**Checkpoint**: At this point, User Story 1 should be fully functional — `docker compose up` brings all 9 services online

---

## Phase 4: User Story 2 — Developer Verifies Each Service Health (Priority: P2)

**Goal**: `curl` each backend service's `/health` endpoint and receive HTTP 200 `{"status":"ok"}`

**Independent Test**: `for svc in drivers admin clickstream gis analytics; do curl -f http://localhost/api/v1/$svc/health; done` — all return 200

### Implementation for User Story 2

- [ ] T026 [P] [US2] Verify driver-service `/health` endpoint at `services/driver-service/src/main.rs` returns `{"status":"ok"}` on port 8081
- [ ] T027 [P] [US2] Verify admin-service `/health` endpoint at `services/admin-service/src/main.rs` returns `{"status":"ok"}` on port 8082
- [ ] T028 [P] [US2] Verify clickstream-service `/health` endpoint at `services/clickstream-service/src/main.rs` returns `{"status":"ok"}` on port 8083
- [ ] T029 [P] [US2] Verify gis-worker `/health` endpoint at `services/gis-worker/src/main.rs` returns `{"status":"ok"}` on port 8084
- [ ] T030 [P] [US2] Verify analytics-writer `/health` endpoint at `services/analytics-writer/src/main.rs` returns `{"status":"ok"}` on port 8085
- [ ] T031 [US2] Verify Traefik routing: curl `http://localhost/api/v1/drivers/health` routes to driver-service and returns `{"status":"ok"}` with 200 status code
- [ ] T032 [P] [US2] Verify Traefik routing for admin-service at `http://localhost/api/v1/admin/health`
- [ ] T033 [P] [US2] Verify Traefik routing for clickstream-service at `http://localhost/api/v1/clickstream/health`
- [ ] T034 [P] [US2] Verify Traefik routing for gis-worker at `http://localhost/api/v1/gis/health`
- [ ] T035 [P] [US2] Verify Traefik routing for analytics-writer at `http://localhost/api/v1/analytics/health`

**Checkpoint**: At this point, all 5 `/health` endpoints return 200 through both direct container access and Traefik routing

---

## Phase 5: User Story 3 — Operator Validates Infrastructure Dependencies (Priority: P3)

**Goal**: From inside any container, validate PostgreSQL, RabbitMQ, and Keycloak are reachable by internal DNS names

**Independent Test**: `docker compose exec driver-service sh -c 'cat /dev/null > /dev/tcp/postgres.internal/5432 && echo OK'` returns `OK`

### Implementation for User Story 3

- [ ] T036 [P] [US3] Verify PostgreSQL reachability from inside a container via `postgres.internal:5432` using `/dev/tcp` probe
- [ ] T037 [P] [US3] Verify RabbitMQ reachability from inside a container via `rabbitmq.internal:5672` using `/dev/tcp` probe
- [ ] T038 [P] [US3] Verify Keycloak reachability from inside a container via `keycloak.internal:8080` using `/dev/tcp` probe
- [ ] T039 [P] [US3] Verify 3 PostgreSQL databases exist (keycloak_db, platform_db with PostGIS, analytics_db) by connecting to postgres from inside a container
- [ ] T040 [P] [US3] Verify RabbitMQ management API responds on `rabbitmq.internal:15672` from inside a container
- [ ] T041 [P] [US3] Verify Keycloak OIDC metadata served at `keycloak.internal:8080/realms/bornemap/.well-known/openid-configuration` from inside a container

**Checkpoint**: All 3 infrastructure dependencies verified reachable and functional from inside the Docker network

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification, edge case handling, and documentation

- [ ] T042 Run `docker compose down -v`, then `docker compose up -d` to verify deterministic fresh start (US1 edge case)
- [ ] T043 Run `docker compose config --services` and verify output lists exactly 9 services
- [ ] T044 Run the full quickstart verification sequence from `specs/002-runtime-infrastructure/quickstart.md`
- [ ] T045 [P] Verify Traefik fallback behavior when Docker socket is inaccessible (edge case from spec.md) — stop Traefik container, confirm file-based routes still respond

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Verify prerequisites — no code changes
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational — core bring-up
- **US2 (Phase 4)**: Depends on US1 (needs stack running)
- **US3 (Phase 5)**: Depends on US1 (needs stack running), can run in parallel with US2
- **Polish (Phase 6)**: Depends on US1 completion

### User Story Dependencies

- **US1 (P1)**: Starts after Foundational — no dependencies on other stories
- **US2 (P2)**: Must have US1 complete (stack running to test health endpoints)
- **US3 (P3)**: Must have US1 complete (stack running to test infrastructure reachability)

### Parallel Opportunities

- Phase 1 tasks T001-T003 can run in parallel
- Phase 2 tasks T004-T013 — T004+T005 must be sequential (config.yml before traefik.yml), all others [P] can run in parallel
- Tasks within same file (T006, T014-T024 in docker-compose.yml) must be sequential
- US2 health verification tasks T026-T030 can run in parallel
- US3 infrastructure check tasks T036-T041 can run in parallel
- US2 and US3 can run in parallel once US1 is complete

---

## Parallel Example: Phase 2 Foundational

```bash
# Create Traefik config (sequential: config.yml then traefik.yml):
Task: "Create Traefik file provider config at infra/compose/traefik/config.yml"
Task: "Create Traefik static config at infra/compose/traefik/traefik.yml"

# Independent tasks that can run in parallel:
Task: "Create infra/compose/docker-compose.override.yml"
Task: "Verify infra/compose/postgres/init-dbs.sh"
Task: "Verify infra/compose/keycloak/bornemap-realm.json"
Task: "Update infra/env/postgres.env.example"
Task: "Update infra/env/keycloak.env.example"
Task: "Update infra/env/rabbitmq.env.example"
```

## Parallel Example: User Story 2

```bash
# Verify all 5 health endpoints in parallel:
Task: "Verify driver-service /health on port 8081"
Task: "Verify admin-service /health on port 8082"
Task: "Verify clickstream-service /health on port 8083"
Task: "Verify gis-worker /health on port 8084"
Task: "Verify analytics-writer /health on port 8085"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup verification
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: User Story 1 — all 9 services healthy
4. **STOP and VALIDATE**: `docker compose up -d && docker compose ps` — all services healthy
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready (Traefik config, compose structure, env files)
2. Add User Story 1 → Test independently → `docker compose up` works (MVP!)
3. Add User Story 2 → Test independently → health endpoints all return 200
4. Add User Story 3 → Test independently → infrastructure reachability verified
5. Each story adds value without breaking previous stories

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- After US1, run `docker compose up` and confirm all services stay healthy for 60s
- T001-T003 are verification tasks — confirm existing files are correct
- T014-T024 all modify `infra/compose/docker-compose.yml` — must be done carefully to avoid conflicts
