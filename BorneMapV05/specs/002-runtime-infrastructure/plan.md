# Implementation Plan: Runtime Infrastructure (Docker Compose v1)

**Branch**: `002-runtime-infrastructure` | **Date**: 2026-06-02 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/002-runtime-infrastructure/spec.md`

## Summary

Bring the full Bornemap platform online locally via Docker Compose with 9 services (5 Rust backend services + 4 infrastructure services: Traefik, Keycloak, PostgreSQL, RabbitMQ). All backend services serve HTTP `GET /health` endpoints returning `{"status":"ok"}`. Infrastructure dependencies are configured with proper startup ordering (PostgreSQL → RabbitMQ → Keycloak → Traefik → backends), internal-only Docker networking (only Traefik exposed on host ports), and Traefik path-based routing at `localhost/api/v1/{service}/*` with prefix stripping.

## Technical Context

**Language/Version**: Rust 1.85+ (edition 2024), Node.js 22 LTS, Docker Engine 24+ / Compose v2

**Primary Dependencies**:
- Infrastructure: Docker Compose v2, Traefik v3 (latest), PostgreSQL 17 with PostGIS (latest), RabbitMQ 4.x management (latest), Keycloak 26 Quarkus (latest)
- Backend: Rust with tokio (raw TCP listener, no web framework — health endpoint uses std `TcpListener`)

**Storage**: PostgreSQL — 3 databases (`keycloak_db`, `platform_db` with PostGIS, `analytics_db`)

**Testing**: Manual verification — `docker compose up`, `docker compose ps`, `curl` health endpoint on each service

**Target Platform**: Linux x86_64 Docker containers (host-agnostic via Docker)

**Project Type**: Monorepo — 5 Rust backend services + 4 infrastructure services in Docker Compose

**Performance Goals**: N/A (Sprint 2 is infrastructure only; no business logic or load)

**Constraints**:
- Docker Compose only; no Kubernetes, no service mesh, no registry dependency
- Internal-only Docker bridge network; only Traefik exposes host ports (80, 443)
- No `curl` in Rust base images — health checks use `/dev/tcp` shell probes for backends
- Each service owns its own config via `.env.example` files; no cross-service env coupling

**Scale/Scope**: 9 containers on a single host, local development only

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Verdict | Notes |
|-----------|---------|-------|
| I. Data-First Source of Truth | ✅ PASS | No business data in Sprint 2; databases initialized empty |
| II. Strict Domain & Service Separation | ✅ PASS | 9 services with clear boundaries; no cross-service coupling |
| III. Ownership-Enforced Authorization | ✅ PASS | Not applicable — Keycloak configured with roles but no auth enforcement yet |
| IV. Contract-Driven REST APIs | ✅ PASS | Health endpoint returns constitution-mandated envelope |
| V. Event-Driven & Derived State | ✅ PASS | Not applicable — no events or queues in Sprint 2 |
| VI. Soft Delete & Auditability | ✅ PASS | Not applicable — no data models in Sprint 2 |
| VII. Verification Discipline | ✅ PASS | Health endpoints per constitution; manual verification via curl |
| Technology & Infrastructure Constraints | ✅ PASS | Docker Compose + Traefik + 3PG + internal network; matches constitution |
| Development Workflow & Quality Gates | ✅ PASS | Sprint 2 is foundational infra; CI not yet configured |

No violations. Complexity Tracking not required.

## Project Structure

### Documentation (this feature)

```text
specs/002-runtime-infrastructure/
├── plan.md              # This file
├── research.md          # Phase 0 — technology decisions & patterns
├── data-model.md        # Phase 1 — database layout, network topology, port map
├── quickstart.md        # Phase 1 — how to bring up the stack
├── contracts/
│   └── health-endpoint.json  # Health endpoint response schema
└── tasks.md             # Created by /speckit.tasks
```

### Source Code (repository root)

```text
infra/
├── compose/
│   ├── docker-compose.yml         # Main compose file (9 services)
│   ├── docker-compose.override.yml # Local dev port exposures
│   ├── traefik/
│   │   └── config.yml             # Static Traefik routing rules
│   ├── postgres/
│   │   └── init-dbs.sh            # Database initialization script
│   └── keycloak/
│       └── bornemap-realm.json    # Realm configuration import
└── env/
    ├── postgres.env.example
    ├── keycloak.env.example
    ├── rabbitmq.env.example
    ├── traefik.env.example
    ├── driver-service.env.example
    ├── admin-service.env.example
    ├── clickstream-service.env.example
    ├── gis-worker.env.example
    └── analytics-writer.env.example

services/
├── driver-service/
│   ├── Dockerfile
│   └── src/main.rs
├── admin-service/
│   ├── Dockerfile
│   └── src/main.rs
├── clickstream-service/
│   ├── Dockerfile
│   └── src/main.rs
├── gis-worker/
│   ├── Dockerfile
│   └── src/main.rs
├── analytics-writer/
│   ├── Dockerfile
│   └── src/main.rs
```

**Structure Decision**: The existing monorepo layout (established Sprint 1) is unchanged. Sprint 2 adds the compose override file, Traefik config, and minor Dockerfile/env updates. No new directories or crates.

## Complexity Tracking

Not required — no constitution violations.
