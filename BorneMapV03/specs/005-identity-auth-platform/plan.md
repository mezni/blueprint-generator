# Implementation Plan: Identity, Authentication & Authorization Platform

**Branch**: `004-ci-cd-pipeline` | **Date**: 2026-05-31 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/005-identity-auth-platform/spec.md`

## Summary

Establish the complete identity and access management foundation for the BorneMap EV platform. This epic integrates Keycloak as the centralized identity provider, defines 3 user roles (registered_driver, partner, admin), registers 5 OAuth2 clients with appropriate flows, and implements token validation at both the Traefik gateway and individual service layers. Deliverables include realm configuration, a shared auth library (`packages/auth-client`), frontend auth adapters for all 4 apps, and authentication integration tests for the CI/CD pipeline.

## Technical Context

**Language/Version**: Rust (stable), TypeScript/Node.js 20, Keycloak 25.0

**Primary Dependencies**: Keycloak (identity provider), Traefik v3.0 (gateway), Docker Compose v2 (runtime), `jsonwebtoken` crate (Rust JWT validation), OAuth2 client library for frontends

**Storage**: PostgreSQL 16 (via PostGIS) — user identities stored in `users` schema per EPIC 2 data model; Keycloak's internal H2/PostgreSQL database for realm config

**Testing**: `cargo test` (Rust middleware), keycloak-testcontainers or mock server for integration tests, Postman/Newman or `curl`-based auth flow validation scripts

**Target Platform**: Docker Compose (local), Docker Compose (production) — same EPIC 2 runtime

**Project Type**: Infrastructure + shared library — Keycloak realm configuration (infra-as-code), shared auth middleware library (Rust crate), frontend auth adapters (TypeScript package)

**Performance Goals**: Token validation < 10ms p99 per request; login flow completes within 10 seconds (SC-001); token refresh completes within 2 seconds

**Constraints**: Keycloak is the sole identity provider (constitution §System Architecture); RBAC only — no ACL-per-resource (spec FR-005); stateless APIs — tokens only, no server-side sessions (spec §4.5); dual validation at gateway + service layer (clarification Q4); GDPR compliance for auth personal data (clarification Q3)

**Scale/Scope**: 4 frontend apps (driver-web, driver-mobile, admin-dashboard, partner-dashboard), 4 backend services, 3 roles, 5 OAuth2 clients, 1 shared auth library

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Rationale |
|-----------|--------|-----------|
| I. Pragmatic Architecture | ✅ PASS | Reuses existing Keycloak (already in EPIC 2 runtime) — no new identity service; shared auth library prevents per-service auth reimplementation |
| II. Clear Ownership Boundaries | ✅ PASS | Keycloak owns identity; services own authorization via middleware; single source of truth for roles and credentials |
| III. Operational Simplicity | ✅ PASS | No new runtime services — adds only configuration and a shared library; leverages existing Traefik gateway for token screening |
| IV. Evolution over Complexity | ✅ PASS | RBAC is the simplest authorization model; ACL/resource-level permissions explicitly deferred |
| V. Data Separation in PostgreSQL | ✅ PASS | User identities stored in `users` schema per constitution; Keycloak manages its own identity store; no cross-schema access |
| VI. API Standards | ✅ PASS | All protected paths under `/api/v1/*` per existing API versioning contract |
| VII. Security | ✅ PASS | JWT validation at both gateway and service level provides defense in depth; aligns with existing security constraints |
| VIII. CI/CD | ✅ PASS | Extends existing EPIC 3 pipeline with auth validation tests — no new infrastructure needed |

**GATE RESULT**: PASS — all constitution principles satisfied. No complexity violations require tracking.

## Project Structure

### Documentation (this feature)

```text
specs/005-identity-auth-platform/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
borne-map/
├── infra/
│   └── keycloak/
│       └── realm-export.json       # Updated realm config with clients, roles, flows
├── crates/
│   └── common-auth/                 # Shared auth middleware (Rust crate)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs               # Public API: validate_token, extract_roles
│       │   ├── validator.rs         # JWT validation (issuer, expiry, signature)
│       │   ├── middleware.rs        # Tower/Actix middleware for request auth
│       │   └── error.rs             # Auth error types
│       └── tests/
│           └── integration.rs
├── packages/
│   └── auth-client/                 # Frontend auth adapter (TypeScript)
│       ├── package.json
│       ├── src/
│       │   ├── index.ts             # Public API: login, logout, refresh, getUser
│       │   ├── keycloak.ts          # Keycloak JS adapter wrapper
│       │   ├── token-storage.ts     # Secure token storage (httpOnly cookies / memory)
│       │   └── types.ts             # Token, User, Role types
│       └── tests/
│           └── auth.test.ts
├── apps/
│   ├── driver-web/                  # Integrates auth-client
│   ├── admin-dashboard/             # Integrates auth-client + admin role guard
│   ├── partner-dashboard/           # Integrates auth-client + partner role guard
│   └── driver-mobile/               # Integrates auth-client + PKCE flow
├── services/                        # Each backend service uses common-auth
│   ├── admin-service/
│   ├── driver-service/
│   ├── clickstream-service/
│   └── gis-sync-worker/
└── .github/
    └── workflows/
        └── pr-validation.yml        # Extended with auth validation tests (EPIC 3)
```

**Structure Decision**: The shared auth logic lives in two locations — `crates/common-auth` for Rust backend middleware (reused across all 4 backend services) and `packages/auth-client` for TypeScript frontend adapters (reused across all 4 frontend apps). Keycloak realm configuration is version-controlled as JSON under `infra/keycloak/`, following the existing pattern from EPIC 2.

## Complexity Tracking

> No constitution violations detected — all principles satisfied without deviation.
