# Implementation Plan: Foundation & Infrastructure Loop

**Branch**: `003-foundation-infrastructure` | **Date**: 2026-05-24 | **Spec**: spec.md

**Input**: Feature specification from `specs/001-foundation-infrastructure/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.

## Summary

Phase 0 establishes the rapid validation ecosystem: FastAPI backend + PostGIS
database via Docker Compose, React/Vite admin portal with CartoDB Positron map,
Expo Go mobile placeholder, and CI pipelines enforcing Ruff/Black/tsc on every PR.

## Technical Context

**Language/Version**: Python 3.12+ (backend), TypeScript 5+ (frontend), React 18,
Expo SDK 51 (mobile)

**Primary Dependencies**:
- Backend: FastAPI, uvicorn, SQLAlchemy 2.x (async), asyncpg, geoalchemy2, shapely,
  alembic, pydantic-settings
- Web: React 18, Vite 5, Leaflet + react-leaflet, Tailwind CSS + shadcn/ui,
  @tanstack/react-query v5
- Mobile: Expo SDK 51, react-native-maps, @gorhom/bottom-sheet, expo-haptics,
  expo-location
- Dev: Ruff, Black, pytest, httpx, ESLint, Prettier

**Storage**: PostgreSQL 16 + PostGIS 3.4 (local Docker container)

**Testing**: pytest + httpx (backend integration tests), Jest/Vitest (frontend), CI runs
 against real PostGIS container

**Target Platform**: Linux (backend Docker), modern browsers (web admin),
iOS 15+ / Android 9+ (mobile via Expo Go)

**Project Type**: Web application + mobile app + backend API (monorepo)

**Performance Goals**: Health endpoints < 50 ms. Admin portal loads in < 5 s.
CI pipeline completes in < 10 min.

**Constraints**: Expo Go managed runtime only (no eject). PostGIS in Docker only.
GitHub Actions for CI. CartoDB Positron tiles (OSM fallback).

**Scale/Scope**: Phase 0 — development infrastructure only. No business logic,
no data models, no CRUD.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] §6 Feature-scoped directory structure: Phase 0 creates `backend/app/stations/`,
      `backend/app/health/` placeholders.
- [x] §9 Frontend: Admin scaffold uses design tokens, floating panels, green markers.
- [x] §10 Mobile: Expo Go placeholder confirms managed runtime.
- [x] §11 CI/CD: Ruff/Black + tsc pipelines enforced on every PR.
- [x] §4 Architecture: FastAPI backend, PostGIS, React/Vite, Expo Go.
- [ ] §8 Spatial: GiST index / GEOGRAPHY standards — not yet applicable (no data).
- [ ] §8 Coordinate [lon, lat]: Not yet applicable (no spatial endpoints in Phase 0).

## Project Structure

### Documentation (this feature)

```text
specs/001-foundation-infrastructure/
├── plan.md              # This file
├── research.md          # Technology & approach research
├── data-model.md        # No data entities in this phase
├── quickstart.md        # Setup guide for new developers
├── contracts/           # Health API contract
├── checklists/
│   └── requirements.md  # Specification quality checklist
└── tasks.md             # Created by /speckit.tasks
```

### Source Code (repository root)

```text
bornemap/
├── backend/
│   ├── app/
│   │   ├── main.py              # FastAPI app entry point
│   │   ├── core/
│   │   │   ├── config.py        # pydantic-settings BaseSettings
│   │   │   ├── database.py      # SQLAlchemy async engine + session
│   │   │   └── dependencies.py  # Depends() factories
│   │   ├── health/
│   │   │   └── router.py        # /health/live, /health/ready
│   │   └── stations/            # Feature-scoped (empty placeholder for Phase 1)
│   │       ├── models.py
│   │       ├── schemas.py
│   │       ├── repository.py
│   │       ├── service.py
│   │       └── router.py
│   ├── migrations/
│   │   └── versions/
│   │       └── 001_create_stations.py  # Initial alembic baseline
│   ├── tests/
│   │   ├── conftest.py
│   │   └── test_health.py
│   ├── alembic.ini
│   ├── pyproject.toml
│   └── Dockerfile

├── web/
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   ├── components/
│   │   │   ├── ui/              # shadcn/ui generated
│   │   │   └── map/             # Leaflet map wrapper
│   │   ├── features/
│   │   │   └── stations/        # Empty placeholder for Phase 1
│   │   ├── lib/api.ts           # API client stub
│   │   └── styles/globals.css   # Tailwind + design tokens
│   ├── index.html
│   ├── vite.config.ts
│   ├── tailwind.config.ts
│   ├── tsconfig.json
│   └── package.json

└── mobile/
    ├── app/
    │   ├── (tabs)/
    │   │   ├── index.tsx        # Map tab placeholder
    │   │   └── favorites.tsx    # Favorites tab placeholder
    │   └── _layout.tsx          # Tab layout
    ├── components/
    │   ├── map/                 # Empty placeholder for Phase 2
    │   └── sheets/              # Empty placeholder for Phase 2
    ├── lib/api.ts               # API client stub
    ├── app.json
    └── package.json

docker-compose.yml            # PostGIS + backend services
.github/workflows/
├── backend.yml               # Ruff, Black, pytest
└── frontend.yml              # tsc, build
```

## Complexity Tracking

> No constitution violations — Complexity Tracking not needed.
