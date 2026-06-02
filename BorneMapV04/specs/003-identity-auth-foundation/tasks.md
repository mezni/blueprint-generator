# Tasks: Identity & Authentication Foundation

**Input**: Design documents from `specs/003-identity-auth-foundation/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No separate test tasks — integration tests are User Story 5.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Monorepo**: `crates/`, `services/`, `infra/` at repository root
- Paths reflect the existing monorepo structure for this project

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add dependencies, env vars, and infrastructure scaffolding

- [ ] T001 [P] Add `jsonwebtoken` (with `jwk` feature) and `reqwest` to workspace dependencies in `Cargo.toml`
- [ ] T002 [P] Add `common-auth` dependency to `services/driver-service/Cargo.toml`
- [ ] T003 [P] Add `common-auth` dependency to `services/admin-service/Cargo.toml`
- [ ] T004 [P] Add `common-auth` dependency to `services/clickstream-service/Cargo.toml`
- [ ] T005 [P] Add `common-auth` dependency to `services/gis-worker/Cargo.toml`
- [ ] T006 [P] Add `common-auth` dependency to `services/analytics-writer/Cargo.toml`
- [ ] T007 [P] Add JWKS env vars to `infra/env/local/driver-service.env`
- [ ] T008 [P] Add JWKS env vars to `infra/env/local/admin-service.env`
- [ ] T009 [P] Add JWKS env vars to `infra/env/local/clickstream-service.env`
- [ ] T010 [P] Add JWKS env vars to `infra/env/local/gis-worker.env`
- [ ] T011 [P] Add JWKS env vars to `infra/env/local/analytics-writer.env`

---

## Phase 2: Foundational — common-auth Crate (Blocking Prerequisite)

**Purpose**: Shared JWT validation library that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T012 [P] Implement `AuthConfig` struct with env var loading in `crates/common-auth/src/config.rs`
- [ ] T013 [P] Implement `Role` enum and `AuthContext` struct in `crates/common-auth/src/claims.rs`
- [ ] T014 [P] Implement `AuthError` enum with Display and IntoResponse in `crates/common-auth/src/error.rs`
- [ ] T015 Implement `JwtValidator` — JWKS fetch, cache, token signature/expiry/issuer/audience validation, and background refresh spawn in `crates/common-auth/src/jwt.rs`
- [ ] T016 Implement `auth_middleware` axum middleware (extract JWT, validate, inject AuthContext) in `crates/common-auth/src/middleware.rs`
- [ ] T017 Implement `require_role` and `require_any_role` axum middleware guards in `crates/common-auth/src/middleware.rs`
- [ ] T018 Wire up public re-exports in `crates/common-auth/src/lib.rs`
- [ ] T019 Verify `common-auth` compiles with `cargo build -p common-auth`

**Checkpoint**: `common-auth` crate ready — JWT validation, role checks, and middleware available

---

## Phase 3: User Story 1 — Keycloak Realm from Code (Priority: P1) 🎯 MVP

**Goal**: Developer can fully re-create the Keycloak `ev-platform` realm from code-managed JSON

**Independent Test**: Tear down Keycloak volumes, restart stack, verify realm/clients/roles via Keycloak admin API

- [ ] T020 [US1] Update `infra/keycloak/realm-export/ev-platform-realm.json` with realm roles `registered_driver`, `partner`, `admin`
- [ ] T021 [US1] Add public clients `driver-web`, `partner-dashboard`, `admin-dashboard`, `driver-mobile` to realm export
- [ ] T022 [US1] Add confidential client `platform-service` to realm export
- [ ] T023 [US1] Add `tenant_id` user attribute protocol mapper to realm export
- [ ] T024 [US1] Verify realm import works: destroy Keycloak volume, `docker compose up`, validate via admin API
- [ ] T024b [US1] Restrict Keycloak admin console to `local` profile in `infra/compose/docker-compose.yml` or env profile configuration

**Checkpoint**: Keycloak `ev-platform` realm fully provisioned from code — clients, roles, and mappers ready

---

## Phase 4: User Story 2 — JWT Validation on Every Request (Priority: P1)

**Goal**: Every protected endpoint in Driver Service and Admin Service rejects unauthenticated requests with valid JWKS-based JWT validation

**Independent Test**: Start platform, obtain JWT via Keycloak, call protected endpoint with and without token, verify HTTP 401/200

- [ ] T025 [P] [US2] Integrate `auth_middleware` into Driver Service router in `services/driver-service/src/main.rs`
- [ ] T026 [P] [US2] Integrate `auth_middleware` into Admin Service router in `services/admin-service/src/main.rs`
- [ ] T027 [US2] Integrate `JwtValidator::init()` with graceful degradation into `services/driver-service/src/main.rs`
- [ ] T028 [US2] Integrate `JwtValidator::init()` with graceful degradation into `services/admin-service/src/main.rs`
- [ ] T029 [P] [US2] Integrate `JwtValidator::init()` into `services/clickstream-service/src/main.rs` (infrastructure only)
- [ ] T030 [P] [US2] Integrate `JwtValidator::init()` into `services/gis-worker/src/main.rs` (infrastructure only)
- [ ] T031 [P] [US2] Integrate `JwtValidator::init()` into `services/analytics-writer/src/main.rs` (infrastructure only)
- [ ] T032 [US2] Add periodic JWKS refresh background task (default 3600s with jitter) in `common-auth`
- [ ] T033 [US2] Verify: request without token returns 401, request with valid token returns 200, request with expired token returns 401

**Checkpoint**: All services load JWKS on startup; Driver Service and Admin Service validate JWT on every protected request

---

## Phase 5: User Story 3 — Route-Level Access Control (Priority: P1)

**Goal**: Driver Service and Admin Service enforce role-based access at the route level; public endpoints remain accessible without auth

**Independent Test**: For each route, verify correct HTTP 200/401/403 for each role and anonymous access

- [ ] T034 [US3] Split Driver Service routes into public and protected routers in `services/driver-service/src/main.rs`
- [ ] T035 [US3] Add `require_role(Role::Admin)` middleware to all Admin Service routes in `services/admin-service/src/main.rs`
- [ ] T036 [US3] Add `require_any_role` guard for `registered_driver` on favorites/reviews/profile routes in Driver Service
- [ ] T037 [US3] Add `require_role(Role::Admin)` guard to Admin Service user management endpoints
- [ ] T038 [US3] Verify: anonymous gets 200 on public endpoints, 401 on protected; wrong role gets 403; correct role gets 200

**Checkpoint**: Route-level role enforcement working in both Driver Service and Admin Service

---

## Phase 6: User Story 4 — Partner Isolation via JWT Tenant Context (Priority: P2)

**Goal**: Partner-role users are scoped to their own tenant via `tenant_id` from JWT; repository-layer enforcement prevents cross-tenant access

**Independent Test**: Log in as partner A (create station), log in as partner B (cannot see/modify partner A's station)

- [ ] T039 [US4] Implement `require_tenant()` on `AuthContext` — extracts tenant_id, returns 403 if partner has no tenant_id in `crates/common-auth/src/claims.rs`
- [ ] T040 [US4] Thread `AuthContext` through Driver Service route handlers to repository layer
- [ ] T041 [US4] Add repository-layer enforcement: `tenant_id` derived from `AuthContext`, never from client input
- [ ] T042 [US4] Verify: partner A creates station, partner B cannot see/modify it via any endpoint

**Checkpoint**: Partner isolation enforced at repository layer — no cross-tenant data leakage

---

## Phase 7: User Story 5 — Auth Integration Test Suite (Priority: P2)

**Goal**: Shell-based test suite validates entire auth flow end-to-end against running Docker Compose stack

**Independent Test**: Run `./scripts/auth-smoke-test.sh` — exits with code 0 on success

- [ ] T043 [P] [US5] Implement Keycloak reachability check in `scripts/auth-smoke-test.sh`
- [ ] T044 [P] [US5] Implement token acquisition for each role type in `scripts/auth-smoke-test.sh`
- [ ] T045 [P] [US5] Implement JWT validation test (call protected endpoint with/without token) in `scripts/auth-smoke-test.sh`
- [ ] T046 [P] [US5] Implement route-level role enforcement tests in `scripts/auth-smoke-test.sh`
- [ ] T047 [P] [US5] Implement partner isolation test (cross-tenant verification) in `scripts/auth-smoke-test.sh`
- [ ] T048 [US5] Run full test suite against fresh stack, verify all checks pass

**Checkpoint**: `./scripts/auth-smoke-test.sh` validates the entire auth system end-to-end

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final verification, documentation, and cleanup

- [ ] T049 [P] Update `infra/env/local/.env.example` with new auth env vars and documentation
- [ ] T050 [P] Verify `cargo build` succeeds for entire workspace
- [ ] T051 Verify end-to-end: `docker compose up` → `./scripts/auth-smoke-test.sh` passes with exit 0

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 — Keycloak Realm (Phase 3)**: No code dependency on common-auth — can run in parallel with Phase 2
- **US2 — JWT Validation (Phase 4)**: Depends on Phase 2 (common-auth) AND Phase 3 (Keycloak realm)
- **US3 — Role Enforcement (Phase 5)**: Depends on Phase 4 (JWT working)
- **US4 — Partner Isolation (Phase 6)**: Depends on Phase 5 (role enforcement)
- **US5 — Integration Tests (Phase 7)**: Depends on Phases 3–6 (all features implemented)
- **Polish (Phase 8)**: Depends on all phases complete

### User Story Dependencies

- **US1 (P1)**: Independent of common-auth — can start alongside Phase 2
- **US2 (P1)**: Blocks after US1 and Phase 2
- **US3 (P1)**: Blocks after US2
- **US4 (P2)**: Blocks after US3
- **US5 (P2)**: Blocks after US3 and US4

### Within Each User Story

- Implementation before verification
- Core logic before endpoint integration
- Story complete before moving to next

### Parallel Opportunities

- All **Setup** tasks marked [P] can run in parallel
- All **Foundational** tasks marked [P] can run in parallel
- **Phase 2 (common-auth)** and **Phase 3 (US1 Keycloak)** can run in FULL PARALLEL (no cross-dependency)
- All **US2** tasks marked [P] can run in parallel
- All **US5** smoke test scripts marked [P] can run in parallel

### Parallel Example

```bash
# Phase 2 + Phase 3 in parallel:
Task: "Phase 2: common-auth crate implementation"
Task: "Phase 3: Keycloak realm export update"

# Once both complete, Phase 4 begins:
Task: "Phase 4: JWT validation in Driver + Admin services"
```

---

## Implementation Strategy

### MVP First (User Stories 1–3 Only)

1. Complete Phase 1: Setup
2. Run Phase 2 (common-auth) and Phase 3 (Keycloak realm) in parallel
3. Complete Phase 4: JWT validation in services
4. Complete Phase 5: Route-level access control
5. **STOP and VALIDATE**: Test MVP (US1 + US2 + US3) independently
6. Deploy/demo if ready

### Incremental Delivery

1. Setup + Foundational + US1 → Foundation + IdP ready
2. Add US2 + US3 → JWT validation + role enforcement (MVP!)
3. Add US4 → Partner isolation (constitution-mandated)
4. Add US5 → Automated verification

### Parallel Team Strategy

With two developers:
- Developer A: Phase 2 (common-auth)
- Developer B: Phase 3 (Keycloak realm)
- Both converge on Phase 4 together
- Developer A drives Phase 5 + Phase 6
- Developer B drives Phase 7

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Phase 2 (common-auth) and Phase 3 (US1) have ZERO dependency on each other — optimize by running in parallel
