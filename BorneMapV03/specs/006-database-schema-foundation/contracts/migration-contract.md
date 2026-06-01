# Contract: Database Migration

## Status

**Adopted**: 2026-05-31
**Scope**: All database schema changes across all environments
**Owner**: Platform Architecture

## Tool

**sqlx migrations** — file-based, versioned, transactional.

## Location

All migration files live in:

```
services/admin-service/migrations/
```

The Admin Service is responsible for running migrations at startup before binding the HTTP listener.

## File Naming

Format: `{YYYYMMDDHHMMSS}_{description}.sql`

Example: `20260531000001_create_schemas.sql`

- Timestamp MUST be UTC.
- Description uses snake_case.
- Files are executed in timestamp order.

## Structure

Each migration file contains:

1. `--` comments describing the change.
2. Plain SQL (DDL, DML, or both).
3. Optionally `-- migrate:wrap false` at the top if the migration cannot run inside a transaction (e.g., `CREATE INDEX CONCURRENTLY`).

## Rules

1. **All migrations MUST be reversible** — provide a `-- migrate:down` section or a separate down migration file.
2. **No destructive changes without a down migration** — DROP COLUMN, DROP TABLE, DROP SCHEMA must have a revert that restores the data.
3. **Migration files MUST NOT be modified after creation** — corrections go in a new migration.
4. **One logical change per migration** — do not mix schema creation with data seeding in the same file.
5. **Transactions**: Each migration runs inside a transaction by default. Add `-- migrate:wrap false` for operations that require manual transaction control.
6. **No multi-statement down migrations**: Each down migration reverses exactly one up migration.

## Recommended Order

```
20260531000001_create_extensions.sql     -- CREATE EXTENSION postgis, uuid-ossp
20260531000002_create_schemas.sql         -- CREATE SCHEMA inventory, users, gis, analytics
20260531000003_create_roles.sql           -- CREATE ROLE admin_service, driver_service, etc.
20260531000004_create_users_schema.sql    -- user_account, favorite
20260531000005_create_inventory_schema.sql -- partner, station, charger, review
20260531000006_create_gis_schema.sql      -- station_enrichment, geo_boundary
20260531000007_create_analytics_schema.sql -- raw_event (partitioned)
20260531000008_grant_privileges.sql       -- GRANT statements per schema-ownership contract
20260531000009_seed_reference_data.sql    -- If needed
```

## Testing

- A test migration suite runs against a test database (sqlx test fixtures or ephemeral Docker container).
- Each migration is applied and rolled back to verify both directions.
- Schema permissions tests verify that each role can only access its authorized schemas.
