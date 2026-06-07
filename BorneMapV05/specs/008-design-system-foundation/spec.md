# Feature Specification: Design System Foundation

**Feature Branch**: `008-design-system-foundation`

**Created**: 2026-06-02

**Status**: Draft

## Clarifications

### Session 2026-06-02

- Q: Are the 5 listed components (Button, Input, Card, Modal, Map container) the only primitives for Sprint 8? → A: Only the 5 listed. Additional primitives deferred to Sprint 9–11 app builds.

**Input**: Sprint 8 — Design System Foundation: create a reusable design token system, utility class theme mapping, primitive component library integration, and foundational components (Button, Input, Card, Modal, Map container shell) with RTL-ready foundation. All four frontends consume the same token/theme base.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Design Tokens & Theme Consistency (Priority: P1)

A developer building a new screen can pull colors, spacing, typography, and shadows from a single source of truth without guessing or copying from other screens.

**Why this priority**: Without tokens, every developer picks inline hex values and arbitrary spacing, creating visual drift that is expensive to fix later. Tokens are the prerequisite for all component work.

**Independent Test**: A test page renders Button, Input, and Card components on all three web frontends (driver-web, partner-dashboard, admin-dashboard) with identical color, spacing, and typography values verified by pixel comparison of CSS custom property values. Mobile (driver-mobile) consumes tokens at the package level only; component parity deferred to Sprint 12.

**Acceptance Scenarios**:

1. **Given** the design-tokens package exports raw values (colors, spacing, typography, shadows, border-radius), **When** a frontend app imports the tokens, **Then** all token values are accessible as typed constants
2. **Given** the utility class system maps tokens to class names, **When** a developer uses `bg-primary` or `text-body`, **Then** the resolved value matches the token definition
3. **Given** a developer needs a new variant, **When** they update tokens in the single source, **Then** all consuming apps reflect the change
4. **Given** the system, **When** a developer inspects rendered HTML, **Then** there are zero inline hex colors, arbitrary spacing values, or hardcoded typography

---

### User Story 2 - Reusable Component Primitives (Priority: P1)

A developer building a form can drop in a pre-styled Button, Input, and Card without writing their own styles, and these components match the design system exactly.

**Why this priority**: Primitive components are the building blocks for every screen. Without them, every developer reimplements the same components with inconsistent styling.

**Independent Test**: A test form using Button, Input, and Card renders identically in driver-web, partner-dashboard, and admin-dashboard with the same spacing, colors, font sizes, and border radius.

**Acceptance Scenarios**:

1. **Given** a Button component, **When** rendered with `variant="primary"`, **Then** the background color matches `color-primary` from tokens
2. **Given** an Input component, **When** in a focused state, **Then** the border color matches `color-focus-ring` from tokens
3. **Given** a Card component, **When** rendered with default props, **Then** the shadow matches `shadow-card` from tokens and corner radius matches `radius-card`
4. **Given** a Modal component, **When** opened, **Then** it renders with a backdrop overlay, centered content, and close button; focus is trapped inside
5. **Given** a Map container shell component, **When** rendered, **Then** it provides a full-height container for an interactive map and exposes mount/unmount lifecycle hooks
6. **Given** any Button, Input, or Card, **When** inspected in browser devtools, **Then** all style values reference design token variables (not hardcoded values)

---

### User Story 3 - RTL-Ready Foundation (Priority: P2)

An Arabic-speaking driver opens the web app and sees the layout mirrored correctly with proper text alignment, spacing, and reading direction.

**Why this priority**: Tunisia is bilingual (Arabic + French). RTL support cannot be retrofitted — it must be baked into the token and component layer from day one.

**Independent Test**: Toggle a `dir="rtl"` attribute on the document and verify that Card text alignment, Input padding direction, and Modal close button position flip correctly.

**Acceptance Scenarios**:

1. **Given** the design tokens, **When** spacing tokens exist, **Then** they use logical (direction-agnostic) properties to handle RTL natively
2. **Given** the utility class system, **When** configured, **Then** it provides utilities for logical properties (inline-start/inline-end) instead of physical directions (left/right)
3. **Given** a Card or Modal, **When** rendered in RTL mode, **Then** text alignment, icon positions, and padding directions mirror correctly without CSS overrides
4. **Given** an Input component, **When** in RTL mode, **Then** the text cursor starts on the right and padding-inline flips

---

### Out of Scope

Sprint 8 is limited to the design system foundation only. The following are explicitly deferred:

- Additional component primitives beyond the 5 listed (Dropdown, Select, Tabs, Tooltip, Badge, Switch, Toast, etc.) — covered in Sprint 9–11 app builds
- Form layout components (Form, Field, Label, ErrorMessage) — will use primitive Input + Card composition in app sprints
- Navigation components (Sidebar, Navbar, Tabs, Breadcrumbs) — app-specific layout decisions
- Data display components (Table, List, DataGrid, Charts) — app-specific needs
- Animation system (enter/exit transitions, page transitions) — deferred until UX polish phase
- Mobile-specific design tokens or RTL handling — React Native Expo has separate RTL mechanism (I18nManager); mobile design tokens will be addressed in Sprint 12
- WCAG compliance auditing — covered in Sprint 16 hardening sprint

### Edge Cases

- What happens when a component is used without a theme provider? It should fall back to default tokens gracefully, not crash
- What happens when an app overrides a token? The override should merge with defaults, not replace them entirely
- What happens in a deeply nested modal (modal on top of modal)? Focus should be trapped in the topmost modal; escape closes one layer at a time
- What happens when Map container is resized or remounted? The map instance should invalidate its size and re-render without memory leaks
- What happens with missing fonts (Arabic glyphs not loaded)? The system degrades to system fallback fonts without layout shift
- What happens when RTL is toggled at runtime? Layout should reflow smoothly, not snap abruptly
- What are the Input component states? Default, focused (accent border via `color-focus-ring`), error (red border + error message below), disabled (muted background, no interaction), full-width by default with optional `className` override
- What Button variants exist? `primary` (solid accent background), `secondary` (outline with text color), `ghost` (transparent, no border) — each with `sm`, `md`, `lg` size variants

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: A `design-tokens` package MUST export raw token values as typed constants covering: colors (primary, secondary, accent, success, warning, error, surface, text, border), spacing scale (4/8/12/16/20/24/32/48/64), typography (font family, size scale, weight, line-height), shadows (sm/md/lg/card/modal), border-radius (sm/md/lg/full)
- **FR-002**: Tokens MUST map to the utility class system so that utility names like `bg-primary`, `text-body`, `p-4`, `shadow-card`, `rounded-lg` resolve to token values
- **FR-003**: Tokens MUST be consumed by all four frontend apps (driver-web, partner-dashboard, admin-dashboard, driver-mobile) from a single source of truth
- **FR-004**: A Modal primitive MUST provide open/close, backdrop overlay, focus trap, escape-to-close, and portal rendering
- **FR-005**: A Map container shell component MUST mount/unmount an interactive map, expose the map instance via ref or callback, handle resize, and clean up on unmount
- **FR-006**: All spacing and alignment MUST use CSS logical properties instead of physical directions to support RTL natively
- **FR-007**: The utility class system MUST provide logical property aliases (inline-start/inline-end) instead of physical direction classes (left/right)
- **FR-008**: Components MUST NOT contain inline hex colors, arbitrary spacing values, or hardcoded typography — all visual properties MUST reference tokens

### Key Entities

- **Design Token**: A named atomic value (color, spacing, typography, shadow) that represents a single design decision. Tokens are the source of truth for all visual properties.
- **Component Primitive**: A reusable React component (Button, Input, Card, Modal, Map container) that implements the design system tokens. These are the building blocks for all screens.
- **Theme Package**: A configuration layer that maps raw tokens to framework-specific formats (utility theme, runtime variables, module exports) so that any frontend app can consume tokens in its preferred way.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All four frontend apps can import and use the same `design-tokens` package with zero configuration drift
- **SC-002**: A Button, Input, Card, Modal, and Map container render with identical styling across all web apps (verified by rendering each in isolation)
- **SC-003**: Toggling `dir="rtl"` on any page with primitive components correctly flips layout direction without custom RTL CSS overrides
- **SC-004**: A developer can create a new screen using only primitive components and token classes without writing any custom CSS
- **SC-005**: Map container mounts an interactive map within 500ms and cleans up all event listeners on unmount (no memory leaks)

## Assumptions

- An accessible, unstyled primitive component library is available for building foundational components
- The frontend framework supports component composition and state management (per established frontend tooling)
- An interactive map library is available (platform decision in Constitution)
- The `packages/design-tokens` directory already exists as a stub from Sprint 1
- A utility-first CSS framework is available (per established frontend tooling)
- The four frontend apps exist in the monorepo at `apps/{driver-web,partner-dashboard,admin-dashboard,driver-mobile}`
- RTL support targets Arabic as the primary RTL language; other RTL languages may be supported later
- Runtime token distribution (e.g., CSS variables) is used for web apps to consume tokens at runtime
