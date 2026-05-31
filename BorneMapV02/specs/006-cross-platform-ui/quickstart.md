# Quickstart: Cross-Platform UI Synchronization

## Prerequisites

- Node.js v24.16.0, npm v11.13.0
- Expo Go (mobile) or a web browser (desktop)
- Backend running: `docker compose -f deployments/docker-compose.yml up -d`

## Getting Started

```bash
# Install dependencies
cd apps/mobile-driver && npm install

# Start the Expo dev server (desktop web)
npm start

# Or start with LAN tunnel for mobile testing
npm run start:lan

# Or start with ngrok tunnel for physical devices
npm run start:tunnel
```

## What's Included

### Desktop Web (browser)

Open `http://localhost:8081` (or Expo dev server URL). You should see:

- **NavBar** at the top with 4 items: Map, Explore, Saved, Profile
- **Map viewport** with Leaflet tiles centered on Tunis
- **Search bar** + filter controls overlaid on the map
- **Zoom controls** (inline, bottom-right)
- **FAB** (floating action button, bottom-center)
- **Station detail panel** (bottom, fixed-height) — opens on marker tap

### Mobile (iOS/Android via Expo Go)

Scan the QR code from the Expo dev server. You should see:

- **Bottom tab bar** with 4 items: Map, Explore, Saved, Profile
- **Compact header** with app title
- **Map viewport** with native maps (`react-native-maps`)
- **Search bar** + filter controls overlaid on the map
- **FAB** (bottom-center, mobile-styled)
- **Bottom sheet** (draggable, peek 120px / expanded 70%) — opens on marker tap

## API Dependencies

The UI requires these backend endpoints (defined in `contracts/api.yaml`):

| Endpoint | Purpose |
|----------|---------|
| `GET /api/v1/stations/nearby?lat=&lng=&distance=` | Existing — nearby stations for map markers |
| `GET /api/v1/search?q=&filters=` | New — text search |
| `GET /api/v1/stations/{id}` | New — station detail |
| `GET /api/v1/filters?session_id=` | New — read active filters |
| `PUT /api/v1/filters?session_id=` | New — write active filters |
| `POST /api/v1/analytics/connect` | Existing — ingest clickstream events |

## Testing

```bash
# Run unit tests (once Jest is configured)
npx jest

# Run with coverage
npx jest --coverage
```

## CI Integration

The existing CI pipeline (`apps/mobile-driver/.github/workflows/ci.yml`) runs:

1. `npm ci`
2. `npx expo export --platform web` (build check)

Add after the build step:

```yaml
- name: Run tests
  run: npx jest --ci --passWithNoTests
```
