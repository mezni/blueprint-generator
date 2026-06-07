# Research: Design System Foundation

**Date**: 2026-06-02

## Overview

No open technical questions required research — all technology choices follow established patterns from the monorepo's frontend conventions (React + Vite, Tailwind CSS, shadcn/ui, Leaflet). This document records the key design decisions for reference.

## Architecture Decisions

### Decision: shadcn/ui as the primitive component library

**Decision**: Use shadcn/ui (which wraps Radix UI primitives) for all components: Button, Input, Card, Modal. shadcn/ui provides unstyled, accessible primitives with Tailwind CSS integration, matching the project's existing frontend stack.

**Rationale**: shadcn/ui is the standard React primitive library for Tailwind-based projects. It provides full accessibility (ARIA, keyboard nav, focus management), supports RTL via Radix's direction context, and generates local component code rather than a black-box dependency — allowing customization of Modal backdrop, Card shadow, and Input ring tokens.

**Alternatives considered**: Headless UI (less RTL support), MUI (heavy, conflicts with Tailwind), rolling own primitives (too much effort for 5 components).

---

### Decision: CSS custom properties for runtime token distribution

**Decision**: Tokens are defined as TypeScript constants in `@bornemap/design-tokens`, then exported as CSS custom properties via a `:root` block injected by each app's global CSS.

**Rationale**: CSS custom properties are the standard runtime token distribution mechanism for web apps. They are inheritable, dynamic, and work with Tailwind's `theme()` function. TypeScript exports provide type safety for component code.

**Alternatives considered**: PostCSS plugin (less runtime-flexible), JavaScript module only (no Tailwind integration), SASS variables (not runtime-overridable).

---

### Decision: RTL via CSS logical properties at the token layer

**Decision**: All spacing/alignment tokens map to CSS logical properties (`padding-inline`, `margin-inline-start`, `inset-inline-end`) via Tailwind v3.3+ logical property utilities. Radix's `DirectionProvider` handles component-level RTL.

**Rationale**: Logical properties are the only CSS-native RTL solution. They invert automatically based on `dir` attribute with zero JavaScript. Tailwind generates both LTR and RTL output from the same utility classes.

**Alternatives considered**: CSS `transform: scaleX(-1)` — fragile, breaks text. Separate LTR/RTL stylesheets — duplication. JavaScript-based flipping — runtime overhead.

---

### Decision: Components live in a shared location (no separate component package)

**Decision**: Component primitives are developed directly in each web app's `src/components/ui/` directory, following the shadcn/ui convention of copy-paste components. Token values are the single source of truth; each app imports tokens and the same shadcn/ui config.

**Rationale**: shadcn/ui components are designed to be copied into each project (they're local code, not an npm dependency). Since all three web apps share the same Tailwind config and token package, the components will render identically. A shared component package would add build complexity (library bundling, tree-shaking, versioning) that's not warranted for 5 primitives.

**Alternatives considered**: Shared `packages/ui` component library — adds build pipeline complexity. Monorepo `ui` workspace — premature for 5 components.
