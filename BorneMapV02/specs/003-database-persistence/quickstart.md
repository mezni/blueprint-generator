# Quickstart: Database Persistence

## Prerequisites

- Docker & Docker Compose
- Rust toolchain (via rustup)
- Node.js v24
- Expo CLI (`npx expo`)

## Step 1: Start PostGIS Database

```bash
docker compose -f deployments/docker-compose.yml up -d
```

Verify the container is healthy:
```bash
docker compose -f deployments/docker-compose.yml ps
```

## Step 2: Run Migrations

```bash
cd backend
psql -h 127.0.0.1 -U borne -d borne_map -f db/migrations/20260528000000_init_spatial_schema.sql
```

## Step 3: Seed Demo Data

```bash
psql -h 127.0.0.1 -U borne -d borne_map -f db/seeds/demo_data.sql
```

## Step 4: Start Backend API Service

```bash
cd backend
DATABASE_URL="postgres://borne:borne@localhost:5432/borne_map" cargo run -p api-service
```

Verify the API is running:
```bash
curl "http://localhost:8080/api/v1/stations/nearby?lat=36.8065&lng=10.1815&show_staged=true" | jq '. | length'
# Expected output: 50

curl http://localhost:8080/health
# Expected: {"status":"ok","database":"connected"}
```

### Verify Seed Data

```bash
psql -h 127.0.0.1 -U borne -d borne_map -f db/seeds/validate_seed.sql
```

## Step 5: Start Mobile App

```bash
cd apps/mobile-driver
npm start
```

Open the Expo Go app on your device and scan the QR code, or press `w` to open in web browser.

## Running Tests

```bash
# Backend tests (requires PostGIS running)
cd backend
DATABASE_URL="postgres://borne:borne@localhost:5432/borne_map" cargo test --workspace

# Frontend build verification
cd apps/mobile-driver
npx expo export --platform web
```

## Response Time Verification

```bash
time curl "http://localhost:8080/api/v1/stations/nearby?lat=36.8065&lng=10.1815&show_staged=true"
# Expected: under 500ms (SC-002)
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://borne:borne@localhost:5432/borne_map` | PostGIS connection string |
| `EXPO_PUBLIC_API_URL` | `http://localhost:8080/api/v1` | Backend API base URL (set for device testing) |
| `RUST_LOG` | `actix_web=info` | Logging verbosity |
