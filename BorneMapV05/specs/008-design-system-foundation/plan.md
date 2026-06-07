# Implementation Plan: Design System Foundation

**Branch**: `008-design-system-foundation` | **Date**: 2026-06-02 | **Spec**: `specs/008-design-system-foundation/spec.md`

**Input**: Feature specification from `/specs/008-design-system-foundation/spec.md`

## Summary

Create a reusable design system foundation for all four frontend applications. Implement `@bornemap/design-tokens` with color, spacing, typography, shadow, and border-radius tokens; integrate with Tailwind CSS theme; build 5 primitive components (Button, Input, Card, Modal, Map container) using shadcn/ui; ensure RTL-ready layout via CSS logical properties. Tokens are the single source of truth — no inline hex values, no arbitrary spacing.

## Technical Context

**Language/Version**: TypeScript 5.x (workspace)

**Primary Dependencies**: React 18+ (web: Vite, mobile: Expo), Tailwind CSS 3.3+ (logical property utilities), shadcn/ui (Radix-based accessible primitives for Button, Input, Card, Modal), Leaflet (Map container), clsx + tailwind-merge (utility composition), `@bornemap/design-tokens` (workspace package)

**Storage**: None — tokens are compile-time constants and runtime CSS custom properties

**Testing**: Vitest (unit), Storybook (visual component isolation), `@testing-library/react` (component behavior), Playwright (cross-app visual regression — deferred to Sprint 9)

**Target Platform**: Browser (web apps: driver-web, partner-dashboard, admin-dashboard), Mobile (driver-mobile — tokens only, no components)

**Project Type**: Frontend design system (shared packages + per-app components)

**Performance Goals**: Map container mounts interactive map within 500ms; components render without measurable overhead (no token resolution at render time)

**Constraints**: No inline hex colors, no arbitrary spacing, no hardcoded typography; all utility classes reference token values; RTL via CSS logical properties only; components are per-app copies with shared tokens+config (no separate UI package); driver-mobile gets tokens only (components deferred to Sprint 12)

**Scale/Scope**: 3 web apps consuming same 5 primitives; ~50 token values; single-region development team

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Data-First Source of Truth | ✅ PASS | Design tokens are the source of truth for all visual properties — no inline overrides, no per-app drift |
| II. Strict Domain & Service Separation | ✅ PASS | Frontend-only sprint; no services, databases, or cross-domain concerns |
| III. Ownership-Enforced Authorization | ✅ PASS | N/A — no auth concerns in design system work |
| IV. Contract-Driven REST APIs | ✅ PASS | N/A — no APIs defined in this sprint |
| V. Event-Driven & Derived State | ✅ PASS | N/A — no events in design system |
| VI. Soft Delete & Auditability | ✅ PASS | N/A — no data entities created |
| VII. Verification Discipline | ✅ PASS | Components testable in isolation (Storybook); cross-app visual parity verifiable by inspection; no DB/app logic to test |

**No violations found.** Complexity Tracking not required.

## Project Structure

### Documentation (this feature)

```text
specs/008-design-system-foundation/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (token data model)
├── quickstart.md        # Phase 1 output (developer guide)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
# Design token package (already exists as stub)
packages/design-tokens/
├── src/
│   ├── index.ts               # Public API barrel
│   ├── colors.ts              # Color tokens (primary, secondary, accent, etc.)
│   ├── spacing.ts             # Spacing scale (4/8/12/16/20/24/32/48/64)
│   ├── typography.ts          # Font family, size scale, weight, line-height
│   ├── shadows.ts             # Shadow tokens (sm/md/lg/card/modal)
│   ├── border-radius.ts       # Border-radius tokens (sm/md/lg/full)
│   └── css.ts                 # CSS custom property generation
├── package.json
└── tsconfig.json

# Tailwind config per web app (shared structure)
apps/driver-web/tailwind.config.ts      # Imports tokens → Tailwind theme
apps/partner-dashboard/tailwind.config.ts
apps/admin-dashboard/tailwind.config.ts

# Component primitives (per-app, same code)
apps/driver-web/src/components/ui/
├── button.tsx
├── input.tsx
├── card.tsx
├── modal.tsx
└── map-container.tsx    # Leaflet wrap (app-specific, not in shared config)
```

**Structure Decision**: Components live per-app (following shadcn/ui's copy-paste convention) with shared tokens as the single source of truth. Token package is the only shared artifact — no `packages/ui` library. Map container is web-only (Leaflet doesn't run in React Native).

## Complexity Tracking

> Not required — no Constitution violations.
