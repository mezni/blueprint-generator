# Research: Runtime Infrastructure & API Gateway

**Phase**: 0 — Research & Resolution
**Date**: 2026-05-31

## Methodology

All decisions are drawn from the feature specification (spec.md), EPIC 2 document (docs/epic02.md), and project constitution. No external research needed — the spec is complete and unambiguous.

## Design Decisions

### Decision 1: Compose File Strategy

- **Decision**: Three-file Compose strategy: base (`docker-compose.yml`) + dev override (`docker-compose.dev.yml`) + prod override (`docker-compose.prod.yml`)
- **Rationale**: Base file defines all services, networks, and volumes. Dev override enables hot-reload, debug ports, and relaxed resource limits. Prod override adds resource constraints, TLS certs, and persistent volume mappings. This pattern is the Docker-recommended multi-file approach.
- **Alternatives considered**:
  - Single monolithic file — rejected: would require conditional sections for dev/prod differences, harder to maintain
  - Separate files per service — rejected: adds orchestration complexity without benefit for 11 co-deployed services

### Decision 2: Traefik Configuration Mode

- **Decision**: File-based dynamic configuration (`infra/traefik/dynamic.yml`) rather than Docker labels
- **Rationale**: File-based config provides a single routing truth table that can be reviewed, validated with `traefik config check`, and version-controlled independently of service metadata. Docker labels work for simple cases but scatter routing rules across individual Compose service definitions.
- **Alternatives considered**:
  - Docker labels on each service — rejected: routing rules embedded in 11 separate Compose blocks, harder to audit

### Decision 3: Health Check Strategy

- **Decision**: Docker Compose `healthcheck` directives per service + Traefik passive health checks for routing eligibility
- **Rationale**: Docker healthchecks control container lifecycle (startup sequence, restart policy). Traefik healthchecks control traffic routing. Separation of concerns keeps each layer focused.
- **Alternatives considered**:
  - Traefik-only healthchecks — rejected: doesn't control container startup order
  - Docker-only healthchecks — rejected: doesn't inform gateway routing decisions

### Decision 4: CI Pipeline Architecture

- **Decision**: Multi-job parallel pipeline (lint, test, build, contract validation, Docker build, GHCR publish) with job-level `needs` constraints
- **Rationale**: Parallelizes independent stages (lint doesn't need test to complete, build doesn't need lint), while enforcing the constitution-mandated order: lint → test → build → contract → Docker → publish. Docker build and publish are main-branch-only.
- **Alternatives considered**:
  - Sequential single-job pipeline — rejected: would not meet 15-minute CI target (SC-004)
  - All-in-one job — rejected: single failure blocks all output, hard to interpret

### Decision 5: Volume Persistence Strategy

- **Decision**: Named volumes for PostgreSQL (persist data across restarts); ephemeral for dev RabbitMQ; production uses named volumes for both
- **Rationale**: Database state must survive restarts for practical development. Ephemeral queues in dev prevent stale message accumulation. Production requires persistent queues for at-least-once delivery guarantee.
- **Alternatives considered**:
  - Ephemeral for all — rejected: losing database state on every restart breaks development workflow
  - Persistent for all — rejected: stale RabbitMQ messages accumulate in dev, cause confusion
