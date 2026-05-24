# Data Model: Phase 0

## Persistent Entities

None. Phase 0 is infrastructure-only — no database tables other than Alembic's
internal `alembic_version` table.

## Alembic Baseline

- `001_create_stations.py` — empty revision that establishes Alembic baseline.
  Tables for Phase 1 (`stations`, etc.) will be added in subsequent migrations.

## Notes

- Database health is verified by `/health/ready` which runs
  `SELECT 1` against PostGIS
- Connection string: `postgresql+asyncpg://borne:map@localhost:5432/borne`
- Schema: `public` (default)
