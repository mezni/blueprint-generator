# Implementation Plan: Monorepo Foundation

**Branch**: `001-monorepo-foundation` | **Date**: 2026-06-01 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-monorepo-foundation/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Establish a fully compilable monorepo with Rust backend workspace (5 service
skeletons, 4 shared crates), React + Vite frontends (3 web apps), Expo mobile
app, shared TypeScript packages (5+), Docker Compose skeleton with env system,
health endpoints, and CI pipeline.

## Technical Context

**Language/Version**: Rust (latest stable 1.70+), TypeScript 5.x, React 18+,
React Native / Expo SDK 50+

**Primary Dependencies**: Rust workspace with Cargo, Vite, React Router,
React Native Expo, PostCSS/Tailwind (future), Docker Compose v2

**Storage**: N/A — no database work in this sprint

**Testing**: cargo test (Rust), vitest (TypeScript/web), Jest via Expo (mobile)

**Target Platform**: Linux (backend services), modern web browsers (frontend),
iOS + Android (mobile)

**Project Type**: Monorepo with backend services (Rust), web frontends
(React/Vite), mobile app (Expo), shared TypeScript packages, Docker Compose
infrastructure

**Performance Goals**: Rust workspace first build < 5 min (cached builds < 30s),
web app dev server start < 30s, CI pipeline complete < 10 min

**Constraints**: No DB schema, no auth, no GIS, no RabbitMQ, no business APIs,
no UI design, no event processing — per sprint scope

**Scale/Scope**: 5 Rust service crates, 4 Rust shared crates, 3 web apps,
1 mobile app, 5 TypeScript packages, Docker Compose with 4+ infra services

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Data-First, Contract-Driven)**: No data work — compliant
- **Principle II (Strict Service Boundaries)**: Service skeletons follow
  bounded contexts — compliant
- **Principle III (Authorization & Tenant Isolation)**: Not addressed —
  out of scope for this sprint
- **Principle IV (REST-Only, Contract-Driven APIs)**: API contracts package
  with standard envelope — compliant
- **Principle V (Event-Driven, Eventually Consistent)**: Not addressed —
  out of scope for this sprint
- **Tech & Infrastructure Constraints**: All choices (Rust, React+Vite, Expo,
  Docker Compose) match constitution — compliant
- **Development & Operational Workflow**: Architecture-first approach followed;
  no startup order dependencies in this sprint — compliant
- **Governance**: No constitution amendments needed

**Result: PASS** — No violations. Proceeding to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-monorepo-foundation/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Monorepo structure — Rust + TypeScript hybrid
.
├── apps/
│   ├── driver-web/              # React + Vite
│   ├── partner-dashboard/       # React + Vite
│   ├── admin-dashboard/         # React + Vite
│   └── driver-mobile/           # React Native Expo
├── services/
│   ├── driver-service/          # Rust service crate
│   ├── admin-service/           # Rust service crate
│   ├── clickstream-service/     # Rust service crate
│   ├── gis-worker/              # Rust service crate
│   └── analytics-writer/        # Rust service crate
├── crates/
│   ├── common-types/            # Shared Rust types
│   ├── common-errors/           # Shared Rust error types
│   ├── common-auth/             # Auth middleware stub
│   ├── common-db/               # DB abstraction stub
│   └── common-observability/    # Observability foundation
├── packages/
│   ├── shared-types/            # Shared TypeScript types
│   ├── api-client/              # API client package
│   ├── auth-client/             # Auth client package
│   ├── design-tokens/           # Design primitives (empty)
│   └── event-taxonomy/          # Event envelope stub
├── infra/
│   ├── compose/
│   │   └── docker-compose.yml   # Base compose
│   └── env/                     # Per-service env files
├── docs/                        # Project documentation
├── Cargo.toml                   # Rust workspace root
└── package.json                 # Root workspace config
```

**Structure Decision**: Monorepo with separate directories for backend
services (`services/`), Rust shared crates (`crates/`), frontend apps
(`apps/`), and TypeScript packages (`packages/`). Infrastructure config
lives under `infra/`. This follows the constitution's pragmatic
monolith-of-services principle while keeping concerns separated.

## Complexity Tracking

> Not needed — no constitution violations to justify.
