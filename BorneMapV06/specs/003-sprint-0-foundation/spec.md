# Feature Specification: Sprint 0 — Foundation

**Feature Branch**: `003-sprint-0-foundation`

**Created**: 2026-06-05

**Status**: Draft

**Input**: User description: "specify sprint 0"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Build System Compiles Successfully (Priority: P1)

A developer checks out the repository and wants to verify that the entire monorepo builds without errors. This is the critical first step before any feature development can proceed.

**Why this priority**: Without a compilable, runnable system, no development can happen. This is the essential foundation.

**Independent Test**: Can be fully tested by running `cargo build` and `pnpm install` and verifying all workspaces compile successfully.

**Acceptance Scenarios**:

1. **Given** a clean checkout of the repository, **When** developer runs `cargo build` in the workspace root, **Then** all Rust binaries and libraries compile without errors or warnings
2. **Given** a clean checkout, **When** developer runs `pnpm install` in the repository root, **Then** all Node.js workspaces and dependencies resolve successfully
3. **Given** compiled binaries exist, **When** developer runs `services/driver-service` binary, **Then** the service starts and listens on the configured port

---

### User Story 2 - Database Compiles & Initializes (Priority: P1)

A developer needs to initialize a local PostgreSQL database with all required schemas, extensions, and migrations. This must work on a fresh database without manual intervention.

**Why this priority**: The database is foundational. All services depend on it. If migrations don't run cleanly, development cannot proceed.

**Independent Test**: Can be fully tested by: (1) starting PostgreSQL in Docker, (2) running migrations with sqlx-cli, (3) verifying all three schemas exist (`inventory`, `gis`, plus system extensions), (4) confirming migrations are idempotent (can run twice without error)

**Acceptance Scenarios**:

1. **Given** a fresh PostgreSQL instance, **When** `db/migrations/0001_extensions.sql` runs, **Then** PostGIS, uuid-ossp, and pgcrypto extensions are enabled
2. **Given** extensions enabled, **When** `db/migrations/0002_inventory_schema.sql` runs, **Then** the `inventory` schema is created
3. **Given** schemas exist, **When** `db/migrations/0003_gis_schema.sql` runs, **Then** the `gis` schema is created and all base tables exist
4. **Given** all migrations applied, **When** migrations run again, **Then** no errors occur (idempotence verified)

---

### User Story 3 - Docker Compose Stack Runs End to End (Priority: P1)

A developer wants to stand up the entire MVP01 stack locally with a single command: `docker compose up`. PostgreSQL, Driver Service, and pgAdmin (for dev convenience) should all start, connect to each other, and be ready for testing.

**Why this priority**: Developers need a reliable local environment. Docker Compose is the standard dev onboarding path. If the stack doesn't start cleanly, the team blocks.

**Independent Test**: Can be fully tested by: (1) running `docker compose up`, (2) verifying Driver Service /health endpoint returns 200 within 10 seconds, (3) checking PostgreSQL is reachable at the configured network address

**Acceptance Scenarios**:

1. **Given** Docker and Docker Compose are installed, **When** developer runs `docker compose up` from the repo root, **Then** all services start without errors
2. **Given** services running, **When** developer polls `http://localhost:8000/health` (or configured port), **Then** Driver Service responds with HTTP 200 OK within 10 seconds
3. **Given** services running, **When** developer opens pgAdmin at the configured URL, **Then** pgAdmin loads and can connect to PostgreSQL without password prompting
4. **Given** Docker stack stopped, **When** developer runs `docker compose down`, **Then** all containers stop cleanly and no orphaned processes remain

---

### User Story 4 - Shared Crates Compile and Export Core Types (Priority: P2)

A developer needs access to shared domain types (NanoIDs, enums, value objects) and utilities (distance, bounding box) from `ev-core`, `ev-geo`, and `ev-db`. These crates must compile and be importable by Driver Service.

**Why this priority**: These are dependencies for the Driver Service implementation in Sprint 2. They need to exist and work correctly, but the full implementation isn't tested until Sprint 2.

**Independent Test**: Can be fully tested by: (1) running `cargo test` on each crate with the unit tests defined in the code, (2) importing each crate in Driver Service and verifying type availability

**Acceptance Scenarios**:

1. **Given** `crates/ev-core` exists with `ids.rs` and `types.rs`, **When** `cargo test -p ev-core` runs, **Then** all unit tests pass (e.g., NanoID generation with prefixes, enum conversions)
2. **Given** `crates/ev-geo` exists with `point.rs`, `bbox.rs`, `distance.rs`, **When** `cargo test -p ev-geo` runs, **Then** all unit tests pass (e.g., haversine distance, bbox containment)
3. **Given** `crates/ev-db` exists with `pool.rs` and `pagination.rs`, **When** `cargo test -p ev-db` runs, **Then** unit tests pass (pagination offset/limit logic)
4. **Given** all crates compiled, **When** Driver Service imports `ev_core`, `ev_geo`, `ev_db`, **Then** types resolve and code compiles

---

### User Story 5 - Frontend Apps Scaffold with Dependencies Installed (Priority: P2)

A developer needs the React Web app and Expo Mobile app to be scaffolded with all dependencies resolved. They should be able to run the dev server (web) or Expo CLI (mobile) without errors.

**Why this priority**: Frontend development can begin in parallel with Sprint 1 database work. Scaffolding + dependency resolution is the prerequisite; real screens come in Sprint 3.

**Independent Test**: Can be fully tested by: (1) running `pnpm dev` in driver-web and verifying dev server starts, (2) running `expo start` in driver-mobile and verifying Expo CLI starts

**Acceptance Scenarios**:

1. **Given** `apps/driver-web` is scaffolded with React + Vite, **When** developer runs `pnpm dev` in that directory, **Then** the dev server starts and serves on localhost (e.g., http://localhost:5173) without errors
2. **Given** `apps/driver-mobile` is scaffolded with Expo, **When** developer runs `pnpm install` in that directory, **Then** Expo dependencies install successfully
3. **Given** dependencies installed, **When** developer runs `expo start`, **Then** the Expo CLI starts and displays QR code for mobile simulator connection
4. **Given** both apps scaffolded, **When** `pnpm install` runs at repo root, **Then** all workspaces resolve without dependency conflicts

---

### Edge Cases

- What happens if PostgreSQL is already running on the configured port? (Docker Compose should fail gracefully with a clear error message)
- What happens if a migration fails halfway through? (Rollback behavior and recovery instructions documented)
- What if Developer Service fails to connect to PostgreSQL on startup? (Error logged; service exits with clear message)
- What if a Node.js workspace has conflicting dependency versions? (pnpm workspace protocol or resolution documented)
- What if ev-core types are imported but crate is not compiled? (Cargo reports clear missing dependency error)

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The Rust monorepo MUST have a Cargo workspace configured at the root with all services and shared crates listed
- **FR-002**: The `services/driver-service` directory MUST contain a Cargo binary with Clean Architecture module structure (domain/, application/, infrastructure/, interface/)
- **FR-003**: The Node.js monorepo MUST have a pnpm workspace configured at the root with all apps and packages listed
- **FR-004**: Database migrations MUST be provided as SQL files in `db/migrations/` with sequential numbering (0001, 0002, 0003)
- **FR-005**: Migration runner (sqlx-cli or equivalent) MUST be configured and documented for local development
- **FR-006**: `docker-compose.yml` MUST include PostgreSQL + PostGIS, Driver Service, and pgAdmin containers
- **FR-007**: Driver Service MUST have a `/health` endpoint that returns HTTP 200 when the service is running
- **FR-008**: Driver Service MUST attempt to connect to PostgreSQL on startup and log connection status
- **FR-009**: The `crates/ev-core` Rust crate MUST implement NanoID generation with prefix support (STN, CHG, PRT, USR, REV, EVT)
- **FR-010**: The `crates/ev-core` Rust crate MUST define shared enums (ConnectorType, ChargerStatus, etc.)
- **FR-011**: The `crates/ev-geo` Rust crate MUST implement LatLng struct and haversine distance calculation
- **FR-012**: The `crates/ev-geo` Rust crate MUST implement bounding box struct for spatial queries
- **FR-013**: The `crates/ev-db` Rust crate MUST provide SQLx PgPool initialization from environment variables
- **FR-014**: The `crates/ev-db` Rust crate MUST provide pagination structs (offset, limit, cursor patterns)
- **FR-015**: The `apps/driver-web` React app MUST be scaffolded with Vite and all dependencies installed
- **FR-016**: The `apps/driver-mobile` React Native Expo app MUST be scaffolded with all dependencies installed
- **FR-017**: The `packages/ui` package MUST be stubbed (empty, but resolvable as a workspace)
- **FR-018**: The `packages/api-client` package MUST be stubbed (empty, but resolvable as a workspace)
- **FR-019**: Environment variables MUST be documented in `infra/env/.env.example` (database URL, service port, etc.)
- **FR-020**: All Rust code MUST follow the Clean Architecture layer rules defined in the constitution

### Key Entities *(include if feature involves data)*

- **Cargo Workspace**: Root manifest defining all Rust services and shared crates
- **pnpm Workspace**: Root manifest defining all Node.js apps and packages
- **PostgreSQL Database**: Single instance with three schemas: `inventory`, `gis`, plus system extensions
- **Docker Compose Stack**: Multi-container orchestration for local development (PostgreSQL, Driver Service, pgAdmin)
- **Shared Crates**: ev-core (IDs, enums), ev-geo (spatial math), ev-db (DB utilities) — reused by all services

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `cargo build` completes without errors or warnings in under 2 minutes on a typical developer machine
- **SC-002**: `pnpm install` resolves all Node.js dependencies without conflicts in under 3 minutes
- **SC-003**: Database migrations run cleanly on a fresh PostgreSQL instance in under 30 seconds
- **SC-004**: `docker compose up` brings the entire stack online (PostgreSQL, Driver Service, pgAdmin) within 1 minute, with all services reporting healthy status
- **SC-005**: Driver Service `/health` endpoint responds with HTTP 200 OK within 1 second of the container becoming healthy
- **SC-006**: PostgreSQL is reachable from the Driver Service container using the configured connection string
- **SC-007**: All four shared crates (`ev-core`, `ev-geo`, `ev-db`, plus Driver Service itself) compile and import without unresolved type errors
- **SC-008**: Developer Web app dev server starts within 15 seconds and serves pages without 404 errors
- **SC-009**: Expo CLI starts successfully and displays a valid QR code for simulator connection
- **SC-010**: Zero build warnings or errors in the entire monorepo after `cargo build` and `pnpm install`

---

## Assumptions

- **Database**: PostgreSQL 14+ with PostGIS 3.2+ extension available (provided via Docker image)
- **Rust Version**: Rust 1.70+ (standard Toolchain; MSRV enforced via rust-toolchain.toml if needed)
- **Node.js**: Node.js 18+ with pnpm 8+ (specified in .npmrc or package.json engines)
- **Docker**: Docker and Docker Compose are installed and available on developer machines
- **Git Workflow**: Feature branches use sequential numbering (001-, 002-, 003-, etc.) as defined in `init-options.json`
- **Clean Architecture**: All Rust services strictly follow the 4-layer structure (domain, application, infrastructure, interface) per constitution section on Engineering Conventions
- **No Real Database Seeds**: Sprint 0 initializes schema and tables only; dev seeds (stations, partners) are added in Sprint 1
- **Sync CLI Tool**: A migration runner (e.g., `sqlx-cli`) is used; if not available, a shell script wrapper is provided (`db/migrate.sh`)
- **Port Availability**: Standard ports (5432 PostgreSQL, 5050 pgAdmin, 8000 Driver Service) are assumed available; .env.example documents port configuration
- **Idempotent Migrations**: All SQL migrations are written to be idempotent (can run multiple times without failure) using `IF NOT EXISTS` patterns
- **No Auth Required**: Health endpoint and database schema setup do not require Keycloak or JWT validation
- **Workspace Root**: Cargo workspace root is at `/home/dali/WORK/BorneMap` with all services and crates discoverable from Cargo.toml
