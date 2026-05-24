<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan at
`specs/001-foundation-infrastructure/plan.md`
<!-- SPECKIT END -->

# BorneMap — Agent Instructions

## Project Identity
- BorneMap: geospatial EV-charging discovery platform (Tunisia)
- Speckit v0.8.11 SDD cycle running on OpenCode
- Binding authority: `.specify/memory/constitution.md`
- Context file: `AGENTS.md` (set in `.specify/init-options.json`)

## Speckit Workflow (primary dev commands)
All commands use `speckit.*` prefix:
- `/speckit.specify "..."` — write/fill a spec
- `/speckit.clarify` — resolve `[NEEDS CLARIFICATION]` markers
- `/speckit.plan` — produce plan.md, data-model.md, contracts/, quickstart.md
- `/speckit.tasks` — produce granular ordered task list
- `/speckit.implement` — implement from tasks.md
- `/speckit.analyze` — constitution compliance check per PR
- `/speckit.checklist` — generate constitution checklist
- `/speckit.constitution` — manage constitution amendments
- `/speckit.taskstoissues` — sync tasks to GitHub issues

Git sub-commands: `speckit.git.initialize`, `.feature`, `.validate`, `.remote`, `.commit`

## Git Conventions
- Branch pattern: `NNN-short-name` (e.g., `001-mvp1-geo-core`)
- Speckit auto-commits at lifecycle hooks — expect staged changes at each step

## What the Model Must Never Do

State these at the start of every prompt session:

- Never put business logic in a router file. Routers call service functions only.
- Never put SQL or ORM queries in a service file. Services call repository functions only.
- Never create files in flat `routers/`, `services/`, or `repositories/` directories. Group files by feature domain: `stations/router.py`, `stations/service.py`, `stations/repository.py`.
- Never use `ST_Distance` as a `WHERE` filter. Only `ST_DWithin` filters. `ST_Distance` is for `ORDER BY` only.
- Never use `GEOMETRY` type. Always `GEOGRAPHY(Point, 4326)`.
- Never write `[lat, lon]` in any API payload or PostGIS WKT. Always `[lon, lat]` / `POINT(lon lat)`. Exception: `react-native-maps` `coordinate` prop uses `{latitude, longitude}` because the library requires it.
- Never define `CoordinatePoint` without a `@model_validator` that enforces longitude ∈ [-180,180] and latitude ∈ [-90,90].
- Never use the default Leaflet blue marker or the default React Native Maps red pin.
- Never put the map in a `ScrollView` or give it a fixed pixel height. Always `StyleSheet.absoluteFillObject` on mobile and `h-screen` on web.
- Never use hardcoded hex colors in component files. Always reference design tokens.
- Never run `expo eject` or `expo prebuild`. Stay in Expo Go managed runtime.
- Never create migrations by manually editing the DB. Always `alembic revision --autogenerate` then review before applying.
- Never hand-write HTTP type definitions on the frontend — generate from OpenAPI spec.
- Never frontend filter-after-fetch of server collections.
- MVP development is gated: real-user validation required before next MVP starts.

## Tech Stack (Constitution §Technology)
- **Backend:** Python 3.12+, FastAPI, SQLAlchemy 2.x, Alembic, PostGIS
- **Frontend:** React 18 + Vite + Leaflet + React Query + shadcn/ui
- **Mobile:** React Native + Expo Go (Managed Workflow, no eject)
- **Auth phases:** Mock JWT (HS256) → argon2 → Keycloak OIDC
- **Async (phase 3+):** RabbitMQ via aio-pika, MinIO/S3 via boto3
- **CI tools:** Ruff, Black, pytest, tsc --noEmit

## Reference Docs
- **Implementation plan:** `docs/plan.md` — phased roadmap (6 phases + immediate backlog)
- **Architecture reference:** `docs/architecture.md` — system context, layers, spatial rules, CI/CD
- **Feature spec plan:** `specs/001-foundation-infrastructure/plan.md` — current Phase 0 implementation plan

## Available Skills
- `frontend-design` — production-grade frontend UI
- `react` — React renderer for json-render JSON specs
