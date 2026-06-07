# Implementation Plan: Driver Web App

**Branch**: `009-driver-web-app` | **Date**: 2026-06-03 | **Spec**: `specs/009-driver-web-app/spec.md`

**Input**: Feature specification from `/specs/009-driver-web-app/spec.md`

## Summary

Build the Driver Web App — a map-first single-page application for discovering EV charging stations. Implements 5 user stories: map-first discovery (P1), station details & search (P1), progressive authentication (P2), favorites & reviews (P2), and clickstream events (P3). The app is a Leaflet-based interactive map with a side panel for station details, collapsible search overlay, and inline favorites filtering.

## Clarifications (from `/speckit.clarify`)

- **Station detail panel**: Side panel (260–400px) that shrinks the map when a marker is selected (Option A)
- **Favorites access**: Inline map filter toggle — no dedicated favorites page (Option A)
- **Search placement**: Collapsible overlay triggered by a search icon in the header (Option B)

## Technical Context

**Language/Version**: TypeScript 5.x (workspace)

**Primary Dependencies**: React 18+ (Vite), Leaflet (map), @tanstack/react-query (server state), react-router (routing), design-tokens + UI primitives from Sprint 8, `@bornemap/api-client` (HTTP), `@bornemap/auth-client` (Keycloak auth), `@bornemap/event-taxonomy` (clickstream events)

**Storage**: None — all data served by driver-service backend APIs (Sprint 7)

**Testing**: Vitest + @testing-library/react (unit/component), Playwright (E2E — deferred to Sprint 10)

**Target Platform**: Browser (driver-web app only)

**Project Type**: Frontend application (monorepo workspace)

**Performance Goals**: Map mounts within 500ms (inherited from Sprint 8 benchmark); viewport queries respond within 1s of pan/zoom stop; search returns results within 500ms of debounce; skeleton screens appear within 200ms

**Constraints**: No dedicated favorites page (inline filter only); auth is progressive (modal only, no login wall); single-region (Tunisia); map-first single-page UX; RTL/LTR support via existing design system; all server state via React Query; clickstream events are fire-and-forget

**Scale/Scope**: Single web app; ~6 page-level component groups; single-region development team

## Architecture & Component Map

```
App (Router)
├── Layout (header + map container)
│   ├── Header
│   │   ├── SearchIcon → SearchOverlay (collapsible)
│   │   ├── FavoritesFilterToggle (inline)
│   │   └── AuthStatus (avatar or login trigger)
│   └── MapView (full-screen, resizes when side panel opens)
│       ├── MapContainer (Leaflet wrapper from Sprint 8)
│       │   └── StationMarkers (clustered via Leaflet.markercluster)
│       ├── MapStateOverlay (skeleton/spinner/empty-state)
│       ├── StationDetailPanel (side panel, 260–400px)
│       │   ├── StationInfo (name, desc, address, distance)
│       │   ├── ChargerList (connector type + power + availability)
│       │   ├── ReviewSection (list + submit form)
│       │   └── FavoriteButton (toggle)
│       └── SearchOverlay (collapsible panel)
│           ├── SearchInput + Filters (connector type, availability)
│           └── SearchResults (list of stations)
├── AuthModal (global, portal-rendered)
└── NotFound (fallback route)
```

## Data Flow

```
User Action → React Component → React Query hook → ApiClient → driver-service API
                                                      ↓
                                                 AuthClient (JWT injection)
                                                      ↓
                                                 EventTaxonomy (fire-and-forget)
```

**Server state**: All station, search, favorite, and review data flows through React Query with:
- `useStationMarkers(lat, lng, radiusKm)` — viewport-driven marker query (debounced 500ms)
- `useStationDetail(id)` — single station fetch on marker click
- `useSearch(query, filters)` — debounced search (300ms)
- `useFavorites(userId)` — user's favorited station IDs
- `useFavoriteToggle()` — optimistic mutation
- `useReviews(stationId)` — reviews for a station
- `useReviewMutation()` — create/update/delete review

**Auth flow**: AuthClient manages Keycloak login. The app checks `isAuthenticated` state. Gated actions (favorite, review) check auth first; if unauthenticated, show AuthModal. On successful login, retry the gated action.

**Clickstream**: A `useClickstream()` hook wraps event emission. Each component emits events on user interactions. Events are fire-and-forget — failures are silently ignored.

## Constitution Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Data-First Source of Truth | ✅ PASS | API is the source of truth; React Query cache is derived state |
| II. Strict Domain & Service Separation | ✅ PASS | Driver-web is a frontend-only app; all backend calls go through driver-service API |
| III. Ownership-Enforced Authorization | ✅ PASS | Auth is delegated to Keycloak via auth-client; app respects JWT scope |
| IV. Contract-Driven REST APIs | ✅ PASS | All API calls driven by known driver-service endpoints |
| V. Event-Driven & Derived State | ✅ PASS | Clickstream events fire-and-forget; React Query cache is derived from API responses |
| VI. Soft Delete & Auditability | ✅ PASS | N/A — no data entities managed by this app (reviews/favorites managed by backend) |
| VII. Verification Discipline | ✅ PASS | Components testable in isolation; React Query hooks testable with msw/server; acceptance scenarios are concrete |

**No violations found.** Complexity Tracking not required.

## Phases

### Phase 0: Research & Setup
Investigate driver-service API contracts, install missing dependencies, set up app shell.

### Phase 1: Map Foundation (User Stories 1 & 3 — Map states)
Implement interactive map with clustered markers, viewport-driven data fetching, and the three map states (idle, active, station selected).

### Phase 2: Station Detail & Search (User Stories 2)
Build station detail side panel, search overlay, wire up React Query hooks.

### Phase 3: Progressive Auth (User Story 3 — Auth flow)
Implement auth-client integration, login modal, JWT handling for gated actions.

### Phase 4: Favorites & Reviews (User Story 4)
Favorite toggle, review submission/edit/delete, inline favorites filter.

### Phase 5: Clickstream Events (User Story 5)
Instrument all user interactions with fire-and-forget events.

### Phase 6: Polish & Cross-Cutting
RTL verification, performance benchmarks, error boundary, edge case review.

## Complexity Tracking

> Not required — no Constitution violations.
