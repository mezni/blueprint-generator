# Data Model: Design System Foundation

**Date**: 2026-06-02

## Overview

The design system data model defines the structure of design tokens — typed constants that represent every visual decision in the platform. Tokens are the single source of truth; no inline hex values or arbitrary spacing values are permitted.

## Entity: Color Token

| Field | Type | Example | Notes |
|-------|------|---------|-------|
| name | `string` | `"primary"` | Semantic identifier |
| value | `string` | `"#2563EB"` | Hex, rgb(), or hsl() string |
| category | `"base" \| "hover" \| "active" \| "muted"` | `"base"` | Variant modifier |

**Scale**: 
- Primary, secondary, accent, success, warning, error, surface, text, border
- Each base color has hover/active/muted variants
- Dark mode variants (deferred unless specified otherwise)

---

## Entity: Spacing Token

| Field | Type | Example | Notes |
|-------|------|---------|-------|
| name | `string` | `"4"` | Numeric key matching Tailwind convention |
| value | `string` | `"4px"` | CSS length value |

**Scale**: 4, 8, 12, 16, 20, 24, 32, 48, 64 (pixels)

---

## Entity: Typography Token

| Field | Type | Example | Notes |
|-------|------|---------|-------|
| name | `string` | `"font-family-sans"` | Token identifier |
| value | `string` | `"Inter, system-ui, sans-serif"` | CSS value |

**Sub-categories**:
- `font-family`: sans, mono
- `font-size`: xs, sm, base, lg, xl, 2xl, 3xl, 4xl
- `font-weight`: normal, medium, semibold, bold
- `line-height`: none, tight, normal, relaxed

---

## Entity: Shadow Token

| Field | Type | Example | Notes |
|-------|------|---------|-------|
| name | `string` | `"card"` | Semantic name |
| value | `string` | `"0 1px 3px 0 rgb(0 0 0 / 0.1)"` | CSS box-shadow value |

**Scale**: sm, md, lg, card, modal

---

## Entity: Border-Radius Token

| Field | Type | Example | Notes |
|-------|------|---------|-------|
| name | `string` | `"lg"` | Semantic name |
| value | `string` | `"8px"` | CSS length value |

**Scale**: sm, md, lg, full (pill/round)

---

## Runtime Distribution

Tokens are exported in two forms:
1. **TypeScript constants** — typed objects for component logic
2. **CSS custom properties** — generated `:root { --color-primary: #2563EB; ... }` block for Tailwind theme() consumption and runtime override

```typescript
// TypeScript (src/colors.ts)
export const colors = {
  primary: { base: "#2563EB", hover: "#1D4ED8", active: "#1E40AF", muted: "#BFDBFE" },
  // ...
} as const;

// Generated CSS (src/css.ts)
// :root {
//   --color-primary: #2563EB;
//   --color-primary-hover: #1D4ED8;
//   --color-primary-active: #1E40AF;
//   --color-primary-muted: #BFDBFE;
//   --spacing-4: 4px;
//   --font-size-base: 16px;
//   ...
// }
```
