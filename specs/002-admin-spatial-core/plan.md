# Implementation Plan: Admin Spatial Core Validation

**Branch**: `004-admin-spatial-core` | **Date**: 2026-05-25 | **Spec**: specs/002-admin-spatial-core/spec.md

**Input**: Feature specification from `specs/002-admin-spatial-core/spec.md`

## Summary

Phase 1 validates administrative spatial data management: station CRUD via interactive map with custom green markers, spatial search (viewport filtering + 20 km radius ST_DWithin queries), mock JWT auth for admin actions, and a complementary stations data table view. Built on the Phase 0 foundation (FastAPI backend, PostGIS, React/Vite admin portal).

## Technical Context

**Language/Version**: Python 3.12+ (backend), TypeScript 5+ (frontend), React 18

**Primary Dependencies**:
- Backend: FastAPI, uvicorn, SQLAlchemy 2.x (async), asyncpg, geoalchemy2, shapely, alembic, pydantic-settings, PyJWT
- Web: React 18, Vite 5, Leaflet + react-leaflet, Tailwind CSS + shadcn/ui, @tanstack/react-query v5
- Dev: Ruff, Black, pytest, httpx, Vitest

**Storage**: PostgreSQL 16 + PostGIS 3.4 (local Docker container). Stations table with GEOGRAPHY(Point, 4326) column and GiST index.

**Testing**: pytest + httpx (backend integration), Vitest (frontend), CI runs against real PostGIS container

**Target Platform**: Modern browsers (Chrome, Firefox, Safari) for admin portal

**Project Type**: Web application (admin portal) + backend API (monorepo)

**Performance Goals**: Station save-to-marker under 3 seconds. Nearby search under 2 seconds. Map at 60 FPS during pan/zoom.

**Constraints**: Coordinates MUST be [longitude, latitude] wire format. ST_DWithin for spatial filtering only. Mock JWT auth (HS256). Map is full-screen, panels float. Custom green markers only — no default blue Leaflet pins. No hardcoded hex colors in components.

**Scale/Scope**: Under 500 stations. Admin-only authentication. Tunisia-focused geography. Mobile driver features explicitly out of scope.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] §4 Validation Architecture: FastAPI + PostGIS + React/Vite + Leaflet — matches constitution
- [x] §6 Clean Architecture: Feature-scoped directories (`stations/`, `users/`) already in place from Phase 0
- [x] §7 Authentication Strategy: Phase 1–2 Mock JWT — matches spec (US3)
- [x] §8 Spatial Standards: GEOGRAPHY(Point, 4326), ST_DWithin filtering, ST_Distance for ORDER BY, GiST index, [lon, lat] wire format
- [x] §9 Frontend Standards: Map-first, floating panels, custom green markers, design tokens, rounded-2xl
- [x] §11 CI/CD: Pipeline already established in Phase 0, extended for Phase 1 tests
- [x] §13 Scaling Rule: <500 stations — no premature optimization needed

## Project Structure

### Documentation (this feature)

```text
specs/002-admin-spatial-core/
├── plan.md              # This file
├── research.md          # Technology & approach research
├── data-model.md        # Station spatial data model
├── quickstart.md        # Phase 1 setup guide
├── contracts/           # API contracts (OpenAPI)
└── tasks.md             # Created by /speckit.tasks
```

### Source Code (repository root)

```text
bornemap/
├── backend/
│   ├── app/
│   │   ├── main.py              # Register spatial routes, mock auth middleware
│   │   ├── core/
│   │   │   ├── config.py        # Add JWT_SECRET, SPATIAL_CONFIG
│   │   │   ├── database.py      # Existing async engine + Base
│   │   │   └── dependencies.py  # Add get_current_user (mock auth)
│   │   ├── health/              # Existing health endpoints
│   │   ├── users/               # Existing user management
│   │   ├── partners/            # Existing partner CRUD
│   │   ├── stations/            # EXTEND: spatial queries, nearby endpoint
│   │   │   ├── models.py        # Station model with GEOGRAPHY column
│   │   │   ├── schemas.py       # StationCreate/Response with CoordinatePoint
│   │   │   ├── repository.py    # ST_DWithin spatial queries, viewport filter
│   │   │   ├── service.py       # Spatial business logic, coordinate validation
│   │   │   └── router.py        # CRUD + GET /stations/nearby?lat&lng&radius
│   │   └── chargers/            # Existing charger CRUD
│   ├── migrations/
│   │   └── versions/
│   │       └── 004_create_stations.py  # Already exists with GiST index
│   ├── tests/
│   │   ├── test_stations.py     # Spatial query integration tests
│   │   └── test_auth.py         # Mock auth integration tests
│   ├── alembic.ini
│   ├── pyproject.toml           # Add PyJWT dependency
│   └── Dockerfile

├── web/
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx              # Add station detail route
│   │   ├── components/
│   │   │   ├── ui/              # Existing shadcn/ui components
│   │   │   ├── map/
│   │   │   │   ├── MapView.tsx          # EXTEND: click-to-add, marker rendering
│   │   │   │   ├── TileFallback.tsx     # Existing tile fallback
│   │   │   │   ├── StationMarker.tsx    # NEW: custom green DivIcon marker
│   │   │   │   └── StationPopup.tsx     # NEW: marker click detail popup
│   │   │   ├── panels/
│   │   │   │   ├── SearchPanel.tsx      # EXTEND: address/coordinate search
│   │   │   │   ├── StationListPanel.tsx # EXTEND: viewport-filtered list
│   │   │   │   └── StationFormPanel.tsx # EXTEND: map-click form, edit mode
│   │   │   └── layout/          # Existing Header, Sidebar, DashboardLayout
│   │   ├── features/
│   │   │   └── stations/        # EXTEND: API hooks for spatial queries, nearby
│   │   ├── lib/
│   │   │   ├── api.ts           # EXTEND: add auth token header support
│   │   │   └── utils.ts         # Existing cn() helper
│   │   └── styles/globals.css
│   ├── vite.config.ts
│   └── tailwind.config.ts

└── docker-compose.yml            # Existing PostGIS + backend
```

## Complexity Tracking

> No constitution violations — Complexity Tracking not needed.
