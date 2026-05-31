# Implementation Plan: Mobile Canvas

**Branch**: `004-mobile-canvas` | **Date**: 2026-05-28 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/004-mobile-canvas/spec.md`

## Summary

Align the project with the Mobile Canvas pattern by: (1) documenting the canonical project directory tree, (2) renaming the `partner_type` ENUM to `partner_classification` in the PostGIS migration, (3) adding CHECK constraints for nanouuid identifier patterns, (4) updating the frontend map and station card UI to handle loading/error states, and (5) documenting data contracts.

## Technical Context

**Language/Version**: Rust 2021 edition (Actix-web 4.4)

**Primary Dependencies**:
- Backend: actix-web 4.4, sqlx 0.8 (Postgres driver), serde 1.0, chrono 0.4
- Frontend: react-native-maps, expo, axios
- Infrastructure: postgis/postgis:15-3.3 Docker image

**Storage**: PostgreSQL 15 with PostGIS 3.3 extension; GEOGRAPHY(Point, 4326) type for spatial queries; GiST index on station geometries

**Testing**: `cargo test` for backend; `npx expo export --platform web` for frontend build verification

**Target Platform**: Linux server (backend); iOS/Android/Web via Expo Go (frontend)

**Project Type**: web-service + mobile-app

**Performance Goals**: Map renders station markers within 3 seconds on standard network connection; CI pipeline completes within 5 minutes

**Constraints**: Docker-based PostGIS required for local development; no auth in scope; API contract remains unchanged

**Scale/Scope**: 5 partners, 50 seeded stations; documentation and schema alignment only — no new API endpoints

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Validation Before Optimization | ✅ PASS | No Redis, RabbitMQ, or caching introduced; pure schema alignment and documentation |
| II. Technical Stack Governance | ✅ PASS | Rust/Actix-web + PostgreSQL/PostGIS + Expo React Native — all locked-stack compliant |
| III. API & Service Architecture | ✅ PASS | `api-service` gateway with `/api/v1` prefix; no cross-service coupling |
| IV. Data Architecture Standards | ✅ PASS | `prt-`/`stn-`/`chg-` nanouuid patterns enforced via CHECK constraints; PostGIS SRID 4326; Tunis center anchoring |
| V. Development & Environment Discipline | ✅ PASS | Docker Compose for DB; Expo for mobile; specs under `/specs/` |
| Additional Constraints | ✅ PASS | No Redis or RabbitMQ; containerized DB only |
| Development Workflow & Quality Gates | ✅ PASS | Documentation sync requirement satisfied by this feature itself; API versioning preserved |

**Gate Result**: ALL PASS — No violations requiring complexity justification.

## Project Structure

### Documentation (this feature)

```text
specs/004-mobile-canvas/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
borne-map/
├── .github/
│   └── workflows/
│       └── ci.yml
├── apps/
│   └── mobile-driver/
│       ├── App.js
│       ├── package.json
│       └── src/
│           ├── components/
│           │   └── StationCard.js
│           ├── screens/
│           │   └── MapScreen.js
│           └── services/
│               └── api.js
├── backend/
│   ├── Cargo.toml
│   ├── api-service/
│   │   └── src/
│   │       ├── main.rs
│   │       └── domains/
│   │           └── locate/
│   │               ├── mod.rs
│   │               ├── model.rs
│   │               └── routes.rs
│   ├── core/
│   ├── db/
│   │   ├── migrations/
│   │   │   └── 20260528000000_init_spatial_schema.sql
│   │   └── seeds/
│   │       └── demo_data.sql
│   └── infra/
├── deployments/
│   └── docker-compose.yml
└── specs/
    └── 004-mobile-canvas/
        └── spec.md
```

**Structure Decision**: Preserved existing Mobile Canvas layout. The `locate/` domain module under `api-service/src/domains/` remains the dedicated feature boundary. New `db/` directory under `backend/` holds migration and seed assets.

## Complexity Tracking

No constitutional violations — complexity justification not required.

## Phase 0: Research

See [research.md](./research.md). No unresolved clarifications — tech stack is fully determined by constitution.

## Phase 1: Design

### Data Model

See [data-model.md](./data-model.md) for entity definitions, fields, relationships, and constraints.

### API Contracts

See [contracts/](./contracts/) for the API contract documentation.

### Quickstart

See [quickstart.md](./quickstart.md) for end-to-end setup instructions.
