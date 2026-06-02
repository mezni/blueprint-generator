# Implementation Plan: Identity & Authentication Foundation

**Branch**: `003-identity-auth-foundation` | **Date**: 2026-06-02 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/003-identity-auth-foundation/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Implement OAuth2/JWT-based authentication across the platform. Update Keycloak realm export with proper clients and roles; create a shared `common-auth` crate for JWT validation via JWKS, role checking, and tenant context extraction; add auth middleware to Driver Service and Admin Service with route-level access control; enforce partner isolation via JWT `tenant_id` claim; provide integration test suite for end-to-end auth validation.

## Technical Context

**Language/Version**: Rust (stable 1.70+, edition 2021)

**Primary Dependencies**:
- `jsonwebtoken` (Rust JWT validation with JWKS support)
- `reqwest` (HTTP client for JWKS fetch from Keycloak)
- `axum` 0.7 (already in workspace — middleware layer for route guards)
- `tracing` 0.1 (already in workspace — auth event logging)
- `serde` / `serde_json` (already in workspace — claim deserialization)

**Storage**: No new databases. Keycloak (existing + updated realm export) is the identity store.

**Testing**: `cargo test` (Rust unit/integration for `common-auth`), shell-based `scripts/auth-smoke-test.sh` against running Docker Compose stack

**Target Platform**: Linux (Docker Engine 24+, Docker Compose v2)

**Project Type**: Distributed backend system — auth middleware crate + service integration

**Performance Goals**: JWT validation adds < 5ms p99 overhead per request; JWKS refresh completes in < 1s under normal conditions

**Constraints**: No DB persistence for User entity this sprint; no token refresh on backend; fail-secure (boot without JWKS but reject all auth); no rate limiting on auth endpoints yet

**Scale/Scope**: 5 Rust services (infrastructure crate), 2 services with enforcement (Driver, Admin), 1 Keycloak realm export update

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Data-First, Contract-Driven)**: Identity source of truth remains Keycloak — no identity data duplicated in DB — compliant
- **Principle II (Strict Service Boundaries)**: New `common-auth` crate is a shared library (not a service), maintaining service boundaries — compliant
- **Principle III (Authorization & Tenant Isolation)**: Core sprint focus — backend + repository layer enforcement, partner isolation from JWT tenant_id, no client-supplied tenant IDs — compliant
- **Principle IV (REST-Only, Contract-Driven APIs)**: Auth enforcement layers on existing REST endpoints; no new API paradigms introduced — compliant
- **Principle V (Event-Driven, Eventually Consistent)**: Auth is synchronous (JWT validation per request) — no conflict with eventing model — compliant
- **Technology & Infrastructure Constraints**: All choices (Rust, axum, Keycloak, Docker Compose, Traefik) match constitution — compliant. New dependency `jsonwebtoken` is a standard Rust JWT library — no violation
- **Development & Operational Workflow**: No constitution amendments needed

**Result: PASS** — No violations. Proceeding to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/003-identity-auth-foundation/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── auth-envelope.md
│   ├── auth-middleware-api.md
│   └── auth-error-codes.md
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
.
├── crates/
│   └── common-auth/                 # Extended from skeleton
│       ├── Cargo.toml               # +jsonwebtoken, +reqwest deps
│       └── src/
│           ├── lib.rs               # Public API re-exports
│           ├── config.rs            # AuthConfig (JWKS_URL, refresh interval, etc.)
│           ├── jwt.rs               # JwtValidator — JWKS fetch, cache, token validation
│           ├── claims.rs            # Claims, AuthContext, Role types
│           ├── middleware.rs        # Axum middleware: require_role, require_any_role
│           └── error.rs            # AuthError enum with distinct error codes
├── infra/
│   ├── compose/
│   │   └── docker-compose.yml      # May need admin console port restriction for FR-020
│   ├── keycloak/
│   │   └── realm-export/
│   │       └── ev-platform-realm.json  # UPDATED: proper clients, roles, mappers
│   └── env/
│       └── local/                   # Updated env files with JWKS-related vars
│           ├── driver-service.env   # +JWKS_URL, JWKS_REFRESH_INTERVAL, ALLOWED_ISSUERS, REQUIRED_AUDIENCE
│           ├── admin-service.env    # +JWKS related vars
│           ├── clickstream-service.env   # +JWKS related vars (infrastructure only)
│           ├── gis-worker.env            # +JWKS related vars (infrastructure only)
│           └── analytics-writer.env      # +JWKS related vars (infrastructure only)
├── services/
│   ├── driver-service/             # +auth middleware on protected routes
│   │   ├── Cargo.toml              # +common-auth dep
│   │   └── src/
│   │       ├── main.rs             # +auth middleware layer, public route split
│   │       └── config.rs           # +auth config fields (delegated to common-auth)
│   ├── admin-service/              # +auth middleware on all routes
│   │   ├── Cargo.toml              # +common-auth dep
│   │   └── src/
│   │       ├── main.rs             # +auth middleware, admin-only guard
│   │       └── config.rs           # +auth config fields
│   ├── clickstream-service/        # +JWKS load infrastructure (no enforcement yet)
│   ├── gis-worker/                 # +JWKS load infrastructure (no enforcement yet)
│   └── analytics-writer/           # +JWKS load infrastructure (no enforcement yet)
└── scripts/
    └── auth-smoke-test.sh          # New: end-to-end auth integration tests
```

**Structure Decision**: Follows the existing monorepo layout. The new `common-auth` crate lives under `crates/` alongside existing common crates. Service configs are extended with auth-related env vars. No new services or databases are created.

## Complexity Tracking

No constitution violations to justify.
