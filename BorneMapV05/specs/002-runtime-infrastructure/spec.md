# Feature Specification: Runtime Infrastructure

**Feature Branch**: `002-runtime-infrastructure`

**Created**: 2026-06-02

**Status**: Clarified

**Input**: User description: "read docs/EXECUTION_PLAN.md and start sprint 2"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Developer Brings Full Stack Online Locally (Priority: P1)

A developer needs to run the entire Bornemap platform on their local machine. They run a single `docker compose up` command and all nine services (Traefik, Keycloak, PostgreSQL, RabbitMQ, and five backend services) start in the correct order. Each service waits for its dependencies to be healthy before starting.

**Why this priority**: Without the full stack running locally, no backend or frontend developer can test integrations, validate database schemas, or verify authentication flows. This is the essential prerequisite for all subsequent sprints.

**Independent Test**: From a clean Docker environment, `docker compose up` starts all services and they remain healthy for at least 60 seconds with no restart loops.

**Acceptance Scenarios**:

1. **Given** Docker is installed on the developer machine, **When** they run `docker compose up` from the `infra/compose/` directory, **Then** all nine services start successfully and report healthy status.
2. **Given** startup ordering is configured, **When** services start, **Then** PostgreSQL starts before RabbitMQ, RabbitMQ starts before Keycloak, and backend services start only after all infrastructure dependencies are healthy.
3. **Given** the Docker Compose configuration, **When** inspected, **Then** only Traefik has host port mappings (80, 443); all other services are on an internal-only network.
4. **Given** a developer runs `docker compose down`, **When** they run `docker compose up` again, **Then** the same deterministic startup sequence occurs.

---

### User Story 2 — Developer Verifies Each Service Health (Priority: P2)

A developer needs to confirm that each backend service is running and responding. They curl each service's `/health` endpoint and receive a JSON `{"status":"ok"}` response, confirming the service binary is live and accepting connections.

**Why this priority**: Health endpoints are the foundation for Docker health checks and operational monitoring. Without them, the platform cannot distinguish between a starting service and a hung one.

**Independent Test**: After `docker compose up` completes, `curl -f` on each backend service's `/health` endpoint returns HTTP 200 for all five services.

**Acceptance Scenarios**:

1. **Given** all services are running, **When** a developer runs `curl -f http://<service>.internal:8081/health`, **Then** the response is HTTP 200 with `{"status":"ok"}`.
2. **Given** the health endpoint pattern works for one service, **When** tested on all five backend services (ports 8081-8085), **Then** each returns HTTP 200.
3. **Given** Traefik is configured with `localhost` as the entry point, **When** a developer accesses `http://localhost/api/v1/{service}/...`, **Then** Traefik routes to the appropriate backend service (e.g., `/api/v1/drivers/health` → driver-service:8081, `/api/v1/admin/health` → admin-service:8082).

---

### User Story 3 — Operator Validates Infrastructure Dependencies (Priority: P3)

An operator needs to confirm that the infrastructure dependencies (Keycloak, PostgreSQL, RabbitMQ) are reachable and functional from within the Docker network.

**Why this priority**: Infrastructure failures are the hardest to diagnose. Validating that the databases, message broker, and identity provider are all functional and reachable by their internal DNS names catches configuration errors before business code runs on them.

**Independent Test**: From within any backend container, `ping postgres.internal`, `ping rabbitmq.internal`, and `ping keycloak.internal` resolve and are reachable. PostgreSQL accepts connections as the `platform_user`, RabbitMQ responds to diagnostics ping, and Keycloak serves its health endpoint.

**Acceptance Scenarios**:

1. **Given** Postgres is healthy, **When** a tool connects with `platform_user` credentials to `postgres.internal:5432`, **Then** the connection succeeds and the databases `keycloak_db`, `platform_db` (with PostGIS extension), and `analytics_db` exist.
2. **Given** RabbitMQ is healthy, **When** the management API is queried at `rabbitmq.internal:15672`, **Then** it responds with the RabbitMQ version and cluster status.
3. **Given** Keycloak is healthy and the realm is imported, **When** the OIDC configuration is fetched from `http://keycloak.internal:8080/realms/bornemap/.well-known/openid-configuration`, **Then** it returns valid metadata including the three realm roles (`registered_driver`, `partner`, `admin`).

### Edge Cases

- What happens when Docker socket permissions prevent Traefik from reading container labels? Traefik falls back to file-based provider and logs a warning; static routes still work.
- What happens when PostgreSQL starts but the init script hasn't finished creating databases? The health check polls `pg_isready` which only returns success once the server is accepting connections — the init script runs before the server becomes available.
- What happens when a backend service starts before Keycloak is ready? The `depends_on` condition prevents this by waiting for Keycloak's health check to pass.
- What happens when the `internal` network's `internal: true` flag prevents DNS resolution of external hostnames? Backend services should not need external DNS — all dependencies are on the internal network by design.
- What happens when `docker compose up` is re-run after a failed partial start? `depends_on` ensures all dependencies are re-checked; no stale state is assumed from a previous run.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The Docker Compose configuration MUST define nine services: traefik, keycloak, postgres, rabbitmq, driver-service, admin-service, clickstream-service, gis-worker, and analytics-writer.
- **FR-002**: All backend services and infrastructure components MUST be placed on a single internal Docker network with `internal: true`.
- **FR-003**: Only Traefik MUST have host port mappings (`80:80`, `443:443`); all other services MUST NOT expose host ports in the base compose file.
- **FR-004**: Each service MUST define a health check that validates the service is ready to serve traffic before it is considered healthy.
- **FR-005**: Service startup MUST respect a dependency chain: PostgreSQL → RabbitMQ → Keycloak → Traefik → backend services → workers.
- **FR-006**: Each Rust backend service MUST serve an HTTP `GET /health` endpoint returning HTTP 200 with content `{"status":"ok"}`.
- **FR-007**: The health endpoint MUST be served on the port configured via the service's `{SERVICE}_PORT` environment variable (e.g., `DRIVER_SERVICE_PORT=8081`, `ADMIN_SERVICE_PORT=8082`, `CLICKSTREAM_SERVICE_PORT=8083`, `GIS_WORKER_PORT=8084`, `ANALYTICS_WRITER_PORT=8085`).
- **FR-008**: PostgreSQL MUST initialize with three databases: `keycloak_db`, `platform_db` (with PostGIS extension enabled), and `analytics_db`.
- **FR-009**: Keycloak MUST start with the `bornemap` realm pre-imported, containing the three roles (`registered_driver`, `partner`, `admin`) and the `bornemap-api` client configured as a public client.
- **FR-010**: Keycloak MUST be configured to use the PostgreSQL instance as its database backend (not its embedded H2 database).
- **FR-011**: Each service MUST source its configuration from a corresponding `.env.example` file in `infra/env/`, with Docker-compatible variable names for infrastructure services.
- **FR-012**: Traefik MUST be configured with `localhost` as the single entry point, and route using path prefixes: `/api/v1/drivers/*` → driver-service:8081, `/api/v1/admin/*` → admin-service:8082, `/api/v1/clickstream/*` → clickstream-service:8083, `/api/v1/gis/*` → gis-worker:8084, `/api/v1/analytics/*` → analytics-writer:8085. Routes MUST preserve path prefix stripping so backend services receive clean paths (e.g., `/health` not `/api/v1/drivers/health`).
- **FR-013**: The `docker-compose.override.yml` MUST expose infrastructure ports (5432, 5672, 15672, 8080) for local development convenience without modifying the base compose file.
- **FR-014**: Rust backend services MUST read their port configuration from environment variables at runtime with sensible defaults hardcoded (8081 for driver-service, 8082 for admin-service, 8083 for clickstream-service, 8084 for gis-worker, 8085 for analytics-writer).

### Key Entities *(include if feature involves data)*

- **Docker Compose service**: A containerized runtime component (Traefik, Keycloak, PostgreSQL, RabbitMQ, or a  Rust backend) defined in `docker-compose.yml` with build context, environment, network, health check, and dependency configuration.
- **Health endpoint**: A minimal HTTP `GET /health` endpoint served by each Rust backend service that validates the service is alive and returns HTTP 200 with `{"status":"ok"}`.
- **Docker internal network**: A bridge network with `internal: true` that prevents containers from reaching the host network or internet, ensuring all inter-service communication stays within the Docker environment.
- **Database init script**: A shell script mounted to PostgreSQL's `docker-entrypoint-initdb.d/` directory that creates the three required databases and enables PostGIS on `platform_db`.
- **Keycloak realm import**: A JSON file defining the `bornemap` realm, its roles, clients, and seed users, auto-imported by Keycloak on first startup via the `--import-realm` flag.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `docker compose up` completes with all nine services reporting healthy within 120 seconds on a developer machine with cached Docker images.
- **SC-002**: `curl -f http://localhost/api/v1/{service}/health` through Traefik returns HTTP 200 for all five backend services within 5 seconds of the stack being fully up.
- **SC-003**: A developer can verify all three infrastructure dependencies (PostgreSQL connectivity, RabbitMQ ping, Keycloak OIDC metadata) from within the Docker network without external tools.
- **SC-004**: `docker compose config --services` lists exactly nine services matching the canonical names from the project specification.

## Clarifications

The following decisions were made during the clarify phase (2026-06-02):

- **Traefik routing scheme**: Single domain (`localhost`) with `/api/v1/{service}/*` path-based routing, with path prefix stripping enabled. Five routes mapping path prefixes to internal ports 8081-8085.
- **docker-compose.override.yml**: Included in Sprint 2 scope, exposing infrastructure ports (5432, 5672, 15672, 8080) for local development.
- **Docker image pinning**: Use `latest` tags for PostgreSQL, RabbitMQ, and Keycloak images (no version pinning in this sprint).

## Assumptions

- Docker Engine 24+ and Docker Compose v2 will be available on the developer machine.
- Rust backend health endpoints are stubs in this sprint — they return `{"status":"ok"}` without validating actual database or queue connectivity. Real dependency checks will be added when those dependencies are implemented in later sprints.
- PostgreSQL images use the official `postgis/postgis` distribution which includes PostGIS pre-installed.
- Keycloak 26 (Quarkus distribution) is used for compatibility with the `--import-realm` auto-import feature and `KC_*` environment variable conventions.
- The `docker-compose.override.yml` is for local development only and is not used in production-like deployments.
- Frontend applications (web and mobile) are not part of the Docker Compose topology in this sprint — they run outside Docker via their respective dev servers.
- The `/health` endpoint is exempt from the constitution-mandated JSON envelope (Principle IV) because it is an operational/liveness probe, not a business API. Real business endpoints in Sprint 3+ will use the full envelope.
