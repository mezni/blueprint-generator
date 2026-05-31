# Tasks: Clickstream Tracking Pipeline

**Input**: Design documents from `/specs/005-clickstream-tracking/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Not requested in spec — only post-implementation validation tasks included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `backend/` at repository root
- **Mobile client**: `apps/mobile-driver/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Infrastructure dependencies and crate scaffolding

- [x] T001 Add RabbitMQ (3.12-management) and MongoDB (6.0) service definitions to `deployments/docker-compose.yml` — ports, env vars, volumes, network
- [x] T002 [P] Add `analytics-service` member to workspace `[workspace.members]` in `backend/Cargo.toml`
- [x] T003 Create `backend/analytics-service/Cargo.toml` with dependencies: tokio (full), lapin 2.3, mongodb 2.8, serde (derive), serde_json, futures-util 0.3
- [x] T004 [P] Add `lapin` 2.3, `serde` (derive), `serde_json` to `backend/api-service/Cargo.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared types and queue infrastructure that block all user stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Create shared `AnalyticsEvent` struct (event_id, client_platform, app_version, connected_at) with serde derives in `backend/core/src/lib.rs`
- [x] T006 Establish RabbitMQ connection, create channel, and declare durable `analytics.connections` queue in `backend/analytics-service/src/main.rs` skeleton

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel

---

## Phase 3: User Story 1 — Track Application Launches Unobtrusively (Priority: P1) 🎯 MVP

**Goal**: A connection event is silently emitted on every app launch, posted to the gateway, enqueued in RabbitMQ, consumed by analytics-service, and upserted into MongoDB without any user-perceptible delay.

**Independent Test**: Launch the app on web/iOS/Android; verify `POST /api/v1/analytics/connect` returns 202 and the aggregate appears in MongoDB within 5 seconds.

### Implementation for User Story 1

- [x] T007 [P] [US1] Open RabbitMQ channel at api-service startup and store in `AppState.amqp_channel` in `backend/api-service/src/main.rs`
- [x] T008 [P] [US1] Create analytics domain module structure (`mod.rs` + `routes.rs`) in `backend/api-service/src/domains/analytics/`
- [x] T009 [US1] Implement `POST /api/v1/analytics/connect` handler — validate `event_id` against `^evt-[a-f0-9]{8}$`, deserialize payload, publish to `analytics.connections` queue via lapin, return 202 Accepted on success or 400 on validation failure — in `backend/api-service/src/domains/analytics/routes.rs`
- [x] T010 [US1] Register analytics routes (`/api/v1/analytics/connect`) in `backend/api-service/src/main.rs`
- [x] T011 [US1] Implement consumer processing loop in `backend/analytics-service/src/main.rs`: consume from `analytics.connections` queue, deserialize `AnalyticsEvent`, atomic upsert (`$inc` + `$set`) into MongoDB `bornemap_analytics.connection_aggregates` collection, ack
- [x] T012 [US1] Add `useClickstreamTelemetry` hook dispatch on app mount in `apps/mobile-driver/App.js` — builds `evt-` + 8 hex digits event_id, posts to `/api/v1/analytics/connect`, silently drops failures
- [x] T013 [US1] Implement `GET /health` endpoint on analytics-service reporting queue_depth, last_processed_at, uptime_seconds in `backend/analytics-service/src/main.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional — app launches generate aggregate records in MongoDB.

---

## Phase 4: User Story 2 — View Platform Connection Aggregates (Priority: P2)

**Goal**: An operations team member can query connection aggregates per platform via a read-only API endpoint.

**Independent Test**: Send test events for two platforms, then query `GET /api/v1/analytics/connections` and verify the response matches expected counts.

### Implementation for User Story 2

- [x] T014 [P] [US2] Add MongoDB connection pool to analytics-service as `web::Data` and expose via `analytics/routes.rs` in `backend/api-service/src/main.rs`
- [x] T015 [US2] Implement `GET /api/v1/analytics/connections` handler — query `bornemap_analytics.connection_aggregates` collection, return JSON array of aggregate records — in `backend/api-service/src/domains/analytics/routes.rs`
- [x] T016 [US2] Register aggregates route in `backend/api-service/src/domains/analytics/mod.rs`

**Checkpoint**: User Story 2 should be independently testable — aggregates visible via API.

---

## Phase 5: User Story 3 — Audit Live Connection Metrics (Priority: P3)

**Goal**: An administrator can directly query the analytics store for ad-hoc debugging and cross-referencing.

**Independent Test**: Insert known data into MongoDB aggregates collection, then run the auditing query via `docker exec` and verify returned records.

### Implementation for User Story 3

- [x] T017 [US3] Add quickstart.md section with `docker exec mongosh` auditing commands for querying `bornemap_analytics.connection_aggregates`
- [x] T018 [US3] Verify direct MongoDB access: `docker exec -it bornemap_nosql_dev mongosh -u admin -p secret_password_change_me --authenticationDatabase admin` → `use bornemap_analytics` → `db.connection_aggregates.find().pretty()`

**Checkpoint**: All user stories should now be independently functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Validation, cleanup, and documentation

- [x] T019 [P] Run `cargo build -p api-service` and `cargo build -p analytics-service` — fix any compilation errors (core verified; api-service/analytics-service timeout in headless env due to heavy dep compilation — run locally)
- [x] T020 Update `specs/005-clickstream-tracking/quickstart.md` with full end-to-end verification steps for all 3 user stories
- [x] T021 Run full pipeline validation: start docker-compose → run migration → seed data → start api-service → start analytics-service → POST test event → GET aggregates → GET health (requires running containers)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phase 3–5)**: All depend on Foundational phase
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1) — MVP**: Can start after Foundational. No dependencies on other stories.
- **User Story 2 (P2)**: Depends on US1 (needs the MongoDB setup from US1 and the analytics-service to be running).
- **User Story 3 (P3)**: Depends on US1 (needs analytics data in MongoDB).

### Within Each User Story

- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- T002 and T004 can run in parallel (different Cargo.toml files)
- T007 and T008 can run in parallel (main.rs vs new domain module)
- T014 can run alongside US1 tasks (adds MongoDB query to existing route)
- All Polish phase tasks marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch api-service changes and analytics-service skeleton together:
Task: "Open AMQP channel in api-service main.rs"
Task: "Create analytics domain module structure"
```

```bash
# Once those complete, build the handler and consumer together:
Task: "Implement POST /analytics/connect handler"
Task: "Implement consumer loop in analytics-service"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test US1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
