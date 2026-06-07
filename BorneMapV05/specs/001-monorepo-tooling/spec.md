# Feature Specification: Monorepo + Tooling Foundation

**Feature Branch**: `001-monorepo-tooling`

**Created**: 2026-06-02

**Status**: Draft

**Input**: User description: "read docs/EXECUTION_PLAN.md and start sprint 1"

## Clarifications

### Session 2026-06-02

- Q: What minimum toolchain versions should Sprint 1 enforce? → A: Pin Rust edition = "2024" in Cargo.toml and add `.nvmrc` with Node.js 22 LTS.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Backend Engineers Set Up the Rust Workspace (Priority: P1)

A developer needs to build and test backend services. They clone the repository, run a single command, and all Rust crates (service binaries and shared libraries) compile successfully with no errors. The workspace links internal crates so changes to shared types are immediately reflected across all services without manual path configuration.

**Why this priority**: Every backend service depends on shared crates (`common-types`, `common-errors`, `common-auth`, `common-db`). Without the workspace, no service can be developed or tested. This is the foundation Sprint 1 exists to create.

**Independent Test**: A fresh clone of the repository builds with `cargo build` without requiring any pre-installed dependencies beyond Rust toolchain. The output shows zero errors for all workspace members.

**Acceptance Scenarios**:

1. **Given** a developer has cloned the repository and installed the Rust toolchain, **When** they run `cargo build` from the repository root, **Then** all workspace members compile successfully and the build exits with code 0.
2. **Given** the workspace includes shared crates (`common-types`, `common-errors`, `common-auth`, `common-db`), **When** a service crate references a type from `common-types`, **Then** the import resolves without additional configuration.
3. **Given** the build runs, **When** a compilation error exists in a shared crate, **Then** that error is surfaced alongside any dependent service errors (workspace-aware, not siloed).

---

### User Story 2 — Frontend Engineers Bootstraps Application Shells (Priority: P1)

A frontend developer needs to start working on any of the three web apps (driver, partner, admin) or the mobile app. They run a single command per app and see an empty screen rendering in the browser or simulator. All apps share a common design system base and API client, so changes to shared packages are usable from any app without copy-pasting code.

**Why this priority**: Four frontend apps must be created for the platform. They share contracts, types, design tokens, and API clients. Establishing the shared package structure and app shells is required before any UI work can start.

**Independent Test**: Each web app starts with a dev server and renders a page with its app name. The mobile app launches in Expo Go and shows its shell screen. All apps import a symbol from each shared TypeScript package without errors.

**Acceptance Scenarios**:

1. **Given** a developer runs the dev server for `driver-web`, **When** they open the browser, **Then** they see a page displaying "Driver Web App" and the page title includes the app name.
2. **Given** a developer runs the dev server for `partner-dashboard`, **When** they open the browser, **Then** they see a page displaying "Partner Dashboard."
3. **Given** a developer runs the dev server for `admin-dashboard`, **When** they open the browser, **Then** they see a page displaying "Admin Dashboard."
4. **Given** a developer runs the mobile app via Expo, **When** it loads on a device or simulator, **Then** it displays a shell screen with "Driver Mobile."
5. **Given** all apps are open, **When** a shared package is updated, **Then** all apps reflect the change on rebuild without manual intervention.

---

### User Story 3 — Teams Use Shared Contracts Across the Stack (Priority: P2)

Backend and frontend developers need a single source of truth for event schemas, API response envelopes, error codes, and type definitions. When the event taxonomy or API contract changes in one place, all consumers (all four frontends and all five backend services) see the updated shape.

**Why this priority**: The Constitution mandates contract-driven development. Duplicated or inconsistent contracts are the root cause of integration bugs. Shared packages eliminate drift.

**Independent Test**: A type defined in a shared package is imported and used in at least one Rust crate, one web app, and the mobile app without type errors.

**Acceptance Scenarios**:

1. **Given** a shared type exists in the event-taxonomy package, **When** a backend service and a frontend app both import it, **Then** both compile with matching types.
2. **Given** an envelope schema in the api-contracts package, **When** a frontend sends a request matching that envelope and a backend validates it, **Then** the shapes are structurally identical.
3. **Given** the design-tokens package exists, **When** a web app imports a token value, **Then** it resolves to the correct raw value.

---

### User Story 4 — Developers Preview Infrastructure Configuration (Priority: P3)

An infrastructure engineer needs a working Docker Compose skeleton and Traefik configuration that mirrors the target production layout. They run `docker compose up` and see placeholder services responding with a health check but no business logic yet. This validates the networking, naming, and env injection pattern before real code is written.

**Why this priority**: Sprint 2 will bring the full infrastructure online. Having the Compose skeleton and Traefik config ready in Sprint 1 unblocks the networking wiring and means Sprint 2 can focus on making services functional rather than creating files.

**Independent Test**: `docker compose config` outputs a valid configuration with all service names, internal networks, and Traefik routing rules present. Each service has a defined health check endpoint, even if it returns a placeholder.

**Acceptance Scenarios**:

1. **Given** the infra directory contains Docker Compose files, **When** a developer runs `docker compose config`, **Then** it outputs valid YAML with services for: Traefik, Keycloak, PostgreSQL, RabbitMQ, driver-service, admin-service, clickstream-service, gis-worker, and analytics-writer.
2. **Given** the Traefik configuration, **When** inspected, **Then** it defines internal-only routing for all backend services and does not expose anything except Traefik's ports.
3. **Given** a developer reads the service structure, **When** they look at any service definition, **Then** it references environment variables from a corresponding `.env.example` file.

### Edge Cases

- What happens when the Rust workspace has a member with a conflicting dependency version? The workspace Cargo.lock must produce a single resolved version per dependency.
- What happens when a shared TypeScript package has a peer dependency that one app provides but another does not? The app that lacks the dependency must fail at build time with a clear error, not at runtime.
- What happens when the Expo mobile app requires native modules that are not installed? The build or launch must fail with a message identifying the missing native dependency rather than a cryptic error.
- What happens when a developer runs `cargo build` but only one crate was modified? Cargo should only rebuild the affected crate and its dependents (incremental compilation), not the entire workspace.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST contain a monorepo structure with dedicated top-level directories for backend services, backend shared libraries, frontend applications, frontend shared packages, infrastructure configuration, and documentation.
- **FR-002**: The Rust workspace MUST include all five backend service crates as binary targets and all four shared library crates, resolvable via a single `cargo build` invocation.
- **FR-003**: The Rust shared library crates MUST provide: common type definitions (ID prefixes, common enums), error types with standard codes, an auth validation module (stub), and a database connection module (stub).
- **FR-004**: Each backend service crate MUST compile as a standalone binary with an empty main function that exits successfully.
- **FR-005**: The TypeScript workspace MUST include three React + Vite web applications: one for drivers, one for partner operations, and one for administration.
- **FR-006**: The TypeScript workspace MUST include a React Native Expo application for mobile drivers.
- **FR-007**: Each web app MUST render a distinct landing page with its application name when run under the dev server.
- **FR-008**: The mobile app MUST launch and display a shell screen in Expo Go on both iOS and Android simulators/devices.
- **FR-009**: Shared TypeScript packages MUST be created for: common type definitions, API client base, authentication client (stub), design tokens (empty), event taxonomy schema (locked), and API response contracts (envelopes and error codes).
- **FR-010**: The event taxonomy package MUST define the canonical event envelope structure with all required fields (event_id, event_name, occurred_at, ingested_at, channel, session_id) and list all event names from the taxonomy catalog.
- **FR-011**: The API contracts package MUST define the standard success envelope, error envelope, pagination metadata structure, and the canonical error codes.
- **FR-012**: Shared packages MUST be importable from backend Rust crates via Cargo workspace paths and from frontend TypeScript apps via npm workspace references.
- **FR-013**: The monorepo MUST use npm workspaces with a root `package.json` referencing all apps and packages, using `npm install` for dependency management.
- **FR-014**: The infra directory MUST contain a placeholder Docker Compose skeleton with service stubs for all nine runtime components (Traefik, Keycloak, PostgreSQL, RabbitMQ, 5 backend services).
- **FR-015**: The infra directory MUST contain a Traefik configuration template with internal routing rules.
- **FR-016**: Each service in the Docker Compose skeleton MUST define an internal DNS name following the `<service>.internal` convention and be placed on an internal-only network.
- **FR-017**: Per-service `.env.example` files MUST exist for each backend service and for Traefik, containing the configuration variables listed in the project's configuration specification with safe default values.

### Key Entities *(include if feature involves data)*

- **Service workspace member**: A Rust binary crate (driver-service, admin-service, clickstream-service, gis-worker, analytics-writer) that depends on shared library crates. Represents a deployable backend unit.
- **Library workspace member**: A Rust library crate (common-types, common-errors, common-auth, common-db) providing reusable code. Has no binary target.
- **Web application**: A React + Vite project (driver-web, partner-dashboard, admin-dashboard) that consumes shared TypeScript packages.
- **Mobile application**: A React Native Expo project (driver-mobile) that consumes shared TypeScript packages.
- **Shared TypeScript package**: A reusable npm workspace package (shared-types, api-client, auth-client, design-tokens, event-taxonomy, api-contracts) providing types, utilities, or schemas consumed by multiple apps.
- **Infrastructure component**: A Docker Compose service definition (Traefik, Keycloak, PostgreSQL, RabbitMQ, each backend service) specifying the runtime configuration.
- **Event envelope**: The canonical JSON structure wrapping all analytics events with required metadata (event_id, event_name, timestamps, channel, session_id, actor identity, payload).
- **API envelope**: The standard JSON response wrapper for all API calls (success shape, error shape, pagination meta).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All workspace compilations pass with zero errors (Rust: `cargo build`, TypeScript: `npm run build` across all apps and packages) within 5 minutes on a developer machine.
- **SC-002**: A developer can set up the entire development environment from a fresh clone and have all apps compiling in under 15 minutes, following only the steps in the README.
- **SC-003**: Each shared package is consumed by at least one consumer outside its own directory (Rust crate imported by a service; TypeScript package imported by an app), verifying that workspace references work correctly.
- **SC-004**: The Docker Compose skeleton passes `docker compose config` validation, producing a complete multi-service topology with all nine services, correct networking, and defined health checks.

## Assumptions

- The Rust toolchain (rustc, cargo) will be installed on the developer machine. The workspace uses Rust edition 2024 (pinned in root Cargo.toml).
- Node.js 22 LTS and npm will be available (pinned via `.nvmrc` at repository root).
- Expo CLI and either an Android/iOS simulator or Expo Go app will be available for mobile development testing.
- Shared Rust crates are stubs in this sprint (they compile but contain minimal or no real logic). Implementation details will be filled in later sprints.
- The Docker Compose skeleton is non-functional in Sprint 1 (services won't actually connect to databases or serve traffic) — it exists to validate the structure and naming conventions.
- Event taxonomy definitions are compile-time constants in this sprint; no runtime validation or ingestion logic is built yet.
- CI configuration and pre-commit hooks are explicitly out of scope for Sprint 1 and will be addressed in a later hardening phase.
