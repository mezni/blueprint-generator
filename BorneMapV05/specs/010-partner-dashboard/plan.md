# Implementation Plan: Sprint 10 — Partner Dashboard

**Branch**: `010-partner-dashboard` | **Date**: 2026-06-04 | **Spec**: `specs/010-partner-dashboard/spec.md`

**Input**: Feature specification from `specs/010-partner-dashboard/spec.md`

## Summary

Build an operational dashboard for charging station partners to manage their stations, chargers, and availability. Partner API endpoints were already implemented in Sprint 5 (admin-service) but unreachable due to missing Traefik routing and stale Docker images. This sprint exposes the partner API and builds the frontend dashboard.

## Technical Context

**Language/Version**: TypeScript 6.0 (frontend), Rust 1.87 (backend — existing)

**Primary Dependencies**:
- React 19 + Vite 8 + Tailwind CSS 4
- `@tanstack/react-query` 5 for server state
- `react-router` 7 for routing
- `keycloak-js` 26 for auth
- `@bornemap/api-client`, `@bornemap/auth-client`, `@bornemap/design-tokens` (shared packages)
- `class-variance-authority` for UI variants
- leaflet for map display

**Storage**: N/A (frontend-only; backend API is the data source)

**Testing**: `vitest` (planned, not yet implemented)

**Target Platform**: Browser (Chromium, Firefox, Safari — latest 2 versions)

**Project Type**: Web application (frontend) + configuration changes (backend)

**Performance Goals**: Page load < 2s; station list render < 500ms for 20 items; API calls within platform baseline (< 200ms p95)

**Constraints**: Must authenticate via Keycloak with `partner` role; all API calls go through Traefik proxy on port 80; no direct backend access from browser

**Scale/Scope**: Single dashboard for ~10–50 partners in MVP; up to 200 stations per partner

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Data-First Source of Truth | ✅ Pass | Frontend reads/writes via API; no client-side data authority |
| II. Strict Domain & Service Separation | ✅ Pass | Partner dashboard talks only to admin-service; no cross-service coupling |
| III. Ownership-Enforced Authorization | ✅ Pass | Partner role gated at backend; frontend uses auth middleware |
| IV. Contract-Driven REST APIs | ✅ Pass | Uses existing `success`/`error` envelope from admin-service partner endpoints |
| V. Event-Driven & Derived State | ✅ Pass | Station mutations emit events via outbox (existing backend behavior) |
| VI. Soft Delete & Auditability | ✅ Pass | Station delete uses soft-delete (existing backend behavior) |
| VII. Verification Discipline | ⚠️ Partial | No frontend tests yet; integration tested via `curl` against live endpoints |

## Project Structure

### Documentation (this feature)

```text
specs/010-partner-dashboard/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 — technical decisions
├── data-model.md        # Phase 1 — entity definitions
├── quickstart.md        # Phase 1 — setup instructions
├── contracts/           # Phase 1 — API contracts
└── tasks.md             # Phase 2 (future: /speckit.tasks)
```

### Source Code (repository root)

```text
apps/partner-dashboard/
├── src/
│   ├── main.tsx                    # Root with QueryClient + BrowserRouter + AuthProvider
│   ├── App.tsx                     # Routes + AuthGate + Header + Main layout
│   ├── index.css                   # Tailwind + design token CSS variables
│   ├── lib/
│   │   ├── api.ts                  # ApiClient singleton for /api/v1/partner
│   │   ├── clickstream.ts          # Event emission (partner_dashboard channel)
│   │   ├── types.ts                # Domain types: Station, Charger, Profile, envelopes
│   │   └── utils.ts                # cn() utility (clsx + tailwind-merge)
│   ├── hooks/
│   │   ├── useAuth.tsx             # Keycloak auth context provider
│   │   ├── usePartnerStations.ts   # Station list/create/update/delete queries + mutations
│   │   ├── usePartnerChargers.ts   # Charger list/create/update queries + mutations
│   │   ├── usePartnerAvailability.ts # Availability toggle mutation
│   │   └── usePartnerProfile.ts    # Profile query
│   ├── components/
│   │   ├── Header.tsx              # Top nav with route links + user info + logout
│   │   ├── AuthGate.tsx            # Auth guard — shows login prompt if unauthenticated
│   │   ├── ErrorBoundary.tsx       # React error boundary
│   │   ├── Modal.tsx               # Portal modal with overlay + Escape key close
│   │   ├── StationForm.tsx         # Create/edit station form (name, address, lat, lng, status)
│   │   ├── ChargerForm.tsx         # Create/edit charger form (type, power_kw)
│   │   └── ui/                     # Shared UI primitives (button, card, input)
│   └── pages/
│       ├── StationsPage.tsx        # List + create/edit/delete stations + inline availability + chargers
│       ├── ChargersPage.tsx        # Table view of all chargers with edit
│       └── ProfilePage.tsx         # Partner profile details

infra/compose/
├── traefik/dynamic/routes.yml      # Added partner router
├── docker-compose.yml              # Updated admin-service config (env_file, migrations volume)

services/admin-service/
├── src/main.rs                     # Made migrations non-fatal
├── migrations/0016_seed_data.up.sql # Fixed ON CONFLICT clause

packages/
├── api-client/src/index.ts         # Added headers parameter to request methods
├── api-contracts/src/envelope.ts   # Added ItemEnvelope type
```

**Structure Decision**: Web application (frontend) following existing driver-web conventions. Backend changes are minimal configuration and one-line code changes.

## Complexity Tracking

*No Constitution violations detected — Complexity Tracking not required.*

## Phase 1 Artifacts

- [research.md](research.md) — Technical decisions
- [data-model.md](data-model.md) — Entity definitions
- [quickstart.md](quickstart.md) — Setup/run instructions
