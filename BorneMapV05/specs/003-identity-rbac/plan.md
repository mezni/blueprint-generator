# Implementation Plan: Identity & RBAC

**Branch**: `003-identity-rbac` | **Date**: 2026-06-02 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/003-identity-rbac/spec.md`

## Summary

Implement the authentication and authorization backbone for the Bornemap platform. Configure Keycloak realm `bornemap` with three roles (`registered_driver`, `partner`, `admin`) and the `bornemap-api` OIDC client (Google/Facebook providers deferred). Keycloak is internal-only — auth traffic reaches it through Traefik proxying `/auth/*`. Build Rust JWT validation middleware in `common-auth` that validates issuer/JWKS/audience, extracts role, and enforces access control via reusable auth guards. In degraded mode (JWKS unreachable), validate cached JWTs using stale keys. Implement first-login provisioning to create `users.user_account` records on first valid JWT and `partner_membership` records when applicable. Enforce the rule that `partner_id` is never accepted from clients — always derived from membership. Log auth failures and provisioning events for observability.

## Technical Context

**Language/Version**: Rust (edition 2021) — mandated by Constitution and monorepo setup

**Primary Dependencies**: 
- `common-auth` crate (JWT validation middleware, auth guards)
- Keycloak OIDC endpoints (issuer, JWKS, token introspection)
- Rust ecosystem: JWKS fetching + JWT validation, HTTP client for JWKS endpoint, async trait support

**Storage**: 
- `keycloak_db` — Keycloak-owned identity store (not directly accessed by services)
- `platform_db` — schema `users` tables: `user_account`, `partner_membership`, `user_profile`

**Testing**: `cargo test` with isolated test Keycloak realm + test platform_db; integration tests cover JWT validation, role gating, first-login provisioning, partner_id derivation

**Target Platform**: Linux (Docker containers), internal Docker network

**Project Type**: Backend service library (`crates/common-auth`) consumed by all 5 backend services

**Performance Goals**: JWT validation complete in under 50ms (p95); JWKS caching to avoid per-request network fetch; first-login provisioning under 200ms

**Constraints**: 
- Keycloak is the ONLY authentication system — no local password storage
- Keycloak is internal-only — never publicly exposed; Traefik proxies `/auth/*` to Keycloak
- gis-worker and analytics-writer are internal-only — no public port exposure
- Exactly 3 roles: no additional roles ever
- `partner_id` NEVER from client — always derived from `partner_membership`
- Standard API envelopes for auth errors (UNAUTHENTICATED, TOKEN_EXPIRED, INSUFFICIENT_ROLE, FORBIDDEN)
- `/health` endpoints exempt from JWT validation
- Env-var-driven configuration (AUTH_ISSUER, AUTH_JWKS_URL, AUTH_AUDIENCE)
- Auth enforced at backend (primary) AND repository layer (mandatory)
- Auth failures and provisioning events MUST be logged with structured JSON (FR-014)
- JWKS keys cached with TTL; degraded mode validates stale cached keys when JWKS is unreachable

**Scale/Scope**: Single-region, moderate concurrency (<100 events/sec baseline). 3 roles across 5 backend services. First-login provisioning must be idempotent (exactly once per Keycloak user).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Status | Notes |
|------|--------|-------|
| **I. Data-First Source of Truth** | ✅ PASS | Identity source = Keycloak; user_account bridge = platform_db; no dual-write or event-driven identity |
| **II. Strict Domain & Service Separation** | ✅ PASS | Auth lives in `common-auth` crate, consumed by services; no new service needed |
| **III. Ownership-Enforced Authorization** | ✅ PASS | partner_id derived from membership DB, never client; exactly 3 roles; backend-enforced |
| **IV. Contract-Driven REST APIs** | ✅ PASS | Standard envelope error codes (UNAUTHENTICATED, TOKEN_EXPIRED, INSUFFICIENT_ROLE, FORBIDDEN); URL-based versioning unchanged |
| **V. Event-Driven & Derived State** | ✅ N/A | Auth is synchronous; no event emission for identity events in this sprint |
| **VI. Soft Delete & Auditability** | ✅ PASS | user_account supports audit fields; no soft-delete needed for identity (Keycloak manages user lifecycle) |
| **VII. Verification Discipline** | ✅ PASS | All auth paths have acceptance scenarios; integration tests with isolated Keycloak realm required |

**No violations found. Constitution fully satisfied.**

## Project Structure

### Documentation (this feature)

```text
specs/003-identity-rbac/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output — resolved design decisions
├── data-model.md        # Phase 1 output — entity definitions
├── quickstart.md        # Phase 1 output — developer guide
├── contracts/           # Phase 1 output — API contract specs
└── checklists/          # Quality checklists
```

### Source Code (repository root)

```text
crates/common-auth/           # NEW — JWT validation middleware + auth guards
├── Cargo.toml
└── src/
    ├── lib.rs                # Public API: AuthMiddleware, require_role, etc.
    ├── jwt.rs                # JWT validation, JWKS fetching + caching
    ├── guards.rs             # Auth guard middleware (public, authenticated, role-gated)
    ├── provisioning.rs       # First-login user_account + partner_membership creation
    └── errors.rs             # Auth-specific error types → standard envelope codes

services/driver-service/      # MODIFY — add auth middleware to HTTP handler
services/admin-service/       # MODIFY — add auth middleware to HTTP handler
services/clickstream-service/ # MODIFY — add auth middleware (accept anonymous)
services/gis-worker/          # MODIFY — health endpoint exemption
services/analytics-writer/    # MODIFY — health endpoint exemption

infra/compose/                # MODIFY — Keycloak realm import config
```

**Structure Decision**: New `common-auth` crate under `crates/` following existing monorepo pattern. Each backend service adds the auth middleware to its router. Keycloak realm configuration is managed as a realm export file imported on startup. Only Traefik is publicly exposed — Keycloak sits behind Traefik's `/auth/*` route; gis-worker and analytics-writer are internal-only with no Traefik route.

## Complexity Tracking

No constitution violations to justify.
