# Feature Specification: Monorepo Foundation

**Feature Branch**: `001-monorepo-foundation`

**Created**: 2026-06-01

**Status**: Draft

**Input**: User description: "Sprint 1 — Monorepo & Foundation: Engineering workspace setup for Rust backend services, React+Vite web apps, Expo mobile app, shared TypeScript packages, Docker Compose skeleton, env system, health endpoints, and CI pipeline."

## Clarifications

### Session 2026-06-01

- Q: Should the spec document explicit out-of-scope declarations (non-goals) for Sprint 1? → A: Yes — add the non-goals to the spec.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Backend Service Compilation (Priority: P1)

A developer clones the repository and builds all backend services from a single command, verifying that the Rust workspace compiles without errors across all service skeletons and shared crates.

**Why this priority**: Without a compiling backend workspace, no business logic can be developed. This is the foundational dependency for all subsequent features.

**Independent Test**: Clone the repo to a clean directory and run `cargo build` from workspace root. All crates compile without errors.

**Acceptance Scenarios**:

1. **Given** a clean clone of the repository, **When** the developer runs the Rust workspace build command, **Then** all services and shared crates compile successfully
2. **Given** a compilation error in any crate, **When** the developer inspects the build output, **Then** the error is clearly attributed to the specific crate and file

---

### User Story 2 - Frontend Application Boot (Priority: P1)

A developer starts each web application and verifies it renders a default page without errors across all three apps (driver, partner dashboard, admin dashboard).

**Why this priority**: Frontend apps are the user-facing products; they must boot from day one to enable parallel UI development.

**Independent Test**: Start each app's dev server and open it in a browser. Each app displays a landing page with no console errors.

**Acceptance Scenarios**:

1. **Given** the repository is set up, **When** the developer starts the driver web app dev server, **Then** the app opens in a browser showing a functional page
2. **Given** the repository is set up, **When** the developer starts each dashboard app, **Then** each renders without TypeScript or runtime errors

---

### User Story 3 - Cross-Stack Shared Contracts (Priority: P2)

A developer imports shared type definitions, API contracts, and event taxonomy packages into both backend and frontend contexts, verifying type-safe cross-boundary communication.

**Why this priority**: Shared contracts are the backbone of the contract-driven architecture. They must work before any service-to-service integration begins.

**Independent Test**: Create a test file in a frontend app that imports and uses a type from shared-types and a function from api-client, then verify compilation succeeds.

**Acceptance Scenarios**:

1. **Given** the shared packages are built, **When** a frontend app imports a shared type and API client, **Then** there are no path resolution or type errors
2. **Given** the event taxonomy package is built, **When** it is imported in both TypeScript and Rust contexts, **Then** the event envelope structure compiles correctly

---

### User Story 4 - Infrastructure Validation (Priority: P2)

A developer validates the Docker Compose configuration and environment variable system to ensure the local development infrastructure is correctly defined.

**Why this priority**: The infrastructure skeleton (Traefik, Postgres, Keycloak, RabbitMQ, backend services) must be wired correctly before any container-based development or testing can proceed.

**Independent Test**: Run `docker compose config` and verify the output is valid YAML with all expected services listed.

**Acceptance Scenarios**:

1. **Given** the Docker Compose file is created, **When** the developer runs the config validation command, **Then** it passes without errors and shows all defined services
2. **Given** the environment files are set up, **When** each service loads its configuration, **Then** all required env variables are present and no hardcoded values exist in source code

---

### User Story 5 - Health Check Confirmation (Priority: P3)

An operator verifies that each backend service exposes a health endpoint returning a consistent JSON response, confirming the service skeleton is alive.

**Why this priority**: Health endpoints are the minimum observability contract required by the constitution. They confirm services are running before any business logic is added.

**Independent Test**: Send an HTTP GET request to each service's `/health` endpoint and verify the response is a consistent JSON with a 200 status code.

**Acceptance Scenarios**:

1. **Given** a backend service is running, **When** an HTTP GET is sent to its `/health` endpoint, **Then** the response status is 200 and the body is valid JSON
2. **Given** multiple services are running, **When** health checks are performed across all of them, **Then** the response format is consistent (same envelope structure)

---

### User Story 6 - Mobile App Launch (Priority: P3)

A developer starts the Expo-based mobile app on a device or emulator and confirms it renders without crashing.

**Why this priority**: The mobile app is a key product surface. Establishing it early prevents integration surprises later.

**Independent Test**: Start the Expo dev server and open the app on an emulator. The app launches to a default screen with no runtime crash.

**Acceptance Scenarios**:

1. **Given** the Expo project is initialized, **When** the developer starts the Metro bundler and loads the app, **Then** it renders without crash
2. **Given** TypeScript is enabled, **When** the app code is type-checked, **Then** no TypeScript errors are present

---

### User Story 7 - CI Pipeline Verification (Priority: P3)

A developer pushes changes and the CI pipeline automatically runs build and typecheck jobs for both backend and frontend, ensuring no regressions.

**Why this priority**: Automated CI gates protect the monorepo from broken builds. This must be in place before any team collaboration begins.

**Independent Test**: Push a commit to a branch and verify CI triggers and passes all build and typecheck jobs.

**Acceptance Scenarios**:

1. **Given** CI workflows are defined, **When** a commit is pushed, **Then** the Rust build check, frontend build check, and typecheck jobs all execute
2. **Given** a build-breaking change is introduced, **When** CI runs, **Then** the relevant job fails and clearly indicates the error

### Edge Cases

- What happens when a shared package has a dependency that is not yet installed?
- How does the system handle a CI job that times out due to first-time dependency download?
- What happens when environment variables are missing for a specific service?
- How does Docker Compose handle missing service images during initial validation?
- What happens when the mobile app is loaded on an unsupported platform?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST support a single-command build for all backend services
- **FR-002**: Each service skeleton MUST compile without errors when the workspace is built
- **FR-003**: Each frontend application MUST start a dev server and render a page without errors
- **FR-004**: Shared packages MUST be importable by at least one frontend application without resolution errors
- **FR-005**: The event taxonomy package MUST define an event envelope interface that compiles in both TypeScript and Rust contexts
- **FR-006**: The API contracts package MUST define a standard response envelope (success, data, error)
- **FR-007**: Docker Compose configuration MUST pass validation (`docker compose config`) with all services listed
- **FR-008**: Each backend service MUST expose a `/health` endpoint returning a 200 status code with consistent JSON
- **FR-009**: Environment variables MUST be configurable per service via external files with no hardcoded values in source code
- **FR-010**: The mobile app MUST launch on an emulator or device without crashing
- **FR-011**: CI MUST run build and typecheck jobs for the Rust workspace and all frontend apps on every push

### Key Entities *(include if feature involves data)*

> This feature does not introduce new business entities. It establishes the engineering workspace structure. Key structural elements include:

- **Service skeleton**: An empty but compilable service crate with a health endpoint, ready for business logic
- **Shared crate**: A reusable library (e.g., common-types, common-errors) consumed by multiple services
- **Shared package**: A TypeScript package (e.g., shared-types, api-client) consumed by frontend apps
- **Design token package**: A minimal package defining design primitives (colors, spacing, typography) with no styling logic

### Out of Scope

The following areas are explicitly excluded from this sprint:

- No database schema work (no SQL, no ORM setup, no migrations)
- No authentication implementation (no Keycloak setup, no JWT logic)
- No GIS logic (no spatial queries, no PostGIS setup)
- No RabbitMQ integration (no queue setup, no event publishing/consuming)
- No business APIs (no CRUD endpoints, no domain logic)
- No UI design work (no styling, no components beyond minimal skeleton)
- No event processing logic (no clickstream ingestion, no analytics)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The Rust workspace builds entirely in under 5 minutes on a standard developer machine (first build incl. dependency download)
- **SC-002**: All three web apps start their dev server and render within 30 seconds
- **SC-003**: The mobile app renders on both iOS and Android emulators without code changes
- **SC-004**: Docker Compose configuration validates in under 5 seconds with no errors
- **SC-005**: Every backend service responds to `/health` within 1 second of starting
- **SC-006**: CI completes all jobs within 10 minutes on a push event
- **SC-007**: No TypeScript errors exist across any frontend app or shared package

## Assumptions

- The development team has Rust, Node.js, and Docker installed locally
- The CI runner has sufficient resources to compile the Rust workspace (at least 4GB RAM)
- iOS development requires macOS; Android development works on any OS
- First-time builds will be slower due to dependency caching
- The monorepo structure follows standard workspace conventions for each language
- The constitution's Technology & Infrastructure Constraints are respected (Rust backend, React+Vite frontend, Expo mobile, Docker Compose deployment)
