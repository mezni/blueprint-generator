# Implementation Plan: Runtime Foundation

**Branch**: `002-runtime-foundation` | **Date**: 2026-06-01 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/002-runtime-foundation/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Bring the platform to a runnable local distributed system. Complete the Docker Compose orchestration with Postgres, RabbitMQ, Keycloak, and Traefik; implement runtime config loading, health/readiness probes, structured JSON logging, DB and RabbitMQ connectivity in all Rust services; provide environment profiles and a smoke test script. Single `docker compose up` must boot 9 healthy containers with no restart loops.

## Technical Context

**Language/Version**: Rust (stable 1.70+, edition 2021)

**Primary Dependencies**: axum 0.7 (HTTP), tokio 1.x (async runtime), serde/serde_json (config parsing), sqlx 0.8 (PostgreSQL), lapin 2.x (RabbitMQ AMQP), tracing 0.1 (structured logging), tracing-subscriber (JSON log output)

**Storage**: PostgreSQL 16 (three databases: users_db, inventory_db, analytics_db), RabbitMQ 4 (queues: clickstream.raw, gis.sync, analytics.ingest)

**Testing**: cargo test (Rust unit/integration), shell-based smoke test (curl-based validation against running containers)

**Target Platform**: Linux (Docker Engine 24+, Docker Compose v2)

**Project Type**: Distributed backend system (multi-service Docker Compose stack with Rust services, PostgreSQL, RabbitMQ, Keycloak, Traefik)

**Performance Goals**: Full stack boots within 120s, health endpoints respond under 500ms, ready endpoints respond under 30s post-dependency-availability

**Constraints**: No TLS for internal communication in this sprint; management UIs accessible on dev profiles only; no auth enforcement; secrets never committed; services crash on missing required env vars

**Scale/Scope**: 5 Rust services, 4 infrastructure containers, 1 Traefik router, single-node Docker Compose deployment

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Data-First, Contract-Driven)**: This sprint establishes DB connectivity plumbing but defines no business data schemas — compliant
- **Principle II (Strict Service Boundaries)**: Service boundaries unchanged from Sprint 1 — each service connects only to its designated database per docs/db.md — compliant
- **Principle III (Authorization & Tenant Isolation)**: Explicitly out of scope per sprint spec — compliant
- **Principle IV (REST-Only, Contract-Driven APIs)**: Health and readiness endpoints follow a standard JSON envelope pattern — compliant
- **Principle V (Event-Driven, Eventually Consistent)**: RabbitMQ connectivity foundation is established; queue topology matches the event taxonomy from Sprint 1 — compliant
- **Tech & Infrastructure Constraints**: All choices (Rust, axum, PostgreSQL, RabbitMQ, Keycloak, Traefik, Docker Compose) match constitution — compliant
- **Development & Operational Workflow**: Startup order enforced (PostgreSQL → RabbitMQ → Keycloak → Traefik → Services), health checks per constitution — compliant
- **Governance**: No constitution amendments needed

**Result: PASS** — No violations. Proceeding to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/002-runtime-foundation/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── api-envelope.md
│   ├── health-endpoint.md
│   └── ready-endpoint.md
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
# Monorepo structure (established in Sprint 1, extended in Sprint 2)
.
├── infra/
│   ├── compose/
│   │   └── docker-compose.yml    # Updated with full infra + health checks + volumes
│   ├── postgres/
│   │   └── init/                 # DB initialization scripts
│   │       ├── 01-create-users-db.sql
│   │       ├── 02-create-inventory-db.sql
│   │       └── 03-create-analytics-db.sql
│   ├── rabbitmq/
│   │   └── init/                 # Queue/Exchange declarations
│   │       └── 01-declare-queues.sh
│   ├── keycloak/
│   │   └── realm-export/         # Realm configuration
│   │       └── ev-platform-realm.json
│   └── env/                      # Environment profiles
│       ├── .env.example
│       ├── local/                # local profile configs
│       │   ├── postgres.env
│       │   ├── rabbitmq.env
│       │   ├── keycloak.env
│       │   ├── traefik.env
│       │   ├── driver-service.env
│       │   ├── admin-service.env
│       │   ├── clickstream-service.env
│       │   ├── gis-worker.env
│       │   └── analytics-writer.env
│       └── docker/               # docker profile configs
│           └── ...
├── crates/
│   └── common-observability/     # Extended with tracing/logging init
├── services/
│   ├── driver-service/           # + config loader, DB conn, health/ready, JSON logging
│   ├── admin-service/            # + config loader, DB conn, health/ready, JSON logging
│   ├── clickstream-service/      # + config loader, RMQ conn, health/ready, JSON logging
│   ├── gis-worker/               # + config loader, RMQ conn, health/ready, JSON logging
│   └── analytics-writer/         # + config loader, DB+RMQ conn, health/ready, JSON logging
├── scripts/
│   └── smoke-test.sh             # Runtime smoke test
└── docs/
    ├── WORKSPACE_CONVENTIONS.md
    └── ...
```

**Structure Decision**: Follows the Sprint 1 monorepo layout. This sprint adds infrastructure initialization files under `infra/`, extends existing service `src/main.rs` with runtime capabilities, adds a shared observability crate for logging initialization, and creates a smoke test script at `scripts/smoke-test.sh`.

## Complexity Tracking

No constitution violations to justify.

