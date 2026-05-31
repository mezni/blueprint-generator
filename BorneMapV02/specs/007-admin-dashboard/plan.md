# Implementation Plan: Admin Dashboard

**Branch**: `007-admin-dashboard` | **Date**: 2026-05-29 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/007-admin-dashboard/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.

## Summary

Add a standalone admin dashboard React application (`apps/admin-dashboard`) with a sidebar navigation tree, overview KPI cards, and partners/stations data tables — all powered by static client-side mock data with zero backend integration. Also align existing `web-driver` Leaflet map portal and `mobile-driver` native MapView screen to the cross-platform UI blueprint matrix.

## Technical Context

**Language/Version**: JavaScript / Node.js v24.16.0 / npm v11.13.0

**Primary Dependencies**: React (Vite), Leaflet / react-leaflet, react-native-maps (existing), react-native (existing Expo SDK 54)

**Storage**: N/A — all data is static mock arrays inlined in components

**Testing**: Manual visual verification per acceptance scenarios (sandbox-only; no test framework requirements)

**Target Platform**: Web browser (admin-dashboard, web-driver), iOS/Android via Expo Go (mobile-driver)

**Project Type**: Frontend web application (admin-dashboard) + existing frontends (web-driver, mobile-driver)

**Performance Goals**: Admin tab switching < 100ms; full dashboard load < 3s; map interactions responsive within frame budget

**Constraints**: No backend integration; no real telemetry, database, or message-broker connections; static client-side mock data only

**Scale/Scope**: Three applications: admin-dashboard (NEW), web-driver (UPDATE), mobile-driver (UPDATE). Mock data: 2–3 partners, 2–10 stations. No real users or traffic.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Validation Before Optimization | ✅ PASS | No caching, no message broker, no scaling infrastructure — pure static frontend |
| II. Technical Stack Governance | ✅ PASS | Admin portal uses React (Vite) per constitution; mobile uses Expo; no new language/framework introduced |
| III. API & Service Architecture | ✅ PASS | No API changes — all mock, no backend integration. No new services created. |
| IV. Data Architecture Standards | ⚠️ GATE — see below | Mock IDs use simple patterns (`stn-00000001`). Constitution requires `XXX-nanouuid`. |
| V. Development & Environment Discipline | ✅ PASS | No Docker Compose changes; offline mode respected; docs kept under `/specs` |

**Gate IV — Data Architecture**: The spec reference code uses simple sequential IDs (`stn-00000001`, `prt-total`) which violate the `XXX-nanouuid` pattern (`stn-e3b0c442`). Since this is a static mock sandbox with no persistence, the mock IDs may use simplified patterns for readability. **Justification**: Mock data readability trumps production ID format in sandbox mode. Real IDs will use nanouuid when backend integration occurs.

## Project Structure

### Documentation (this feature)

```text
specs/007-admin-dashboard/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (UI contracts / component spec)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
# Admin Dashboard (NEW)
apps/admin-dashboard/
├── index.html
├── package.json
├── vite.config.js
├── src/
│   ├── main.jsx
│   ├── App.jsx
│   ├── screens/
│   │   └── DashboardOverview.jsx
│   ├── components/
│   │   ├── Sidebar.jsx
│   │   ├── TopBar.jsx
│   │   ├── OverviewMetrics.jsx
│   │   ├── PartnersTable.jsx
│   │   ├── StationsTable.jsx
│   │   └── FallbackView.jsx
│   ├── data/
│   │   └── mockData.js
│   └── styles/
│       └── theme.js

# Web Driver (UPDATE existing)
apps/web-driver/
├── src/
│   ├── components/
│   │   └── MapPortal.jsx     # Rewrite per blueprint spec
│   └── ...

# Mobile Driver (UPDATE existing)
apps/mobile-driver/
├── src/
│   ├── screens/
│   │   └── MapScreen.js      # Rewrite per blueprint spec
│   └── ...
```

**Structure Decision**: Monorepo with three independent application directories at `apps/`. Admin dashboard uses Vite for fast dev iteration. Web-driver and mobile-driver already exist and need targeted component rewrites per the blueprint matrix.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Mock IDs use sequential patterns (`stn-00000001`) instead of `XXX-nanouuid` per Constitution Principle IV | Sandbox readability — sequential IDs are easier to reason about during UI development | Using nanouuid in mock data adds cognitive overhead with zero benefit in a no-backend sandbox |
