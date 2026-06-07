# Implementation Plan: Driver Service MVP

**Branch**: `007-driver-service-mvp` | **Date**: 2026-06-02 | **Spec**: `specs/007-driver-service-mvp/spec.md`

**Input**: Feature specification from `/specs/007-driver-service-mvp/spec.md`

## Summary

Implement the Driver Service MVP — a Rust web service providing station discovery (bbox/radius/detail/search), favorites, reviews, and driver profile APIs under `/api/v1/driver/*`. The service reuses existing `common-auth`, `common-types`, and `common-db` crates, enforces station visibility rules via PostGIS, and authenticates review/favorite/profile endpoints with the `registered_driver` role.

## Technical Context

**Language/Version**: Rust 2024 edition (workspace)

**Primary Dependencies**: axum 0.7 (HTTP framework), sqlx 0.8 (PostgreSQL + PostGIS), tokio 1.x (async runtime), common-auth/0.1 (JWT validation + RBAC middleware), common-types/0.1 (shared enums, API envelopes, ULID generation), common-db/0.1 (pool factory), serde + serde_json (serialization), chrono (timestamps), tower-http CORS, thiserror (error handling)

**Storage**: PostgreSQL `platform_db` — `inventory.station` (with PostGIS GEOGRAPHY(Point,4326) + GIST index), `inventory.charger`, `inventory.station_availability`, `users.favorite_station`, `users.station_review`, `users.user_account`, `users.user_profile`

**Testing**: `cargo test` — 4 unit tests (error type validation); integration tests deferred to test harness with real PgPool

**Target Platform**: Linux (Docker), internal network, no public port exposure

**Project Type**: web-service (REST API binary within monorepo)

**Performance Goals**: Station search ≤ 200ms p95; spatial queries use GIST index (EXPLAIN ANALYZE verified); pagination prevents full-table scans

**Constraints**: Station visibility rule enforced (`is_live=true AND deleted_at IS NULL AND status='active' AND is_public=true`); max radius 50km; Tunisia default center (36.8065, 10.1815); standard JSON envelopes; registered_driver role required for authenticated endpoints

**Scale/Scope**: Moderate concurrency; < 100 events/sec baseline; single-region deployment

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Data-First Source of Truth | ✅ PASS | driver-service is read-heavy; writes to `users.favorite_station`, `users.station_review`, `users.user_profile` are owner-scoped business data |
| II. Strict Domain & Service Separation | ✅ PASS | Standalone crate in `services/driver-service`; no cross-schema coupling beyond existing FK relationships |
| III. Ownership-Enforced Authorization | ✅ PASS | Review/favorite/profile endpoints require `registered_driver` role; station discovery is public; no `partner_id` accepted from client |
| IV. Contract-Driven REST APIs | ✅ PASS | Standard `{success, data, meta}` envelopes; pagination on list endpoints; URL versioning (`/api/v1/driver/*`) |
| V. Event-Driven & Derived State | ✅ PASS | driver-service does not emit outbox events (read-heavy discovery); reviews write directly to `users.station_review` (authoritative store) |
| VI. Soft Delete & Auditability | ✅ PASS | Review soft-delete (`status='deleted'`); station visibility filters `deleted_at IS NULL`; ULID ID strategy (`REV-`) |
| VII. Verification Discipline | ✅ PASS | Error type unit tests; structured JSON logging; `/health` endpoint; auth enforcement testable |

**No violations found.** Complexity Tracking not required.

## Project Structure

### Documentation (this feature)

```text
specs/007-driver-service-mvp/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output (API quickstart)
├── spec.md              # Feature specification
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
services/driver-service/
├── Cargo.toml
├── Dockerfile
└── src/
    ├── main.rs           # App bootstrap, routes, middleware
    ├── config.rs          # Env-driven configuration
    ├── db.rs              # PgPool initialization
    ├── error.rs           # ServiceError enum + IntoResponse
    ├── extractors.rs      # PaginationParams
    ├── models/
    │   ├── mod.rs
    │   ├── station.rs     # StationListItem, StationDetail, query structs
    │   ├── charger.rs     # Charger
    │   ├── review.rs      # Review, ReviewCreate, ReviewUpdate
    │   ├── favorite.rs    # FavoriteStation
    │   └── user.rs        # UserProfile, DriverProfile, ProfileUpdate
    ├── repository/
    │   ├── mod.rs
    │   ├── station_repo.rs # Spatial queries, search, detail
    │   ├── review_repo.rs  # CRUD + ownership checks
    │   ├── favorite_repo.rs # Add/remove/list favorites
    │   └── user_repo.rs    # Profile read/upsert
    └── routes/
        ├── mod.rs
        ├── public.rs       # /health
        ├── discovery.rs    # /api/v1/driver/stations*
        ├── favorites.rs    # /api/v1/driver/favorites*
        ├── reviews.rs      # /api/v1/driver/reviews*
        └── profile.rs      # /api/v1/driver/me
```

**Structure Decision**: Single Rust service binary following the established admin-service pattern. Repository layer for data access, route modules for HTTP handlers, models for serialization.

## Complexity Tracking

> Not required — no Constitution violations.
