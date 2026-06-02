# Feature Specification: Runtime Foundation

**Feature Branch**: `002-runtime-foundation`

**Created**: 2026-06-01

**Status**: Draft

**Input**: User description: Sprint 2 — Runtime Foundation: bring the platform to a runnable local distributed system via a single `docker compose up` command.

## User Scenarios & Testing

### User Story 1 - Developer boots the full platform locally (Priority: P1)

A developer runs a single command and the entire distributed system starts — all infrastructure (PostgreSQL, RabbitMQ, Keycloak, Traefik) and application services (driver, admin, clickstream, gis-worker, analytics-writer) boot in dependency order with no restart loops.

**Why this priority**: This is the core sprint goal — every other story depends on a reliably bootable system.

**Independent Test**: After `docker compose up`, all 9 containers report healthy within 120 seconds. Verified by `docker ps --filter "status=running"` showing all expected containers.

**Acceptance Scenarios**:

1. **Given** the host has Docker and Docker Compose installed, **When** `docker compose up` is executed from the project root, **Then** all 9 containers start without manual intervention.
2. **Given** the platform is running, **When** `docker compose ps` is executed, **Then** all services show status "Up" (healthy).
3. **Given** containers are starting, **When** dependency checks run, **Then** PostgreSQL starts before RabbitMQ, RabbitMQ starts before Keycloak, Keycloak starts before Traefik, and all infrastructure starts before application services.

---

### User Story 2 - Operator verifies service health and readiness (Priority: P1)

A platform operator can query any service's `/health` endpoint for static liveness and `/ready` endpoint for dependency-aware readiness, enabling monitoring and orchestration decisions.

**Why this priority**: Without health/readiness probes, the runtime is unobservable and unreliable.

**Independent Test**: Curl each service's `/health` and `/ready` endpoints and verify correct HTTP response codes and JSON bodies.

**Acceptance Scenarios**:

1. **Given** a service is running with all dependencies available, **When** `/health` is called, **Then** the response is HTTP 200 with JSON `{"status":"ok","service":"<name>","version":"<semver>"}`.
2. **Given** a service is running, **When** `/ready` is called and all dependencies (DB, RabbitMQ) are reachable, **Then** the response is HTTP 200.
3. **Given** a service is running, **When** `/ready` is called and a dependency is unreachable, **Then** the response is HTTP 503 with a JSON body describing which dependency is unavailable.

---

### User Story 3 - Developer configures a service via environment files (Priority: P2)

A developer can edit a service's `.env` file and restart to change database credentials, RabbitMQ addresses, log levels, and service ports without code changes.

**Why this priority**: Configuration flexibility is essential for local development and deployment across environments.

**Independent Test**: Change a service port in its env file, restart, and verify the service listens on the new port.

**Acceptance Scenarios**:

1. **Given** a service uses env-driven configuration, **When** a required env var is missing, **Then** the service fails to start with a clear error message identifying the missing variable.
2. **Given** a service with valid configuration, **When** it starts, **Then** it logs a sanitized summary of its resolved configuration (secrets redacted).
3. **Given** the platform is configured for `local` profile, **When** services boot, **Then** they use relaxed validation rules suitable for development.

---

### User Story 4 - Developer accesses infrastructure UIs (Priority: P2)

A developer can reach the Keycloak admin console, RabbitMQ management UI, and Traefik dashboard through the browser to inspect and debug infrastructure state.

**Why this priority**: Infrastructure debugging requires direct access to management interfaces during development.

**Independent Test**: Navigate to each infrastructure management UI and verify the login page or dashboard loads without errors.

**Acceptance Scenarios**:

1. **Given** the platform is running, **When** a developer navigates to `http://localhost:8080` (Keycloak), **Then** the admin console login page is displayed.
2. **Given** RabbitMQ is running, **When** a developer navigates to `http://localhost:15672`, **Then** the management UI login page is displayed.
3. **Given** Traefik is running, **When** a developer navigates to `http://localhost:8080/dashboard/`, **Then** the Traefik dashboard is displayed showing all configured routes and backends.

---

### User Story 5 - Operator observes service startup lifecycle (Priority: P3)

An operator can monitor service logs during boot and observe structured lifecycle events: config loading, dependency connection attempts, route registration, and ready state.

**Why this priority**: Observability of the boot sequence is critical for debugging startup failures and understanding system behavior.

**Independent Test**: Restart a service and verify its logs contain the expected lifecycle stages in order.

**Acceptance Scenarios**:

1. **Given** a service starts, **When** it loads configuration, **Then** a structured JSON log entry is emitted with the stage `"config_load"` and a summary of loaded variables.
2. **Given** a service with dependencies, **When** it attempts to connect to each dependency, **Then** structured log entries are emitted for each connection attempt (success or failure).
3. **Given** a service completes startup, **When** it registers HTTP routes, **Then** a log entry is emitted with stage `"route_registration"`.
4. **Given** a service is fully ready, **When** it reaches ready state, **Then** a log entry is emitted with stage `"ready"`.

---

### Edge Cases

- What happens when infrastructure containers restart? Application services should reconnect to dependencies with retry logic.
- How does the system handle duplicate container names from previous runs? Docker Compose should clean up or reuse existing containers.
- What happens when a required env var is missing from a profile? The service must crash immediately with a clear error message identifying the missing variable.
- How does the system behave when a dependency (e.g., PostgreSQL) is unreachable at startup? The service must retry with backoff and eventually crash if the dependency never becomes available.

## Requirements

### Functional Requirements

- **FR-001**: Docker Compose MUST start all 9 containers (PostgreSQL, RabbitMQ, Keycloak, Traefik, driver-service, admin-service, clickstream-service, gis-worker, analytics-writer) with a single command.
- **FR-002**: All containers MUST include health checks that cause restart on failure; no container may enter an infinite restart loop.
- **FR-003**: Containers MUST communicate via internal Docker DNS only using the `<service>.internal` naming convention.
- **FR-004**: PostgreSQL MUST provision four databases automatically on first boot: `keycloak_db`, `inventory_db`, `analytics_db`. A `users_db` placeholder is also created for future identity/profile data (no consumers in this sprint).
- **FR-005**: RabbitMQ MUST boot with management UI enabled and pre-declare three durable queues: `clickstream.raw`, `gis.sync`, `analytics.ingest`.
- **FR-006**: Keycloak MUST boot with persistent database storage, a pre-configured `ev-platform` realm, and placeholder clients for `driver-web`, `partner-dashboard`, `admin-dashboard`, and `driver-mobile`.
- **FR-007**: Traefik MUST route public paths `/api/driver`, `/api/admin`, and `/auth` to the corresponding services, with the dashboard accessible on port 8080.
- **FR-008**: Every Rust service MUST implement a runtime configuration loader that validates required env vars before starting and crashes immediately if validation fails.
- **FR-009**: Every Rust service MUST expose a `/health` endpoint returning HTTP 200 with service name and version.
- **FR-010**: Every Rust service with dependencies MUST expose a `/ready` endpoint returning HTTP 200 when all dependencies are reachable, and HTTP 503 with dependency details when any dependency is unavailable.
- **FR-011**: All services MUST emit structured JSON logs containing timestamp, service name, environment, severity, and correlation_id placeholder.
- **FR-012**: Service startup logs MUST include lifecycle stages: config load, dependency check, route registration, ready state.
- **FR-013**: `driver-service` and `admin-service` MUST implement DB connection bootstrap to `inventory_db` with retry logic, timeout handling, and startup validation.
- **FR-014**: `analytics-writer` MUST implement DB connection bootstrap to `analytics_db` with retry logic, timeout handling, and startup validation.
- **FR-015**: `clickstream-service`, `gis-worker`, and `analytics-writer` MUST implement RabbitMQ connection bootstrap with retry logic and reconnect strategy.
- **FR-016**: Container startup ordering MUST enforce: PostgreSQL → RabbitMQ → Keycloak → Traefik → Application services.
- **FR-017**: A runtime smoke test script MUST validate: DB connectivity, RabbitMQ queue access, Keycloak availability, Traefik routing, and all service `/health` and `/ready` endpoints.
- **FR-018**: The platform MUST support environment profiles (`local`, `docker`, `staging` placeholder) switchable via env var with no code changes.
- **FR-019**: Infrastructure containers MUST use named volumes for persistent data (PostgreSQL data, RabbitMQ data, Keycloak data).
- **FR-020**: All infrastructure containers MUST use the internal Docker network and MUST NOT expose ports to the host except Traefik (port 80) and management UIs (dev only).

### Key Entities

- **Container Image**: Prebuilt Docker image for each service and infrastructure component, tagged with version.
- **Docker Network**: Internal overlay network (`bornemap-net`) with DNS resolution for inter-service communication.
- **Environment Profile**: Named configuration set (`local`, `docker`, `staging`) controlling env var defaults and validation strictness.
- **Health Probe**: HTTP endpoint on each service returning liveness status (always OK) or readiness status (dependent on dependency availability).
- **Structured Log Entry**: JSON-formatted log line with mandatory fields: `timestamp`, `service`, `level`, `message`, `stage`, `correlation_id`.

## Success Criteria

### Measurable Outcomes

- **SC-001**: `docker compose up` completes with all 9 containers healthy within 120 seconds on a modern development machine.
- **SC-002**: After boot, no container shows restart count > 0 in `docker compose ps` output.
- **SC-003**: Every service's `/health` endpoint returns HTTP 200 in under 500ms.
- **SC-004**: Every service's `/ready` endpoint returns HTTP 200 within 30 seconds of container start (after dependencies become available).
- **SC-005**: All internal DNS names (`postgres.internal`, `rabbitmq.internal`, `keycloak.internal`, `driver.internal`, `admin.internal`, `clickstream.internal`, `gis.internal`, `analytics.internal`) resolve from any container.
- **SC-006**: All three PostgreSQL databases (`users_db`, `inventory_db`, `analytics_db`) exist and are accessible after PostgreSQL startup.
- **SC-007**: All three RabbitMQ queues exist and are visible in the management UI after RabbitMQ startup.
- **SC-008**: Traefik dashboard shows all configured routes as "UP" after all services are healthy.
- **SC-009**: Running `./scripts/smoke-test.sh` exits with code 0 and reports all checks passed.
- **SC-010**: Switching from `local` to `docker` profile requires no code changes — only an env var change and restart.

## Assumptions

- Docker Engine 24+ and Docker Compose v2 are available on the host.
- No external DNS or service discovery infrastructure exists — all resolution uses Docker's embedded DNS with container names.
- Infrastructure images (PostgreSQL 16, RabbitMQ 4, Keycloak 26+, Traefik v3) are pulled from public registries.
- Application service images are built locally from the monorepo during development.
- Management UIs (Keycloak admin, RabbitMQ management, Traefik dashboard) are accessible for debugging in development only and will be locked down in production.
- Connection retry uses exponential backoff: base delay 100ms, multiplier 2x, max delay 30s, with ±25% jitter.
- No TLS/SSL is required for internal container communication in this sprint.
- The `ev-platform` Keycloak realm is a placeholder with no real user data — authentication enforcement is explicitly out of scope.
- The constitution's `platform_db` is split into separate databases (`inventory_db`, `users_db`) in this sprint to align with individual service connectivity boundaries. These may be consolidated into a single `platform_db` with schemas in a future sprint.
