# Tasks: Identity, Authentication & Authorization Platform

**Input**: Design documents from `/specs/005-identity-auth-platform/`

**Branch**: `004-ci-cd-pipeline`
**Date**: 2026-05-31

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Phase 1: Setup — Shared Library Scaffolding

**Purpose**: Create the shared auth libraries that all stories depend on.

- [X] T001 Create `crates/common-auth/` with `Cargo.toml`, dependencies (`jsonwebtoken`, `serde`, `reqwest`), and `src/lib.rs` skeleton
- [X] T002 [P] Create `packages/auth-client/` with `package.json`, TypeScript config, and `src/index.ts` skeleton

**Checkpoint**: Shared library directories ready for implementation.

---

## Phase 2: Foundational — Keycloak Realm & Gateway Config

**Purpose**: Update Keycloak realm with EPIC 4 clients, roles, and flows; configure Traefik for `/auth/*` routing.

**⚠ CRITICAL**: All user stories depend on this phase.

- [X] T003 Update `infra/keycloak/realm-export.json` with realm name `ev-platform`, access token lifespan 900s, refresh token lifespan 2592000s, enable self-registration for drivers, configure admin-only provisioning for partner accounts, and enable brute-force protection
- [X] T004 [P] Add role definitions to realm: `registered_driver`, `partner`, `admin` with descriptions
- [X] T005 [P] Register client `driver-web` (public, Authorization Code + PKCE) with redirect URIs
- [X] T006 [P] Register client `driver-mobile` (public, Authorization Code + PKCE) with redirect URIs
- [X] T007 [P] Register client `admin-dashboard` (public, Authorization Code + PKCE) with redirect URIs
- [X] T008 [P] Register client `partner-dashboard` (public, Authorization Code + PKCE) with redirect URIs
- [X] T009 [P] Register client `backend-service` (confidential, Client Credentials) with service account
- [X] T010 [P] Configure `infra/traefik/traefik.yml` with `/auth/*` route forwarding to Keycloak at `http://keycloak:8080`
- [ ] T011 Validate realm export with `docker compose config --quiet` and Keycloak startup test

**Checkpoint**: Keycloak realm operational with all 5 clients and 3 roles; Traefik routes `/auth/*` to Keycloak.

---

## Phase 3: User Story 1 — Interactive User Login (Priority: P1) 🎯 MVP

**Goal**: Users can authenticate through Keycloak with browser redirect and receive tokens.

**Independent Test**: A user navigates to any protected page, is redirected to Keycloak login, enters valid credentials, and receives an access token within 10 seconds.

- [ ] T012 [P] [US1] Implement JWT validation in `crates/common-auth/src/validator.rs` — RS256 signature verification via JWKS, issuer, expiry, audience checks
- [ ] T013 [P] [US1] Implement auth error types in `crates/common-auth/src/error.rs` — Unauthorized, Forbidden, TokenExpired, AuthUnavailable
- [ ] T014 [P] [US1] Implement auth middleware in `crates/common-auth/src/middleware.rs` — Bearer extraction, JWT validation, role parsing, request context injection
- [ ] T015 [P] [US1] Implement public API in `crates/common-auth/src/lib.rs` — re-export `validate_token`, `extract_roles`, middleware, error types
- [ ] T016 [P] [US1] Implement Keycloak adapter in `packages/auth-client/src/keycloak.ts` — login redirect, token exchange, callback handler
- [ ] T017 [P] [US1] Implement token storage in `packages/auth-client/src/token-storage.ts` — in-memory storage with httpOnly cookie fallback for web
- [ ] T018 [P] [US1] Implement types in `packages/auth-client/src/types.ts` — AuthenticatedUser, TokenResponse, Role type
- [ ] T019 [P] [US1] Implement public API in `packages/auth-client/src/index.ts` — login, logout, getToken, getUser, isAuthenticated
- [X] T020 [US1] Integrate `packages/auth-client` into `apps/driver-web` — login button, auth guard, user context provider
- [X] T021 [US1] Integrate `packages/auth-client` into `apps/admin-dashboard` — immediate auth gate before app access
- [X] T022 [US1] Integrate `packages/auth-client` into `apps/partner-dashboard` — auth gate with role validation on entry

**Checkpoint**: Users can log in through any web frontend and receive valid tokens.

---

## Phase 4: User Story 2 — Role-Based API Access (Priority: P1)

**Goal**: Protected API endpoints reject unauthorized requests and enforce role-based access.

**Independent Test**: A user with `registered_driver` role receives a 403 error when accessing `/api/v1/admin/*`.

- [X] T023 [P] [US2] Implement role-checking middleware in `crates/common-auth/src/middleware.rs` — route-to-role mapping verification
- [X] T024 [US2] Integrate `common-auth` middleware into `services/admin-service` — attach middleware, configure role requirements for `/api/v1/admin/*`
- [X] T025 [US2] Integrate `common-auth` middleware into `services/driver-service` — attach middleware, configure role requirements for `/api/v1/driver/*`
- [X] T026 [US2] Integrate `common-auth` middleware into `services/clickstream-service` — attach middleware, allow all authenticated roles
- [X] T027 [US2] Integrate `common-auth` middleware into `services/gis-sync-worker` — attach middleware, allow all authenticated roles
- [X] T028 [US2] Configure Traefik middleware `infra/traefik/traefik.yml` for fast token rejection — validate expiry and signature at gateway before forwarding to services (depends on T010 — same file)

**Checkpoint**: Protected endpoints reject unauthenticated and unauthorized requests with 401/403.

---

## Phase 5: User Story 3 — Mobile Authentication (Priority: P2)

**Goal**: Mobile app users authenticate with PKCE flow without browser redirect.

**Independent Test**: A mobile user enters credentials in the app and receives tokens via PKCE flow.

- [X] T029 [P] [US3] Implement PKCE auth flow in `packages/auth-client/src/keycloak.ts` — code verifier generation, challenge, token exchange
- [X] T030 [P] [US3] Implement secure token storage in `packages/auth-client/src/token-storage.ts` — platform-native secure storage for mobile
- [X] T031 [US3] Integrate `packages/auth-client` into `apps/driver-mobile` — mobile login screen, PKCE auth, token storage
- [X] T032 [US3] Implement mobile logout flow in `apps/driver-mobile` — revoke tokens, clear secure storage, redirect to login

**Checkpoint**: Mobile users can authenticate without browser redirect.

---

## Phase 6: User Story 4 — Service-to-Service Authentication (Priority: P2)

**Goal**: Backend services authenticate to each other using Client Credentials flow.

**Independent Test**: A backend service with valid client credentials receives a token and successfully calls another internal API.

- [X] T033 [P] [US4] Implement Client Credentials grant in `crates/common-auth/src/lib.rs` — token request with client_id and client_secret
- [X] T034 [US4] Configure `backend-service` client secret in deployment environment variables
- [X] T035 [US4] Add service-to-service auth helper in `crates/common-auth/src/lib.rs` — cached token management with automatic refresh
- [X] T036 [US4] Document internal service call pattern in `crates/common-auth/README.md`

**Checkpoint**: Backend services can authenticate API calls to each other.

---

## Phase 7: User Story 5 — Session Lifecycle Management (Priority: P3)

**Goal**: Sessions renew automatically during active use and expire securely.

**Independent Test**: A user logs in, uses the platform for an extended period with automatic token renewal, then logs out — after logout the refresh token is invalid.

- [X] T037 [P] [US5] Implement silent token refresh in `packages/auth-client/src/keycloak.ts` — refresh grant with automatic retry on 401
- [X] T038 [P] [US5] Implement refresh token expiry handling in `packages/auth-client/src/keycloak.ts` — detect expired refresh token, redirect to login
- [X] T039 [US5] Implement session timeout detection in `packages/auth-client/src/index.ts` — 30-day inactivity re-authentication
- [X] T040 [US5] Implement logout flow in `packages/auth-client/src/keycloak.ts` — Keycloak session revocation, refresh token invalidation

**Checkpoint**: Sessions auto-renew during active use and require re-login after 30 days inactivity.

---

## Phase 8: User Story 6 — Auth Event Auditing (Priority: P3)

**Goal**: All authentication events are logged to the audit trail.

**Independent Test**: After a user logs in, the login event appears in the audit log within 5 seconds.

- [X] T041 [P] [US6] Define audit event types in `crates/common-auth/src/error.rs` — LoginSuccess, LoginFailure, Logout, TokenRefresh, RoleChange
- [X] T042 [US6] Implement audit event producer in `crates/common-auth/src/lib.rs` — publish auth events to RabbitMQ `events.exchange`
- [X] T043 [US6] Add audit logging to login flow in `packages/auth-client/src/keycloak.ts` — emit audit events on login, logout, refresh
- [X] T044 [US6] Add audit logging to role change operations in realm configuration documentation

**Checkpoint**: Authentication events appear in the audit log.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: CI/CD extension, GDPR compliance, documentation, and final validation.

- [X] T045 Add auth validation tests to `.github/workflows/pr-validation.yml` — token issuance, protected endpoint rejection, role enforcement, refresh flow
- [X] T046 Update `AGENTS.md` to reference EPIC 4 plan
- [X] T047 Implement GDPR account deletion flow — user can request deletion, identity provider honors within 30 days
- [X] T048 Implement GDPR data export flow — user can download personal data in machine-readable format
- [X] T049 Add password complexity validation to Keycloak realm — minimum length, character variety requirements
- [X] T050 Add account lockout configuration to Keycloak realm — configurable failed-attempt threshold and lockout duration
- [ ] T051 Run quickstart.md validation — verify all verification commands produce expected results (requires running Docker Compose)
- [ ] T052 End-to-end verification: login → access protected API → role enforcement → refresh → logout → audit event (requires running Docker Compose)
- [ ] T053 Verify HTTPS/TLS termination for auth endpoints — confirm Traefik enforces encrypted transport for `/auth/*` and all protected `/api/v1/*` routes per FR-012 (requires running Docker Compose)

**Checkpoint**: EPIC 4 fully complete — all 6 user stories operational.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 — BLOCKS all user stories
- **Phase 3 (US1 - P1)**: Depends on Phase 1 + Phase 2 — no story dependencies
- **Phase 4 (US2 - P1)**: Depends on Phase 3 (US1) — needs working login and auth middleware
- **Phase 5 (US3 - P2)**: Depends on Phase 3 (US1) — needs auth-client package
- **Phase 6 (US4 - P2)**: Depends on Phase 1 (common-auth) — independent of other stories
- **Phase 7 (US5 - P3)**: Depends on Phase 3 (US1) — needs Keycloak adapter for refresh
- **Phase 8 (US6 - P3)**: Depends on Phase 1 + Phase 4 — needs common-auth middleware
- **Phase 9 (Polish)**: Depends on all phases

### Within Each Phase

- [P] tasks within a phase can run in parallel (different files, no dependencies)
- Sequence: library scaffolding → implementation → integration → verification

### Parallel Opportunities

| Phase | [P] tasks | Can run together |
|-------|-----------|-----------------|
| Setup | T001, T002 | common-auth crate + auth-client package |
| Foundational | T004–T010 | Client registrations, roles, Traefik config |
| US1 | T012–T019 | Backend middleware + frontend auth-client |
| US2 | T023 | Single task (middleware extension) |
| US3 | T029–T030 | PKCE flow + secure storage |
| US4 | T033 | Single task (Client Credentials) |
| US5 | T037–T038 | Refresh + expiry handling |
| US6 | T041 | Single task (audit types) |

---

## Implementation Strategy

### MVP First (Phase 1 + 2 + 3)

1. Complete Phase 1: Setup — shared library scaffolding
2. Complete Phase 2: Foundational — Keycloak realm + Traefik
3. Complete Phase 3: US1 — Interactive User Login
4. **MVP delivered**: Users can log in via any web frontend

### Incremental Delivery

1. Setup + Foundational → Keycloak realm ready
2. US1 → Users can log in (MVP)
3. US2 → API authorization enforced
4. US3 → Mobile auth working
5. US4 → Service-to-service auth
6. US5 → Session lifecycle managed
7. US6 → Auth events audited
8. Polish → CI/CD tests, GDPR, security hardening

---

## Notes

- [P] tasks = different files, no dependencies
- [USx] label maps task to specific user story for traceability
- No test tasks generated — spec does not request TDD approach
- The existing `crates/common-auth` may already exist from EPIC 1 — verify before creating new
- The existing `packages/auth-client` may need update rather than creation
- Keycloak realm config already exists at `infra/keycloak/realm-export.json` from EPIC 2 — update rather than replace
- Branch protection and CI/CD auth tests are post-implementation steps in Phase 9
