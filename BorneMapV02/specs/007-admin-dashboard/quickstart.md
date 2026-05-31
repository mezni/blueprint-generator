# Quickstart: Admin Dashboard

## Prerequisites

- Node.js v24.16.0, npm v11.13.0
- Web browser (admin-dashboard, web-driver)
- Expo Go (mobile-driver) or a web browser

## Getting Started

```bash
# Install admin dashboard dependencies
cd apps/admin-dashboard && npm install

# Start the Vite dev server
npm run dev

# In another terminal — start web-driver (if not already running)
cd apps/web-driver && npm start

# In another terminal — start mobile-driver (if not already running)
cd apps/mobile-driver && npm start
```

## What's Included

### Admin Dashboard (browser)
Open `http://localhost:5173`. You should see:
- **Top status bar**: "BorneMap Sandbox Master Console (No Integration)" + MOCK ENGINE ACTIVE badge
- **Left sidebar**: OVERVIEW, ENTITIES (collapsible → PARTNERS, STATIONS), USERS, ANALYTICS, SETTINGS, LOGS
- **Overview tab**: Three metric cards — PARTNERS (148), STATIONS (1,240), MOCK TELEMETRY HITS (Offline)
- **Partners tab**: Data table with inline text search
- **Stations tab**: Data table with inline text search
- **Other tabs**: Fallback view indicating mock-rendered state

### Desktop Web Driver (browser)
Open `http://localhost:8081`. You should see:
- **NavBar**: ABOUT, APP, MAP, CONTACT + REGISTER NOW button
- **Map**: Full-screen Leaflet map centered on Tunis
- **Search overlay**: Centered with filter pills (Fast 50kW+, CCS2, Available)
- **Zoom controls**: Circular +/− buttons on the right
- **Detail card**: Bottom-center popup on marker click

### Mobile Driver (Expo Go)
Scan QR code from `npm start`. You should see:
- **Header**: ☰ BORNE MAP + 👤 Reg button
- **Map**: Full-screen native MapView with markers
- **Zoom controls**: Circular +/− buttons on the right
- **Bottom sheet**: Slides up 35% on marker tap

## Testing

All testing is manual visual verification per acceptance scenarios in spec.md. No automated test framework is required for sandbox mode.

## CI Integration

No CI changes required — all three apps build independently. Admin dashboard uses Vite (`npm run build` outputs to `dist/`).
