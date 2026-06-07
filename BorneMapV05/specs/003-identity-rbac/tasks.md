# Tasks: Identity & RBAC

**Input**: Design documents from `/specs/003-identity-rbac/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust crates**: `crates/common-auth/`
- **Backend services**: `services/{service-name}/`
- **Infrastructure**: `infra/compose/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize common-auth crate, set up dependencies, and create the Keycloak realm configuration.

- [x] T001 Create `crates/common-auth/Cargo.toml` with dependencies: axum, tower, jsonwebtoken, reqwest (rustls-tls), serde, serde_json, tokio, tracing, common-types, common-errors
- [x] T002 [P] Create Keycloak realm export at `infra/compose/keycloak/realm-export.json` with realm `bornemap`, three roles (`registered_driver`, `partner`, `admin`), OIDC client `bornemap-api`, and stub identity providers (Google, Facebook)
- [x] T003 Add `/auth/*` route to `infra/compose/traefik/dynamic/routes.yml` proxying to Keycloak container on port 8080
- [x] T004 Add auth environment variables to each service's `.env.example` under `infra/env/`: `AUTH_ISSUER`, `AUTH_JWKS_URL`, `AUTH_AUDIENCE`
- [x] T005 Create `crates/common-auth/src/lib.rs` with public module declarations and re-exports

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core auth crate building blocks that MUST be complete before any user story can be implemented.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T006 Create `crates/common-auth/src/errors.rs` with `AuthError` enum mapping to standard error codes (Unauthenticated, TokenExpired, InsufficientRole) and implementing `IntoResponse` for Axum
- [x] T007 [P] Create `crates/common-auth/src/jwt.rs` with JWT claims struct (sub, iss, aud, exp, iat, email, realm_access.roles), JWKS key structs, and `validate_token()` function that verifies signature, issuer, audience, and expiration
- [x] T008 [P] Implement JWKS fetch and cache in `crates/common-auth/src/jwt.rs` with `JwksCache` struct using `tokio::sync::RwLock`, configurable TTL, and degraded mode (use stale keys when JWKS unreachable)
- [x] T009 Create `CurrentUser` struct in `crates/common-auth/src/lib.rs` with fields: user_id, keycloak_user_id, email, role, partner_id (Option)

**Checkpoint**: Foundation ready — `common-auth` can validate a JWT in isolation. User story implementation can now begin.

---

## Phase 3: User Story 1 - User Login and Token-Based Access (Priority: P1) 🎯 MVP

**Goal**: Users can log in through Keycloak (via Traefik proxy) and receive a JWT. Services validate the JWT on every authenticated request. First-time logins auto-provision a `user_account` record.

**Independent Test**: Obtain a valid JWT from Keycloak, call a protected test endpoint, verify the request succeeds. Call without a token, verify UNAUTHENTICATED error. Create a first-time login, verify a `user_account` row exists in platform_db.

### Implementation for User Story 1

- **Step 1**: Create first-login provisioning and auth middleware

- [x] T010 [P] [US1] Create `crates/common-auth/src/provisioning.rs` with `provision_user()` that upserts `users.user_account` on first valid JWT, mapping `keycloak_user_id = JWT.sub`
- [x] T011 [US1] Create `crates/common-auth/src/guards.rs` with `AuthLayer` that extracts `Authorization: Bearer <token>`, calls `validate_token()`, calls `provision_user()`, and populates request extensions with `CurrentUser`
- [x] T012 [US1] Wire `AuthLayer` into the Axum router builder and make it available as `common_auth::auth_layer()`

- **Step 2**: Migrate services from raw TCP to Axum with auth middleware

- [x] T013 [US1] Refactor `services/driver-service/src/main.rs` from `TcpListener` to Axum, add `/health` route (exempt from auth), add `AuthLayer`, listen on `DRIVER_SERVICE_PORT`
- [x] T014 [P] [US1] Refactor `services/admin-service/src/main.rs` to Axum with `/health` exemption and `AuthLayer`
- [x] T015 [P] [US1] Refactor `services/clickstream-service/src/main.rs` to Axum with `/health` exemption and `AuthLayer`
- [x] T016 [P] [US1] Refactor `services/gis-worker/src/main.rs` to Axum with `/health` exemption
- [x] T017 [P] [US1] Refactor `services/analytics-writer/src/main.rs` to Axum with `/health` exemption

**Checkpoint**: At this point, User Story 1 should be fully functional. Users can receive a JWT from Keycloak and access services with it. First login creates a `user_account` record.

---

## Phase 4: User Story 2 - Role-Based Access Control (Priority: P1)

**Goal**: The three roles (`registered_driver`, `partner`, `admin`) are enforced at the API layer. Requests with insufficient role are rejected with INSUFFICIENT_ROLE.

**Independent Test**: Obtain JWTs for each role. Call a role-gated endpoint with each token — only the correct role should succeed. Wrong roles get INSUFFICIENT_ROLE.

### Implementation for User Story 2

- [ ] T018 [P] [US2] Implement `require_role(Role)` guard in `crates/common-auth/src/guards.rs` that checks `CurrentUser.role` against required role, returns `INSUFFICIENT_ROLE` on mismatch
- [ ] T019 [P] [US2] Implement `require_authenticated()` guard in `crates/common-auth/src/guards.rs` that rejects requests without a valid `CurrentUser` in extensions
- [ ] T020 [US2] Expose guard functions from `crates/common-auth/src/lib.rs`: `pub fn require_role(role: Role) -> AuthGuard`, `pub fn require_authenticated() -> AuthGuard`
- [ ] T021 [US2] Add a test route to `services/admin-service/src/main.rs` gated by `require_role(Role::Admin)` and verify role enforcement

**Checkpoint**: RBAC enforced. Role-gated endpoints reject wrong roles.

---

## Phase 5: User Story 3 - Partner Membership and Tenant Isolation (Priority: P2)

**Goal**: Partner users automatically receive a `partner_membership` record on first login. The `partner_id` is derived from the membership table and is never accepted from the client.

**Independent Test**: Pre-configure a Keycloak user with a `partner_id` attribute. Log in for the first time — verify a `partner_membership` record is created. Attempt to pass a different `partner_id` in a request — verify it is ignored.

### Implementation for User Story 3

- [ ] T022 [P] [US3] Extend `crates/common-auth/src/provisioning.rs` to check Keycloak user attributes for `partner_id` on first login and create `partner_membership` record with default role `viewer`
- [ ] T023 [US3] Add `partner_id` derivation from `partner_membership` table in `crates/common-auth/src/provisioning.rs` and populate `CurrentUser.partner_id` on every login
- [ ] T024 [US3] Add `validate_no_client_partner_id()` helper in `crates/common-auth/src/guards.rs` that rejects requests containing `partner_id` in body/query for partner-scoped endpoints

**Checkpoint**: Partner membership auto-provisioned on first login. `partner_id` always derived server-side.

---

## Phase 6: User Story 4 - Auth-Guard Middleware for API Development (Priority: P3)

**Goal**: Developers can quickly add auth to new endpoints using the reusable `common-auth` guards. Three guard modes (public, authenticated, role-gated) are documented and demonstrable.

**Independent Test**: Create a new route with each guard mode, verify correct behavior. Measure time from route definition to passing test — should be under 5 minutes.

### Implementation for User Story 4

- [ ] T025 [US4] Add `TestAuthLayer` to `crates/common-auth/src/guards.rs` that injects a known `CurrentUser` for integration tests
- [ ] T026 [US4] Add doc comments to all public functions in `crates/common-auth/src/lib.rs` with examples for each guard mode
- [ ] T027 [US4] Add a `common-auth` usage example in `services/driver-service/src/main.rs` showing all three guard modes on separate routes

**Checkpoint**: Auth guards are reusable, documented, and testable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Auth audit logging, degraded mode hardening, integration testing, and verification.

- [ ] T028 Add structured auth logging in `crates/common-auth/src/jwt.rs` and `provisioning.rs`: log auth failures (missing token, invalid signature, expired, wrong role) and provisioning events with request correlation ID and error code
- [ ] T029 [P] Validate backend services access JWKS via Keycloak's internal Docker hostname (not through Traefik) in `crates/common-auth/src/jwt.rs`
- [ ] T030 [P] Add integration test in `services/driver-service/tests/` verifying JWKS degraded mode: simulate JWKS unreachable, verify cached JWTs still validate, new unauthenticated requests fail
- [ ] T031 Run `cargo build` on entire workspace and fix any compilation errors
- [ ] T032 Run `cargo test` on `common-auth` crate and verify all auth paths pass
- [ ] T033 Run `specs/003-identity-rbac/quickstart.md` validation steps manually to verify end-to-end auth flow

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational — core login flow
- **US2 (Phase 4)**: Depends on Foundational and US1 — needs AuthLayer and JWT validation before role guards
- **US3 (Phase 5)**: Depends on Foundational and US1 — needs provisioning and CurrentUser
- **US4 (Phase 6)**: Depends on Phase 2–4 — needs all guards and auth infrastructure before documenting
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — first working auth
- **US2 (P1)**: Depends on US1 (needs AuthLayer) — role guards wrap the auth layer
- **US3 (P2)**: Depends on US1 (needs provisioning) — membership is part of the auth flow
- **US4 (P3)**: Depends on US1 + US2 (needs full guard API) — documentation wraps all guard types

### Within Each User Story

- Core auth modules before service wiring
- Service refactors can happen in parallel
- Story complete when its independent test passes

### Parallel Opportunities

| Phase | Parallel Tasks |
|-------|----------------|
| Phase 1 | T002 (Keycloak export), T003 (Traefik route), T004 (env templates) |
| Phase 3 | T014–T017 (all service refactors) — each is a separate main.rs file |
| Phase 4 | T018, T019 (require_role and require_authenticated) — different functions in same file |
| Phase 7 | T029, T030 (internal JWKS access, degraded mode test) |

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: US1 — Login and Token Access
4. **STOP and VALIDATE**: Test login flow end-to-end via quickstart.md
5. This is the MVP — working auth with JWT validation and first-login provisioning

### Incremental Delivery

1. Setup + Foundational → Foundation ready (common-auth crate compiles)
2. Add US1 → Login flow works → MVP!
3. Add US2 → RBAC enforced → Secure API
4. Add US3 → Partner membership → Tenant isolation ready
5. Add US4 → Developer DX → Reusable patterns documented
6. Polish → Observability and hardening

### Parallel Strategy

With multiple developers:

1. Developer A: Phase 1 + Phase 2 (Setup + Foundational)
2. Once foundational is done:
   - Developer A: US1 (service refactors)
   - Developer B: US2 + US4 (role guards + documentation)
   - Developer C: US3 (partner membership)
3. Polish (Phase 7) can be distributed

---

## Summary

| Phase | Description | Tasks | Runtime |
|-------|-------------|-------|---------|
| Phase 1 | Setup | T001–T005 | Sequential + parallel |
| Phase 2 | Foundational | T006–T009 | Sequential + parallel |
| Phase 3 | US1: Login & Token Access | T010–T017 | Sequential + parallel |
| Phase 4 | US2: Role-Based Access Control | T018–T021 | Sequential + parallel |
| Phase 5 | US3: Partner Membership | T022–T024 | Sequential + parallel |
| Phase 6 | US4: Auth-Guard Middleware | T025–T027 | Sequential |
| Phase 7 | Polish | T028–T033 | Sequential + parallel |
| **Total** | | **33 tasks** | |
