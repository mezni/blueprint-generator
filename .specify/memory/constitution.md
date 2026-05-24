<!--
Sync Impact Report
==================
Version change: 5.0.0 → 1.0.0
Bump rationale: Full reset — all version numbers returned to 1.0.0 as a clean
                starting point. Constitution rewritten and consolidated from 12
                principles + 3 supporting sections into 14 compact sections
                covering the same scope.

Modified sections:
  - All sections reorganized from numbered principles I–XII + supporting sections
    → 14 flat sections.

Added sections:
  - Section 1: Mission Statement
  - Section 13: Scaling Rule
  - Section 14: Core Principle

Removed sections (content absorbed):
  - Standalone "Technology Stack & Constraints" → absorbed into Section 4
  - Standalone "Development Workflow & Quality Gates" → absorbed into relevant sections
  - Standalone "Governance" → retained

Templates requiring updates:
  ✅ .specify/templates/plan-template.md
  ✅ .specify/templates/spec-template.md
  ✅ .specify/templates/tasks-template.md
  ✅ .specify/templates/constitution-template.md
  ✅ .specify/templates/checklist-template.md
  ✅ .opencode/commands/*.md

Follow-up TODOs: none
-->

# BorneMap Constitution

> **Canonical Speckit constitution.** Every `/speckit.*` command and every implementer
> (human or LLM) MUST treat this file as the binding source of truth. In any conflict,
> **this file wins**.

**Version**: 1.0.0 | **Status**: Approved | **Mode**: Validation-First Spatial Platform

## 1. Mission Statement

BorneMap is a geospatial EV charging discovery platform for Tunisia. Its purpose is
to validate and deliver a premium spatial discovery experience for EV charging
infrastructure while maintaining strict execution speed.

The primary optimization objective is fast product validation through rapid iteration.

**System Access Profiles:**

- **Public Users** — Discover charging stations, visualize infrastructure on an
  interactive map, and inspect station metadata.
- **Registered Drivers** — Personalize experience by saving favorites and reviewing
  operational station locations.
- **Administrators** — Manage physical infrastructure data nodes and maintain strict
  spatial accuracy.

## 2. Product Philosophy — "Validation Before Optimization"

Product decisions SHALL prioritize:

- Feature validation over system architecture sophistication.
- Direct client feedback over theoretical scaling patterns.
- UX iteration speed over premature optimization.
- Implementation simplicity across all software layers.

> System optimization, language migration, or infrastructural scaling occurs only
> after validated user demand is measured and proven.

## 3. Non-Goals (Validation Boundaries)

To ensure maximum speed, the platform SHALL NOT include during the validation phase:

- OCPP integration, charging session signaling, or direct hardware communications.
- Native billing, invoicing, or payment processing workflows.
- Smart charging optimization or grid load balancing telemetry.
- Real-time hardware status metrics or continuous charger telemetry tracking.
- Distributed microservices, service meshes, or event-driven distributed streaming.
- Native mobile compilation pipelines or customized native modules before validation.
- Infrastructure autoscaling policies or advanced distributed tracing stacks.

## 4. Validation Architecture

### 4.1 Backend Engine

| Property | Value |
|----------|-------|
| Language Runtime | Python 3.12+ |
| Framework | FastAPI |
| Strategic Purpose | Maximum implementation velocity, contract agility, compile-free hot reloading, and deployment simplicity |

### 4.2 Database Layer

| Property | Value |
|----------|-------|
| Primary Engine | PostgreSQL + PostGIS |
| Spatial Column Type | `GEOGRAPHY(Point, 4326)` |
| Strategic Note | Spatial persistence and PostGIS indexing topologies remain stable across all delivery phases — this is a permanent architectural foundation |

### 4.3 Frontend Topology

| Client | Stack |
|--------|-------|
| Web Admin Portal | React · Vite · Tailwind CSS · shadcn/ui · Leaflet |
| Mobile Driver Client | React Native · Expo Go managed runtime |

## 5. Migration Principle — The Rust Escape Hatch

The backend implementation language is strictly provisional. Migration to Rust is
permitted if, and only if:

- Measurable performance constraints emerge under production workloads.
- Core operational bottlenecks are proven through profiling telemetry.
- Product feature validation is fully finalized.

**Migration Isolation Constraints:**

Any future language migration MUST preserve:

- The canonical API contracts (OpenAPI definitions generated from Pydantic schemas).
- The underlying PostgreSQL/PostGIS database schema layout.
- Absolute frontend, web, and mobile app protocol compatibility.

## 6. Clean Architecture Rules

Validation speed SHALL NOT justify architectural disorder or code rot. The dependency
topology is explicitly unidirectional:

```text
API Layer → Service Layer → Repository Layer → Database
```

> Strict Constraint: Reverse dependencies or layer-skipping calls are strictly
> forbidden. The API layer never imports from the Repository.

**File Organization:** Source code MUST be grouped by **feature domain**, not by
technical type. Every feature domain gets its own package with models, schemas,
repository, service, and router files. Flat `routers/`, `services/`, or `repositories/`
directories containing files from multiple domains are forbidden.

```
# CORRECT — feature-scoped
stations/router.py
stations/service.py
stations/repository.py
stations/schemas.py
stations/models.py

# WRONG — type-scoped (flat directory by type)
routers/stations_router.py
services/stations_service.py
repositories/stations_repository.py
```

## 7. Authentication Strategy

| Phase | Strategy |
|-------|----------|
| Phase 1–2 | Mock Authentication — locally generated, unverified JWT structures passed via request headers. Payloads must supply `sub`, `username`, `roles`, `iat`, `exp`. |
| Phase 3–4 | Internal Identity System — authoritative user lookup, credential storage in a secure state-managed database boundary. |
| Phase 5+ | External Identity Integration — Keycloak, enforcing standard OAuth2 Authorization Code Flow with PKCE protection. |

## 8. Spatial Engineering Standards

- **Canonical Coordinate Format:** All spatial endpoints, payloads, coordinates, and
  models SHALL express positions as `[longitude, latitude]`.
- **Database Performance:** All geospatial columns require explicit GiST index bindings.
- **Spatial Filtering:** Viewport lookups MUST use `ST_DWithin` as the indexed
  filtering predicate.
- **ST_Distance:** Restricted to secondary execution scopes for ranking/sorting only.
  Never in `WHERE` clauses.
- **Schema Evolution:** All data mutations must track sequentially via Alembic
  migration files.

## 9. Frontend Experience Standards

BorneMap SHALL provide a modern, spatial-first user interface optimizing map real
estate.

| Principle | Specification |
|-----------|---------------|
| Map-First Dominance | The map is the application canvas. All secondary metadata cards, search bars, and controls float over the map layer with clean drop-shadows. |
| Modern Minimalism | No heavy borders, dense grid outlines, or dated table layouts. Whitespace, consistent typography, `rounded-xl` / `rounded-2xl` border radii. |
| Micro-interactions | Sheets, drawers, and overlays must snap gracefully. Spring physics or crisp easing (`transition-all duration-200 ease-out`). |
| Premium Accents | High-contrast electric green (`#22c55e`) accent colors for charger indicators against clean Positron map tiles. |

## 10. Mobile Runtime Strategy

The mobile client SHALL use the **Expo Go managed workflow** exclusively during
validation.

Forbidden before complete validation:

- Custom native iOS/Android code modules.
- Running native build scripts.
- `npx expo eject`.

The application must remain pure TypeScript within the standard managed Expo framework.

## 11. CI/CD Requirements

Automated verification pipelines are mandatory from day one and must be enforced on
protected main branches before merge clearance.

- **Backend Pipeline** — Ruff/Black formatting analysis, syntax linting, test suite
  execution, app initialization check loops.
- **Frontend Pipeline** — Dynamic style checking, strict TypeScript type checking
  (`tsc`), compiler build target verification.

## 12. Validation Observability

Geospatial exploration and endpoint activity tracking must remain lightweight to
ensure code velocity.

**Included:**

- Structured JSON application logging.
- Multi-stage container health handshakes (`/health/live`, `/health/ready`).
- Inbound request correlation tracking IDs (`X-Request-ID`).

**Deferred:**

- Metrics aggregation systems.
- Central trace scraping arrays.
- Distributed tracing infrastructure.

## 13. Scaling Rule

The platform SHALL remain Python-based until explicit, empirical metrics suggest a
performance bottleneck cannot be optimized via basic query refinement or caching.
Premature optimization is treated as a foundational system failure.

## 14. Core Principle

BorneMap optimizes for validated learning, rapid client feedback, and execution speed
over implementation sophistication.

## Governance

This constitution supersedes every other practice, style guide, or convention in the
repository.

**Amendment procedure:**

1. Open a PR titled `constitution: <change>` modifying
   `.specify/memory/constitution.md`.
2. Include a `Rationale` section in the PR description referencing affected sections
   by number/name.
3. Bump version per SemVer:
   - **MAJOR**: backward-incompatible governance or principle changes.
   - **MINOR**: new section or materially expanded guidance.
   - **PATCH**: clarifications, wording, typo fixes, non-semantic refinements.
4. Provide a migration note for any in-flight MVP whose scope or acceptance criteria
   are affected.
5. Require explicit approval from the platform owner of record.
6. Update the footer line (Version / Ratified / Last Amended).

**Compliance:**

- `/speckit.analyze` runs the constitution gate for every PR.
- Any violation must either be fixed before merge or accompanied by a Complexity
  Tracking entry in the feature's `plan.md` justifying the deviation.

**Version**: 1.0.0 | **Ratified**: 2026-05-24 | **Last Amended**: 2026-05-24
