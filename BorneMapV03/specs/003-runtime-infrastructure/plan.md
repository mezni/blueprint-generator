# Implementation Plan: Runtime Infrastructure & API Gateway

**Branch**: `003-runtime-infrastructure` | **Date**: 2026-05-31 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/003-runtime-infrastructure/spec.md`

## Summary

Build the complete containerized runtime platform — Docker Compose orchestration with 11 services, Traefik API gateway enforcing `/api/v1` versioning, PostgreSQL/PostGIS, RabbitMQ, Keycloak identity, service health monitoring, and a GitHub Actions CI pipeline. Single-command boot. Zero runtime business logic.

## Technical Context

**Language/Version**: YAML (Compose), HCL (Traefik), shell (health checks)

**Primary Dependencies**: Docker Compose v2+, Traefik v3, PostgreSQL 16+ with PostGIS, RabbitMQ 3.x (management-enabled), Keycloak 25+

**Storage**: PostgreSQL 16+ with PostGIS extension (persistent volume); RabbitMQ queues (ephemeral in dev, persistent in prod)

**Testing**: `docker compose config --quiet` for syntax validation; `curl`-based health endpoint checks; CI pipeline with six stages (lint, test, build, contract validation, Docker build, GHCR publish)

**Target Platform**: Linux server (production bare metal), developer machine (local Docker Compose)

**Project Type**: Infrastructure orchestration — Docker Compose + Traefik config + GitHub Actions

**Performance Goals**: Platform fully operational within 5 minutes on a developer machine (SC-001); CI pipeline completes within 15 minutes (SC-004)

**Constraints**: Single-command boot (`docker compose up -d`); no staging environment; only Traefik exposes ports externally; all backend routes under `/api/v1/*`; no hardcoded secrets; reproducible container builds

**Scale/Scope**: 11 services across 2 Docker networks (public_network, internal_backend); 2 environments (local, production); CI publishes 4 Docker images to GHCR

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Rationale |
|-----------|--------|-----------|
| I. Pragmatic Architecture | ✅ PASS | 11 services = entire platform; each has a distinct role; no fragmentation |
| II. Clear Ownership Boundaries | ✅ PASS | Each service runs in its own container with independent connection pools; no cross-service DB access |
| III. Operational Simplicity | ✅ PASS | Docker Compose only; no k8s, no staging, bare metal deployment |
| IV. Evolution over Complexity | ⚠ SEE TRACKING | 11 services require justification (see Complexity Tracking) |
| V. Data Separation | ✅ PASS | 4-schema rule (inventory, users, gis, analytics) inherited from EPIC 0; EPIC 2 configures the single database instance |

**GATE RESULT**: PASS with complexity tracking note.

### Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| 11 services (traefik, postgres, rabbitmq, keycloak, 4 backend, 3 frontend) | Full platform must boot as a single unit for deterministic local development and production deployment | Starting services individually would violate FR-001 (single-command boot) and introduce manual sequencing errors |
| 2 networks (public, internal) | Network isolation is a security requirement (US5, FR-004) — only Traefik may be publicly reachable | Single flat network would expose PostgreSQL, RabbitMQ, and backend services to external access |
| CI pipeline with 5 job types | Pipeline must validate all artifacts (Rust, TypeScript, Docker) before publishing to GHCR per constitution CI/CD mandate | Single monolithic job would fail to parallelize lint/test/build and wouldn't meet the 15-minute CI target |

## Project Structure

### Documentation (this feature)

```text
specs/003-runtime-infrastructure/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
borne-map/
├── infra/
│   ├── compose/
│   │   ├── docker-compose.yml          # Base Compose (Traefik, all services)
│   │   ├── docker-compose.dev.yml       # Dev overrides (ports, volumes, debug)
│   │   ├── docker-compose.prod.yml      # Prod overrides (resource limits, TLS)
│   │   └── .env.example                 # Environment variable template
│   ├── traefik/
│   │   ├── traefik.yml                  # Static config (entrypoints, providers)
│   │   └── dynamic.yml                  # Dynamic routing (routers, services, middleware)
│   └── docker/
│       ├── admin-service.Dockerfile     # Multi-stage Rust build + distroless runtime
│       ├── driver-service.Dockerfile
│       ├── clickstream-service.Dockerfile
│       └── gis-sync-worker.Dockerfile
├── .github/
│   └── workflows/
│       └── ci.yml                       # Six-stage CI pipeline
└── .env                                 # Local runtime configuration (gitignored)
```

**Structure Decision**: Infrastructure files live under `infra/` (compose + traefik + docker). CI pipeline under `.github/workflows/`. All runtime config centralized in `.env`. Matches EPIC 1 directory structure and EPIC 2 Deliverables section.
