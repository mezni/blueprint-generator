# Implementation Plan: Monorepo Bootstrap

**Branch**: `002-monorepo-bootstrap` | **Date**: 2026-05-31 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/002-monorepo-bootstrap/spec.md`

## Summary

Bootstrap a Rust + TypeScript monorepo containing 4 backend services (Cargo workspace), 3 web apps (Vite+React), 1 mobile app (Expo), 6 shared Rust crates, 4 shared TypeScript packages, Docker scaffolding, and a root Makefile for build/lint/test/format — all compilable with a single command. Zero runtime business logic.

## Technical Context

**Language/Version**: Rust stable (latest), Node.js 20+, TypeScript 5+

**Primary Dependencies**: Cargo workspace, Vite, React 18, Expo SDK 50+, npm workspaces

**Storage**: N/A (scaffolding only — no data layer)

**Testing**: `cargo test --workspace`, `tsc --noEmit`, `expo doctor`

**Target Platform**: Linux server (services), Web (web apps), iOS/Android (mobile app)

**Project Type**: Monorepo — backend services + web apps + mobile app + shared libraries

**Performance Goals**: N/A (scaffolding only)

**Constraints**: Single-command build (`make build-all`); no duplicate DTOs outside contracts crate; no runtime business logic; all API routes MUST use `/api/v1` prefix per api-versioning-contract.md

**Scale/Scope**: 4 services, 6 crates, 3 web apps, 1 mobile app, 4 packages

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Rationale |
|-----------|--------|-----------|
| I. Pragmatic Architecture | ✅ PASS | 4 services are the minimum mandated by EPIC 0 architecture; no fragmentation |
| II. Clear Ownership Boundaries | ✅ PASS | contracts crate = single source of truth for cross-service types; each service/crate is an isolated Cargo package |
| III. Operational Simplicity | ✅ PASS | Docker placeholders only; no staging infra; matches bare-metal/Docker Compose model |
| IV. Evolution over Complexity | ⚠ SEE TRACKING | Justification required for 6 crates + 4 packages (see Complexity Tracking below) |
| V. Data Separation | ✅ PASS | No data layer in this epic |

**GATE RESULT**: PASS with complexity tracking note.

### Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| 6 shared crates vs. 1 | Clean Architecture requires 4 distinct layers (domain, application, infrastructure, interfaces) per service — common crates factor out shared concerns across all services | Single crate would violate layer separation and force cross-cutting dependency chains; EPIC 0 architecture mandates these boundaries |
| 4 frontend packages vs. 1 | design-system (UI), api-client (REST), auth-client (OAuth), analytics-client (events) have zero overlap and distinct dependency trees | Monolithic package would couple UI work to API/auth changes; three web apps need independent versioning |
| 4 services vs. fewer | Per constitution System Architecture section, 5 runtime services (including Keycloak) are the minimum; this epic scaffolds 4 of the 5 (no Keycloak — external dependency) | Merging services would violate principle I's own mandate for clear domain boundaries; services map 1:1 to EPIC 0 schemas |

## Project Structure

### Documentation (this feature)

```text
specs/002-monorepo-bootstrap/
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
├── apps/                    # Frontend applications
│   ├── driver-web/               # Vite + React — driver portal
│   ├── partner-dashboard/        # Vite + React — partner dashboard
│   ├── admin-dashboard/          # Vite + React — admin dashboard
│   └── driver-mobile/            # React Native + Expo — mobile app
├── services/                 # Rust backend services (workspace members)
│   ├── admin-service/             # Cargo package — inventory CRUD
│   ├── driver-service/            # Cargo package — station discovery, reviews
│   ├── clickstream-service/       # Cargo package — event ingestion
│   └── gis-sync-worker/           # Cargo package — GIS enrichment worker
├── crates/                   # Shared Rust libraries (workspace members)
│   ├── contracts/                 # Cross-service DTOs, events, enums, IDs
│   ├── common-auth/               # Auth/authorization utilities
│   ├── common-config/             # Configuration loading
│   ├── common-db/                 # DB connection pool, migrations
│   ├── common-errors/             # Shared error types
│   └── common-types/              # Shared domain types
├── packages/                 # Shared TypeScript packages (npm workspaces)
│   ├── design-system/             # UI components, design tokens
│   ├── api-client/                # Typed REST client
│   ├── analytics-client/          # Clickstream event emitter
│   └── auth-client/               # OAuth token management
├── infra/                    # Infrastructure scaffolding
│   ├── docker/                    # Placeholder Dockerfiles per service
│   └── compose/                   # docker-compose.dev.yml placeholder
├── scripts/                  # Helper scripts
├── docs/                     # Project documentation
├── .github/                  # CI/CD workflows (EPIC 3)
├── Cargo.toml                # Root workspace manifest
├── Makefile                  # Build/lint/test/format targets
├── package.json              # Root npm workspace manifest
└── tsconfig.base.json        # Shared TypeScript config
```

**Structure Decision**: Multi-language monorepo with separate top-level directories per concern type (apps, services, crates, packages, infra). Single Cargo workspace for all Rust code; npm workspaces for all TypeScript code. Matches EPIC 01 specification exactly.
