# Implementation Plan: Cross-Platform UI Synchronization

**Branch**: `006-cross-platform-ui` | **Date**: 2026-05-28 | **Spec**: `specs/006-cross-platform-ui/spec.md`

**Input**: Feature specification from `/specs/006-cross-platform-ui/spec.md`

## Summary

Synchronize the desktop web and mobile app UI layouts, navigation, map rendering, search/filter, station detail views, and clickstream analytics events so users experience a consistent interface across devices. No new backend infrastructure is required — new API routes are added to the existing `api-service` Actix-web crate, and the frontend adds components within the existing `apps/mobile-driver/` Expo project.

## Technical Context

**Language/Version**: JavaScript (React 19.1, React Native 0.81.5, Expo SDK 54)

**Primary Dependencies**: react-leaflet + leaflet (web), react-native-maps (mobile), axios (API client), @react-navigation/native + @react-navigation/bottom-tabs (navigation), jest-expo + @testing-library/react-native (testing)

**Storage**: localStorage (web) / AsyncStorage (mobile) for session_id persistence; in-memory HashMap (api-service) for ephemeral filter sync state

**Testing**: Jest (via jest-expo) + React Native Testing Library for component tests

**Target Platform**: Web (desktop via Expo web export) + Mobile (iOS/Android via Expo Go)

**Project Type**: Mobile app + Web frontend (single Expo workspace at `apps/mobile-driver/`)

**Performance Goals**: ≤500ms p95 for marker-tap → station detail view (SC-002). Poll-based filter sync ≤60s interval (SC-006).

**Constraints**: WCAG 2.1 AA accessibility (screen reader labels, 44pt touch targets, keyboard nav), ≤200 stations per viewport (clustering threshold), last-writer-wins conflict resolution for filter sync.

**Scale/Scope**: ~200 stations per viewport at city zoom. Single session per user device. Events per session: bounded by user interaction frequency (no automated event generation).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Result | Notes |
|------|--------|-------|
| Principle I: Validation Before Optimization | **PASS** | No new infrastructure (no Redis, no RabbitMQ, no caching layer). Filter sync uses in-memory store in existing api-service. |
| Principle II: Stack LOCKED | **PASS** | Uses existing React Native / Expo stack. No new languages or frameworks. Desktop web uses existing Expo Web (react-native-web + react-leaflet) path already in the codebase. |
| Principle III: API & Service Architecture | **PASS** | New endpoints follow `/api/v1` prefix. Routes added to existing `api-service` domain modules. No circular dependencies. |
| Principle IV: ID Pattern (nanouuid) | **PASS** | Station IDs already use `stn-` prefix. session_id is a standard UUID (not a DB entity ID — exempt). No new DB entities introduced. |
| Principle V: Docker Compose | **PASS** | No new infrastructure services. Filter sync uses in-memory state, not a database. |
| Principle V: Docs sync | **PASS** | All artifacts under `specs/006-cross-platform-ui/`. |

## Project Structure

### Documentation (this feature)

```text
specs/006-cross-platform-ui/
├── spec.md              # Feature specification
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (testing, navigation, filter sync decisions)
├── data-model.md        # Phase 1 output (client state, API shapes, component catalog)
├── quickstart.md        # Phase 1 output (setup, usage, CI integration)
├── contracts/
│   └── api.yaml         # Phase 1 output (OpenAPI 3.0 contract)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
apps/mobile-driver/
├── App.js                     # Root: wrap with NavigationContainer + filter sync background poll
├── package.json               # Add deps: @react-navigation/native, @react-navigation/bottom-tabs
└── src/
    ├── screens/
    │   ├── MapScreen.js       # Existing — enhance with search/filter bar, detail sheet, FAB
    │   ├── ExploreScreen.js   # NEW — placeholder screen
    │   ├── SavedScreen.js     # NEW — placeholder screen
    │   └── ProfileScreen.js   # NEW — placeholder screen
    ├── components/
    │   ├── MapView.native.js  # Existing — no changes needed (already correct)
    │   ├── MapView.web.js     # Existing — no changes needed (already correct)
    │   ├── StationCard.js     # Existing — no changes needed (reused in detail sheet)
    │   ├── NavBar.js          # NEW — desktop web horizontal navigation bar
    │   ├── BottomTabBar.js    # NEW — mobile bottom tab bar wrapper
    │   ├── SearchBar.js       # NEW — text search input with debounce
    │   ├── FilterControls.js  # NEW — filter chips/dropdowns
    │   ├── StationDetailPanel.js  # NEW — desktop fixed-height bottom panel
    │   ├── StationDetailSheet.js  # NEW — mobile draggable bottom sheet
    │   ├── ZoomControls.js    # NEW — zoom in/out + locate-me
    │   └── FAB.js            # NEW — floating action button
    ├── services/
    │   ├── api.js             # Existing — add search(), getStationDetail(), getFilters(), setFilters()
    │   ├── analytics.js       # NEW — clickstream event helper (reuses POST /analytics/connect)
    │   └── session.js         # NEW — session_id generation + persistence
    ├── hooks/
    │   ├── useNavigation.js   # NEW — shared nav state context
    │   ├── useSearch.js       # NEW — debounced search with API call
    │   ├── useFilters.js      # NEW — filter state with poll-based sync
    │   ├── useStationDetail.js # NEW — fetch station detail on marker tap
    │   └── useAnalytics.js    # NEW — emit clickstream events
    ├── context/
    │   └── AppContext.js      # NEW — shared app state provider (nav, filters, viewport)
    └── styles/
        └── theme.js           # NEW — shared design tokens (colors, spacing, breakpoints)

backend/
├── api-service/
│   └── src/
│       └── domains/
│           ├── locate/
│           │   ├── mod.rs     # Existing — extend with search + station-detail routes
│           │   ├── model.rs   # Existing — Station/Charger models already defined
│           │   └── routes.rs  # Existing — add GET /search, GET /stations/{id}
│           ├── analytics/
│           │   ├── mod.rs     # Existing — no changes needed
│           │   └── routes.rs  # Existing — no changes needed
│           └── filters/
│               ├── mod.rs     # NEW — init routes: GET /filters, PUT /filters
│               └── routes.rs  # NEW — in-memory HashMap backed, session-keyed
```

**Structure Decision**: Follow the existing monorepo structure. Frontend additions go into `apps/mobile-driver/` (the single Expo workspace already supports both web and native targets via platform-specific file extensions). Backend additions go into the existing `api-service` domain modules — the `locate` module is extended with search/detail routes, and a new `filters` domain module is created for filter sync. This reuses the existing Actix-web server, connection pool, and middleware stack.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| *(none — all gates passed)* | | |
