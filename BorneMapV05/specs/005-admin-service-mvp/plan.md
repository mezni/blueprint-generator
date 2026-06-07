# Implementation Plan: Admin Service MVP

**Branch**: `005-admin-service-mvp` | **Date**: 2026-06-02 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/005-admin-service-mvp/spec.md`

## Summary

Implement the admin-service as a fully functional REST API serving `/api/v1/admin/*` and `/api/v1/partner/*` endpoints. The service provides partner-scoped station/charger/availability CRUD, admin global CRUD for partners/stations/reviews, GIS outbox event insertion on station mutations, idempotent station creation, optimistic concurrency control, and full partner isolation enforced at the repository layer. The existing skeleton (health endpoint + auth middleware) is extended with a database layer (sqlx::PgPool), modular route handlers, and new shared crate capabilities.

## Technical Context

**Language/Version**: Rust 1.87 (edition 2024, per workspace Cargo.toml)

**Primary Dependencies**: axum 0.7 (HTTP), sqlx 0.8 (PostgreSQL/PostGIS), serde 1 (serialization), tokio 1 (async runtime), ulid 1 (ULID generation), common-auth/common-db/common-errors/common-types (workspace crates)

**Storage**: PostgreSQL `platform_db` (PostGIS-enabled) — schemas: `inventory`, `users`, `gis`

**Testing**: cargo test (unit + integration), sqlx test fixtures, testcontainers (PostgreSQL + Keycloak for CI)

**Target Platform**: Linux server (Docker Compose, x86_64)

**Project Type**: web-service (REST API binary)

**Performance Goals**: write endpoints ≤500ms p95, list endpoints ≤200ms p95

**Constraints**: soft delete only, partner isolation at repository layer, ULID+prefix IDs, standard response envelope, no cross-schema writes without service boundary, all mutations auditable

**Scale/Scope**: <100 events/sec, moderate concurrency, single-region, ~20 REST endpoints

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Data-First Source of Truth | PASS | All reads/writes target `platform_db` as authoritative. GIS outbox is projection trigger. |
| II. Strict Domain & Service Separation | PASS | admin-service owns `inventory` + `users` + `gis.sync_queue` writes within `platform_db`. No cross-service coupling. |
| III. Ownership-Enforced Authorization | PASS | `partner_id` derived from `users.partner_membership` only. Partner scoping enforced at repository layer. Admin has global scope. |
| IV. Contract-Driven REST APIs | PASS | All endpoints follow `/api/v1/{domain}/{resource}`, standard envelope, pagination on lists. |
| V. Event-Driven & Derived State | PASS | Station mutations insert `gis.sync_queue` outbox row synchronously in same transaction. Analytics events deferred per spec. |
| VI. Soft Delete & Auditability | PASS | Soft delete on station/partner. Audit fields populated. ULID+prefix IDs. |
| VII. Verification Discipline | PASS | Plan includes unit, integration, and contract tests for auth, isolation, correctness, soft-delete. |

**Post-Phase 1 re-check**: All gates still pass. No violations.

## Project Structure

### Documentation (this feature)

```text
specs/005-admin-service-mvp/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── partner-api.yaml
│   └── admin-api.yaml
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
services/admin-service/
├── Cargo.toml
├── Dockerfile
├── migrations/
│   ├── 0000_enable_postgis.up.sql          # (existing)
│   ├── ...                                  # (existing 0000-0017)
│   ├── 0017_smoke_test.sql                 # (existing)
│   ├── 0018_create_inventory_idempotency_key.up.sql
│   └── 0018_create_inventory_idempotency_key.down.sql
└── src/
    ├── main.rs                              # App bootstrap, DB pool, router assembly
    ├── config.rs                            # Env-var config struct
    ├── db.rs                                # PgPool factory + migration runner
    ├── error.rs                             # Service error type → IntoResponse
    ├── extractors.rs                        # Custom axum extractors (pagination, idempotency-key, If-Match)
    ├── routes/
    │   ├── mod.rs                           # Router assembly
    │   ├── partner.rs                       # /api/v1/partner/* handlers
    │   └── admin.rs                         # /api/v1/admin/* handlers
    ├── models/
    │   ├── mod.rs
    │   ├── partner.rs                       # Partner DB row + DTO types
    │   ├── station.rs                       # Station DB row + DTO types
    │   ├── charger.rs                       # Charger DB row + DTO types
    │   ├── availability.rs                  # StationAvailability DB row + DTO types
    │   ├── review.rs                        # Review DB row + DTO types
    │   ├── user.rs                          # UserAccount + PartnerMembership types
    │   └── outbox.rs                        # GisSyncQueue insert helpers
    └── repository/
        ├── mod.rs
        ├── partner_repo.rs                  # Partner queries (scoped + global)
        ├── station_repo.rs                  # Station queries (partner-scoped + admin global)
        ├── charger_repo.rs                  # Charger queries (partner-scoped)
        ├── availability_repo.rs             # Availability queries
        ├── review_repo.rs                   # Review queries (admin global)
        ├── user_repo.rs                     # User queries (admin global)
        ├── outbox_repo.rs                   # gis.sync_queue insertion
        └── idempotency_repo.rs              # Idempotency key lookup + insert

crates/
├── common-db/
│   ├── Cargo.toml                           # +sqlx dependency
│   └── src/
│       └── lib.rs                           # PgPool factory, migration runner
├── common-errors/
│   ├── Cargo.toml                           # +axum dependency
│   └── src/
│       └── lib.rs                           # +ConcurrentModification error code, IntoResponse impl
├── common-types/
│   ├── Cargo.toml                           # +ulid dependency
│   └── src/
│       ├── lib.rs                           # +generate_id(EntityPrefix) function
│       └── api.rs                           # +IntoResponse impls, item envelope type
└── common-auth/
    └── src/
        ├── provisioning.rs                  # Updated: real DB lookup instead of stub
        └── guards.rs                        # Updated: populate partner_id from membership
```

**Structure Decision**: The service follows a layered architecture inside `services/admin-service/src/` — routes (handlers) → repository (data access) → models (types). Shared crates are updated incrementally. This matches the monorepo layout from `docs/EXECUTION_PLAN.md` §3.1 and the existing pattern from Sprints 1-3.

## Complexity Tracking

> No Constitution violations. All principles satisfied as-is.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (none) | — | — |
