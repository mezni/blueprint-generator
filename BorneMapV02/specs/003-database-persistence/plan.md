# Implementation Plan: Database Persistence & Spatial Query Engine

**Branch**: `003-database-persistence` | **Date**: 2026-05-28 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/003-database-persistence/spec.md`

## Summary

Replace the in-memory mock data store with a persistent PostgreSQL/PostGIS database for EV charging station data. The backend serves stations from the database via spatial queries (`ST_DWithin`), respects `is_live` visibility flags, exposes health and status-update endpoints, and is backed by Docker Compose for local development. CI runs against a real PostGIS service container.

## Technical Context

**Language/Version**: Rust 2021 edition (Actix-web 4.4)

**Primary Dependencies**:
- Backend: actix-web 4.4, sqlx 0.8 (Postgres driver with compile-time checked queries), serde 1.0, chrono 0.4, env_logger 0.11, actix-cors 0.7
- Frontend: react-native-maps 1.20, leaflet 1.9 + react-leaflet 5.0, axios ^1.6
- Infrastructure: postgis/postgis:15-3.3 Docker image

**Storage**: PostgreSQL 15 with PostGIS 3.3 extension; spatial queries via `GEOGRAPHY(Point, 4326)` type and `ST_DWithin` for radius searches

**Testing**: `cargo test` for backend unit + integration tests against live PostGIS container; `npx expo export --platform web` for frontend build verification; API contract via manual curl verification

**Target Platform**: Linux server (backend); iOS/Android/Web via Expo Go (frontend)

**Project Type**: web-service + mobile-app

**Performance Goals**: API responds in under 500ms for 50 stations within 15 km search radius; health endpoint returns in under 100ms

**Constraints**: Docker-based PostGIS required for local development; open API with no auth (network-level security); rate limiting deferred to future iteration; expected scale <2,000 stations in first year

**Scale/Scope**: 5 partners, 50 seeded stations, <500 at launch, <2,000 within year one

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Validation Before Optimization | ✅ PASS | Persistent DB is core requirement, not premature optimization; no Redis/RabbitMQ |
| II. Technical Stack Governance | ✅ PASS | Rust/Actix-web + PostgreSQL/PostGIS + Expo React Native — all locked-stack compliant |
| III. API & Service Architecture | ✅ PASS | `api-service` gateway with `/api/v1` prefix; no cross-service coupling |
| IV. Data Architecture Standards | ✅ PASS | `prt-`/`stn-`/`chg-` nanouuid patterns; PostGIS SRID 4326; Tunis center anchoring |
| V. Development & Environment Discipline | ✅ PASS | Docker Compose for DB; Expo for mobile; specs under `/specs/` |
| Additional Constraints | ✅ PASS | No Redis or RabbitMQ; containerized DB only |
| Development Workflow & Quality Gates | ✅ PASS | CI validates both backend (tests) and frontend (web export); API versioned under `/api/v1` |

**Gate Result**: ALL PASS — No violations requiring complexity justification.

## Project Structure

### Documentation (this feature)

```text
specs/003-database-persistence/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (separate command)
```

### Source Code (repository root)

```text
borne-map/
├── .github/
│   └── workflows/
│       └── ci.yml               # Updated with PostGIS service container
├── apps/
│   └── mobile-driver/           # Frontend (unchanged structure)
│       ├── App.js
│       ├── package.json
│       └── src/
│           ├── components/
│           │   ├── MapView.web.js
│           │   ├── MapView.native.js
│           │   └── StationCard.js
│           ├── screens/
│           │   └── MapScreen.js
│           └── services/
│               └── api.js
├── backend/
│   ├── Cargo.toml               # Workspace manifest
│   ├── api-service/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs          # Updated: PgPool state, health endpoint, CORS
│   │       └── domains/
│   │           └── locate/
│   │               ├── mod.rs
│   │               ├── model.rs # Updated: PartnerSnapshot, is_live
│   │               └── routes.rs# Updated: SQLx spatial queries, show_staged param
│   ├── core/                    # Shared library (unchanged)
│   ├── db/                      # NEW: Database assets
│   │   ├── migrations/
│   │   │   └── 20260528000000_init_spatial_schema.sql
│   │   ├── seeds/
│   │   │   └── demo_data.sql    # 5 partners, 50 stations
│   │   └── osm/
│   │       ├── import_tunisia.sh
│   │       └── ev_filter.lua
│   └── infra/                   # NEW: Database connection utility
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs           # connect_pool helper
└── deployments/
    └── docker-compose.yml      # PostGIS container definition
```

**Structure Decision**: Existing Integrated Domain Architecture Layout preserved. New `db/` directory added under `backend/` for migration, seed, and OSM assets. New `infra/` crate added to the workspace for database connection pooling.

## Complexity Tracking

No constitutional violations — complexity justification not required.

## Phase 0: Research

See [research.md](./research.md) for detailed technology decisions and alternatives considered.

Key decisions:
1. **sqlx** for async PostgreSQL with compile-time query verification (vs. diesel for simpler setup with spatial queries)
2. **postgis/postgis:15-3.3** Docker image (matching constitution's PostGIS requirement)
3. **Raw SQL migrations** via sqlx `query!` macro (no ORM; direct spatial SQL control)
4. **ST_DWithin** on `GEOGRAPHY` for radius queries (automatic meter-based distance at SRID 4326)

## Phase 1: Design

### Data Model

See [data-model.md](./data-model.md) for full entity definitions with fields, types, constraints, and relationships.

### API Contracts

See [contracts/](./contracts/) for the `GET /api/v1/stations/nearby` contract specification.

### Quickstart

See [quickstart.md](./quickstart.md) for end-to-end setup instructions.
