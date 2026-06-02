# Feature Specification: Identity & Authentication Foundation

**Feature Branch**: `003-identity-auth-foundation`

**Created**: 2026-06-02

**Status**: Draft

**Input**: User description: Sprint 3 — Identity & Authentication Foundation: Implement a universal User entity with Keycloak-backed authentication. Every service validates JWT tokens via config-loaded JWKS. The Driver Service and Admin Service both check auth on ingress with route-level access control. Role-based API authorization using Keycloak realm roles. Per-tenant data isolation where tenant context is extracted from the JWT (tenant_id claim). Full integration test suite with docker compose testing via shell scripts. All IdP configuration is code-managed.

## User Scenarios & Testing

### User Story 1 - Developer provisions Keycloak realm from code (Priority: P1)

A developer can fully re-create the Keycloak `ev-platform` realm from a code-managed JSON export. No manual Keycloak admin console steps are needed to set up clients, roles, or identity providers. Running `docker compose up` applies the realm automatically on first boot.

**Why this priority**: IdP configuration-as-code is the prerequisite for all auth functionality — without it, every auth feature is manually configured and unreproducible.

**Independent Test**: Tear down Keycloak volumes, restart the stack, and verify the realm, clients, and roles are re-created identically via `curl` against the Keycloak admin API.

**Acceptance Scenarios**:

1. **Given** the Keycloak container has no prior state, **When** `docker compose up` runs, **Then** the `ev-platform` realm is imported automatically with clients `driver-web`, `partner-dashboard`, `admin-dashboard`, `driver-mobile`, and roles `registered_driver`, `partner`, `admin`.
2. **Given** the platform is running, **When** an operator queries the Keycloak admin API, **Then** all realm configuration matches exactly what is defined in `infra/keycloak/realm-export/ev-platform-realm.json`.
3. **Given** the realm JSON is modified in source control, **When** the stack is re-deployed, **Then** the changes are applied to the Keycloak instance.

---

### User Story 2 - Service validates every request with JWT (Priority: P1)

Every protected endpoint in Driver Service and Admin Service rejects unauthenticated requests. Services fetch JWKS from Keycloak on startup and validate JWT signatures, expiry, issuer, and audience on every incoming request without reaching out to Keycloak on each call.

**Why this priority**: This is the core authentication mechanism — all other auth stories depend on it.

**Independent Test**: Start the platform, obtain a valid JWT via Keycloak login, call a protected endpoint with and without the token, and verify correct HTTP 401/200 responses.

**Acceptance Scenarios**:

1. **Given** a service is running with JWKS loaded, **When** a request arrives without an `Authorization: Bearer` header, **Then** the service responds HTTP 401 with a standard error envelope.
2. **Given** a service is running, **When** a request arrives with an expired/malformed/invalid JWT, **Then** the service responds HTTP 401.
3. **Given** a service is running, **When** a request arrives with a valid JWT from Keycloak, **Then** the service extracts claims (sub, realm_roles, tenant_id) and passes them to route handlers.
4. **Given** a service starts, **When** it boots, **Then** it logs the JWKS fetch result (success or failure) before accepting requests.

---

### User Story 3 - Route-level access control enforces roles (Priority: P1)

Driver Service and Admin Service enforce role-based access at the route level. Unauthorized role access returns HTTP 403. Public endpoints (station discovery, search) remain accessible without authentication.

**Why this priority**: Role enforcement is the core authorization primitive — all partner isolation and admin capabilities depend on correct role checks.

**Independent Test**: For each protected route, verify that requests with each role type (or anonymous) produce the correct HTTP 200/403/401 responses.

**Acceptance Scenarios**:

1. **Given** the Driver Service is running, **When** an anonymous request hits `/api/v1/stations` (public), **Then** the response is HTTP 200.
2. **Given** the Driver Service is running, **When** a request with `registered_driver` role hits `/api/v1/favorites`, **Then** the response is HTTP 200.
3. **Given** the Driver Service is running, **When** an anonymous request hits `/api/v1/favorites`, **Then** the response is HTTP 401.
4. **Given** the Admin Service is running, **When** a request with `admin` role hits `/api/v1/admin/users`, **Then** the response is HTTP 200.
5. **Given** the Admin Service is running, **When** a request with `registered_driver` role hits `/api/v1/admin/users`, **Then** the response is HTTP 403.
6. **Given** any service, **When** a request with a `partner` role hits a `driver`-scoped endpoint, **Then** access is denied according to route configuration.

---

### User Story 4 - Partner isolation via JWT tenant context (Priority: P2)

When a `partner` role user makes requests, the `tenant_id` claim from their JWT is used to scope all data access. No client-supplied tenant ID is accepted. Repository-layer enforcement ensures partners can only access their own stations, chargers, and reports.

**Why this priority**: Per-tenant isolation is a constitution-mandated critical rule and prevents cross-tenant data leakage.

**Independent Test**: Log in as partner A, create a station, log in as partner B, verify partner B cannot see or modify partner A's station via any API endpoint.

**Acceptance Scenarios**:

1. **Given** partner A is authenticated, **When** they create a station, **Then** the station is associated with their `tenant_id`.
2. **Given** partner B is authenticated with a different `tenant_id`, **When** they list stations, **Then** they do not see partner A's station.
3. **Given** partner B is authenticated, **When** they attempt to update partner A's station by ID, **Then** the request is denied with HTTP 403 or the station is treated as not found (404).
4. **Given** a `registered_driver` user is authenticated, **When** they hit any partner-scoped route, **Then** the response is HTTP 403.

---

### User Story 5 - Developer runs full auth integration tests (Priority: P2)

A developer can run a shell-based test suite that validates the entire auth flow: Keycloak reachability, token acquisition, JWT validation, route-level role enforcement, and partner isolation — all against a running Docker Compose stack.

**Why this priority**: Automated verification prevents regressions in the auth system and validates the feature end-to-end.

**Independent Test**: Run `./scripts/auth-smoke-test.sh` against a running stack and verify it exits with code 0.

**Acceptance Scenarios**:

1. **Given** the platform is running, **When** `./scripts/auth-smoke-test.sh` executes, **Then** it verifies Keycloak admin API is reachable.
2. **Given** the platform is running, **When** the test script runs, **Then** it obtains tokens for each role type (`registered_driver`, `partner`, `admin`) and validates them against service JWT endpoints.
3. **Given** the platform is running, **When** the test script runs, **Then** it tests route access for each role and confirms expected HTTP 200/401/403 responses.
4. **Given** the platform is running, **When** the test script runs, **Then** it validates partner isolation by creating and querying cross-tenant resources.

---

### Edge Cases

- What happens when Keycloak is temporarily unreachable at service startup? Services must retry JWKS fetch with backoff; if JWKS never loads, the service should still boot but reject all authenticated requests (fail-secure).
- What happens when a JWT has an unknown `tenant_id`? The service should reject with HTTP 403 and log the event as a potential configuration error.
- What happens when a JWT contains roles not in the allowed set? Extra roles should be ignored, not rejected.
- How does system handle JWKS rotation? Services should periodically refresh JWKS (configurable interval, default 1 hour) without restart.
- What happens when a realm role is renamed in Keycloak? The role name in the JWT changes; services use the new name immediately after token refresh.
- What about token refresh flow? The backend does not implement refresh — that is a client-side concern using OAuth2 refresh tokens.

## Requirements

### Functional Requirements

- **FR-001**: Keycloak realm MUST be imported from a code-managed JSON export file at container startup with zero manual steps.
- **FR-002**: The `ev-platform` Keycloak realm MUST define exactly three roles: `registered_driver`, `partner`, `admin`.
- **FR-003**: The `ev-platform` realm MUST define clients: `driver-web`, `partner-dashboard`, `admin-dashboard`, `driver-mobile`, and a confidential service client `platform-service` for machine-to-machine auth.
- **FR-004**: Every Rust service MUST load JWKS from Keycloak on startup via the `{keycloak_url}/realms/ev-platform/protocol/openid-connect/certs` endpoint.
- **FR-005**: Every service MUST validate incoming JWT tokens on every request: signature verification, expiry check, issuer check (`{keycloak_url}/realms/ev-platform`), and audience check.
- **FR-006**: JWT validation MUST be performed locally using cached JWKS — no Keycloak round-trip per request.
- **FR-007**: Services MUST support periodic JWKS refresh (configurable interval via env var, default 3600s) to handle key rotation.
- **FR-008**: Driver Service MUST enforce route-level access:
  - Public (no auth): `GET /api/v1/stations`, `GET /api/v1/stations/{id}`, `GET /api/v1/search`
  - `registered_driver`: favorites, reviews, profile endpoints
  - `partner`: partner-scoped station management endpoints (future)
  - `admin`: not applicable (handled by Admin Service)
- **FR-009**: Admin Service MUST enforce route-level access:
  - `admin` role required for all endpoints
  - All other roles AND anonymous requests return HTTP 403
- **FR-010**: Partner isolation MUST be enforced at the service layer: `tenant_id` extracted from JWT claims is injected into repository queries.
- **FR-011**: Partner isolation MUST also be enforced at the repository/data-access layer: no SQL query may accept a tenant_id from client input.
- **FR-012**: Services MUST use a shared auth middleware/crate for JWT validation to ensure consistent behavior across all services.
- **FR-013**: A new common crate `common-auth` MUST be created under `crates/` containing the JWT validation logic, role checking utilities, and tenant context extraction.
- **FR-014**: `common-auth` MUST expose:
  - `AuthConfig` — loaded from env vars (JWKS URL, allowed issuers, refresh interval)
  - `JwtValidator` — validates tokens, returns `AuthContext` with claims
  - `AuthContext` — contains `sub`, `roles: Vec<Role>`, `tenant_id: Option<String>`
  - `require_role(Role)` — axum middleware that checks for a required role
  - `require_any_role(&[Role])` — axum middleware that checks for any of the listed roles
- **FR-015**: Services MUST support graceful degradation: if JWKS fetch fails at startup, the service boots but rejects all authenticated requests until JWKS is successfully loaded.
- **FR-016**: All JWT-related errors (expired, invalid signature, wrong issuer, missing audience) MUST return HTTP 401 with distinct `error.code` values for debugging.
- **FR-017**: All role-based access denials MUST return HTTP 403 with the standard error envelope.
- **FR-018**: Integration test suite MUST verify: Keycloak reachability, token acquisition for each role, JWT validation on protected endpoints, route-level role enforcement, partner isolation, public endpoint access.
- **FR-019**: Public endpoints MUST remain accessible without authentication and MUST NOT perform JWT validation.
- **FR-020**: Keycloak admin console access MUST be configured in the `local` profile only and disabled in `docker`/`staging` profiles.

### Key Entities

- **JwtValidator**: Shared component that parses and validates JWT tokens using configured JWKS, producing an `AuthContext`.
- **AuthContext**: Request-scoped security context containing user identifier (`sub`), assigned realm roles, and optional tenant identifier (`tenant_id`).
- **AuthMiddleware**: Axum middleware layer inserted into service router that performs JWT validation on protected routes and injects `AuthContext` into request extensions.
- **RouteGuard**: Declarative access control configuration mapping HTTP method + path patterns to required roles (or `public`).
- **RealmExport**: Code-managed JSON file defining the Keycloak realm (clients, roles, identity providers) — source of truth for IdP configuration.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All three role types (`registered_driver`, `partner`, `admin`) can obtain valid JWTs from Keycloak via the standard OAuth2 flow.
- **SC-002**: Every protected endpoint in Driver Service and Admin Service returns HTTP 401 for unauthenticated requests and HTTP 200 for authorized requests.
- **SC-003**: No protected endpoint accepts a JWT with an invalid signature, expired expiry, wrong issuer, or wrong audience.
- **SC-004**: Partner isolation is verified: partner A cannot access partner B's data via any endpoint.
- **SC-005**: Public endpoints return HTTP 200 without requiring any authentication.
- **SC-006**: `./scripts/auth-smoke-test.sh` passes on a fresh `docker compose up` stack with zero manual Keycloak setup.
- **SC-007**: Keycloak realm can be fully recreated from `infra/keycloak/realm-export/ev-platform-realm.json` by destroying and restarting the Keycloak container.

## Clarifications

### Session 2026-06-02

- Q: Should a persistent User entity (e.g., `user_account` table) be implemented this sprint? → A: No. The User entity is the `AuthContext` extracted from JWT claims. DB persistence for `user_account` is deferred to a future sprint when `platform_db` consolidation occurs.

## Assumptions

- The existing Keycloak container and realm configuration from Sprint 2 are the starting point — the realm export needs to be updated with proper client configurations and role definitions.
- The User entity is represented by the `AuthContext` extracted from JWT claims (sub, roles, tenant_id) — no DB persistence in this sprint. `users_db` exists (created in Sprint 2) but the `user_account` table is deferred to a future sprint when `platform_db` consolidation occurs.
- JWT tokens use the `sub` claim for user identification and `realm_access.roles` for role information.
- Tenant ID is conveyed via a custom `tenant_id` claim in the JWT (added by Keycloak via a user attribute mapper).
- JWT `tenant_id` claim is a temporary bridge. The constitution requires `tenant_id` to be derived from `users.partner_membership` — this will be implemented when the `user_account` table is created in a future sprint.
- Services already have `keycloak_url` in their env configuration from Sprint 2; additional JWKS-related env vars (`JWKS_URL`, `JWKS_REFRESH_INTERVAL`, `ALLOWED_ISSUERS`, `REQUIRED_AUDIENCE`) are added in this sprint.
- No refresh token handling on the backend — clients manage their own token refresh.
- No user registration flow in this sprint — users are created manually in Keycloak for testing.
- The `platform-service` confidential client is used for service-to-service communication in future sprints.
- Auth-related integration tests are added to `scripts/auth-smoke-test.sh` alongside any updates to the existing `scripts/smoke-test.sh`.
