# Quickstart: Mobile Canvas

**Date**: 2026-05-28

## Prerequisites

- Docker & Docker Compose
- Rust toolchain (rustc, cargo)
- Node.js v24.16.0, npm v11.13.0
- Expo CLI

## Setup

### 1. Start the Database

```bash
cd deployments
docker compose up -d
```

### 2. Apply Migration

```bash
# The migration is located at backend/db/migrations/20260528000000_init_spatial_schema.sql
# Apply it via psql:
PGPASSWORD=borne psql -h localhost -U borne -d borne_map -f backend/db/migrations/20260528000000_init_spatial_schema.sql
```

### 3. Seed Demo Data

```bash
PGPASSWORD=borne psql -h localhost -U borne -d borne_map -f backend/db/seeds/demo_data.sql
```

### 4. Start the API Service

```bash
cd backend
DATABASE_URL="postgres://borne:borne@localhost:5432/borne_map" cargo run -p api-service
```

The API is now available at `http://localhost:8080`.

### 5. Verify

```bash
# Health check
curl http://localhost:8080/health

# Nearby stations
curl "http://localhost:8080/api/v1/stations/nearby?lat=36.8065&lng=10.1815&show_staged=true"
```

### 6. Start the Mobile App

```bash
cd apps/mobile-driver
npm install
npx expo start --web
```

The web version opens at `http://localhost:8081`.

## Key Files

| File | Purpose |
|------|---------|
| `backend/db/migrations/20260528000000_init_spatial_schema.sql` | Database schema with PostGIS |
| `backend/db/seeds/demo_data.sql` | 5 partners, 50 stations, 100 chargers |
| `backend/api-service/src/main.rs` | API entry point |
| `backend/api-service/src/domains/locate/routes.rs` | Nearby stations handler |
| `apps/mobile-driver/src/screens/MapScreen.js` | Main map view |
| `apps/mobile-driver/src/components/StationCard.js` | Station detail sheet |

## Identifier Patterns

| Entity | Pattern | Example |
|--------|---------|---------|
| Partner | `^prt-[a-f0-9]{8}$` | `prt-a1b2c3d4` |
| Station | `^stn-[a-f0-9]{8}$` | `stn-e3b0c442` |
| Charger | `^chg-[a-f0-9]{8}$` | `chg-7b2a19f4` |
