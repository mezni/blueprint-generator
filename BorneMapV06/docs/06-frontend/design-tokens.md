# Design Tokens

Design tokens are defined in `packages/ui/src/tokens/` and exported to both
web and mobile apps. The bright theme brings a clean, modern aesthetic to the
driver applications.

## Bright Theme Color Palette

The bright theme uses a crisp, high-contrast palette optimized for outdoor
visibility and accessibility:

| Token | Color | Usage |
|-------|-------|-------|
| `ev-bg` | #F8FAF6 | Ultra-bright app background canvas |
| `ev-surface` | #FFFFFF | Clean white card layers, modals |
| `ev-mapBg` | #EAF0E6 | Light organic map terrain base |
| `ev-green` | #007943 | Primary brand green (high-contrast) |
| `ev-glow` | #00E676 | Neon green for active/live map pins |
| `ev-muted` | #6B7280 | Slate gray for secondary text labels |
| `ev-border` | #E5E7EB | Subtle light gray divider lines |

### Implementation in Tailwind

All driver applications use `tailwind.config.js` with the `ev` color namespace:

```js
colors: {
  ev: {
    bg: '#F8FAF6',
    surface: '#FFFFFF',
    mapBg: '#EAF0E6',
    green: '#007943',
    glow: '#00E676',
    muted: '#6B7280',
    border: '#E5E7EB',
  }
}
```

Use in templates: `bg-ev-bg`, `text-ev-green`, `border-ev-border`, etc.

## Typography (`typography.ts`)

- **Font family:** Plus Jakarta Sans, Inter, system UI sans-serif stack
  (optimized for readability on maps and mobile)
- **Scale:** 12/14/16/18/20/24/30/36px
- **Weights:** 400 (regular), 500 (medium), 600 (semibold), 700 (bold)

## Spacing (`spacing.ts`)

- 4px base unit
- Scale: 0/1/2/3/4/5/6/8/10/12/16/20/24/32/40/48/64

## Border Radius (`radius.ts`)

- sm: 4px
- md: 8px
- lg: 12px
- xl: 16px
- full: 9999px

## Shadows (`shadows.ts`)

- sm/md/lg/xl following Tailwind CSS conventions
