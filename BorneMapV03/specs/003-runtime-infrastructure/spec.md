# Feature Specification: Runtime Infrastructure & API Gateway

**Feature Branch**: `003-runtime-infrastructure`

**Created**: 2026-05-31

**Status**: Draft

**Input**: User description: "from docs/epic02.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Single-Command Platform Boot (Priority: P1)

As a platform engineer, I want to boot the entire platform with a single command so that I can start developing and testing against a fully operational runtime environment without manual setup steps.

**Why this priority**: Every other task (API testing, feature development, integration) depends on the runtime being available. Without single-command boot, onboarding and iteration are blocked.

**Independent Test**: A developer can run the platform boot command on a fresh clone and reach all frontend applications and API endpoints within 5 minutes without any manual intervention.

**Acceptance Scenarios**:

1. **Given** a fresh clone of the repository, **When** executing the platform boot command, **Then** all services (API gateway, backend services, frontend applications, database, message broker, identity provider) start and report healthy within 5 minutes.
2. **Given** the platform is booting, **When** infrastructure services (database, message broker) start, **Then** dependent services wait for them before starting.
3. **Given** all services are running, **When** the boot command is re-executed, **Then** no duplicate instances or conflicts occur.

---

### User Story 2 — Versioned API Access via Gateway (Priority: P1)

As an API consumer (frontend developer or mobile app), I want all backend APIs accessible through a single gateway under versioned paths so that I can make predictable, forward-compatible API calls without knowing internal service locations.

**Why this priority**: The API gateway is the single point of entry for all clients. Without it, services are inaccessible, routing is undefined, and future API versioning is impossible.

**Independent Test**: An API consumer can reach every backend service endpoint under `/api/v1/<service>/...` and receives a rejection for any unversioned path.

**Acceptance Scenarios**:

1. **Given** the platform is running, **When** making a request to `/api/v1/driver/stations`, **Then** the request reaches the driver service and returns a response.
2. **Given** the platform is running, **When** making a request to `/api/v1/admin/users`, **Then** the request reaches the admin service and returns a response.
3. **Given** the platform is running, **When** making a request to `/api/v1/events/ingest`, **Then** the request reaches the clickstream service and returns a response.
4. **Given** the platform is running, **When** making a request to a backend path without the `/api/v1/` prefix (e.g., `/stations`), **Then** the request is rejected.

---

### User Story 3 — Infrastructure Services Automated (Priority: P2)

As a platform engineer, I want the database, message broker, and identity provider to start automatically with the platform so that backend services have their required dependencies available without manual provisioning.

**Why this priority**: Backend services cannot function without these dependencies. Automation ensures reproducibility and eliminates manual setup errors.

**Independent Test**: After platform boot, the database, message broker, and identity provider are all accessible and ready for connections.

**Acceptance Scenarios**:

1. **Given** the platform boot completes, **When** attempting to connect to the database, **Then** the connection succeeds without additional configuration.
2. **Given** the platform boot completes, **When** attempting to publish a message to the event broker, **Then** the message is accepted.
3. **Given** the platform boot completes, **When** attempting to authenticate via the identity provider, **Then** authentication succeeds with the configured roles.

---

### User Story 4 — Automated Build and Validation Pipeline (Priority: P2)

As a developer, I want every code change automatically built, tested, and validated so that I can catch integration issues early and have confidence that my changes work in the runtime environment.

**Why this priority**: Without automated validation, integration issues are discovered late, increasing rework and delaying delivery.

**Independent Test**: A developer can push code to any branch and see the pipeline run lint, test, build, and contract validation stages, with results reported within 15 minutes.

**Acceptance Scenarios**:

1. **Given** a code change is pushed to a branch, **When** the CI pipeline runs, **Then** lint, test, build, and contract validation stages all complete successfully.
2. **Given** a code change introduces a DTO outside the shared contracts crate, **When** contract validation runs, **Then** it fails and reports the violation.
3. **Given** a change is merged to the main branch, **When** the CI pipeline runs, **Then** Docker images are built and published to the container registry.

---

### User Story 5 — Secure Runtime Environment (Priority: P2)

As a security engineer, I want the platform runtime to enforce network isolation, externalize all secrets, and expose only the API gateway publicly so that internal services are protected from direct external access.

**Why this priority**: Security is a platform requirement. Exposing internal services directly would violate the architecture contract and create unacceptable risk.

**Independent Test**: A security reviewer can verify that only the API gateway exposes ports externally, no secrets are hardcoded in configuration files, and internal services cannot be reached from outside the platform network.

**Acceptance Scenarios**:

1. **Given** the platform is running, **When** scanning externally accessible ports, **Then** only the API gateway port is open to the host.
2. **Given** the platform configuration, **When** inspecting configuration files, **Then** no database passwords, API keys, or service secrets are hardcoded.
3. **Given** the platform is running, **When** attempting to directly connect to the database or message broker from outside the platform network, **Then** the connection is refused.

---

### Edge Cases

- What happens if the platform boot command is run before all prerequisite tools (container engine) are installed? A clear error message should indicate the missing dependency.
- What happens if the `.env` configuration file is missing? The platform should fail early with a descriptive error listing required variables.
- What happens if a service repeatedly fails its health check? The gateway should stop routing traffic to that service and log the failure.
- What happens when a CI pipeline stage fails (e.g., lint error)? The pipeline should halt, report the failure, and not proceed to subsequent stages.
- What happens if the container registry is unreachable during CI image publish? The build stage should complete successfully but the publish stage should fail with a clear error.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The platform MUST boot all services with a single command — no manual steps, no additional orchestration tools.
- **FR-002**: All backend APIs MUST be accessible through a single gateway under the path prefix `/api/v1/`.
- **FR-003**: The gateway MUST reject requests made to backend paths that do not start with `/api/v1/`.
- **FR-004**: Only the API gateway MAY expose ports externally — all internal services MUST remain unreachable from outside the platform network.
- **FR-005**: A database service MUST start automatically and be ready for connections before dependent services start.
- **FR-006**: A message broker MUST start automatically and accept messages from backend services.
- **FR-007**: An identity provider MUST start automatically with the roles `registered_driver`, `partner`, and `admin` configured.
- **FR-008**: All services MUST expose a health endpoint at `GET /health` returning HTTP 200 with body `{"status":"ok"}`. Backend containers MUST implement Docker HEALTHCHECK using a TCP probe or busybox-based curl (distroless images do not include curl/wget natively).
- **FR-009**: All services MUST output structured JSON logs to standard output with fields: `service_name`, `request_id`, `trace_id`, `user_id`, `event_type`.
- **FR-010**: All runtime configuration MUST be loaded from an environment file — no hardcoded secrets in source code, container images, or configuration files.
- **FR-011**: Services MUST start in dependency order: infrastructure services first, then identity, then gateway, then backend services, then frontend applications.
- **FR-012**: A CI pipeline MUST run on every push, executing lint, test, build, and contract validation stages.
- **FR-013**: When code is merged to the main branch, the CI pipeline MUST build and publish container images to a registry.
- **FR-014**: Container builds MUST be reproducible — no dependency on host-installed tools.
- **FR-015**: The platform MUST support exactly two environments: local development and production. A staging environment is not required.
- **FR-016**: The driver API router MUST enforce rate limiting at 100 requests per minute per IP with a burst of 20.

### Key Entities *(include if feature involves data)*

- **Environment Configuration**: A set of variables loaded from an environment file that controls service connectivity, credentials, and runtime behavior across all platform services.
- **Service Health State**: The operational status of each service, reported via health endpoints and used by the gateway for routing decisions.
- **CI Pipeline Result**: The outcome of each automated validation stage (lint, test, build, contract validation), determining whether a code change is safe to deploy.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer can boot the full platform on a fresh clone with one command and reach all frontend applications and API endpoints within 5 minutes.
- **SC-002**: All backend API calls made under `/api/v1/*` are successfully routed; all calls to unversioned backend paths are rejected.
- **SC-003**: A security scan of externally accessible ports shows only the gateway port open — zero internal services exposed.
- **SC-004**: The CI pipeline completes all stages (lint, test, build, contract validation) within 15 minutes on every push.
- **SC-005**: Container images are published to a registry for every main branch merge, tagged with the commit identifier.
- **SC-006**: Reviewing the environment configuration and container images reveals zero hardcoded secrets or credentials.

## Assumptions

- The target machine has a container engine (Docker) and Compose plugin installed.
- The CI runner has network access to the container registry (GHCR).
- Backend services are already compiled and available as build artifacts from EPIC 1.
- This feature assumes no runtime business logic changes — only infrastructure and configuration.
- The API versioning contract (`/api/v1/*`) is the canonical rule and must not be overridden per service.
- Identity provider configuration (realm, roles, clients) is initialized once and persisted across restarts.
