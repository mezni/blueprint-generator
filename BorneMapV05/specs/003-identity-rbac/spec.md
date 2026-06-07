# Feature Specification: Identity & RBAC

**Feature Branch**: `003-identity-rbac`

**Created**: 2026-06-02

**Status**: Draft

**Input**: User description: "read docs/EXECUTION_PLAN.md and start sprint 3"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - User Login and Token-Based Access (Priority: P1)

A user (registered driver, partner, or admin) authenticates through their identity provider (Google, Facebook, or email/password managed by Keycloak). Keycloak is internal-only (no public port exposure) — all auth traffic reaches it through Traefik which proxies `/auth/*` requests to the Keycloak container. Upon successful login, a JWT token is issued. The user's subsequent API requests carry this token, and the system validates it before granting access to protected resources. On first login, the user's identity is automatically provisioned in the platform database.

**Why this priority**: Without authentication, no protected API can be accessed. All four frontends and all authenticated endpoints depend on this working correctly. This is the foundational capability.

**Independent Test**: Can be fully tested by initiating a login flow through Keycloak, obtaining a valid JWT, and using it to access a protected endpoint. Delivers the core value of authenticated API access.

**Acceptance Scenarios**:

1. **Given** a user with valid credentials, **When** they complete the Keycloak login flow, **Then** they receive a signed JWT with the correct role claim
2. **Given** a valid JWT, **When** the user calls a protected endpoint, **Then** the request succeeds (HTTP 200)
3. **Given** an expired or malformed JWT, **When** the user calls a protected endpoint, **Then** the request is rejected with UNAUTHENTICATED error
4. **Given** a first-time user logging in, **When** the JWT is validated, **Then** a `user_account` record is created exactly once

---

### User Story 2 - Role-Based Access Control (Priority: P1)

Three distinct user roles exist: `registered_driver`, `partner`, and `admin`. Each role has access to a specific set of API endpoints. A user with insufficient role privileges is blocked from accessing endpoints beyond their authorization scope. The system enforces authorization at the backend for every request.

**Why this priority**: RBAC is the core security model of the platform. The Constitution mandates exactly three roles and backend-enforced authorization. All business logic in subsequent sprints depends on this enforcement.

**Independent Test**: Can be tested by obtaining JWTs for each role and verifying that only the correct role can access role-specific endpoints. Delivers provable role isolation.

**Acceptance Scenarios**:

1. **Given** a `registered_driver` JWT, **When** the user calls an admin-only endpoint, **Then** the request is rejected with INSUFFICIENT_ROLE error
2. **Given** a `partner` JWT, **When** the user calls a driver-only endpoint, **Then** the request is rejected with INSUFFICIENT_ROLE error
3. **Given** an `admin` JWT, **When** the user calls any endpoint, **Then** the request is authorized if the endpoint exists

---

### User Story 3 - Partner Membership and Tenant Isolation (Priority: P2)

A partner user's organization affiliation (`partner_id`) is derived exclusively from the `partner_membership` table in the database, never from client-supplied data. On first login, if the Keycloak user is pre-configured as a partner member, the `partner_membership` record is automatically created. Every partner-scoped query is filtered by this derived identity.

**Why this priority**: This enforces the Constitution's partner isolation principle. Although the partner business APIs come in Sprint 5, the identity bridge must be set up now to avoid data model changes later.

**Independent Test**: Can be tested by provisioning a partner user with a known `partner_id` and verifying that the ID is correctly derived from the membership table on login. Delivers tenant isolation proof.

**Acceptance Scenarios**:

1. **Given** a partner user logging in for the first time with a pre-configured Keycloak `partner_id` attribute, **When** the JWT is validated, **Then** a `partner_membership` record is created linking the user to the correct partner
2. **Given** a partner user with an existing membership, **When** they call a partner-scoped endpoint, **Then** the `partner_id` is derived from the membership table, not from request parameters

---

### User Story 4 - Auth-Guard Middleware for API Development (Priority: P3)

Developers building API endpoints can apply reusable auth-guard middleware that validates the JWT, extracts the user identity and role, and rejects unauthorized requests before the request reaches the handler. The middleware supports three guard levels: public (no auth required), authenticated (any valid user), and role-gated (specific role required).

**Why this priority**: This accelerates development of all authenticated endpoints in subsequent sprints. Without a reusable auth framework, each endpoint would need duplicated auth logic.

**Independent Test**: Can be tested by applying the middleware to a test endpoint and verifying each guard level behaves correctly with valid/invalid tokens and matching/non-matching roles.

**Acceptance Scenarios**:

1. **Given** a public-guarded endpoint, **When** a request arrives without a token, **Then** the request is processed normally
2. **Given** an authenticated-guarded endpoint, **When** a request arrives with a valid token, **Then** the user identity and role are available to the handler
3. **Given** a role-gated endpoint requiring `admin`, **When** a request arrives with a `registered_driver` token, **Then** the request is rejected with INSUFFICIENT_ROLE

---

### Edge Cases

- What happens when a user's Keycloak account is disabled but a `user_account` record exists? (The JWT validation fails, preventing access; the account remains in DB for audit trail.)
- How does the system handle a user who was a `registered_driver` and is later promoted to `partner`? (The Keycloak role is updated; on next login the system re-syncs the role. The `partner_membership` must be created by an admin beforehand.)
- What happens when a partner user's membership is revoked? (The Keycloak role is changed; the membership record remains but the role no longer grants partner access.)
- How does the system behave when the JWKS endpoint is unreachable? (Degraded mode: validate cached JWTs using stale keys for existing sessions; requests requiring a new JWKS fetch (cache miss) are rejected with UNAUTHENTICATED. Health-check endpoints remain accessible.)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST authenticate users exclusively through Keycloak. No passwords or sessions are stored in the platform.
- **FR-002**: System MUST support exactly three roles: `registered_driver`, `partner`, and `admin`.
- **FR-003**: System MUST validate every authenticated API request by verifying the JWT's signature, issuer, audience, and expiration against a JWKS endpoint.
- **FR-004**: System MUST reject requests with missing, expired, or invalid tokens with a UNAUTHENTICATED error code.
- **FR-005**: System MUST reject requests where the authenticated user's role is insufficient for the endpoint with an INSUFFICIENT_ROLE error code.
- **FR-006**: System MUST automatically create a `users.user_account` record on first valid JWT login, mapping `keycloak_user_id` to the JWT `sub` claim.
- **FR-007**: System MUST derive `partner_id` exclusively from `users.partner_membership` table, NEVER from client-supplied data.
- **FR-008**: System MUST automatically create a `partner_membership` record on first login if the Keycloak user attributes include a valid partner association. An admin pre-provisions the partner association by setting a custom `partner_id` attribute on the Keycloak user via the Keycloak admin console before the user's first login.
- **FR-009**: System MUST provide a reusable auth-guard middleware supporting three modes: public (no auth), authenticated (any valid JWT), role-gated (specific role required).
- **FR-010**: System MUST configure Keycloak realm `bornemap` with the three roles and the `bornemap-api` OIDC client. Google and Facebook identity provider stubs may be included in the realm export but are not wired with real credentials in this sprint.
- **FR-011**: System MUST NOT expose Keycloak, gis-worker, or analytics-writer directly to external networks. Only Traefik is publicly exposed. Keycloak auth traffic reaches it through Traefik which proxies `/auth/*` to Keycloak's HTTP port. Backend services access the JWKS endpoint directly via Keycloak's internal Docker hostname.
- **FR-012**: System MUST reject `partner_id` if it appears in request bodies or query parameters for endpoints that derive it from membership.
- **FR-013**: System MUST provision the Keycloak realm configuration as code (not manual UI setup), so it is reproducible across environments.
- **FR-014**: System MUST log all auth failure events (missing, expired, invalid tokens; wrong role) and all first-login provisioning events, with structured JSON including request correlation ID and error code. Successful auth validations are not logged unless explicitly required for compliance.

## Clarifications

### Session 2026-06-02

- Q: What auth events must be logged? → A: Log auth failures (missing/expired/invalid token, wrong role) and first-login provisioning events. Successful validations not logged.
- Q: How should services behave during Keycloak unavailability? → A: Degraded mode — validate cached JWTs using stale keys; reject requests needing a fresh JWKS fetch (cache miss).
- Q: Are Google/Facebook OAuth provider integrations in scope for this sprint? → A: Realm + OIDC client only; Google/Facebook providers deferred.
- Q: Is Keycloak publicly accessible? → A: No. Keycloak is internal-only. Traefik proxies `/auth/*` to Keycloak. Backend services access Keycloak directly via internal Docker hostname.
- Q: Are gis-worker and analytics-writer internal or external? → A: Internal-only. Only Traefik is publicly exposed.

### Key Entities *(include if feature involves data)*

- **Keycloak Realm `bornemap`**: The identity domain. Contains roles, users, identity providers (Google, Facebook), and OIDC client configuration.
- **JWT (JSON Web Token)**: The authentication token. Contains claims for `sub` (user ID), `realm_access.roles` (role), `iss` (issuer), `aud` (audience), and `exp` (expiration).
- **`users.user_account`**: Platform-side identity bridge. Maps Keycloak user ID (`keycloak_user_id`) to an internal ULID (`USR-*`). One record per Keycloak user.
- **`users.partner_membership`**: Associates a `user_account` with a `partner` organization and defines their role within it (`owner`, `manager`, `operator`, `viewer`).
- **JWKS (JSON Web Key Set)**: Public key set used to verify JWT signatures. Served by Keycloak's OIDC endpoint.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can complete the login flow and receive a valid JWT within 3 seconds under normal network conditions.
- **SC-002**: Protected endpoints consistently reject unauthenticated requests with UNAUTHENTICATED or TOKEN_EXPIRED errors before any business logic executes.
- **SC-003**: Role-gated endpoints reject requests from users with insufficient roles 100% of the time (zero false accepts).
- **SC-004**: First-login provisioning creates a `user_account` record for 100% of first-time logins, exactly once (no duplicate records).
- **SC-005**: The auth-guard middleware enforces all three guard modes (public, authenticated, role-gated) with zero false accepts across all test scenarios.
- **SC-006**: A new developer can add a role-gated endpoint in under 5 minutes using the auth-guard middleware, measured from endpoint definition to passing auth test.

## Assumptions

- Keycloak is already deployed and operational (completed in Sprint 2).
- The `platform_db` schema with `users.user_account` and `users.partner_membership` tables is available (planned in Sprint 4 — this sprint will depend on Sprint 4 or define the schema as needed).
- Google and Facebook OIDC provider credentials are out of scope for this sprint. The realm export may include provider stubs, but wiring real credentials is deferred.
- Partner membership is pre-configured in Keycloak user attributes by an admin before the partner's first login.
- The JWKS URL and issuer URL follow standard Keycloak OIDC patterns and are provided via environment variables.
- Health-check endpoints (`/health`) are exempt from JWT validation to allow load balancer and monitoring systems to probe services without tokens.
- Only Traefik is publicly exposed. Keycloak, gis-worker, and analytics-writer are internal-only — no public port exposure. Traefik proxies the OIDC login flow paths (e.g., `/auth/realms/bornemap/*`) to Keycloak internally. Backend services access Keycloak directly via internal Docker hostname for JWKS fetching.
