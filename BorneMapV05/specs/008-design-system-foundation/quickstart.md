# Design System — Quickstart

## Using Design Tokens

```ts
import { colors, spacing, typography } from "@bornemap/design-tokens";

// Access typed values
const primaryColor = colors.primary.base; // "#2563EB"
const paddingMd = spacing[16]; // "16px"
```

## Using in Tailwind

```ts
// tailwind.config.ts
import { colors, spacing, typography, shadows, borderRadius } from "@bornemap/design-tokens";

export default {
  theme: {
    extend: {
      colors: { ...colors },
      spacing: { ...spacing },
      fontFamily: { ...typography.fontFamily },
      fontSize: { ...typography.fontSize },
      boxShadow: { ...shadows },
      borderRadius: { ...borderRadius },
    },
  },
};
```

Then use in templates: `bg-primary`, `text-body`, `p-4`, `shadow-card`, `rounded-lg`.

## Using Components

Each web app has its own `src/components/ui/` directory following the shadcn/ui convention. Import the same component from your app's local copy:

```tsx
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card } from "@/components/ui/card";
import { Modal } from "@/components/ui/modal";
import { MapContainer } from "@/components/ui/map-container";
```

### Button

```tsx
<Button variant="primary" onClick={handleClick}>
  Add Station
</Button>
<Button variant="secondary" size="sm">Cancel</Button>
```

### Input

```tsx
<Input placeholder="Search stations..." />
<Input error="Required field" />
```

### Card

```tsx
<Card>
  <Card.Header>Station Details</Card.Header>
  <Card.Content>Content here</Card.Content>
  <Card.Footer>Actions</Card.Footer>
</Card>
```

### Modal

```tsx
<Modal open={isOpen} onClose={() => setOpen(false)}>
  <Modal.Header>Confirm Delete</Modal.Header>
  <Modal.Content>Are you sure?</Modal.Content>
  <Modal.Footer>
    <Button variant="secondary" onClick={() => setOpen(false)}>Cancel</Button>
    <Button variant="primary">Confirm</Button>
  </Modal.Footer>
</Modal>
```

### Map Container

```tsx
<MapContainer className="h-[500px]" onMount={(map) => console.log("Map ready", map)} />
```

## RTL Support

RTL works automatically when `dir="rtl"` is set on the `<html>` element:

```html
<html dir="rtl" lang="ar">
```

All components use CSS logical properties — padding, margin, and text alignment flip without custom CSS.

## Available Tokens

| Category | Tokens |
|----------|--------|
| Colors | primary, secondary, accent, success, warning, error, surface, text, border (each with base/hover/active/muted) |
| Spacing | 4, 8, 12, 16, 20, 24, 32, 48, 64 (px) |
| Typography | font-family (sans, mono), font-size (xs–4xl), font-weight (normal–bold), line-height (none–relaxed) |
| Shadows | sm, md, lg, card, modal |
| Border-radius | sm, md, lg, full |
