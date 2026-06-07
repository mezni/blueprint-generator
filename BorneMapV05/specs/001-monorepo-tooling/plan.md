# Implementation Plan: Monorepo + Tooling Foundation

**Branch**: `001-monorepo-tooling` | **Date**: 2026-06-02 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-monorepo-tooling/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Set up the Bornemap monorepo engineering skeleton: Rust workspace (5 service binaries + 4 shared library crates), TypeScript workspace (3 React+Vite web apps + Expo mobile app + 6 shared packages), and infrastructure scaffolding (Docker Compose skeleton + Traefik config). All targets compile and produce empty shells. Shared contracts (event taxonomy, API envelopes) are defined as compilable types.

## Technical Context

**Language/Version**: Rust edition 2024 (stable toolchain), Node.js 22 LTS

**Primary Dependencies**:
- Backend: Rust workspace (cargo, no external framework stubs yet)
- Frontend: React 19, Vite 6, React Native Expo SDK 52
- Shared packages: None external (all internal first-party packages)
- Infra: Docker Compose v2, Traefik v3

**Storage**: Not applicable (build-tooling sprint; no runtime storage yet)

**Testing**: Not applicable (no business logic in Sprint 1 — build verification is the acceptance gate)

**Target Platform**: Linux developer machine (Ubuntu 24.04 / Debian); frontend dev servers on localhost

**Project Type**: Multi-project monorepo (Rust workspace + npm workspaces + Docker Compose)

**Performance Goals**: Full workspace build ≤5 minutes (single developer machine); incremental build ≤30 seconds

**Constraints**:
- Rust edition 2024, Node.js 22 LTS (pinned in `.nvmrc`)
- npm workspaces (root `package.json`)
- Kebab-case naming throughout
- No hardcoded paths; all inter-package references via workspace protocol
- All crate/app/package names match their directory names exactly

**Scale/Scope**:
- 5 Rust service binaries: `driver-service`, `admin-service`, `clickstream-service`, `gis-worker`, `analytics-writer`
- 4 Rust library crates: `common-types`, `common-errors`, `common-auth`, `common-db`
- 3 React+Vite apps: `driver-web`, `partner-dashboard`, `admin-dashboard`
- 1 Expo app: `driver-mobile`
- 6 TypeScript packages: `shared-types`, `api-client`, `auth-client`, `design-tokens`, `event-taxonomy`, `api-contracts`
- 1 Docker Compose skeleton + 1 Traefik config + 8 `.env.example` files

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Principle II — Strict Domain & Service Separation**: Sprint 1 creates the physical monorepo structure enforcing service boundaries. Each service gets its own crate, each app its own directory. No cross-boundary coupling in the file layout. ✅ Pass.

**Principle IV — Contract-Driven REST APIs**: Sprint 1 creates the `api-contracts` and `event-taxonomy` shared packages with compile-time type definitions for envelopes, error codes, and event schemas. This directly implements the contract-first mandate. ✅ Pass.

**Principle VII — Verification Discipline**: Sprint 1's acceptance gates are compilation-based (`cargo build`, `npm run build`, `docker compose config`). No runtime tests are needed since no business logic exists yet. Build success is the verification signal. ✅ Pass.

**Technology & Infrastructure Constraints**: Sprint 1 uses the exact stack mandated by the Constitution (Rust backend, React+Vite frontend, Expo mobile, Docker Compose, Traefik). No deviations. ✅ Pass.

**No violations. Complexity Tracking is not needed.**

## Project Structure

### Documentation (this feature)

```text
specs/001-monorepo-tooling/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 research (tool version rationale)
├── data-model.md        # Phase 1 data model (shared contract schemas)
├── quickstart.md        # Phase 1 quickstart guide
├── contracts/           # Phase 1 interface contracts
│   ├── api-envelope.json
│   ├── error-codes.json
│   └── event-taxonomy.json
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Future (/speckit.tasks command)
```

### Source Code (repository root)

```text
borNEMap/
│
├── apps/
│   ├── driver-web/           # React + Vite
│   ├── partner-dashboard/    # React + Vite
│   ├── admin-dashboard/      # React + Vite
│   └── driver-mobile/        # React Native Expo
│
├── services/
│   ├── driver-service/       # Rust binary
│   ├── admin-service/        # Rust binary
│   ├── clickstream-service/  # Rust binary
│   ├── gis-worker/           # Rust binary
│   └── analytics-writer/     # Rust binary
│
├── crates/
│   ├── common-types/         # Rust library
│   ├── common-errors/        # Rust library
│   ├── common-auth/          # Rust library (stub)
│   └── common-db/            # Rust library (stub)
│
├── packages/
│   ├── shared-types/         # TS package
│   ├── api-client/           # TS package (base client)
│   ├── auth-client/          # TS package (stub)
│   ├── design-tokens/        # TS package (empty)
│   ├── event-taxonomy/       # TS package + Rust types
│   └── api-contracts/        # TS package + Rust types
│
├── infra/
│   ├── compose/
│   │   ├── docker-compose.yml
│   │   ├── docker-compose.override.yml
│   │   └── traefik/
│   │       ├── traefik.yml
│   │       └── dynamic/
│   └── env/
│       ├── traefik.env.example
│       ├── keycloak.env.example
│       ├── platform-db.env.example
│       ├── analytics-db.env.example
│       ├── rabbitmq.env.example
│       ├── driver-service.env.example
│       ├── admin-service.env.example
│       └── analytics.env.example
│
├── docs/
│   └── WORKSPACE_CONVENTIONS.md
│
├── Cargo.toml               # Root Rust workspace
├── package.json             # Root npm workspace
├── .nvmrc                   # Node.js version pin
└── tsconfig.base.json       # Shared TS base config
```

**Structure Decision**: Monorepo with separate top-level directories per concern (`services/`, `crates/`, `apps/`, `packages/`, `infra/`). This matches the Constitution's "Pragmatic Monolith-of-Services" principle — independent but tightly governed, with shared contracts as the bridge between backend and frontend.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations. This section left empty intentionally.

---

## Phase 0 — Research

### Unknowns from Technical Context

The Technical Context has zero `NEEDS CLARIFICATION` markers — all technology choices are settled in the spec and constitution:

- **Rust edition 2024**: Clarified via `/speckit.clarify` (Q1)
- **Node.js 22 LTS**: Clarified via `/speckit.clarify` (Q1)
- **npm workspaces**: Clarified via `/speckit.specify` (FR-013)

No additional research is needed. The build tooling choices are standard, well-documented, and don't require investigation. `research.md` will document the rationale for each decision.

### Research Output

Writing `research.md` with decision records for each resolved choice.

---

## Phase 1 — Design & Contracts

### Data Model

The data model for Sprint 1 is structural — it defines the shared type schemas that cross the backend/frontend boundary. Key entities are:

1. **API Envelope** — standard success/error response shapes
2. **Error Codes** — canonical error code enum
3. **Event Taxonomy** — event envelope + event name catalog
4. **Common Types** — ID format, role enum, status enums

### Interface Contracts

Three JSON contract files will define the canonical interface shapes:

1. `contracts/api-envelope.json` — Success/Error envelopes + pagination meta
2. `contracts/error-codes.json` — Canonical error codes enum
3. `contracts/event-taxonomy.json` — Event envelope schema + event catalog

### Quickstart

A `quickstart.md` will guide a new developer from clone to compiling all targets.

### Agent Context

The plan reference in `AGENTS.md` will be updated to point to this file.
