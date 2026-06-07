# Quickstart: Sprint 1 — OSM Data & Station Discovery

**Date**: 2026-06-05 | **Status**: Development Ready

This guide enables developers to set up, test, and iterate on Sprint 1 features (OSM data, station discovery, favorites, partner management) locally.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Local Setup](#local-setup)
3. [Database Initialization](#database-initialization)
4. [OSM Data Import](#osm-data-import)
5. [Running Services](#running-services)
6. [Testing](#testing)
7. [Common Tasks](#common-tasks)
8. [Troubleshooting](#troubleshooting)

---

## Prerequisites

**Required**:
- Docker & Docker Compose (latest)
- Rust 1.75+
- PostgreSQL 15+ client tools (`psql`, `pg_dump`)
- Node.js 18+ (for frontend, if building locally)

**Optional**:
- Traefik (for routing, if not using Docker Compose)
- osm2pgsql binary (for manual OSM import; included in Docker image)

**Estimated Setup Time**: 15-20 minutes

---

## Local Setup

### 1. Clone Repository

```bash
git clone https://github.com/yourusername/bornemap.git
cd bornemap
git checkout 004-sprint-1-foundation
```

### 2. Environment Configuration

Create `.env` file at repository root:

```bash
# Database
DATABASE_URL=postgresql://bornemap_dev:dev_password@localhost:5432/platform_db
DATABASE_ANALYTICS_URL=postgresql://bornemap_dev:dev_password@localhost:5432/analytics_db

# Keycloak (for local testing)
KEYCLOAK_REALM_URL=http://localhost:8080/realms/bornemap
KEYCLOAK_CLIENT_ID=driver-service
KEYCLOAK_CLIENT_SECRET=dev_secret_do_not_use_in_prod
KEYCLOAK_ADMIN_USER=admin
KEYCLOAK_ADMIN_PASSWORD=admin

# Services
DRIVER_SERVICE_PORT=3001
PARTNER_SERVICE_PORT=3002
GIS_WORKER_INTERVAL_SECS=30

# Logging
RUST_LOG=debug
```

### 3. Docker Compose Startup

Start PostgreSQL, Keycloak, and other infrastructure:

```bash
docker-compose up -d
```

**Services Started**:
- PostgreSQL 15 (port 5432)
- Keycloak 21+ (port 8080)
- PgAdmin (port 5050, optional)

**Verify PostgreSQL is Ready**:

```bash
psql postgresql://bornemap_dev:dev_password@localhost:5432/platform_db -c "SELECT 1;"
```

---

## Database Initialization

### 1. Run Migrations

Migrations are version-controlled in `crates/driver-service/migrations/`:

```bash
# Using SQLx CLI
cargo install sqlx-cli
sqlx migrate run --database-url $DATABASE_URL

# OR using embedded migrations (during `cargo run`)
cargo run --manifest-path crates/driver-service/Cargo.toml --bin migrate
```

**Expected Output**:
```
[Migration] Running migration 001_create_inventory_schema.sql
[Migration] Running migration 002_create_station_table.sql
[Migration] Running migration 003_create_gis_schema.sql
...
All migrations completed successfully.
```

### 2. Verify Schema Creation

```bash
psql $DATABASE_URL -c "\dn"  # List schemas
psql $DATABASE_URL -c "\dt inventory.*"  # List inventory tables
psql $DATABASE_URL -c "\dt gis.*"  # List GIS tables
```

**Expected Schemas**: `public`, `inventory`, `users`, `gis`

**Expected Tables**:
- `inventory.station`
- `inventory.charger`
- `inventory.partner`
- `inventory.station_outbox`
- `users.user`
- `users.favorite`
- `users.review`
- `gis.osm_ways`
- `gis.osm_nodes`
- `gis.station_locations`

### 3. Create Spatial Indexes

Indexes should be created by migrations, but verify:

```bash
psql $DATABASE_URL -c "SELECT indexname FROM pg_indexes WHERE tablename = 'station_locations';"
```

**Expected**: Indexes on `geom` column (GIST type)

---

## OSM Data Import

### 1. Download Tunisia OSM Data

```bash
# Create OSM data directory
mkdir -p data/osm
cd data/osm

# Download Tunisia OSM PBF (50-100 MB)
wget https://download.geofabrik.de/africa/tunisia-latest.osm.pbf

# OR use smaller test area (for quick testing)
wget https://download.geofabrik.de/africa/tunisia-latest.osm.pbf \
  --output-document=tunisia-test.osm.pbf

cd ../..
```

**Alternative** (using Docker):
```bash
docker run -it --rm \
  -v $(pwd)/data/osm:/data \
  curlimages/curl \
  curl -o /data/tunisia-latest.osm.pbf \
  https://download.geofabrik.de/africa/tunisia-latest.osm.pbf
```

### 2. Import OSM Data

**Option A: Using Docker (Recommended)**

```bash
# Build OSM import image
docker build -f Dockerfile.osm -t bornemap:osm-importer .

# Run import
docker run --rm \
  -e DATABASE_URL=$DATABASE_URL \
  -v $(pwd)/data/osm:/data \
  bornemap:osm-importer \
  /scripts/osm-import.sh /data/tunisia-latest.osm.pbf
```

**Option B: Using Local osm2pgsql**

```bash
# Install osm2pgsql
cargo install osm2pgsql  # OR: brew install osm2pgsql (macOS)

# Run import
osm2pgsql \
  --database platform_db \
  --username bornemap_dev \
  --password \
  --host localhost \
  --port 5432 \
  --style /path/to/ev-platform/scripts/osm2pgsql-style.lua \
  --create \
  --number-processes 4 \
  data/osm/tunisia-latest.osm.pbf
```

**Expected Output**:
```
Processing node 1/50000000...
Processing way 1/10000000...
Processing relations...
Creating indexes...
All data imported successfully. (Duration: ~8 minutes)
```

### 3. Verify Import

```bash
psql $DATABASE_URL << EOF
SELECT COUNT(*) as ways FROM gis.osm_ways;
SELECT COUNT(*) as nodes FROM gis.osm_nodes;
SELECT COUNT(*) as has_index FROM pg_indexes 
  WHERE tablename = 'osm_ways' AND indexname LIKE '%geom%';
EOF
```

**Expected**:
- `osm_ways`: 10,000+ records
- `osm_nodes`: 50,000+ records
- Index count: >= 1 (GIST index on geom)

---

## Running Services

### 1. Build Services

```bash
# Build all crates
cargo build --release

# OR build specific service
cargo build --release --package driver-service
```

### 2. Start Driver Service (Station Discovery)

```bash
cargo run --bin driver-service --manifest-path crates/driver-service/Cargo.toml

# Expected output:
# [INFO] Starting driver-service on 0.0.0.0:3001
# [INFO] Listening for requests...
```

**Test Public Discovery Endpoint**:

```bash
# Request nearby stations
curl "http://localhost:3001/api/v1/stations/nearby?lat=36.8&lng=10.1&radius=5000"

# Expected response (if stations in database):
# {
#   "stations": [
#     {
#       "id": "STN-ABC123XYZ1234",
#       "name": "Tunis Central",
#       "latitude": 36.802,
#       "longitude": 10.155,
#       "distance_m": 450,
#       "availability_status": "available"
#     }
#   ],
#   "query": {
#     "latitude": 36.8,
#     "longitude": 10.1,
#     "radius_m": 5000
#   }
# }
```

### 3. Start GIS Sync Worker

```bash
cargo run --bin gis-worker --manifest-path crates/gis-worker/Cargo.toml

# Expected output:
# [INFO] GIS Sync Worker started (interval: 30s)
# [INFO] Polling outbox for changes...
```

### 4. Start Partner Service (Optional)

```bash
cargo run --bin partner-service --manifest-path crates/partner-service/Cargo.toml

# Expected output:
# [INFO] Starting partner-service on 0.0.0.0:3002
# [INFO] Listening for requests...
```

---

## Testing

### 1. Unit Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_nearby_stations_validation

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

### 2. Integration Tests

```bash
# Integration tests use test PostgreSQL containers
cargo test --test '*' -- --nocapture

# Run specific integration test file
cargo test --test integration_nearby_stations
```

**Test Database Setup**:
- Uses `testcontainers` crate to spin up ephemeral PostgreSQL
- Runs migrations automatically
- Cleans up after test completion

### 3. Contract Tests (Independent Tests)

Contract tests validate API contracts as specified in the feature spec. Run them against a running service:

```bash
# Start the service first (in separate terminal)
cargo run --bin driver-service

# Then run contract tests
cargo test --test contract_nearby_stations -- --nocapture

# Expected output:
# test contract_nearby_stations::test_us1_ac1_stations_within_radius ... ok
# test contract_nearby_stations::test_us1_ac2_empty_radius ... ok
# test contract_nearby_stations::test_us1_ac3_invalid_coordinates ... ok
```

**Contract Test Files**:
- `crates/driver-service/tests/contract/api_v1_nearby.rs`
- `crates/driver-service/tests/contract/api_v1_favorites.rs`
- `crates/partner-service/tests/contract/api_v1_partner_stations.rs`

### 4. Test Data Setup

Populate test data for manual testing:

```bash
# Run seed script
psql $DATABASE_URL -f scripts/seed-test-data.sql

# Expected:
# - 1 test partner (PRT-PARTNER001ABC)
# - 50+ test stations across Tunisia
# - 5+ test drivers
# - Sample favorites and reviews
```

**Verify Test Data**:

```bash
psql $DATABASE_URL << EOF
SELECT COUNT(*) as stations FROM inventory.station WHERE deleted_at IS NULL;
SELECT COUNT(*) as chargers FROM inventory.charger WHERE deleted_at IS NULL;
SELECT COUNT(*) as partners FROM inventory.partner WHERE deleted_at IS NULL;
EOF
```

---

## Common Tasks

### Create a Test Station

```bash
# Create station via partner API (requires JWT token)
# First, get Keycloak token

TOKEN=$(curl -X POST http://localhost:8080/realms/bornemap/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id=driver-service" \
  -d "client_secret=dev_secret_do_not_use_in_prod" \
  -d "grant_type=client_credentials" \
  | jq -r '.access_token')

# Create station
curl -X POST http://localhost:3002/api/v1/partner/stations \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Station",
    "address": "123 Test Street, Tunis",
    "latitude": 36.8,
    "longitude": 10.1,
    "capacity": 4
  }'
```

### Query Nearby Stations

```bash
# Simple query
curl "http://localhost:3001/api/v1/stations/nearby?lat=36.8&lng=10.1&radius=5000" | jq '.'

# With different radius
curl "http://localhost:3001/api/v1/stations/nearby?lat=36.8&lng=10.1&radius=10000" | jq '.stations | length'

# Invalid coordinates (test validation)
curl "http://localhost:3001/api/v1/stations/nearby?lat=100&lng=10.1" | jq '.error'
```

### Monitor GIS Sync Worker

```bash
# Check outbox for pending events
psql $DATABASE_URL << EOF
SELECT id, station_id, event_type, created_at, processed_at 
FROM inventory.station_outbox 
WHERE processed_at IS NULL
LIMIT 10;
EOF

# Check GIS projection
psql $DATABASE_URL << EOF
SELECT id, name, ST_AsText(geom), synced_at 
FROM gis.station_locations 
LIMIT 5;
EOF
```

### Clear Test Data

```bash
# Soft-delete all test stations
psql $DATABASE_URL << EOF
UPDATE inventory.station SET deleted_at = NOW() WHERE deleted_at IS NULL;
DELETE FROM inventory.station_outbox;  -- Clear outbox
EOF

# Re-seed
psql $DATABASE_URL -f scripts/seed-test-data.sql
```

---

## Troubleshooting

### PostgreSQL Connection Issues

**Error**: `FATAL: remaining connection slots are reserved`

**Solution**:
```bash
# Increase max connections
psql postgresql://postgres@localhost:5432/postgres -c "ALTER SYSTEM SET max_connections = 200;"
docker restart bornemap-postgres
```

### OSM Import Fails

**Error**: `osm2pgsql: ERROR: Unknown lua style file`

**Solution**:
- Verify style file exists: `scripts/osm2pgsql-style.lua`
- Use absolute path: `--style /full/path/to/style.lua`
- Use default style if custom missing: omit `--style` flag

### GIS Queries Return No Results

**Symptoms**: `stations` array is empty even after import

**Debugging**:
1. Verify OSM data was imported: `SELECT COUNT(*) FROM gis.osm_ways;`
2. Check if test stations exist: `SELECT COUNT(*) FROM inventory.station;`
3. Verify GIS sync completed: `SELECT * FROM gis.station_locations LIMIT 1;`
4. Check outbox for failed events: `SELECT * FROM inventory.station_outbox WHERE processed_at IS NULL;`

**Solution**:
```bash
# Manually sync a station (if sync worker fails)
psql $DATABASE_URL << EOF
INSERT INTO gis.station_locations (id, station_id, name, partner_id, geom, synced_at)
SELECT id, id, name, partner_id, 
       ST_SetSRID(ST_Point(longitude, latitude), 4326), NOW()
FROM inventory.station
WHERE deleted_at IS NULL
ON CONFLICT (id) DO UPDATE SET geom = EXCLUDED.geom, synced_at = NOW();
EOF
```

### Rate Limiting Block

**Error**: `HTTP 429 Too Many Requests`

**Solution**:
- Wait 60 seconds before retrying
- Check rate limit config in `driver-service/src/interface/middleware/rate_limiter.rs`
- In local testing, increase limit: `requests_per_minute: 10000`

### Keycloak Token Errors

**Error**: `401 Unauthorized: Invalid token`

**Debugging**:
1. Verify Keycloak is running: `curl http://localhost:8080/`
2. Check token expiry: `jq -R 'split(".") | .[1] | @base64d' <<< $TOKEN`
3. Get fresh token: See "Create a Test Station" section above

---

## Next Steps

1. **Run acceptance tests** to verify all user stories pass
2. **Review data model** in `specs/004-sprint-1-foundation/data-model.md`
3. **Read API contracts** in `specs/004-sprint-1-foundation/contracts/`
4. **Start task breakdown** when ready for implementation (`/speckit.tasks`)

---

## Resources

- **Plan**: `specs/004-sprint-1-foundation/plan.md`
- **Research**: `specs/004-sprint-1-foundation/research.md`
- **Data Model**: `specs/004-sprint-1-foundation/data-model.md`
- **API Contracts**: `specs/004-sprint-1-foundation/contracts/`
- **Architecture**: `docs/03-architecture/clean-architecture.md`
- **Keycloak Setup**: `docs/02-platform/keycloak.md`

---

**Questions?** Check the [Troubleshooting](#troubleshooting) section or consult the project constitution in `.specify/memory/constitution.md`.
