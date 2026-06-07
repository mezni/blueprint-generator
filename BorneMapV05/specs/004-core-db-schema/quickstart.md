# Quickstart: Core Database Schema

**Feature**: Sprint 4 — Core Database Schema  
**Date**: 2026-06-02

## Prerequisites

- Docker & Docker Compose installed
- PostgreSQL 15+ with PostGIS 3.x accessible (via Docker Compose from Sprint 2)
- `sqlx-cli` installed (`cargo install sqlx-cli --no-default-features --features postgres`)
- `psql` client available for smoke tests

## Environment Setup

1. Start the database infrastructure (from repo root):
   ```bash
   docker compose -f infra/compose/docker-compose.yml up -d postgres
   ```

2. Verify connectivity:
   ```bash
   psql "postgresql://bornemap:bornemap@localhost:5432/platform_db" -c "SELECT 1"
   psql "postgresql://bornemap:bornemap@localhost:5432/analytics_db" -c "SELECT 1"
   ```

## Running Migrations

### platform_db (inventory, users, gis schemas)

```bash
cd services/admin-service
sqlx migrate run --database-url "postgresql://bornemap:bornemap@localhost:5432/platform_db"
```

### analytics_db (analytics schema)

```bash
cd services/analytics-writer
sqlx migrate run --database-url "postgresql://bornemap:bornemap@localhost:5432/analytics_db"
```

## Verifying Migrations

### Check applied migrations
```bash
psql "$PLATFORM_DB_URL" -c "SELECT * FROM _sqlx_migrations ORDER BY version"
```

### Verify schemas exist
```bash
psql "$PLATFORM_DB_URL" -c "\dn"
# Expected: inventory, users, gis

psql "$ANALYTICS_DB_URL" -c "\dn"
# Expected: analytics
```

### Verify tables and indexes
```bash
psql "$PLATFORM_DB_URL" -c "\dt inventory.*"
psql "$PLATFORM_DB_URL" -c "\dt users.*"
psql "$PLATFORM_DB_URL" -c "\dt gis.*"
psql "$PLATFORM_DB_URL" -c "\di inventory.*"
```

### Verify PostGIS and spatial index
```bash
psql "$PLATFORM_DB_URL" -c "SELECT PostGIS_Version()"
psql "$PLATFORM_DB_URL" -c "EXPLAIN SELECT * FROM inventory.station WHERE geom && ST_MakeEnvelope(9.0, 36.0, 11.0, 37.5, 4326)"
# Should show "Index Scan using station_geom_idx"
```

### Verify visible_stations view
```bash
psql "$PLATFORM_DB_URL" -c "SELECT count(*) FROM inventory.visible_stations"
```

### Verify partner delete guard trigger
```bash
psql "$PLATFORM_DB_URL" -c "
  UPDATE inventory.partner SET deleted_at = NOW() WHERE id = 'PRT-XXXX';
  -- Should raise ACTIVE_STATIONS_EXIST if partner has active stations
"
```

## Running Seed Data

Seed data runs as migration 0016 (idempotent — checks before inserting):

```bash
cd services/admin-service
sqlx migrate run --database-url "postgresql://bornemap:bornemap@localhost:5432/platform_db"
```

## Running Smoke Test

The smoke test is a standalone verification script (not a migration):

```bash
psql "$PLATFORM_DB_URL" -f services/admin-service/migrations/0017_smoke_test.sql
```

Expected output: all assertions pass (spatial query returns results, indexes verified, constraints enforced).

## Rollback

To rollback the most recent migration:
```bash
cd services/admin-service
sqlx migrate revert --database-url "postgresql://bornemap:bornemap@localhost:5432/platform_db"
```

To tear down and start fresh:
```bash
docker compose -f infra/compose/docker-compose.yml down -v
docker compose -f infra/compose/docker-compose.yml up -d postgres
sqlx migrate run  # re-apply all
```

## Common Commands

| Action | Command |
|--------|---------|
| Run platform_db migrations | `sqlx migrate run` (from `services/admin-service`) |
| Run analytics_db migrations | `sqlx migrate run` (from `services/analytics-writer`) |
| Create new migration | `sqlx migrate add <name>` |
| Check migration status | `sqlx migrate info` |
| Revert last migration | `sqlx migrate revert` |
| Verify schema | `psql "$URL" -c "\d+ inventory.station"` |
| Test spatial query | `psql "$URL" -c "EXPLAIN SELECT ... WHERE geom && ..."` |
