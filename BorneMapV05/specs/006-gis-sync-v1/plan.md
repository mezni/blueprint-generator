# Implementation Plan: GIS Sync System v1

**Branch**: `006-gis-sync-v1` | **Date**: 2026-06-02 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/006-gis-sync-v1/spec.md`

## Summary

Implement the `gis-worker` as a background worker that polls `gis.sync_queue` for pending outbox rows, converts station lat/lng to PostGIS geometry (SRID 4326), and transitions rows through `pending → processing → done|failed → dead_letter`. The worker is fully idempotent and replay-safe with exponential backoff retry. A one-time OSM Tunisia import script downloads and loads road/administrative boundary data from Geofabrik.

## Technical Context

**Language/Version**: Rust 1.87 (edition 2024, per workspace Cargo.toml)

**Primary Dependencies**: sqlx 0.8 (PostgreSQL/PostGIS), tokio 1 (async runtime), serde 1 (serialization), tracing 0.1 (structured logging), common-types/common-db/common-errors (workspace crates). No HTTP framework needed beyond the existing `/health` endpoint (axum already in Cargo.toml).

**Storage**: PostgreSQL `platform_db` (PostGIS-enabled) — schemas: `gis` (sync_queue), `inventory` (station.geom)

**Testing**: cargo test (unit + integration), sqlx test fixtures with real PostgreSQL/PostGIS

**Target Platform**: Linux server (Docker Compose, x86_64)

**Project Type**: background worker (internal service, no public HTTP port)

**Performance Goals**: process a full batch (50 rows) within `GIS_WORKER_POLL_INTERVAL_MS` (5000ms); individual row processing < 100ms for valid geometry

**Constraints**: idempotent processing (replaying same row produces identical state), FIFO queue ordering with parallel batch execution, retry/backoff with dead-letter after max retries, stale processing row recovery on startup

**Scale/Scope**: <100 events/sec, single worker instance, ~50 outbox rows per batch max, single OSM import for Tunisia (~100MB PBF)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Data-First Source of Truth | PASS | `gis-worker` reads from `gis.sync_queue` (written by admin-service) and updates `inventory.station.geom`. GIS is derived state, never authoritative. |
| II. Strict Domain & Service Separation | PASS | gis-worker only touches `gis.sync_queue` (read + status transitions) and `inventory.station` (geom column). No cross-service coupling. |
| III. Ownership-Enforced Authorization | PASS | Worker is internal-only (no public port). No auth needed. Station geometry updates are by entity_id, bypassing partner scoping (GIS is derived). |
| IV. Contract-Driven REST APIs | N/A | Worker has no external API. Only `/health` for liveness probes (already exists). |
| V. Event-Driven & Derived State | PASS | Outbox pattern: admin-service writes `gis.sync_queue` → gis-worker consumes → updates geometry. At-least-once with idempotent processing. |
| VI. Soft Delete & Auditability | PASS | Handles soft-deleted stations: `delete` operation sets geom=NULL. No hard deletes. |
| VII. Verification Discipline | PASS | Plan includes unit + integration tests for idempotency, state transitions, error handling, and stale recovery. |

**Post-Phase 1 re-check**: All gates still pass. No violations.

## Project Structure

### Documentation (this feature)

```text
specs/006-gis-sync-v1/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
services/gis-worker/
├── Cargo.toml                           # +sqlx, chrono deps
├── Dockerfile                           # (exists, no changes needed)
├── migrations/
│   ├── 0001_create_gis_osm_tables.up.sql
│   └── 0001_create_gis_osm_tables.down.sql
└── src/
    ├── main.rs                           # Bootstrap: DB pool, config, poll loop
    ├── config.rs                         # Env-var config struct
    ├── db.rs                             # PgPool factory + migration runner
    ├── error.rs                          # Worker error type
    ├── models.rs                         # GisQueueEntry + OSM table row types
    ├── worker.rs                         # Main poll loop: fetch batch, process, transition
    ├── geometry.rs                       # Geometry computation (ST_MakePoint wrapper)
    ├── retry.rs                          # Exponential backoff + retry logic
    ├── osm_import.rs                     # One-time OSM Tunisia import CLI
    └── health.rs                         # /health endpoint handler (already exists in main.rs)

infra/env/
├── gis-worker.env.example               # Extend with new env vars

infra/compose/
├── docker-compose.yml                    # gis-worker already defined
```

**Structure Decision**: The worker follows a simple single-threaded async architecture inside `services/gis-worker/src/` — main loop (poll → process → transition), with retry, geometry, and OSM import as separate modules. This matches the pattern used by other services in the monorepo.

## Complexity Tracking

> No Constitution violations. All principles satisfied as-is.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (none) | — | — |
