# Feature Specification: Monorepo Bootstrap

**Feature Branch**: `002-monorepo-bootstrap`

**Created**: 2026-05-31

**Status**: Draft

**Input**: User description: "read from docs/epic01.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Monorepo Directory Structure (Priority: P1)

As a platform engineer, I want the monorepo directory structure created exactly
as specified so that all teams know where their code lives and boundaries are
clear from day one.

**Why this priority**: Every other task (Rust workspace, frontend apps, shared
packages) depends on the directory structure existing first.

**Independent Test**: A reviewer can verify that the directory tree matches
the spec exactly — every required directory exists and nothing is in the wrong
place.

**Acceptance Scenarios**:

1. **Given** the repository root, **When** listing top-level directories,
   **Then** `apps/`, `services/`, `crates/`, `packages/`, `infra/`,
   `scripts/`, `docs/`, and `.github/` all exist.
2. **Given** the `apps/` directory, **When** listing its contents,
   **Then** it contains `driver-web/`, `partner-dashboard/`,
   `admin-dashboard/`, and `driver-mobile/`.
3. **Given** the `services/` directory, **When** listing its contents,
   **Then** it contains `admin-service/`, `driver-service/`,
   `clickstream-service/`, and `gis-sync-worker/`.
4. **Given** the `crates/` directory, **When** listing its contents,
   **Then** it contains `common-auth/`, `common-config/`, `common-db/`,
   `common-errors/`, `common-types/`, and `contracts/`.
5. **Given** the `packages/` directory, **When** listing its contents,
   **Then** it contains `design-system/`, `api-client/`, `analytics-client/`,
   and `auth-client/`.

---

### User Story 2 — Rust Workspace with Shared Crates (Priority: P1)

As a backend developer, I want the Rust workspace initialized with all service
stubs and shared crates so that I can start implementing business logic against
a compiling foundation.

**Why this priority**: All backend services depend on the shared crates
(auth, types, errors, DB, contracts). The workspace must compile before any
service logic can be built.

**Independent Test**: Running `cargo build --workspace` from the repository
root produces no compilation errors.

**Acceptance Scenarios**:

1. **Given** the Cargo workspace at the repository root, **When** building,
   **Then** all 4 service stubs (`admin-service`, `driver-service`,
   `clickstream-service`, `gis-sync-worker`) compile as workspace members.
2. **Given** the shared crates (`common-auth`, `common-config`, `common-db`,
   `common-errors`, `common-types`, `contracts`), **When** building,
   **Then** each crate compiles independently and as part of the workspace.
3. **Given** the `contracts` crate, **When** inspected,
   **Then** it defines API DTOs, event schemas, RBAC enums, and ID formats
   — no service crate defines duplicate types.

---

### User Story 3 — Frontend Apps and Shared Packages (Priority: P1)

As a frontend developer, I want all web apps scaffolded with Vite and the
shared packages initialized so that I can start building UI against a working
foundation.

**Why this priority**: Frontend development cannot start without a compiling
project structure and the shared UI library.

**Independent Test**: Running `npm run build` in each app and `tsc --noEmit`
for all packages produces no errors.

**Acceptance Scenarios**:

1. **Given** the `apps/` directory, **When** building each web app,
   **Then** `driver-web`, `partner-dashboard`, and `admin-dashboard` all
   compile with Vite without errors.
2. **Given** the shared packages (`design-system`, `api-client`, `auth-client`,
   `analytics-client`), **When** type-checked,
   **Then** `tsc --noEmit` passes for all packages.
3. **Given** the mobile app `driver-mobile`, **When** initialized,
   **Then** `expo doctor` passes and the app compiles without errors.

---

### User Story 4 — Shared Contract System (Priority: P2)

As a platform architect, I want the `contracts` crate and `api-client` package
defining all cross-service DTOs, event schemas, RBAC enums, and ID formats so
that no service or app defines its own duplicate types.

**Why this priority**: The contracts system is central to type safety but the
individual services can be stubbed with placeholder types temporarily.

**Independent Test**: A reviewer can verify that no `struct` or DTO outside
the `contracts` crate duplicates a type defined there.

**Acceptance Scenarios**:

1. **Given** the `contracts` crate, **When** inspected,
   **Then** it defines `StationDTO`, `UserDTO`, `PartnerDTO`, `ReviewDTO`
   with NanoID-prefixed ID fields.
2. **Given** the `contracts` crate, **When** inspected,
   **Then** it defines the `ClickstreamEventEnvelope` struct and an
   `EventType` enum with all 9 v1 event types.
3. **Given** the `contracts` crate, **When** inspected,
   **Then** it defines a `Role` enum with `RegisteredDriver`, `Partner`,
   and `Admin` variants.

---

### User Story 5 — Tooling and Makefile (Priority: P2)

As a platform engineer, I want linting, formatting, and build commands
standardized so that all developers can run the full system check with a
single command.

**Why this priority**: Tooling standardization is essential for CI readiness
but individual tools can be verified incrementally.

**Independent Test**: Running `make lint-all && make build-all && make test-all`
from the repository root succeeds without errors.

**Acceptance Scenarios**:

1. **Given** the Makefile at the repository root, **When** running
   `make format-all`, **Then** `cargo fmt` and `prettier` are invoked on
   all relevant files.
2. **Given** the Makefile, **When** running `make lint-all`,
   **Then** `cargo clippy -- -D warnings` and `eslint` pass on all code.
3. **Given** the Makefile, **When** running `make build-all`,
   **Then** `cargo build --workspace` and `vite build` for all web apps
   succeed sequentially.
4. **Given** the Makefile, **When** running `make test-all`,
   **Then** `cargo test --workspace` executes all Rust tests.

---

### Edge Cases

- What happens if a developer adds a new Rust crate outside the workspace
  members list? The crate will not compile with `cargo build --workspace`
  — the Makefile should warn but not block.
- What happens if a frontend app imports types directly instead of through
  `api-client`? This is a code review gate violation — the Makefile cannot
  enforce this; enforcement is via CI and code review.
- What happens when a new service is added later? The workspace members list
  in `Cargo.toml` must be updated and the service directory created under
  `services/`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Repository MUST have the following top-level directories:
  `apps/`, `services/`, `crates/`, `packages/`, `infra/`, `scripts/`,
  `docs/`, `.github/`.
- **FR-002**: The `apps/` directory MUST contain `driver-web/`,
  `partner-dashboard/`, `admin-dashboard/` (Vite + React), and
  `driver-mobile/` (React Native + Expo).
- **FR-003**: The `services/` directory MUST contain `admin-service/`,
  `driver-service/`, `clickstream-service/`, and `gis-sync-worker/`
  — each a Cargo package with a `main.rs` entrypoint.
- **FR-004**: The `crates/` directory MUST contain `common-auth/`,
  `common-config/`, `common-db/`, `common-errors/`, `common-types/`,
  and `contracts/` — each a Cargo library crate.
- **FR-005**: The `packages/` directory MUST contain `design-system/`,
  `api-client/`, `analytics-client/`, and `auth-client/` — each a
  TypeScript npm package with `package.json`.
- **FR-006**: The Rust workspace `Cargo.toml` at the repository root MUST
  include all `services/*` and `crates/*` packages as workspace members.
- **FR-007**: All Rust service stubs MUST compile with
  `cargo build --workspace` without errors.
- **FR-008**: The `contracts` crate MUST define API DTOs (`StationDTO`,
  `UserDTO`, `PartnerDTO`, `ReviewDTO`) with NanoID-prefixed string IDs.
- **FR-009**: The `contracts` crate MUST define a `ClickstreamEventEnvelope`
  struct and an `EventType` enum with variants for all 9 v1 events.
- **FR-010**: The `contracts` crate MUST define a `Role` enum with
  `RegisteredDriver`, `Partner`, and `Admin` variants.
- **FR-011**: NO service crate MAY define its own DTOs or enums that
  duplicate types defined in the `contracts` crate.
- **FR-012**: All frontend npm packages MUST compile with `tsc --noEmit`
  and `vite build` without errors.
- **FR-013**: The `driver-mobile` Expo app MUST pass `expo doctor` checks.
- **FR-014**: A Makefile MUST exist at the repository root with targets:
  `build-all`, `test-all`, `lint-all`, `format-all`.
- **FR-015**: The `infra/docker/` directory MUST contain a placeholder
  `Dockerfile` for each service (no build logic required — CI ready only).
- **FR-016**: The `infra/compose/` directory MUST contain a placeholder
  `docker-compose.dev.yml` for local development scaffolding.
- **FR-017**: All API clients MUST assume `baseURL = "/api/v1"` — no
  unversioned paths may be hardcoded; enforced via `packages/api-client/src/index.ts`.

### Key Entities *(include if feature involves data)*

- **Cargo Workspace**: A shared Rust build context defined by `Cargo.toml`
  at the repository root. Includes all service and crate packages.
- **Contract Crate (`crates/contracts`)**: The single source of truth for
  cross-service data structures — DTOs, event schemas, enums, ID formats.
- **API Client (`packages/api-client`)**: A typed TypeScript wrapper that
  consumes the Rust contract definitions and exposes them to frontend apps.
- **Design System (`packages/design-system`)**: Reusable UI components,
  design tokens, and layout primitives shared across web apps.
- **Analytics Client (`packages/analytics-client`)**: An event emitter that
  sends clickstream events to the Clickstream Service in the contract-defined
  envelope format.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Directory structure matches the spec exactly — all 8 top-level
  directories and their required subdirectories exist.
- **SC-002**: `cargo build --workspace` completes with zero errors on a
  fresh clone.
- **SC-003**: `npm run build` succeeds for all 3 web apps and `expo doctor`
  passes for the mobile app.
- **SC-004**: The `contracts` crate compiles independently and its types
  are importable by any service crate.
- **SC-005**: `make build-all`, `make test-all`, `make lint-all`, and
  `make format-all` all succeed from the repository root.
- **SC-006**: A reviewer can verify that no duplicate DTOs exist outside
  the `contracts` crate (zero violations found).

## Assumptions

- The Rust toolchain (rustc, cargo) is installed at latest stable version.
- Node.js 20+ and npm are available for frontend development.
- Expo CLI is installed globally or as a dev dependency.
- This feature produces no runtime business logic — only scaffolds,
  stubs, and configuration files.
- Docker Compose and Traefik setup is handled in EPIC 2, not here.
- CI/CD pipeline configuration (GitHub Actions) is handled in EPIC 3.
- The `contracts` crate types will evolve as services are implemented;
  this epic only establishes the initial structure and example types.
- API versioning follows `specs/001-architecture-contracts/contracts/api-versioning-contract.md`:
  all endpoints under `/api/v1/*`; no unversioned routes.
