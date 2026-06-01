# Quickstart: Database & Schema Foundation

## Prerequisites

- Docker Compose (from EPIC 2 runtime infrastructure)
- Rust 1.79+ with `cargo sqlx` CLI installed
- Access to the monorepo at project root

## Setup

### 1. Install sqlx CLI

```bash
cargo install sqlx-cli --features postgres,rustls
```

### 2. Start the database

```bash
docker compose -f infra/compose/docker-compose.yml up -d postgres
```

Wait for the healthcheck to pass:

```bash
docker compose -f infra/compose/docker-compose.yml ps postgres
# Should show "healthy"
```

### 3. Create the database

```bash
sqlx database create --database-url "postgres://borne:devpassword@localhost:5432/borne_map"
```

### 4. Create PostGIS extension

```sql
-- Connect to the database and run:
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```

### 5. Run migrations

```bash
cd services/admin-service
DATABASE_URL="postgres://borne:devpassword@localhost:5432/borne_map" sqlx migrate run
```

### 6. Verify schema

```sql
-- Check that schemas exist
SELECT schema_name FROM information_schema.schemata
WHERE schema_name IN ('inventory', 'users', 'gis', 'analytics');

-- Check that tables exist
SELECT table_schema, table_name FROM information_schema.tables
WHERE table_schema IN ('inventory', 'users', 'gis', 'analytics');
```

## Running Tests

```bash
cd services/admin-service
# Set up test database
DATABASE_URL="postgres://borne:devpassword@localhost:5432/borne_map_test" sqlx database create
DATABASE_URL="postgres://borne:devpassword@localhost:5432/borne_map_test" sqlx migrate run

# Run tests
cargo test -- --test-threads=1
```

## Development Workflow

### Adding a new migration

```bash
cd services/admin-service
sqlx migrate add -r <description>
# Creates two files: YYYYMMDDHHMMSS_<description>.up.sql and .down.sql
```

### Rolling back a migration

```bash
cd services/admin-service
DATABASE_URL="postgres://borne:devpassword@localhost:5432/borne_map" sqlx migrate revert
```

### Preparing offline mode (compile-time SQL verification)

```bash
cargo sqlx prepare --database-url "postgres://borne:devpassword@localhost:5432/borne_map"
# Generates sqlx-data.json for offline compilation
```

## Verifying Schema Ownership

Connect with each role and verify permissions:

```sql
-- As admin_service: should be able to INSERT into inventory.station
-- As driver_service: should be REJECTED when INSERTing into inventory.station
-- As gis_worker: should be able to SELECT from inventory.station but not INSERT
```

## Common Commands

```bash
# View migration status
sqlx migrate info

# Reset database (dev only)
sqlx database reset

# Create a fresh migration
sqlx migrate add create_partner_table
```
