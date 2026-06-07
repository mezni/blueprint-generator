# Implementation Plan: Sprint 1 — OSM Data & Station Discovery

**Branch**: `004-sprint-1-foundation` | **Date**: 2026-06-05 | **Spec**: `/specs/004-sprint-1-foundation/spec.md`

**Input**: Feature specification from `/specs/004-sprint-1-foundation/spec.md`

**Note**: This plan documents the design decisions, architecture, and implementation roadmap for Sprint 1 OSM data integration and station discovery features.

## Summary

**Primary Requirement**: Implement OpenStreetMap data ingestion and geospatial station discovery for the BorneMap charging station platform. Enable public drivers to find nearby stations without authentication via a proximity-based API, while enabling registered drivers to save favorites and partners to manage their stations.

**Technical Approach**: 
- Ingest OSM data for Tunisia using **osm2pgsql** tool into dedicated `gis` schema
- Implement synchronous geospatial queries using PostGIS proximity calculations (Haversine formula via `ev-geo` crate)
- Build asynchronous GIS Sync Worker to project `inventory.station` records to `gis.station_locations` via outbox pattern
- Expose public `/api/v1/stations/nearby` endpoint with IP-based rate limiting (100 req/min)
- Implement authenticated endpoints for favorites and partner station management with Keycloak JWT validation
- Enforce partner scope isolation at API layer (partners see ONLY their own stations)

## Technical Context

**Language/Version**: Rust 1.75+ (backend services), React 18+ (frontend apps)

**Primary Dependencies**: 
- Backend: Actix-Web (HTTP), SQLx (async query builder), PostGIS (spatial queries), Keycloak (auth)
- Frontend: React, Vite, Tanstack Query, Tailwind CSS
- Infrastructure: PostgreSQL 15+ with PostGIS extension, Docker Compose, Traefik

**Storage**: PostgreSQL 15+ with three schemas:
- `inventory`: Station, Charger, Partner, User business data
- `users`: User profiles, Favorites, Reviews
- `gis`: OSM data (ways, nodes, boundaries), Station locations (derived)

**Testing**: 
- Backend: `cargo test` (unit), integration tests with test PostgreSQL containers
- Frontend: Vitest + React Testing Library
- E2E: Playwright (API contracts tested via independent tests in spec)

**Target Platform**: Linux server (bare metal + Docker), browser (Chrome, Firefox, Safari)

**Project Type**: Web service (backend API) + web applications (frontend)

**Performance Goals**:
- Nearby queries: <500ms p95, median <300ms (SC-001, SC-002)
- 100 concurrent nearby searches supported
- 1000 concurrent authenticated users without degradation (SC-005)

**Constraints**:
- OSM import: <10 minutes for Tunisia dataset (10k+ ways, 50k+ nodes) (SC-003)
- GIS Sync Worker: 5-minute SLA for location changes (SC-004)
- Station isolation: Partner A cannot access Partner B's stations in ANY response (SC-006)
- Favorites: Zero data loss guarantee; all user-saved stations persist (SC-007)
- Input validation: 100% rejection of invalid coordinates with clear error messages (SC-008)

**Scale/Scope**: 
- Initial: 50+ charging stations in Tunisia (test data)
- Growth: 100+ concurrent users, 1000+ stations
- Data: Tunisia OSM dataset (~10k roads, 50k+ POIs)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Compliance Assessment

**✅ PASS: Core Principles**
- **Simplicity First**: Architecture uses proven patterns (outbox, async workers, PostGIS). No over-engineering.
- **Single Source of Truth**: Inventory schema is source of truth for stations; GIS is derived projection only. Keycloak handles identity exclusively.
- **Clear Separation of Concerns**: Driver-service (stations/discovery), user-service (auth/profiles), gis-worker (async projections). Each follows Clean Architecture.
- **Manual Operations Allowed**: No automated deployment required. OSM import is manual or scheduled task.

**✅ PASS: Non-Negotiable Rules**
1. ✅ Stations live in `inventory.station`
2. ✅ GIS is derived projection only; failures do NOT block station updates
3. ✅ Keycloak is ONLY identity provider; JWT validation mandatory
4. ✅ Partner scope is enforced at API layer (every query scoped to authenticated user's partner org)
5. ✅ Soft deletes only (`deleted_at` timestamp); public discovery filters `deleted_at IS NULL`
6. ✅ GIS updates async + event-driven; outbox pattern prevents station update blocking
7. ✅ Public `/api/v1/stations/nearby` accessible without authentication

**✅ PASS: Engineering Conventions**
- **Clean Architecture**: Driver-service adheres to four-layer structure (domain/application/infrastructure/interface)
- **Actix-Web**: HTTP framework confirmed
- **Rust monorepo**: Single workspace with shared domain packages (auth, config, errors, IDs, observability)
- **Identifiers**: 16-char prefixed NanoIDs (STN-*, CHG-*, PRT-*, USR-*, REV-*)
- **Testing**: Backend uses `cargo test` with integration tests; frontend uses Vitest

**⚠️ CLARIFICATIONS APPLIED (from Sprint 1 clarification session)**
- Keycloak down → authenticated endpoints fail immediately (simplest, safest)
- Favorite deletion → hard delete (no audit trail needed for favorites)
- Concurrent updates → last-write-wins (inventory always overwrites GIS)
- OSM tool → osm2pgsql (standard, battle-tested)
- Rate limiting → IP-based, 100 req/min per IP on public discovery

**GATE RESULT**: ✅ **PASS** — All constitutional requirements met. No violations. Ready for Phase 0 research.

## Project Structure

### Documentation (this feature)

```text
specs/004-sprint-1-foundation/
├── plan.md              # This file (implementation plan)
├── research.md          # Phase 0 output (research findings, tech selection justifications)
├── data-model.md        # Phase 1 output (entity definitions, relationships, validation)
├── quickstart.md        # Phase 1 output (developer onboarding, setup, first test)
├── contracts/           # Phase 1 output (API contracts)
│   ├── api-v1-nearby.json         # GET /api/v1/stations/nearby contract
│   ├── api-v1-favorites.json      # POST/GET /api/v1/favorites contracts
│   └── api-v1-partner-stations.json  # GET /api/v1/partner/stations contract
└── tasks.md             # Phase 2 output (task breakdown with estimates)
```

### Source Code (Rust Monorepo)

```text
ev-platform/
├── Cargo.workspace.toml
│
├── crates/
│   ├── ev-geo/                    # Spatial geometry utilities (Haversine, distance)
│   │   ├── src/
│   │   ├── tests/
│   │   └── Cargo.toml
│   │
│   ├── ev-domain/                 # Shared domain (entities, value objects)
│   │   ├── src/
│   │   │   ├── station.rs         # Station entity, validation
│   │   │   ├── charger.rs
│   │   │   ├── user.rs
│   │   │   ├── favorite.rs
│   │   │   └── ids.rs             # NanoID generation (STN-*, CHG-*, etc)
│   │   ├── tests/
│   │   └── Cargo.toml
│   │
│   ├── ev-auth/                   # Auth domain (Keycloak integration)
│   │   ├── src/
│   │   │   ├── jwt_validator.rs
│   │   │   └── claims.rs
│   │   └── Cargo.toml
│   │
│   ├── driver-service/            # Driver-facing API (stations, discovery, favorites)
│   │   ├── src/
│   │   │   ├── domain/            # Pure business logic (distance calc, validation)
│   │   │   │   ├── station_discovery.rs
│   │   │   │   ├── favorite.rs
│   │   │   │   └── mod.rs
│   │   │   ├── application/       # Use cases (orchestration)
│   │   │   │   ├── nearby_stations_usecase.rs
│   │   │   │   ├── create_favorite_usecase.rs
│   │   │   │   └── mod.rs
│   │   │   ├── infrastructure/    # DB access (SQLx queries, repositories)
│   │   │   │   ├── db/
│   │   │   │   │   ├── station_repository.rs
│   │   │   │   │   ├── favorite_repository.rs
│   │   │   │   │   └── gis_repository.rs
│   │   │   │   └── mod.rs
│   │   │   ├── interface/         # HTTP handlers (Actix-Web)
│   │   │   │   ├── nearby_handler.rs
│   │   │   │   ├── favorites_handler.rs
│   │   │   │   ├── middleware/
│   │   │   │   │   ├── auth.rs
│   │   │   │   │   └── rate_limiter.rs
│   │   │   │   └── mod.rs
│   │   │   ├── main.rs
│   │   │   └── config.rs
│   │   ├── tests/
│   │   │   ├── integration/
│   │   │   │   ├── nearby_stations_test.rs
│   │   │   │   ├── favorites_test.rs
│   │   │   │   └── gis_test.rs
│   │   │   └── contract/          # Independent tests (API contracts)
│   │   ├── Cargo.toml
│   │   └── migrations/            # SQLx migrations (inventory, users schemas)
│   │
│   ├── partner-service/           # Partner-facing API (station management, availability)
│   │   ├── src/
│   │   │   ├── domain/
│   │   │   ├── application/
│   │   │   ├── infrastructure/    # DB access (partner-scoped queries)
│   │   │   └── interface/         # Actix-Web handlers, partner scope middleware
│   │   ├── tests/
│   │   └── Cargo.toml
│   │
│   └── gis-worker/                # Async worker (GIS sync from outbox)
│       ├── src/
│       │   ├── domain/
│       │   │   └── gis_projection.rs
│       │   ├── application/
│       │   │   └── sync_usecase.rs
│       │   ├── infrastructure/
│       │   │   ├── outbox_reader.rs
│       │   │   └── gis_projector.rs
│       │   └── main.rs
│       ├── tests/
│       └── Cargo.toml
│
├── docs/
│   ├── 03-architecture/
│   │   ├── clean-architecture.md  # Clean Architecture reference
│   │   └── services.md            # Per-service architecture mapping
│   └── 10-delivery/
│       └── mvp01/                 # MVP roadmap with Sprint 1 tasks
│
├── scripts/
│   ├── osm-import.sh              # OSM data import script (osm2pgsql wrapper)
│   └── setup-gis.sql              # GIS schema initialization
│
└── docker-compose.yml             # PostgreSQL + services
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
