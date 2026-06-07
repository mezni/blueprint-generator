# Tasks: Design System Foundation

**Input**: Design documents from `specs/008-design-system-foundation/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Organization**: Tasks grouped by user story for independent implementation.

## Format: `[ID] [P] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (e.g., US1, US2)
- Include exact file paths in descriptions

## Path Conventions

- **Token package**: `packages/design-tokens/src/`
- **Web apps**: `apps/{driver-web,partner-dashboard,admin-dashboard}/`
- **Components**: `apps/*/src/components/ui/`

---

## Phase 1: Setup (No work needed)

**Monorepo, tooling, and all four frontend apps already exist from Sprint 1.**
**`packages/design-tokens` exists as a stub (`src/index.ts` + `package.json`).**
**No setup tasks required.**

---

## Phase 2: Foundational — Design Token Package

**Purpose**: Implement the `@bornemap/design-tokens` package with all token categories, TypeScript exports, and CSS custom property generation. This blocks all user stories.

- [X] T001 [P] Implement color tokens in `packages/design-tokens/src/colors.ts` (primary, secondary, accent, success, warning, error, surface, text, border; each with base/hover/active/muted)
- [X] T002 [P] Implement spacing tokens in `packages/design-tokens/src/spacing.ts` (scale: 4, 8, 12, 16, 20, 24, 32, 48, 64)
- [X] T003 [P] Implement typography tokens in `packages/design-tokens/src/typography.ts` (font-family sans/mono, font-size xs–4xl, font-weight normal–bold, line-height none–relaxed)
- [X] T004 [P] Implement shadow tokens in `packages/design-tokens/src/shadows.ts` (sm, md, lg, card, modal)
- [X] T005 [P] Implement border-radius tokens in `packages/design-tokens/src/border-radius.ts` (sm, md, lg, full)
- [X] T006 Implement CSS custom property generator in `packages/design-tokens/src/css.ts` that produces `:root { --color-primary: ... }` block from all token modules
- [X] T007 Update `packages/design-tokens/src/index.ts` barrel to export all token modules and CSS generator
- [X] T008 Update `packages/design-tokens/package.json` with proper build script (tsc or tsup) and verify `npm run build` works

**Checkpoint**: Foundation ready — token package builds, all values exported as typed constants

---

## Phase 3: User Story 1 - Design Tokens & Theme Consistency (Priority: P1) 🎯

**Goal**: Tokens are consumable by all four frontend apps via Tailwind theme mapping and CSS custom properties.

**Independent Test**: A test HTML page in any web app can use `bg-primary`, `text-body`, `p-4`, `shadow-card`, `rounded-lg` and see the resolved values match token definitions.

### Implementation for User Story 1

- [X] T009 [P] [US1] Add Tailwind theme imports to `apps/driver-web/tailwind.config.ts` — map tokens to colors, spacing, fontFamily, fontSize, boxShadow, borderRadius
- [X] T010 [P] [US1] Add Tailwind theme imports to `apps/partner-dashboard/tailwind.config.ts` (same mapping as driver-web)
- [X] T011 [P] [US1] Add Tailwind theme imports to `apps/admin-dashboard/tailwind.config.ts` (same mapping as driver-web)
- [X] T012 [P] [US1] Add CSS custom property injection (call `generateCssVars()` from design-tokens) to each web app's global CSS file: `apps/driver-web/src/index.css`, `apps/partner-dashboard/src/index.css`, `apps/admin-dashboard/src/index.css`
- [X] T013 [P] [US1] Add `@bornemap/design-tokens` dependency to `apps/driver-mobile/package.json` and create `apps/driver-mobile/src/theme/tokens.ts` re-exporting the token values for React Native usage (color and spacing constants only; CSS vars not applicable in RN)
- [X] T014 [US1] Verify token resolution: build each web app and confirm `bg-primary` renders the correct color via CSS custom property inspection

**Checkpoint**: US1 complete — tokens build, Tailwind theme resolves correctly in all 3 web apps

---

## Phase 4: User Story 2 - Reusable Component Primitives (Priority: P1)

**Goal**: Developers can use pre-styled Button, Input, Card, Modal, and Map container that match the design system exactly, in any web app.

**Independent Test**: A test page rendering all 5 primitives looks identical in driver-web, partner-dashboard, and admin-dashboard.

### Implementation for User Story 2

- [X] T015 [P] [US2] Install shadcn/ui init and configure `components.json` in `apps/driver-web`; create `apps/driver-web/src/components/ui/` directory
- [X] T016 [P] [US2] Install shadcn/ui init in `apps/partner-dashboard` (same config)
- [X] T017 [P] [US2] Install shadcn/ui init in `apps/admin-dashboard` (same config)
- [X] T018 [P] [US2] Implement Button component in `apps/driver-web/src/components/ui/button.tsx` with variants (primary, secondary, outline) and sizes (sm, md, lg); copy to partner-dashboard and admin-dashboard
- [X] T019 [P] [US2] Implement Input component in `apps/driver-web/src/components/ui/input.tsx` with support for label, error state, placeholder, focus ring; copy to partner-dashboard and admin-dashboard
- [X] T020 [P] [US2] Implement Card component in `apps/driver-web/src/components/ui/card.tsx` with Card.Header, Card.Content, Card.Footer subcomponents; copy to partner-dashboard and admin-dashboard
- [X] T021 [P] [US2] Implement Modal component in `apps/driver-web/src/components/ui/modal.tsx` using shadcn/ui Dialog (backdrop, focus trap, escape-to-close, portal); copy to partner-dashboard and admin-dashboard
- [X] T022 [P] [US2] Implement Map container shell in `apps/driver-web/src/components/ui/map-container.tsx` that mounts an interactive map (Leaflet), exposes map instance via callback, handles resize; copy to partner-dashboard and admin-dashboard
- [X] T023 [P] [US2] Add Button unit test in `apps/driver-web/src/components/ui/__tests__/button.test.tsx` — render variants (primary, secondary, outline), verify click handler, verify disabled state
- [X] T024 [P] [US2] Add Input unit test in `apps/driver-web/src/components/ui/__tests__/input.test.tsx` — render with label, error state, placeholder, focus ring, disabled state
- [X] T025 [P] [US2] Add Card unit test in `apps/driver-web/src/components/ui/__tests__/card.test.tsx` — render Header, Content, Footer subcomponents
- [X] T026 [P] [US2] Add Modal unit test in `apps/driver-web/src/components/ui/__tests__/modal.test.tsx` — verify open/close, backdrop click, escape key, focus trap
- [X] T027 [US2] Verify visual parity: build all three web apps and confirm Button, Input, Card, Modal, Map container render identically

**Checkpoint**: US2 complete — all 5 primitives functional and visually identical across web apps

---

## Phase 5: User Story 3 - RTL-Ready Foundation (Priority: P2)

**Goal**: All tokens and components use CSS logical properties so RTL layout works natively with a `dir="rtl"` toggle.

**Independent Test**: Toggle `dir="rtl"` on document and verify Card text alignment, Input padding, and Modal close button position flip correctly without custom CSS.

### Implementation for User Story 3

- [X] T028 [P] [US3] Audit all token CSS variable generation in `packages/design-tokens/src/css.ts` — verify spacing and alignment tokens use logical property names (margin-inline, padding-inline-start, inset-inline-end) where relevant
- [X] T029 [P] [US3] Update all three web app Tailwind configs to enable Tailwind 3.3+ built-in logical property utilities
- [X] T030 [P] [US3] Audit Button component in all three web apps — replace physical directions (left/right padding) with logical equivalents (padding-inline)
- [X] T031 [P] [US3] Audit Input component in all three web apps — verify text alignment and padding use logical properties
- [X] T032 [P] [US3] Audit Card component in all three web apps — verify text alignment flips correctly in RTL
- [X] T033 [P] [US3] Audit Modal component in all three web apps — verify close button position and backdrop use logical positioning
- [X] T034 [US3] RTL integration test: create a test page in each web app with `dir="rtl"` and all 5 primitives; verify layout mirrors correctly

**Checkpoint**: US3 complete — RTL works natively across all components with no custom CSS overrides

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify everything works end-to-end, validate against spec, documentation finalization.

- [X] T035 Build all three web apps and fix any compilation errors
- [X] T036 Run visual parity check: snapshot-test each primitive in each web app and confirm identical output
- [X] T037 Verify zero inline hex values, arbitrary spacing, or hardcoded typography across all component files (grep check)
- [X] T038 [SC-005] Add map mount performance benchmark: verify `apps/driver-web/src/components/ui/map-container.tsx` mounts under 500ms (use `performance.now()` in a test or manual timing script)
- [X] T039 [SC-004] Create a sample screen in `apps/driver-web/src/app/sandbox.tsx` that uses all 5 primitives (Button, Input, Card, Modal, Map container) with design tokens only — no inline hex/spacing
- [X] T040 Run quickstart.md validation: a developer can follow the quickstart guide end-to-end
- [X] T041 Update AGENTS.md to reference sprint 8 plan (verify link is correct)
- [X] T042 Run `cargo build --workspace` to verify no Rust workspace regressions (if applicable)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No work needed — monorepo ready
- **Foundational (Phase 2)**: Blocks all user stories — token package must exist
- **US1 (Phase 3)**: Depends on Phase 2 — blocks components that consume tokens
- **US2 (Phase 4)**: Depends on Phase 2 + Phase 3 — components need tokens + Tailwind config
- **US3 (Phase 5)**: Depends on Phase 2, 3, 4 — RTL audit happens on tokens + Tailwind + components
- **Polish (Phase 6)**: Depends on all user stories

### Within Each User Story

- Token files are independent (T001–T005 can be parallel)
- Tailwind configs per app are independent (T009–T011 can be parallel)
- Each component is independent of other components (T018–T022 can be parallel within a single app)
- Components are identical across apps and copied from driver-web

### Parallel Opportunities

| Phase | Parallel tasks |
|-------|---------------|
| Phase 2 | T001–T005 (all token files) |
| Phase 3 | T009–T012 (per-app Tailwind config + CSS vars) + T013 (driver-mobile tokens) |
| Phase 4 | T015–T017 (per-app shadcn/ui init) + T018–T022 (per-component in driver-web, then copy) |
| Phase 5 | T028–T029 (tokens + Tailwind), T030–T033 (per-component audit in driver-web, then propagate) |

---

## Parallel Example: User Story 2

```bash
# Init shadcn/ui in all three apps in parallel:
Task: "shadcn/ui init in driver-web"
Task: "shadcn/ui init in partner-dashboard"
Task: "shadcn/ui init in admin-dashboard"

# Build all primitives in driver-web in parallel:
Task: "Button component in driver-web"
Task: "Input component in driver-web"
Task: "Card component in driver-web"
Task: "Modal component in driver-web"
Task: "Map container in driver-web"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 2: Foundational tokens
2. Complete Phase 3: US1 (Tokens + Tailwind + CSS vars)
3. **STOP and VALIDATE**: Verify tokens resolve correctly in all 3 web apps
4. Demo ready — tokens provide immediate value (no more inline hex)

### Incremental Delivery

1. Phase 2 + Phase 3 → Token system live → all 4 apps consume same colors/spacing
2. Add Phase 4 → Button, Input, Card, Modal, Map container ready in all web apps
3. Add Phase 5 → RTL support baked into tokens and components
4. Add Phase 6 → Polish and cross-app verification

---

## Notes

- Components are built once in `driver-web` then copied to `partner-dashboard` and `admin-dashboard` (shadcn/ui convention)
- No shared `packages/ui` component library — maintaining 3 copies is acceptable for 5 primitives
- CSS logical properties are the single RTL implementation — no separate RTL stylesheets
- Dark mode tokens are out of scope for Sprint 8
- `driver-mobile` (React Native) gets tokens only — no components
