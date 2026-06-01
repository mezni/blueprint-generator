# Implementation Plan: Architecture Contracts

**Branch**: `main` | **Date**: 2026-05-31 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-architecture-contracts/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Define the complete system constitution contract layer for the BorneMap EV
platform before any implementation begins. This includes 8 contract documents
covering: service architecture boundaries, PostgreSQL schema ownership,
clickstream event envelope & types, RBAC model (3 roles), identifier strategy,
communication rules (REST + RabbitMQ), CI/CD pipeline, database schema specs,
observability standards, caching rules, security boundaries, migration
governance, and data lifecycle policies.

## Technical Context

**Language/Version**: Rust (latest stable) for backend services; TypeScript
(React + Vite) for frontend; React Native + Expo for mobile.

**Primary Dependencies**: Axum (Rust HTTP), sqlx (Rust DB), Keycloak (identity),
RabbitMQ (messaging), PostGIS (geospatial), Traefik (reverse proxy).

**Storage**: PostgreSQL 16+ with PostGIS extension — single instance, four
schemas (`inventory`, `users`, `gis`, `analytics`).

**Testing**: `cargo test` for Rust; peer review for contract documents.
No automated contract-testing framework in scope for this feature.

**Target Platform**: Linux (bare metal); Docker Compose for local dev.

**Project Type**: Architecture contract definitions (documentation-heavy with
scaffolded monorepo structure). Produces no runtime code — downstream EPLCs
(Admin Service, Driver Service, etc.) will implement against these contracts.

**Performance Goals**: <500ms p95 for discovery listings; <2s p99 for geo-queries.
Documented as contract targets, not implemented in this feature.

**Constraints**: Per constitution: no cross-service DB access, no new schemas
without approval, exactly 3 roles, Traefik-only public entrypoint,
partner isolation at repository level, soft delete enforced.

**Scale/Scope**: Tunisia-wide deployment (<500 stations, <50K users,
<100K events/day), stateless services, single-instance MVP.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate | Status |
|-----------|------|--------|
| I. Pragmatic Architecture | Feature defines minimum service boundaries (5 services + Keycloak + Traefik) — no fragmentation. | ✅ PASS |
| II. Clear Ownership Boundaries | Data ownership matrix assigns each schema to exactly one owning service. Cross-service DB access explicitly forbidden. | ✅ PASS |
| III. Operational Simplicity | Contracts define Docker Compose + Traefik + GHCR model. No staging environment. Manual deployment only. | ✅ PASS |
| IV. Evolution over Complexity | Contracts document exactly what is in scope for MVP. Out-of-scope items (OCPP, payments, routing) explicitly excluded. | ✅ PASS |
| V. Data Separation | Four PostgreSQL schemas defined with strict ownership. No new schemas without approval. | ✅ PASS |
| Engineering & Quality | Testing, security, observability, and DoD standards reflected in contract docs. | ✅ PASS |

All gates pass. No violations to justify.

## Project Structure

### Documentation (this feature)

```text
specs/001-architecture-contracts/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── architecture-contract.md
│   ├── service-matrix.md
│   ├── event-spec-v1.md
│   ├── rbac-model.md
│   ├── id-strategy.md
│   ├── communication-rules.md
│   ├── ci-cd-contract.md
│   └── database-schema-contract.md
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

This feature produces no runtime code. Contracts live under `specs/001-architecture-contracts/contracts/`. The downstream EPICs will create the actual service directories:

```text
services/
├── admin-service/
├── driver-service/
├── clickstream-service/
└── gis-sync-worker/

crates/
├── auth/
├── config/
├── errors/
├── id-generation/
├── observability/
└── shared-types/

apps/
├── driver-web/
├── partner-dashboard/
├── admin-dashboard/
└── driver-mobile/

packages/
├── design-tokens/
└── ui-components/

docs/
├── constitution.md
├── plan.md
├── tasks.md
└── epic00.md
```

**Structure Decision**: Monorepo with Rust workspace under `services/` + `crates/`,
frontend apps under `apps/`, shared packages under `packages/`. Per constitution
section 7 (Repository Architecture).

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations. Complexity tracking not required.
