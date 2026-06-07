# Implementation Plan: Core Database Schema

**Branch**: `004-core-db-schema` | **Date**: 2026-06-02 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/004-core-db-schema/spec.md`

## Summary

Implement all database migrations for the Bornemap platform: create `inventory`, `users`, and `gis` schemas in `platform_db` (with PostGIS), plus the `analytics` schema in `analytics_db`. Includes all tables, constraints, indexes, GIST spatial indexing, a geom auto-population trigger, a `visible_stations` view, an `ACTIVE_STATIONS_EXIST` partner delete guard trigger, pre-created analytics partitions, seed data, and a spatial query smoke test.

## Technical Context

**Language/Version**: Rust (workspace not yet initialized; migrations as raw SQL files runnable via `sqlx-cli` or `psql`)

**Primary Dependencies**: PostgreSQL 15+, PostGIS 3.x, sqlx-cli (migration runner)

**Storage**: PostgreSQL — `platform_db` (schemas: inventory, users, gis) + `analytics_db` (schema: analytics)

**Testing**: pgTAP or plain SQL assertions for constraint/index verification; smoke test via `psql` scripts

**Target Platform**: Linux server (Docker Compose local development)

**Project Type**: Database migration layer (infrastructure)

**Performance Goals**: bbox spatial queries < 50ms p95 on 1K stations; migrations complete < 30s

**Constraints**: Idempotent migrations; no auto-runtime migration; soft delete only for partner/station/review; ULID+prefix IDs; partner isolation at data layer

**Scale/Scope**: 3 schemas in platform_db, 1 schema in analytics_db, 12 pre-created monthly partitions, ~10 seed entities per type

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Data-First Source of Truth | PASS | Migrations define `platform_db` as authoritative; GIS/analytics are derived |
| II. Strict Domain & Service Separation | PASS | No cross-schema writes; each schema owned by a service boundary |
| III. Ownership-Enforced Authorization | PASS | `partner_membership` enforces 1:1; `partner_id` never client-supplied; delete trigger guards isolation |
| IV. Contract-Driven REST APIs | N/A | No APIs in this sprint |
| V. Event-Driven & Derived State | PASS | `gis.sync_queue` outbox pattern defined; idempotent by design |
| VI. Soft Delete & Auditability | PASS | All mutable tables have audit fields + `deleted_at`; trigger blocks partner delete with active stations |
| VII. Verification Discipline | PASS | Smoke test, constraint validation, spatial index verification in exit criteria |

**No violations.** All gates pass.

## Project Structure

### Documentation (this feature)

```text
specs/004-core-db-schema/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
infra/
├── compose/                          # Docker Compose files (Sprint 2)
└── env/                              # .env templates

services/
├── admin-service/
│   └── migrations/                   # Admin-service owned migrations
│       ├── 0001_create_inventory_schema.up.sql
│       ├── 0001_create_inventory_schema.down.sql
│       ├── 0002_create_inventory_partner.up.sql
│       ├── 0002_create_inventory_partner.down.sql
│       ├── 0003_create_inventory_station.up.sql
│       ├── 0003_create_inventory_station.down.sql
│       ├── 0004_create_inventory_charger.up.sql
│       ├── 0004_create_inventory_charger.down.sql
│       ├── 0005_create_inventory_availability.up.sql
│       ├── 0005_create_inventory_availability.down.sql
│       ├── 0006_create_inventory_visible_stations_view.up.sql
│       ├── 0006_create_inventory_visible_stations_view.down.sql
│       ├── 0007_create_users_schema.up.sql
│       ├── 0007_create_users_schema.down.sql
│       ├── 0008_create_users_user_account.up.sql
│       ├── 0008_create_users_user_account.down.sql
│       ├── 0009_create_users_user_profile.up.sql
│       ├── 0009_create_users_user_profile.down.sql
│       ├── 0010_create_users_partner_membership.up.sql
│       ├── 0010_create_users_partner_membership.down.sql
│       ├── 0011_create_users_favorite_station.up.sql
│       ├── 0011_create_users_favorite_station.down.sql
│       ├── 0012_create_users_station_review.up.sql
│       ├── 0012_create_users_station_review.down.sql
│       ├── 0013_create_gis_schema.up.sql
│       ├── 0013_create_gis_schema.down.sql
│       ├── 0014_create_gis_sync_queue.up.sql
│       ├── 0014_create_gis_sync_queue.down.sql
│       ├── 0015_create_triggers.up.sql
│       ├── 0015_create_triggers.down.sql
│       ├── 0016_seed_data.up.sql
│       ├── 0016_seed_data.down.sql
│       └── 0017_smoke_test.sql        # Non-migration: verification script
└── analytics-writer/
    └── migrations/                   # Analytics-writer owned migrations
        ├── 0001_create_analytics_schema.up.sql
        ├── 0001_create_analytics_schema.down.sql
        ├── 0002_create_raw_event.up.sql
        ├── 0002_create_raw_event.down.sql
        ├── 0003_create_raw_event_partitions.up.sql
        ├── 0003_create_raw_event_partitions.down.sql
        ├── 0004_create_event_dead_letter.up.sql
        └── 0004_create_event_dead_letter.down.sql
```

**Structure Decision**: Migrations live under the service that owns the data domain. `admin-service` owns `inventory`, `users`, and `gis` schemas in `platform_db`. `analytics-writer` owns the `analytics` schema in `analytics_db`. The seed and smoke test are co-located with `admin-service` since it's the primary business data service. Each migration has paired `.up.sql` / `.down.sql` files following `sqlx-cli` conventions. Sequential 4-digit numbering ensures ordering.

## Complexity Tracking

No constitution violations to justify.
