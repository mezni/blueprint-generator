# Implementation Plan: Sprint 0 — Foundation

**Branch**: `003-sprint-0-foundation` | **Date**: 2026-06-05 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/003-sprint-0-foundation/spec.md`

## Summary

Establish complete foundational infrastructure for MVP01 public discovery vertical slice. Initialize Rust and Node.js monorepos with Clean Architecture services, set up PostgreSQL with three-schema database model, create Docker Compose development environment, and scaffold all shared crates and frontend apps. This is the prerequisite for all subsequent sprint work.

## Technical Context

**Language/Version**: Rust 1.70+, Node.js 18+

**Primary Dependencies**: 
- Rust: Actix-Web, SQLx, PostGIS, Tokio, Serde
- Node.js: Vite, React, React Native Expo, pnpm, Tailwind CSS

**Storage**: PostgreSQL 14+ with PostGIS 3.2+ extension (3 databases: keycloak_db, platform_db, analytics_db)

**Testing**: `cargo test` (Rust), Jest/Vitest (JavaScript), integration tests against test database

**Target Platform**: Linux server (backend), Web browser (driver-web), iOS/Android (driver-mobile)

**Project Type**: Distributed system: Rust monorepo backend (Actix-Web service) + Node.js monorepo frontend (web app, mobile app, dashboards)

**Performance Goals**: 
- `/health` endpoint: <1s response time
- Build time: <2 minutes (cargo), <3 minutes (pnpm)
- Database migration: <30 seconds on fresh DB
- Docker stack startup: <1 minute

**Constraints**: 
- All services must compile without warnings
- All migrations must be idempotent
- Clean Architecture strictly enforced (4-layer separation)
- Zero direct database access from frontend
- Docker Compose must orchestrate all services locally

**Scale/Scope**: Foundation layer only (monorepo scaffolding, schemas, crates, stubs). No real feature logic, no database seeds beyond schema, no Keycloak integration.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle 1: Simplicity First ✅
- Monorepo structure is minimal — single Rust workspace, single Node.js workspace
- Database schema is straightforward: 3 databases, 2 schemas in platform_db (inventory, gis)
- Docker Compose uses standard images (PostgreSQL, pgAdmin)
- No over-engineered abstractions; straightforward service architecture

### Principle 2: Single Source of Truth ✅
- Identity will be managed by Keycloak (scaffolded in future sprints, out of scope Sprint 0)
- Business data lives in `platform_db` (inventory, gis, users schemas)
- Analytics will live in `analytics_db` (scaffolded in future sprints)
- No direct DB access from frontend (API gateway pattern via Actix-Web)
- Secrets in host environment files (.env example provided)

### Principle 3: Clear Separation of Concerns ✅
- Backend follows Clean Architecture: 4-layer separation (domain, application, infrastructure, interface)
- Single Driver Service in Sprint 0 (public endpoints only, no auth required)
- Shared crates (ev-core, ev-geo, ev-db) encapsulate cross-cutting concerns
- Frontend apps (driver-web, driver-mobile) share ui package; no backend logic in frontend

### Principle 4: Manual Operations Allowed ✅
- Docker Compose is the deployment mechanism for local development
- No CI/CD automation in Sprint 0 (build/test only in future; deployment is manual per constitution)
- Migrations are manual via sqlx-cli or shell script
- No automated scaling or container orchestration beyond Docker Compose

### Principle 5: Identity & Security ✅
- Public endpoints (health, discovery) do NOT require JWT or Keycloak (correct per spec)
- Partner scope enforcement will be added in Sprint 2 (out of scope Sprint 0)
- Only Traefik will expose public ports (documented in infra/compose/)
- No hardcoded secrets; all config via environment variables

### Non-Negotiable Rules ✅
1. ✅ Stations will live in `inventory.station` (schema scaffolded, seeds in Sprint 1)
2. ✅ GIS is projection layer only; source of truth is inventory.station (schema separation enforced)
3. ✅ Analytics in analytics_db (scaffolded for future use)
4. ✅ No Keycloak in Sprint 0 (public endpoints don't require it)
5. ✅ Partner scope will be enforced in Sprint 2+ (out of scope Sprint 0)
6. ✅ Routing/navigation out of scope (correct — MVP01 is discovery only)
7. ✅ Public access without login available (health, discovery endpoints public)
8. ✅ Soft delete required (schema includes deleted_at columns for future use)
9. ✅ GIS async/event-driven (outbox pattern documented for Sprint 2+)

### Engineering Conventions ✅
- ✅ Single monorepo with Rust workspace (services/, crates/)
- ✅ Shared domain packages (ev-core, ev-geo, ev-db)
- ✅ Clean Architecture enforced (domain/, application/, infrastructure/, interface/ per service)
- ✅ Actix-Web as HTTP framework (scaffolded in Sprint 0)
- ✅ 16-char prefixed NanoIDs (ev-core implements ID generation)
- ✅ packages/ui shared component library (scaffolded with bright theme)
- ✅ Frontend apps: driver-web, driver-mobile (scaffolded with dependencies)

**GATE RESULT**: ✅ **PASS** — Sprint 0 fully complies with all constitution principles and non-negotiable rules.

## Project Structure

### Documentation (this feature)

```text
specs/003-sprint-0-foundation/
├── plan.md              # This file (filled by /speckit.plan)
├── research.md          # Phase 0 output (unknown dependencies, best practices)
├── data-model.md        # Phase 1 output (database schema, entity definitions)
├── quickstart.md        # Phase 1 output (how to run Sprint 0 locally)
├── contracts/           # Phase 1 output (API contracts, Docker Compose interface)
├── checklists/
│   └── requirements.md  # Quality validation checklist
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
Rust Workspace:
services/driver-service/
├── Cargo.toml
└── src/
    ├── main.rs              # Actix-Web entrypoint
    ├── config.rs            # Environment configuration
    ├── errors.rs            # Typed error enums
    ├── domain/              # Pure domain logic (no external deps)
    │   ├── station.rs       # Entities: Station, StationSummary, etc.
    │   ├── favorite.rs      # Stub (out of scope)
    │   ├── review.rs        # Stub (out of scope)
    │   └── mod.rs
    ├── application/         # Use cases (depends on domain only)
    │   ├── stations.rs      # get_nearby, get_markers, search, get_detail
    │   └── mod.rs
    ├── infrastructure/      # DB, external systems (implements traits)
    │   ├── db/
    │   │   ├── pool.rs      # SQLx PgPool setup
    │   │   └── stations.rs  # StationRepository impl
    │   └── mod.rs
    ├── interface/           # HTTP handlers, middleware
    │   ├── handlers/
    │   │   ├── stations.rs  # GET /stations/* handlers
    │   │   └── health.rs    # GET /health handler
    │   ├── middleware/
    │   │   └── logging.rs   # Request logging
    │   ├── router.rs        # Route definitions
    │   └── mod.rs
    └── Cargo.toml

crates/ev-core/             # Shared domain types
├── Cargo.toml
├── src/
│   ├── ids.rs             # NanoID generation (STN-, CHG-, PRT-, etc.)
│   ├── types.rs           # Enums: ConnectorType, ChargerStatus
│   └── lib.rs
└── tests/                 # Unit tests for IDs and types

crates/ev-geo/              # Spatial math
├── Cargo.toml
├── src/
│   ├── point.rs           # LatLng struct, PostGIS conversion
│   ├── bbox.rs            # Bounding box operations
│   ├── distance.rs        # Haversine calculation
│   └── lib.rs
└── tests/                 # Unit tests for spatial ops

crates/ev-db/               # Database utilities
├── Cargo.toml
├── src/
│   ├── pool.rs            # SQLx PgPool from env vars
│   ├── pagination.rs      # Offset/limit/cursor pagination
│   └── lib.rs
└── tests/                 # Unit tests for pagination

Cargo.toml                  # Workspace root (defines workspace members)

Node.js Workspace:
apps/driver-web/
├── vite.config.js
├── package.json
├── src/
│   ├── main.jsx
│   ├── App.jsx
│   ├── index.css
│   └── pages/             # Stubs for Home, StationDetail, Search, List
├── tailwind.config.js     # Bright theme config
└── public/

apps/driver-mobile/
├── app.json               # Expo config
├── package.json
├── src/
│   ├── App.js
│   ├── index.js
│   └── screens/           # Stubs for Map, List, Detail, Search
├── tailwind.config.js     # Bright theme config (native compatible)
└── assets/

apps/admin-dashboard/
├── vite.config.js         # Stub (scaffolded for future sprints)
├── package.json
└── tailwind.config.js     # Admin theme config

apps/partner-dashboard/
├── vite.config.js         # Stub (scaffolded for future sprints)
├── package.json
└── tailwind.config.js     # Admin theme config

packages/ui/
├── package.json
├── src/
│   ├── index.ts           # Main export
│   ├── tokens/            # Design tokens (colors, typography, spacing)
│   ├── components/        # Stubs for Button, Input, Badge, etc.
│   └── native/            # React Native compatible exports
└── tailwind.config.js     # Shared Tailwind config

packages/api-client/
├── package.json
├── src/
│   ├── index.ts           # Main export
│   ├── driver/
│   │   └── stations.ts    # Stubs for /nearby, /markers, /search, /:id
│   └── types.ts           # Shared TypeScript interfaces
└── tsconfig.json

pnpm-workspace.yaml         # Node.js workspace root
package.json               # Root package (shared dependencies)

Database:
db/migrations/
├── 0001_extensions.sql    # Enable PostGIS, uuid-ossp, pgcrypto
├── 0002_inventory_schema.sql
├── 0003_gis_schema.sql
└── migrate.sh             # Migration runner script (or sqlx-cli wrapper)

Infrastructure:
infra/
├── compose/
│   └── docker-compose.yml # PostgreSQL, Driver Service, pgAdmin
├── osm/
│   └── import.sh          # OSM import script (Sprint 1)
└── env/
    └── .env.example       # Environment variables template
```

**Structure Decision**: This is a **distributed system** with separate Rust backend and Node.js frontend workspaces:
- **Rust workspace** at repo root with all services (driver-service) and shared crates (ev-core, ev-geo, ev-db)
- **Node.js workspace** at repo root with all frontend apps (driver-web, driver-mobile, dashboards) and shared packages (ui, api-client)
- **Database** migrations in `db/migrations/` with sequential numbering
- **Infrastructure** configuration in `infra/` for Docker, environment templates, and data import scripts
- Clean Architecture enforced strictly in all Rust services

## Complexity Tracking

No constitution violations. All required complexity is justified:

| Component | Why Needed | Simpler Alternative |
|-----------|-----------|-------------------|
| Separate Rust & Node workspaces | Different ecosystems (Cargo vs npm); different language requirements | Single monorepo languages would be worse for both backend and frontend |
| 3 databases (keycloak_db, platform_db, analytics_db) | Constitution mandates separation of identity, business, analytics | Single database violates SoT principle and security boundaries |
| 4-layer Clean Architecture | Constitution mandates strict separation; enables testability, DI, and maintainability | Layered architecture is non-negotiable per constitution |
| Shared crates (ev-core, ev-geo, ev-db) | Cross-service DRY (IDs, spatial math, DB pool); enforced by constitution | Code duplication or monolithic service would increase maintenance burden |

---

## Phase 0: Research & Clarifications

*No [NEEDS CLARIFICATION] markers exist in the specification.*

All technical context is defined by the constitution and existing project documentation:

### Research Findings

#### 1. Rust Monorepo Setup with Cargo Workspace

**Decision**: Use Cargo workspaces with `services/` and `crates/` directories, root `Cargo.toml` as workspace manifest.

**Rationale**: Constitution mandates single monorepo. Cargo workspaces enable shared dependencies and concurrent compilation while maintaining clear service/crate boundaries.

**Alternatives Considered**:
- Separate Git repos per service: Would violate monorepo principle and require complex dependency management
- Single Cargo package: Would violate Clean Architecture separation and create circular dependency risks

**References**: 
- `docs/03-architecture/clean-architecture.md` — Layer structure
- `docs/03-architecture/services.md` — Service definitions

#### 2. Node.js pnpm Workspace for Frontend

**Decision**: Use pnpm workspaces with `apps/` for applications and `packages/` for shared libraries, root `pnpm-workspace.yaml`.

**Rationale**: pnpm is lighter weight than npm/yarn, enforces stricter dependency isolation, and is specified in constitution as Node.js package manager.

**Alternatives Considered**:
- Lerna: Would add complexity for a small monorepo
- npm workspaces: Heavier than pnpm; less strict about dependencies

**References**: Constitution specifies "Single monorepo with Rust backend workspace and shared domain packages"

#### 3. PostgreSQL 14+ with PostGIS 3.2+

**Decision**: PostgreSQL 14+ with PostGIS 3.2+ extension, 3 separate databases for Keycloak, platform business logic, and analytics.

**Rationale**: PostGIS is required for geospatial queries (station locations, bounding boxes). 3-database separation enforces Single Source of Truth per constitution.

**Alternatives Considered**:
- SQLite: Would not support PostGIS or scaling to production volume
- Single database: Would violate constitution's separation of concerns (identity, business, analytics)

**References**: Constitution Rule #2 (Single Source of Truth), Rule #3 (Separation of Concerns)

#### 4. Clean Architecture 4-Layer Enforcement

**Decision**: Domain → Application → Infrastructure → Interface layers, with strict dependency rules enforced via Rust module organization and imports.

**Rationale**: Constitution mandates this structure. Enables testability (mock infrastructure), clear dependencies, and DI via traits.

**Alternatives Considered**:
- MVC: Would not separate concerns cleanly; infrastructure logic bleeds into handlers
- 3-layer: Missing application layer leads to use cases scattered across handlers

**References**: `docs/03-architecture/clean-architecture.md`

#### 5. Actix-Web as HTTP Framework

**Decision**: Actix-Web for Rust services.

**Rationale**: Constitution specifies "Rust services use Actix-Web as the HTTP framework." High performance, excellent async support, and integrates well with trait-based DI.

**Alternatives Considered**:
- Axum: Newer, smaller, but constitution chose Actix-Web explicitly

**References**: Constitution Engineering Conventions section

#### 6. React + Vite for Driver Web App

**Decision**: React with Vite build tool, Tailwind CSS for styling.

**Rationale**: Vite offers fast HMR and builds. React ecosystem is mature. Tailwind CSS aligns with bright theme design system.

**Alternatives Considered**:
- Next.js: Adds complexity (SSR, routing) not needed for MVP01
- Vue: Constitutional choice was React

**References**: `docs/06-frontend/applications.md`

#### 7. React Native Expo for Driver Mobile App

**Decision**: React Native with Expo managed framework.

**Rationale**: Code sharing with driver-web (some business logic, types). Expo handles OTA updates and simulator management. Reduces CI/CD complexity.

**Alternatives Considered**:
- Flutter: Would require separate codebase; no code sharing with web app
- Native iOS/Android: Would require separate teams per platform

**References**: Constitution specifies "Driver Mobile App (React Native Expo)"

#### 8. Docker Compose for Local Development

**Decision**: Docker Compose with PostgreSQL, Driver Service, and pgAdmin containers.

**Rationale**: Standard local development environment. Easy onboarding. Mirrors production structure (separate containers per service).

**Alternatives Considered**:
- Docker Swarm: Unnecessary complexity for local dev
- Kubernetes: Out of scope; manual deployment per constitution

**References**: Constitution Rule #4 (Manual Operations Allowed)

#### 9. SQLx for Database Access (No ORM)

**Decision**: SQLx with compile-time checked queries (`sqlx::query_as!` macro).

**Rationale**: Type-safe queries without runtime ORM overhead. Compiles to exactly the SQL you write. Clear repository pattern implementation.

**Alternatives Considered**:
- Diesel: Heavier ORM with more magic; not needed for simple queries
- sqlc: Requires Go; SQLx integrates better with Rust

**References**: Spec requires "raw SQL queries via `sqlx::query_as!`"

#### 10. 16-Character Prefixed NanoIDs

**Decision**: Implement `NanoID` in `ev-core` with 16-character length and prefix support (STN, CHG, PRT, USR, REV, EVT).

**Rationale**: Balances uniqueness (2^96 combinations per prefix), readability, and database storage vs. standard UUIDs. Constitution specifies 16-char prefixed IDs.

**Alternatives Considered**:
- UUIDs (36 chars): Wastes space; less readable
- 12-char IDs: Higher collision risk at scale
- Sequential IDs: No guarantee of global uniqueness in distributed systems

**References**: Constitution specifies "All identifiers use 16-character prefixed NanoIDs"

---

## Phase 1: Design & Contracts

### Data Model

**File**: `data-model.md` (to be generated)

Entities defined by feature spec and drawn from MVP01 documentation:

#### 1. Partner (inventory.partner)
- `id` TEXT PRIMARY KEY (PRT-xxxxxxxxxxxxxxxx)
- `name` TEXT NOT NULL
- `created_at` TIMESTAMPTZ
- `updated_at` TIMESTAMPTZ
- `deleted_at` TIMESTAMPTZ (soft delete)

#### 2. Station (inventory.station)
- `id` TEXT PRIMARY KEY (STN-xxxxxxxxxxxxxxxx)
- `partner_id` TEXT REFERENCES inventory.partner(id)
- `name` TEXT NOT NULL
- `address` TEXT
- `latitude` NUMERIC(10,7) NOT NULL
- `longitude` NUMERIC(10,7) NOT NULL
- `created_at` TIMESTAMPTZ
- `updated_at` TIMESTAMPTZ
- `deleted_at` TIMESTAMPTZ (soft delete)

#### 3. Charger (inventory.charger)
- `id` TEXT PRIMARY KEY (CHG-xxxxxxxxxxxxxxxx)
- `station_id` TEXT REFERENCES inventory.station(id)
- `connector_type` TEXT NOT NULL (enum-like: CCS2, Type2, TeslaSupercharger)
- `power_kw` NUMERIC(6,2)
- `status` TEXT NOT NULL DEFAULT 'available' (available, in_use, maintenance)
- `created_at` TIMESTAMPTZ
- `updated_at` TIMESTAMPTZ
- `deleted_at` TIMESTAMPTZ (soft delete)

#### 4. Station Location (gis.station_locations)
- `station_id` TEXT PRIMARY KEY REFERENCES inventory.station(id)
- `geom` GEOMETRY(Point, 4326)
- `snapped_road_id` BIGINT
- `region_id` BIGINT
- `updated_at` TIMESTAMPTZ

### Interface Contracts

**File**: `contracts/docker-compose.md`

Defines Docker Compose services contract:
- PostgreSQL service (port 5432, POSTGRES_PASSWORD, POSTGRES_USER, POSTGRES_DB environment vars)
- Driver Service (port 8000, DATABASE_URL env var, depends_on PostgreSQL)
- pgAdmin service (port 5050 for web UI, read-only dev tool)

**File**: `contracts/api.md`

Public API endpoints (no auth required in Sprint 0):
- `GET /health` → JSON `{ "status": "ok" }`
- (Actual station endpoints implemented in Sprint 2; stubs in Sprint 0)

### Quickstart

**File**: `quickstart.md` (to be generated)

Steps for developer to run Sprint 0:
1. Clone repo and check out branch `003-sprint-0-foundation`
2. Copy `.env.example` to `.env` (uses defaults)
3. Run `cargo build` (compiles all Rust workspaces)
4. Run `pnpm install` (resolves all Node.js dependencies)
5. Run `docker compose up` (starts PostgreSQL, Driver Service, pgAdmin)
6. Verify `curl http://localhost:8000/health` returns 200
7. Verify `cargo test -p ev-core && cargo test -p ev-geo && cargo test -p ev-db`
8. Verify `pnpm dev` in `apps/driver-web` starts dev server
9. Verify `expo start` in `apps/driver-mobile` starts Expo CLI

---

## Delivery Artifacts

### Phase 0 Output
- [x] research.md — Consolidated research findings (above)

### Phase 1 Output (To Be Generated)
- [ ] data-model.md — Entity schemas and relationships
- [ ] contracts/docker-compose.md — Service interface definitions
- [ ] contracts/api.md — Public API endpoint contracts
- [ ] quickstart.md — Developer onboarding guide

### Phase 2 Output (Generated by /speckit.tasks)
- [ ] tasks.md — Task breakdown for implementation

---

## Next Steps

1. **Phase 1 Finalization**: Generate data-model.md, contracts/, and quickstart.md
2. **Agent Context Update**: Update AGENTS.md `<!-- SPECKIT START -->` section to reference this plan
3. **Phase 2 Task Breakdown**: Run `/speckit.tasks` to generate implementation task list
4. **Implementation**: Execute tasks in order (start with monorepo scaffolding, then database, then services)
