# Implementation Plan: Database & Schema Foundation

**Branch**: `004-ci-cd-pipeline` | **Date**: 2026-05-31 | **Spec**: `specs/006-database-schema-foundation/spec.md`

**Input**: Feature specification from `specs/006-database-schema-foundation/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Establish four isolated PostgreSQL schemas (`inventory`, `users`, `gis`, `analytics`) with strict ownership boundaries, a sqlx-based versioned migration system, Keycloak identity integration (1:1 keycloak_id → user_account mapping), deterministic NanoID-based identifiers with entity prefixes, and PostGIS GEOGRAPHY(Point) spatial support (SRID 4326).

## Technical Context

**Language/Version**: Rust 1.79+ (backend services: Admin, Driver, GIS Sync, Clickstream)

**Primary Dependencies**: sqlx 0.8 (migrations), postgres-extension crate set, PostGIS 3.4, Keycloak 25.0

**Storage**: PostgreSQL 16 with PostGIS 3.4 — four schemas, partitioned by ownership

**Testing**: cargo test + DB integration tests with sqlx test fixtures

**Target Platform**: Linux (Docker Compose deployment)

**Project Type**: Web service (Admin Service), with supporting services (GIS Sync Worker)

**Performance Goals**: Station proximity search <300ms for 10K stations; analytics ingestion ~100K events/day

**Constraints**: 
- Cross-schema writes FORBIDDEN (enforced at DB connection credential level)
- `inventory.station.location` is authoritative geometry; GIS schema is derived
- All spatial data: SRID 4326, GEOGRAPHY(Point)
- IDs: deterministic prefixes (`USR-`, `PRT-`, `STN-`, `CHG-`, `REV-`)
- Keycloak is sole role source — backend MUST NOT override
- No automatic orphaned user cleanup (admin-triggered only)

**Scale/Scope**: ~10K stations, ~100K analytics events/day, ~4 schemas with ~15 initial tables

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Assessment | Status |
|-----------|-----------|--------|
| I. Pragmatic Architecture | 4 schemas + migration system are minimum viable for domain boundaries; no new services created | ✅ PASS |
| II. Clear Ownership Boundaries | Strict schema ownership with credential-scoped access enforces boundary at DB level | ✅ PASS |
| III. Operational Simplicity | sqlx migrations integrated into existing Admin Service startup; no new infra | ✅ PASS |
| IV. Evolution over Complexity | 4 schemas are explicitly constitutional (V. Data Separation); migration system enables future evolution | ✅ PASS |
| V. Data Separation | Uses exactly the 4 approved schemas; adds no new schemas | ✅ PASS |

### Post-Design Re-Evaluation

| Principle | Finding | Status |
|-----------|---------|--------|
| I. Pragmatic Architecture | Design allocates one crate (`common-db`) for pool + migration logic, reuses existing `contracts` crate for ID generation. No service fragmentation. | ✅ PASS |
| II. Clear Ownership Boundaries | Ownership contract defines exact DB roles per schema; credentials scoped per service via env vars. Cross-schema writes impossible at DB level. | ✅ PASS |
| III. Operational Simplicity | sqlx integrates into existing Admin Service `main.rs`; no new infrastructure. Quickstart provides exact CLI commands. | ✅ PASS |
| IV. Evolution over Complexity | Daily partitions are future-proofed for 100K events/day. Migration system enables incremental schema evolution. | ✅ PASS |
| V. Data Separation | Exactly 4 constitutional schemas mapped. No new schemas introduced. | ✅ PASS |

### ID Format Correction

**Issue**: Spec FR-013 used underscores (`USR_`) but constitution §Identifier Standard uses hyphens (`USR-`). Per governance (constitution supersedes), corrected all artifacts to hyphens: spec FR-013, data-model.md, all contracts, research.md.

**Result**: All gates pass. No violations requiring complexity justification.

## Project Structure

### Documentation (this feature)

```text
specs/006-database-schema-foundation/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
services/admin-service/
├── Cargo.toml
├── src/
│   ├── main.rs                  # Startup: run sqlx migrations before accepting connections
│   └── ...
├── migrations/
│   ├── 20260531000001_create_schemas.sql
│   ├── 20260531000002_create_users_schema.sql
│   ├── 20260531000003_create_inventory_schema.sql
│   ├── 20260531000004_create_gis_schema.sql
│   ├── 20260531000005_create_analytics_schema.sql
│   └── 20260531000006_seed_roles.sql
└── tests/
    ├── integration/
    │   ├── schema_permissions_test.rs
    │   ├── migration_test.rs
    │   └── identity_mapping_test.rs
    └── queries/

infra/compose/
├── docker-compose.yml            # Database, Keycloak, Traefik, services
├── docker-compose.dev.yml        # Dev overrides
├── db/
│   ├── init/
│   │   └── 01-postgis-init.sql   # CREATE EXTENSION postgis
│   └── Dockerfile
└── .env

scripts/
└── db/
    ├── migrate.sh                # Wrapper for sqlx migrate run
    └── rollback.sh               # Wrapper for sqlx migrate revert
```

**Structure Decision**: Migrations live in `services/admin-service/migrations/` as sqlx requires. The Admin Service runs them at startup. DB init scripts (extension setup) live in `infra/compose/db/init/` for Docker-first provisioning. Test DB setup mirrors migrations with sqlx test fixtures.

## Complexity Tracking

No violations — all gates passed. The four-schema model is explicitly constitutional.
