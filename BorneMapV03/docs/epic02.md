# EPIC 2 — Runtime Infrastructure & API Gateway

**Epic ID**: `PLAT-EPIC-2`
**Priority**: Critical
**Status**: Planned
**Depends On**: EPIC 1 (Monorepo & Workspace Foundation)

## 1. Objective

Build the complete containerized runtime platform for the EV ecosystem, providing:

- deterministic local + production execution
- service orchestration
- API gateway routing
- infrastructure dependencies
- versioned API exposure
- internal network isolation
- runtime health guarantees

This epic establishes the operational runtime baseline for all services.

## 2. Business Outcome

After completion, the entire platform must boot using one command and expose a fully operational runtime environment for:

- Driver API
- Admin API
- Clickstream ingestion
- Keycloak authentication
- PostgreSQL/PostGIS persistence
- RabbitMQ messaging
- Frontend applications via gateway

## 3. Architecture Scope

### 3.1 Container Runtime

- Docker Compose orchestration
- service lifecycle management
- health checks
- startup sequencing

### 3.2 API Gateway

- Traefik reverse proxy
- path-based routing
- API version enforcement
- frontend routing

### 3.3 Core Infrastructure Services

- PostgreSQL + PostGIS
- RabbitMQ
- Keycloak

### 3.4 Network Security Model

- internal service network
- external exposure restrictions

### 3.5 Configuration Model

- unified environment contract
- deterministic boot configuration

### 3.6 CI/CD Pipeline (this epic)

- GitHub Actions CI workflow
- lint → test → build → contract validation
- deterministic builds

## 4. Core Architectural Constraints

### 4.1 Single Runtime Entry

System must boot with:

```bash
docker compose up -d
```

No additional orchestration tools allowed.

### 4.2 API Versioning Rule (Mandatory)

All APIs must be exposed under:

```
/api/v1/*
```

No unversioned endpoints permitted.

**Valid**:
- `/api/v1/admin/users`
- `/api/v1/driver/stations`
- `/api/v1/events/ingest`

**Invalid**:
- `/admin/users`
- `/driver/stations`
- `/events`

### 4.3 Public Exposure Rule

Only Traefik may expose host ports.

Forbidden direct exposure:
- PostgreSQL
- RabbitMQ
- backend services
- internal workers

### 4.4 Environment Model

Supported environments only:
- `local`
- `production`

No staging environment allowed.

### 4.5 CI Mandate

CI is mandatory per constitution. Pipeline:
- lint → test → build → contract validation → Docker build → GHCR publish

No auto-deployment — only artifact generation.

## 5. Runtime Topology

```
Internet
   |
Traefik Gateway
   |
--------------------------------------------
|             |             |               |
Frontend Apps Versioned APIs Auth Gateway
                     |
---------------------------------------------------
|              |              |                   |
Admin      Driver      Clickstream          Keycloak
Service    Service       Service
                     |
             RabbitMQ Event Bus
                     |
              PostgreSQL/PostGIS
```

## 6. Docker Compose Specification

### 6.1 Required Compose Files

| File | Purpose |
|------|---------|
| `docker-compose.yml` | Base configuration |
| `docker-compose.dev.yml` | Local development overrides |
| `docker-compose.prod.yml` | Production overrides |

### 6.2 Required Services

| Service | Role |
|---------|------|
| **traefik** | API gateway |
| **postgres** | Database + PostGIS extension |
| **rabbitmq** | Event broker |
| **keycloak** | Identity provider |
| **admin-service** | Inventory CRUD backend |
| **driver-service** | Station discovery backend |
| **clickstream-service** | Event ingestion backend |
| **gis-sync-worker** | GIS enrichment worker |
| **driver-web** | Driver portal frontend |
| **admin-dashboard** | Admin panel frontend |
| **partner-dashboard** | Partner dashboard frontend |

## 7. Network Architecture

### 7.1 Networks

| Network | Purpose | Contains |
|---------|---------|----------|
| `public_network` | External ingress | traefik |
| `internal_backend` | Private service communication | all backend services, postgres, rabbitmq, keycloak, workers |

### 7.2 Network Rules

- Traefik may communicate with internal services
- Internal services may not expose public ports
- Database traffic allowed only from services

## 8. API Gateway Specification

### 8.1 Gateway Technology

Traefik v3

### 8.2 Required Responsibilities

- service discovery
- path routing
- request forwarding
- API version (`/api/v1`) enforcement
- health routing
- TLS readiness

### 8.3 Routing Contract

| Path | Target |
|------|--------|
| `/api/v1/driver/*` | Driver Service |
| `/api/v1/admin/*` | Admin Service |
| `/api/v1/events/*` | Clickstream Service |
| `/auth/*` | Keycloak |

### 8.4 Frontend Routes

| Path | Target |
|------|--------|
| `/` | driver-web |
| `/admin` | admin-dashboard |
| `/partner` | partner-dashboard |

### 8.5 Rejection Rule

Requests to backend routes not matching `/api/v1/*` must be rejected (HTTP 404/405).

## 9. Containerization Standards

### 9.1 Backend Services

Each service must provide:
- multi-stage Dockerfile
- release-mode build
- minimal runtime image
- deterministic dependency lock

Preferred runtime image: Distroless or Alpine

### 9.2 Frontend Services

Must support:
- production static asset build
- gateway serving (via Traefik)

### 9.3 Build Reproducibility

Container builds must not depend on host-installed tools.

## 10. Environment Configuration Specification

### 10.1 Configuration Source

All runtime config loaded from `.env`.

### 10.2 Required Variables

| Category | Variable |
|----------|----------|
| Database | `DATABASE_URL`, `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB` |
| Messaging | `RABBITMQ_URL` |
| Identity | `KEYCLOAK_URL`, `KEYCLOAK_REALM`, `KEYCLOAK_CLIENT_ID` |
| Gateway | `TRAEFIK_DOMAIN` |
| Runtime | `RUST_LOG`, `ENVIRONMENT` |

### 10.3 Forbidden Configuration

No hardcoded secrets in:
- source code
- Dockerfiles
- compose files

## 11. PostgreSQL/PostGIS Specification

- **Engine**: PostgreSQL 16+
- **Required extension**: PostGIS
- **Initialization**: Auto-create on first boot
- **Connectivity**: Each service owns independent connection pools. No shared pool process allowed.

## 12. RabbitMQ Specification

- **Role**: Event transport backbone
- **Exchange**: `events.exchange`
- **Queues**: `clickstream.raw`, `gis.sync`
- **Delivery**: At-least-once delivery. Consumers must be idempotent.

## 13. Keycloak Specification

- **Mode**: Containerized standalone deployment
- **Realm**: Platform realm with roles `registered_driver`, `partner`, `admin`
- **Clients**: `driver-web`, `admin-dashboard`, `partner-dashboard`, `driver-mobile`

## 14. Health & Observability

### 14.1 Health Endpoints

Every service must expose:
- `GET /health`
- `GET /ready`

### 14.2 Logging Contract

All services output structured JSON logs to stdout. Required fields:
- `service_name`
- `level`
- `timestamp`
- `message`

### 14.3 Gateway Health Monitoring

Traefik must use health checks for routing eligibility.

## 15. Startup Sequence

| Phase | Services |
|-------|----------|
| Phase 1 | postgres, rabbitmq |
| Phase 2 | keycloak |
| Phase 3 | traefik |
| Phase 4 | backend services (admin, driver, clickstream, gis-sync-worker) |
| Phase 5 | frontend apps (driver-web, admin-dashboard, partner-dashboard) |

## 16. CI/CD Pipeline (GitHub Actions)

### 16.1 Pipeline Stages

1. **Lint** — `cargo clippy -- -D warnings`, `eslint`
2. **Test** — `cargo test --workspace`
3. **Build** — `cargo build --workspace`, `npm run build`
4. **Contract validation** — verify DTO alignment
5. **Docker build** — multi-stage image per service
6. **GHCR publish** — tag `ghcr.io/<service>:<git-sha>`

### 16.2 Workflow Triggers

- Push to `main` and feature branches
- Pull requests targeting `main`

### 16.3 Artifact Tagging

Docker images tagged:
- `ghcr.io/<service>:<git-sha>` (per commit)
- `ghcr.io/<service>:latest` (main branch only)

### 16.4 No Auto-Deployment

CI produces artifacts only. Deployment remains manual via Docker Compose on bare metal.

## 17. Deliverables

### Infrastructure Files

| File | Location |
|------|----------|
| `docker-compose.yml` | `infra/compose/docker-compose.yml` |
| `docker-compose.dev.yml` | `infra/compose/docker-compose.dev.yml` |
| `docker-compose.prod.yml` | `infra/compose/docker-compose.prod.yml` |
| `.env.example` | `infra/compose/.env.example` |

### Gateway Configuration

| File | Location |
|------|----------|
| Traefik static config | `infra/traefik/traefik.yml` |
| Traefik dynamic routing | `infra/traefik/dynamic.yml` |

### Service Runtime Files

| File | Location |
|------|----------|
| Dockerfile per service | `infra/docker/<service>.Dockerfile` |
| healthcheck definitions | Per-service Docker Compose |

### CI/CD

| File | Location |
|------|----------|
| CI workflow | `.github/workflows/ci.yml` |

## 18. Acceptance Criteria

| Category | Criteria |
|----------|----------|
| **Runtime Boot** | Full stack boots via single `docker compose up -d` |
| Start-up order resolves correctly | No manual intervention required |
| **Gateway** | All API traffic routed correctly |
| `/api/v1` enforced globally | Invalid unversioned routes rejected |
| **Infrastructure** | PostgreSQL, RabbitMQ, Keycloak operational |
| **Security** | Only Traefik publicly exposed |
| Internal services isolated | Secrets externalized |
| **Health** | All services expose `/health` and `/ready` |
| Gateway health checks pass | |
| **CI** | Lint → test → build → contract validation all green |
| Docker images publishable to GHCR | |
| **Determinism** | Identical startup across machines |
| No host-specific assumptions | |

## 19. Definition of Done

EPIC 2 is complete when:

1. `docker compose up -d` boots the full platform from a fresh clone
2. All APIs accessible under `/api/v1/*`
3. Unversioned backend routes rejected by Traefik
4. All infrastructure services (PostgreSQL, RabbitMQ, Keycloak) operational
5. All health endpoints respond 200
6. CI pipeline passes: lint → test → build → contract validation
7. Docker images build and publish to GHCR
