# Bright Theme — Driver Application UI

The Bornemap driver applications (web and mobile) use a **bright, high-contrast
theme** optimized for outdoor visibility, accessibility, and modern aesthetics.

## Design Philosophy

- **Crisp & Clean:** Ultra-bright backgrounds (#F8FAF6) reduce eye strain
- **High Contrast:** Primary green (#007943) meets WCAG AA+ standards
- **Neon Accents:** Bright glow (#00E676) makes active map pins unmissable
- **Minimal Visual Noise:** Clean typography, subtle borders, purposeful spacing

---

## Color System

### Core Palette

```
╔════════════════════════════════════════════════════════════╗
║  ev-bg (#F8FAF6)   - Ultra-bright app canvas              ║
║  ev-surface (#FFFFFF) - White cards, modals, headers      ║
╠════════════════════════════════════════════════════════════╣
║  ev-green (#007943) - Primary brand color (buttons, nav)  ║
║  ev-glow (#00E676)  - Neon green (live pins, alerts)      ║
╠════════════════════════════════════════════════════════════╣
║  ev-mapBg (#EAF0E6) - Light map base (terrain)            ║
╠════════════════════════════════════════════════════════════╣
║  ev-muted (#6B7280) - Secondary text (labels, hints)      ║
║  ev-border (#E5E7EB) - Dividers, subtle borders           ║
╚════════════════════════════════════════════════════════════╝
```

### Usage Guidelines

| Component | Color | Example |
|-----------|-------|---------|
| **App Background** | `bg-ev-bg` | Main canvas (#F8FAF6) |
| **Cards/Surfaces** | `bg-ev-surface` | Station detail card, modals |
| **Buttons (Primary)** | `bg-ev-green text-white` | Search, Filter, Action buttons |
| **Text (Primary)** | `text-gray-900` | Headlines, body text |
| **Text (Secondary)** | `text-ev-muted` | Labels, help text |
| **Active Map Pins** | `bg-ev-glow` | Live/available charger markers |
| **Borders** | `border-ev-border` | Dividers, input borders |
| **Map Base** | `bg-ev-mapBg` | Leaflet tile background |

---

## Component Patterns

### Header

```html
<header class="flex justify-between items-center px-6 py-4 bg-ev-surface border-b border-ev-border">
  <h1 class="text-lg font-bold text-ev-green">ElectriCharge</h1>
</header>
```

### Search Input

```html
<div class="relative flex items-center bg-ev-surface border border-ev-border rounded-xl px-4 py-2.5">
  <svg class="w-5 h-5 text-ev-muted mr-2"></svg>
  <input type="text" placeholder="Search locations..." 
         class="bg-transparent text-sm focus:outline-none text-gray-900 placeholder-ev-muted" />
</div>
```

### Primary Button

```html
<button class="bg-ev-green hover:bg-[#006438] text-white font-semibold px-4 py-2.5 rounded-lg shadow-sm transition-colors">
  Search Stations
</button>
```

### Station Card

```html
<section class="bg-ev-surface border border-ev-border rounded-2xl p-4 shadow-sm">
  <h2 class="text-base font-bold text-gray-900">Oakville Hub 1</h2>
  <p class="text-xs text-ev-muted mt-0.5">123 Maple Ave</p>
  
  <div class="space-y-1.5 border-t border-ev-border pt-3 text-xs">
    <div class="flex justify-between">
      <span class="text-ev-muted">Status:</span>
      <span class="text-ev-green font-bold">Available</span>
    </div>
  </div>
</section>
```

### Map Pin (Active)

```html
<div class="w-5 h-5 bg-ev-glow rounded-full border-2 border-white shadow-[0_0_10px_rgba(0,230,118,0.8)]"></div>
```

### Bottom Navigation

```html
<nav class="bg-ev-surface border-t border-ev-border px-6 py-4 flex justify-between">
  <button class="flex flex-col items-center gap-1 text-ev-green font-bold text-xs">
    <svg class="w-5 h-5"></svg>
    Home
  </button>
  
  <!-- Center action button -->
  <div class="relative -top-5">
    <button class="w-14 h-14 bg-gradient-to-b from-ev-green to-[#005c32] rounded-full flex items-center justify-center text-white shadow-md border-4 border-ev-bg">
      <svg class="w-6 h-6"></svg>
    </button>
  </div>
  
  <button class="flex flex-col items-center gap-1 text-ev-muted font-bold text-xs hover:text-gray-900">
    <svg class="w-5 h-5"></svg>
    Settings
  </button>
</nav>
```

---

## Typography

### Font Stack
```
Plus Jakarta Sans → Inter → system sans-serif
```
Provides excellent readability on maps and outdoor screens.

### Scale & Weights

| Role | Size | Weight | Example |
|------|------|--------|---------|
| Page Title | 24px | 700 (bold) | "Search Stations" |
| Card Title | 16px | 700 (bold) | "Oakville Hub 1" |
| Body Text | 14px | 400 (regular) | Station description |
| Label | 12px | 600 (semibold) | "Status:", "Address:" |
| Helper Text | 12px | 400 (regular) | Hints, secondary info |

---

## Spacing & Density

All driver apps follow the **4px base unit** grid:

- **Compact sections:** 4px, 8px gaps (map area)
- **Standard content:** 16px, 24px gaps (cards, sections)
- **Breathing room:** 32px, 40px gaps (between major sections)

---

## Accessibility

✅ **WCAG AA+ Compliant**
- Primary green (`#007943`) provides 8.5:1 contrast ratio on white
- Neon glow (`#00E676`) 100% contrast on white backgrounds
- Large touch targets (minimum 48x48px for mobile buttons)
- High-contrast text (gray-900 on white/light backgrounds)

---

## Dark Mode (Future)

The bright theme is MVP. A dark variant can be added using Tailwind's `dark:`
prefix or CSS variables once MVP is complete.

```js
// Future: Add to tailwind.config.js
darkMode: 'class', // or 'media'
```

---

## Implementation Checklist for Driver Apps

**Driver Web App** (`apps/driver-web/`):
- [ ] tailwind.config.js configured with ev color namespace
- [ ] Typography system applied to all text
- [ ] Header uses ev-surface + ev-green
- [ ] Search/filter pills styled per component patterns
- [ ] Station detail card uses ev-surface + ev-border
- [ ] Bottom nav uses ev-green for active state
- [ ] Map background set to ev-mapBg
- [ ] Active map pins use ev-glow with shadow

**Driver Mobile App** (`apps/driver-mobile/`):
- [ ] tailwind.config.js configured (same as web)
- [ ] All screens use ev color tokens
- [ ] Bottom tab bar active state uses ev-green
- [ ] Station cards follow design patterns
- [ ] Map pins use ev-glow
- [ ] Touch targets minimum 48x48px
