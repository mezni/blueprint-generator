# Feature Specification: MVP 5 — Identity Hardening (Keycloak)

**Feature Branch**: `005-mvp5-identity`
**Created**: 2026-05-23
**Status**: Draft
**Input**: User description: "Replace mock JWT with Keycloak using OAuth2 Authorization Code + PKCE, add Google/Facebook federation, and wire the admin invitation pipeline. Token claim shape stays frozen."

> Predecessors: MVPs 1–4 merged and validated.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Driver signs in with Google (Priority: P1)

A driver picks "Sign in with Google", completes OAuth, and lands in the app with a session — no manual password.

**Independent Test**: A fresh user with a Google account completes signup and reaches the map ≤ 30 s, with a driver-role JWT issued by Keycloak.

**Acceptance Scenarios**:

1. **Given** the user clicks "Sign in with Google", **When** they complete the upstream IdP flow, **Then** Keycloak issues tokens and the backend creates the user idempotently (Principle X).
2. **Given** the same user signs in a second time, **When** they reach the callback, **Then** no duplicate user row is created.
3. **Given** an attacker replays a stale auth code, **When** they call `/callback` without the PKCE verifier, **Then** the request is rejected.

---

### User Story 2 — Admin invites a collaborator (Priority: P1)

An admin enters an email + role in `InviteCollaborators`. The invitee receives a single-use link, completes signup in Keycloak, and is created with the assigned role.

**Independent Test**: A new admin can invite a colleague who completes signup in ≤ 5 min and lands with admin privileges.

**Acceptance Scenarios**:

1. **Given** an admin posts `POST /api/v1/admin/invitations { email, role }`, **When** the server processes, **Then** a signed single-use token is persisted with 24 h TTL.
2. **Given** the invitee opens the link, **When** they accept the invitation with a password, **Then** a Keycloak user is provisioned and a profile is initialized idempotently.
3. **Given** the invitee opens the link after 24 h, **When** they accept, **Then** the response is 410 Gone.
4. **Given** the invitee opens an already-consumed link, **When** they accept, **Then** the response is 409 Conflict.

---

### User Story 3 — Backend validates Keycloak tokens (Priority: P1)

All `/api/v1/*` endpoints validate the JWT signature against Keycloak's JWKS endpoint. Claim shape is unchanged.

**Independent Test**: A token issued by Keycloak passes; a self-issued JWT signed with the mock secret is rejected.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST deploy Keycloak in the data-storage subnet with no public ingress.
- **FR-002**: System MUST expose `GET /api/v1/auth/login-url` returning `{ url, state, code_verifier }` for the PKCE flow.
- **FR-003**: System MUST expose `POST /api/v1/auth/callback { code, state, code_verifier }`, validate the auth code, and return tokens.
- **FR-004**: System MUST validate incoming JWTs against the Keycloak JWKS endpoint, with JWKS cached for ≤ 10 minutes.
- **FR-005**: Token claim shape MUST remain unchanged from MVP 1 — verified by a snapshot test.
- **FR-006**: System MUST configure Keycloak identity providers for Google and Facebook (Realm-level federation).
- **FR-007**: System MUST expose `POST /api/v1/admin/invitations { email, role }` (admin only) producing a single-use signed token (HMAC) stored in `identity_domain.invitations` with `email`, `role`, `expires_at`, `consumed_at`.
- **FR-008**: System MUST expose `POST /api/v1/auth/accept-invitation { token, password? }`. On success, it provisions the Keycloak user (via admin API) and runs idempotent profile init.
- **FR-009**: Mock-auth endpoints from MVP 1–2 MUST be removed in production (kept under a `dev-only` Cargo feature flag for local development).
- **FR-010**: All admin actions (invitation create/consume, role change) MUST emit events to `event_domain.events`.

### Key Entities

- **Invitation**: `id`, `email`, `role`, `token_hash`, `expires_at`, `consumed_at`.
- **KeycloakUser**: External identity mapped by `sub` claim → internal `identity_domain.users.id` (one-to-one).
- **IdentityProvider**: Realm-level federation config for Google / Facebook.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: P95 identity-sync hook (federated login through to local profile materialization) ≤ **500 ms**.
- **SC-002**: Token claim shape diff vs MVP 4 = **0**.
- **SC-003**: External penetration test report attached; zero **Critical** or **High** findings open at gate.
- **SC-004**: ≥ **95%** of invitees complete signup in ≤ **5 min** during validation.
- **SC-005**: All mock-auth code paths are unreachable in production builds (verified by feature-flag check in CI).

### Proceed Criteria → MVP 6 (Conditional)

All SC-001..SC-005 met **AND** at least one MVP 6 trigger observed in production:

- Sustained CPU/memory saturation unresolvable by vertical scaling.
- Per-domain traffic skew ≥ 10×.
- Regulatory / tenancy / contractual isolation requirement.

If no trigger, the platform stays on the modular monolith.

## Assumptions

- Keycloak version: latest 24.x LTS.
- Email transport: configurable SMTP (provider-specific config out of scope here).
- Google + Facebook OAuth client credentials are provisioned manually before MVP start (recorded in `specs/005-…/research.md`).
- Mobile app uses an in-app browser tab (SFSafariViewController / Custom Tabs) for the OAuth dance.

## Constitutional Gates

- [ ] Frozen claim shape preserved (Principle II + §6).
- [ ] No public ingress to Keycloak (Principle V security).
- [ ] Idempotent profile init on every federated and invitation signup (Principle X).
- [ ] Admin invitation is the only path to admin accounts.
- [ ] OpenAPI updated; clients regenerated; mock endpoints removed in prod build.
