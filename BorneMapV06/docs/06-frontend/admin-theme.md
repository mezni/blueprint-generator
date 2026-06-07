# Admin & Partner Dashboard Theme System

## Overview

The admin and partner dashboards use an **extended light-mode theme** built on the same organic design foundation as the driver apps, with specialized utilities for operational monitoring, data visualization, and fleet management.

**Design Principles:**
- **Operational clarity** — Real-time metrics, charts, and status tables at a glance
- **Data density** — Multi-column layouts with persistent sidebars for context
- **Brand consistency** — Emerald green (#007943) as primary action color
- **Professional aesthetics** — Clean white surfaces, subtle borders, crisp typography

---

## Color Palette

### Core Colors

| Token | Hex | Usage | Contrast |
|-------|-----|-------|----------|
| `admin-bg` | #F8FAF6 | Page background canvas | N/A |
| `admin-sidebar` | #FFFFFF | Navigation sidebar, persistent context | - |
| `admin-card` | #FFFFFF | Panel backgrounds, elevated surfaces | - |
| `admin-emerald` | #007943 | Primary brand, active states, CTAs | 8.5:1 (WCAG AAA) |
| `admin-sageLight` | #EAF0E6 | Selected tabs, chart highlights, map regions | - |
| `admin-textMain` | #111827 | Body text, data rows, labels | 18:1 (WCAG AAA) |
| `admin-textMuted` | #6B7280 | Secondary text, column headers, metadata | 9:1 (WCAG AA) |
| `admin-border` | #E5E7EB | Divider lines, table row separators | - |

### Operational Status Colors

| Token | Hex | Status | Icon |
|-------|-----|--------|------|
| `admin-statusGreen` | #10B981 | Available / Healthy | 🟢 |
| `admin-statusOrange` | #F59E0B | In Use / Active | 🟠 |
| `admin-statusRed` | #EF4444 | Needs Maintenance / Error | 🔴 |

---

## Layout System

### Three-Column Architecture

```
┌─────────────────────────────────────────────┐
│  Sidebar (w-64)  │  Header (full width)     │
├─────────────────────────────────────────────┤
│                  │                           │
│   Persistent     │  Main Content Grid       │ Right Sidebar
│   Navigation     │  ┌──────────────┬──────┐ │ h-[260px]
│                  │  │              │      │ │
│   - Overview     │  │  Charts &    │ User │ │
│   - Stations     │  │  Data        │ Logs │ │
│   - Users        │  │  (2 cols)    │      │ │
│   - Analytics    │  │              │      │ │
│                  │  └──────────────┴──────┘ │ Station
│   [Settings]     │                          │ Hardpoint
│                  │                          │ Monitor
│                  │                          │
└─────────────────────────────────────────────┘
```

**Responsive Breakpoints:**
- Desktop (xl): Full 3-column layout
- Tablet (md): Main content grid collapses to 1 column, sidebar hidden or drawer-based
- Mobile: Sidebar drawer, main content single column, right sidebar pushed to bottom

---

## Component Patterns

### 1. Sidebar Navigation

```html
<aside class="w-64 bg-white border-r border-admin-border flex flex-col justify-between z-10">
  <!-- Brand Header -->
  <div class="px-6 py-5 border-b border-gray-100 flex items-center gap-2">
    <div class="w-8 h-8 rounded-lg bg-admin-emerald flex items-center justify-center text-white">
      <!-- Brand Icon (e.g., lightning bolt) -->
    </div>
    <span class="font-extrabold text-base tracking-tight text-admin-emerald">
      ElectriCharge <span class="font-medium text-admin-textMuted text-xs">Admin</span>
    </span>
  </div>

  <!-- Navigation Links -->
  <nav class="p-4 space-y-1 text-sm font-semibold text-admin-textMuted">
    <!-- Default state -->
    <a href="#" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-gray-50 hover:text-admin-textMain">
      <svg class="w-5 h-5"><!-- Icon --></svg>
      Overview
    </a>

    <!-- Active state -->
    <a href="#" class="flex items-center gap-3 px-4 py-2.5 rounded-lg bg-admin-sageLight text-admin-emerald">
      <svg class="w-5 h-5"><!-- Icon --></svg>
      Analytics
      <span class="ml-auto text-xs font-normal bg-gray-100 text-admin-textMuted px-2 py-0.5 rounded-full">
        120
      </span>
    </a>
  </nav>

  <!-- Bottom Actions -->
  <div class="p-4 border-t border-admin-border text-sm font-semibold text-admin-textMuted space-y-1">
    <a href="#" class="flex items-center gap-3 px-4 py-2 rounded-lg hover:bg-gray-50 hover:text-admin-textMain">
      <svg class="w-5 h-5"><!-- Settings Icon --></svg>
      Settings
    </a>
  </div>
</aside>
```

**Styling Rules:**
- Default text: `text-admin-textMuted`
- Hover: `hover:bg-gray-50 hover:text-admin-textMain`
- Active: `bg-admin-sageLight text-admin-emerald`
- Badge counters: `bg-gray-100 text-admin-textMuted text-xs`

---

### 2. Top Header Bar

```html
<header class="h-16 bg-white border-b border-admin-border flex items-center justify-between px-8 z-10">
  <!-- Tab Navigation -->
  <div class="flex gap-6 text-sm font-bold text-admin-textMuted">
    <button class="hover:text-admin-textMain">Overview</button>
    <button class="text-admin-emerald border-b-2 border-admin-emerald h-16 flex items-center">
      Analytics
    </button>
  </div>

  <!-- User Info & Avatar -->
  <div class="flex items-center gap-3">
    <span class="text-xs font-semibold text-admin-textMuted">
      Operator: <span class="text-admin-textMain font-bold">S. Chen</span>
    </span>
    <div class="w-8 h-8 rounded-full bg-gray-200 border border-admin-border overflow-hidden">
      <img src="avatar.jpg" alt="Avatar" class="w-full h-full object-cover">
    </div>
  </div>
</header>
```

---

### 3. Data Cards (Charts & Tables)

```html
<!-- Basic Card Container -->
<div class="bg-admin-card border border-admin-border rounded-2xl p-5 shadow-sm">
  <!-- Card Header -->
  <div class="flex justify-between items-center mb-4">
    <h3 class="text-xs uppercase tracking-wider text-admin-textMuted font-bold">
      Live Revenue
    </h3>
    <button class="text-admin-textMuted hover:text-admin-textMain font-bold">···</button>
  </div>

  <!-- Content Area -->
  <div class="h-44 flex items-end justify-between px-2 relative border-b border-admin-border">
    <!-- Chart.js / Recharts Placeholder -->
    <div class="absolute inset-0 flex items-center justify-center text-xs text-admin-textMuted">
      [Revenue Trend Line Sparkline]
    </div>
  </div>

  <!-- Footer Labels -->
  <div class="flex justify-between text-[10px] text-admin-textMuted font-medium pt-2">
    <span>09:00</span><span>12:00</span><span>15:00</span><span>18:00</span>
  </div>
</div>
```

**Card Styling:**
- Background: `bg-admin-card`
- Border: `border border-admin-border`
- Radius: `rounded-2xl`
- Padding: `p-5`
- Shadow: `shadow-sm`

---

### 4. Status Badges

```html
<!-- Available Status -->
<span class="text-xs bg-emerald-50 text-admin-emerald px-2 py-0.5 rounded-full flex items-center gap-1">
  <span class="w-1 h-1 rounded-full bg-admin-statusGreen"></span>
  Available
</span>

<!-- In Use Status -->
<span class="text-xs bg-amber-50 text-amber-700 px-2 py-0.5 rounded-full flex items-center gap-1">
  <span class="w-1 h-1 rounded-full bg-admin-statusOrange"></span>
  In Use
</span>

<!-- Maintenance Status -->
<span class="text-xs bg-red-50 text-red-700 px-2 py-0.5 rounded-full flex items-center gap-1">
  <span class="w-1 h-1 rounded-full bg-admin-statusRed"></span>
  Maintenance
</span>
```

---

### 5. Data Tables

```html
<table class="w-full text-left border-collapse text-xs">
  <!-- Table Header -->
  <thead>
    <tr class="text-admin-textMuted border-b border-admin-border">
      <th class="pb-2 font-semibold">Station Name</th>
      <th class="pb-2 font-semibold">Status</th>
      <th class="pb-2 font-semibold">Users</th>
    </tr>
  </thead>

  <!-- Table Body -->
  <tbody class="divide-y divide-admin-border font-medium text-admin-textMain">
    <tr class="hover:bg-gray-50">
      <td class="py-2.5">Oakville Hub 1</td>
      <td class="py-2.5">
        <span class="inline-flex items-center gap-1 text-admin-emerald">
          <span class="w-1 h-1 rounded-full bg-admin-statusGreen"></span>
          Available
        </span>
      </td>
      <td class="py-2.5 text-admin-textMuted">3 / 8</td>
    </tr>
  </tbody>
</table>
```

---

### 6. Call-to-Action Cards

```html
<!-- Download Promo Card -->
<div class="bg-gradient-to-br from-admin-emerald to-[#005c32] rounded-2xl p-6 text-white flex flex-col justify-between shadow-md relative overflow-hidden">
  <div>
    <h3 class="text-xl font-black tracking-tight mb-1">Download Now!</h3>
    <p class="text-xs opacity-90 font-medium">
      Manage on the move with native client companion applications.
    </p>
  </div>

  <div class="flex gap-2 mt-4">
    <button class="bg-black/20 hover:bg-black/30 text-[10px] font-bold py-2 px-3 rounded-lg border border-white/20 text-center flex-1">
      App Store
    </button>
    <button class="bg-black/20 hover:bg-black/30 text-[10px] font-bold py-2 px-3 rounded-lg border border-white/20 text-center flex-1">
      Google Play
    </button>
  </div>
</div>
```

---

## Typography

| Element | Font | Size | Weight | Line Height |
|---------|------|------|--------|------------|
| Page Title | Plus Jakarta Sans | 2xl (24px) | 800 | 1.2 |
| Card Header | Plus Jakarta Sans | xs (12px) | 700 | 1.4 |
| Body Text | Inter | sm (14px) | 500 | 1.5 |
| Muted Text | Inter | xs (12px) | 400 | 1.4 |
| Tab Label | Plus Jakarta Sans | sm (14px) | 700 | 1.5 |
| Table Cell | Inter | xs (12px) | 500 | 1.4 |

---

## Spacing & Grid

**Base Unit:** 4px

| Scale | Value | Usage |
|-------|-------|-------|
| xs | 0.5rem (8px) | Small gaps between inline elements |
| sm | 1rem (16px) | Padding within cards, gaps between small components |
| md | 1.5rem (24px) | Card padding, section spacing |
| lg | 2rem (32px) | Page padding, major section gaps |
| xl | 3rem (48px) | Layout margins |

**Grid System:**
- Sidebar: `w-64` (256px)
- Header height: `h-16` (64px)
- Card height: Flexible, min-height `h-[260px]` for right sidebar sections
- Main content gaps: `gap-6` (24px)

---

## Implementation Checklist

### Tailwind Configuration
- [ ] Add `admin` color namespace to `tailwind.config.js`
- [ ] Configure font family with Plus Jakarta Sans + Inter
- [ ] Set up spacing scale (4px base unit)
- [ ] Create rounded-2xl utility for card radius

### Dashboard Scaffolding
- [ ] Create layout wrapper with 3-column grid
- [ ] Build persistent sidebar with navigation
- [ ] Add top header bar with tabs and user profile
- [ ] Create main content area with responsive grid
- [ ] Add right sidebar for logs and monitoring

### Components
- [ ] Data card container with header/footer
- [ ] Chart.js or Recharts integration placeholders
- [ ] Status badge system (green/orange/red)
- [ ] Data table with sorting/filtering
- [ ] Tab navigation component
- [ ] User avatar component with fallback

### Responsive Behavior
- [ ] Desktop (xl): Full 3-column layout
- [ ] Tablet (md): 1-column main content, sidebar drawer
- [ ] Mobile: Drawer navigation, bottom sheet for right sidebar

### Accessibility
- [ ] WCAG AA compliance for all text/background pairs
- [ ] Semantic HTML (nav, header, main, table)
- [ ] ARIA labels for icon-only buttons
- [ ] Keyboard navigation for sidebar and tabs
- [ ] Focus states for interactive elements

---

## Dark Mode (Optional Future)

If dark mode is implemented, maintain the same token structure:

```javascript
// Future dark mode extension
colors: {
  adminDark: {
    bg: '#0F172A',
    sidebar: '#1A2538',
    card: '#1E293B',
    emerald: '#00E676',
    textMain: '#F1F5F9',
    textMuted: '#94A3B8',
    border: '#334155',
  }
}
```

---

## Figma Design Reference

[Link to Figma admin dashboard design system]

---

## Related Documentation

- **Driver App Theme:** `docs/06-frontend/bright-theme.md`
- **Design Tokens:** `docs/06-frontend/design-tokens.md`
- **Applications Overview:** `docs/06-frontend/applications.md`
