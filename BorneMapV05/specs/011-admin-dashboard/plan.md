# Implementation Plan: Sprint 11 — Admin Dashboard

**Branch**: `011-admin-dashboard` | **Date**: 2026-06-04 | **Spec**: `specs/011-admin-dashboard/spec.md`

**Input**: Feature specification from `specs/011-admin-dashboard/spec.md`

## Summary

Build a platform control interface for administrators to manage partners, stations, reviews, and users. Admin API endpoints were already implemented in Sprint 5 (admin-service). The admin-dashboard app scaffold exists from Sprint 1 but has no UI beyond a stub. This sprint builds the full frontend dashboard with 5 pages: Dashboard Overview, Partners, Stations, Reviews, and Users. Reports page is deferred to Sprint 14/15.

## Technical Context

**Language/Version**: TypeScript 6.0 (frontend), Rust 1.87 (backend — existing, no changes needed)

**Primary Dependencies**:
- React 19 + Vite 8 + Tailwind CSS 4
- `@tanstack/react-query` 5 for server state
- `react-router` 7 for routing
- `keycloak-js` 26 for auth
- `@bornemap/api-client`, `@bornemap/auth-client`, `@bornemap/design-tokens`, `@bornemap/api-contracts`, `@bornemap/event-taxonomy` (shared packages)
- `class-variance-authority` + `clsx` + `tailwind-merge` for UI variants
- `@tailwindcss/vite` 4 for Tailwind integration

**Storage**: N/A (frontend-only; admin-service API is the data source)

**Testing**: `vitest` (planned, not yet implemented — same as partner-dashboard)

**Target Platform**: Browser (Chromium, Firefox, Safari — latest 2 versions); desktop-first, tablet functional

**Project Type**: Web application (frontend only; no backend changes required)

**Performance Goals**: Dashboard load < 3s including auth; partner list render < 2s for 20 items with 100+ partners seeded

**Constraints**: Must authenticate via Keycloak with `admin` role; all API calls go through Traefik proxy on port 80 (`/api/v1/admin/*`); no direct backend access from browser; reuse existing design tokens and shadcn/ui primitives

**Scale/Scope**: Single dashboard for platform-wide management; up to 5 pages; ~50 partners, ~500 stations, ~2000 reviews in MVP

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Data-First Source of Truth | ✅ Pass | Frontend reads/writes via API; no client-side data authority |
| II. Strict Domain & Service Separation | ✅ Pass | Admin dashboard talks only to admin-service; no cross-service coupling |
| III. Ownership-Enforced Authorization | ✅ Pass | Admin role gated at backend; frontend uses auth middleware; admin has global scope per design |
| IV. Contract-Driven REST APIs | ✅ Pass | Uses existing `success`/`error` envelope from admin-service endpoints |
| V. Event-Driven & Derived State | ✅ Pass | Station/partner mutations emit events via existing outbox; GIS resync on coord changes |
| VI. Soft Delete & Auditability | ✅ Pass | Partner/station/review delete uses soft-delete (existing backend behavior); admin still respects audit fields |
| VII. Verification Discipline | ⚠️ Partial | No frontend tests yet; integration tested via manual endpoint verification same as partner-dashboard |

**Gate Decision**: PASS (no violations)

## Project Structure

### Documentation (this feature)

```text
specs/011-admin-dashboard/
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
apps/admin-dashboard/
├── src/
│   ├── main.tsx                    # Root with QueryClient + BrowserRouter + AuthProvider
│   ├── App.tsx                     # Routes + AuthGate + Header + Layout
│   ├── index.css                   # Tailwind + design token CSS variables
│   ├── lib/
│   │   ├── api.ts                  # ApiClient singleton for /api/v1/admin
│   │   ├── clickstream.ts          # Event emission (admin_dashboard channel)
│   │   ├── types.ts                # Domain types for admin views
│   │   └── utils.ts                # cn() utility (exists in scaffold)
│   ├── hooks/
│   │   ├── useAuth.tsx             # Keycloak auth context provider
│   │   ├── useAdminOverview.ts     # Dashboard metrics query
│   │   ├── useAdminPartners.ts     # Partner list/create/edit/delete mutations
│   │   ├── useAdminStations.ts     # Station list/edit/delete mutations
│   │   ├── useAdminReviews.ts      # Review list/moderate mutations
│   │   └── useAdminUsers.ts        # User list query
│   ├── components/
│   │   ├── ErrorBoundary.tsx       # React error boundary
│   │   ├── Modal.tsx               # Portal modal with overlay + Escape key
│   │   ├── Header.tsx              # Top nav with sidebar toggle + user info + logout
│   │   ├── Sidebar.tsx             # 260px sidebar with nav items
│   │   ├── Layout.tsx              # Sidebar + Header + main content wrapper
│   │   ├── AuthGate.tsx            # Auth guard with login prompt
│   │   ├── DataCard.tsx            # Metric card for dashboard overview
│   │   ├── PartnerForm.tsx         # Create/edit partner form
│   │   ├── StationForm.tsx         # Edit station form (includes lat/lng with confirm)
│   │   └── ReviewModeration.tsx    # Review status transition controls
│   └── pages/
│       ├── DashboardPage.tsx       # Overview with metric cards
│       ├── PartnersPage.tsx        # Partner list + CRUD modals
│       ├── StationsPage.tsx        # Station list + edit/delete + inline chargers
│       ├── ReviewsPage.tsx         # Review list + moderation
│       └── UsersPage.tsx           # User list (read-only)
```

**Structure Decision**: Follows the same SPA structure as partner-dashboard (apps/admin-dashboard/src with pages/, hooks/, components/, lib/). Layout uses sidebar (260px) + Header + content area pattern confirmed in clarification.

## Complexity Tracking

No Constitution violations to justify.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (none) | — | — |
