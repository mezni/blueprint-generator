# Research: Core Database Schema

**Feature**: Sprint 4 — Core Database Schema  
**Date**: 2026-06-02

## Research Tasks

### R-001: Migration Tooling Choice

**Decision**: Use `sqlx-cli` with raw SQL migration files (`.up.sql` / `.down.sql` pairs).

**Rationale**: 
- The project uses Rust services; `sqlx` is the standard Rust async SQL library and `sqlx-cli` is its migration companion.
- Raw SQL gives full control over PostGIS-specific DDL (GIST indexes, geometry triggers, partitioning) which ORM-style migrations would obscure.
- The constitution mandates "migrations run before service startup, never auto at runtime" — `sqlx-cli` supports `sqlx migrate run` as a pre-startup step.
- Sequential numeric prefixes (`0001_`, `0002_`, ...) provide deterministic ordering.

**Alternatives considered**:
- **refinery**: Another Rust migration framework. Good but less ecosystem momentum than sqlx; would add a dependency with no compelling advantage for raw SQL files.
- **Pure psql scripts**: Simple but lack tracking of applied migrations, requiring manual bookkeeping. sqlx-cli provides the `sqlx_migrations` table automatically.
- **Terraform/pgprovider**: Overkill for application-level schema management; better suited for infrastructure provisioning.

### R-002: PostGIS Extension & Geometry Trigger Pattern

**Decision**: Enable `PostGIS` extension in `platform_db` via a dedicated migration. Use a PL/pgSQL `BEFORE INSERT OR UPDATE` trigger on `inventory.station` to auto-populate `geom` from `(longitude, latitude)`.

**Rationale**:
- PostGIS is required for `GEOGRAPHY(Point, 4326)` type, `ST_SetSRID`, `ST_MakePoint`, and GIST indexing.
- A trigger ensures `geom` is always consistent with lat/lng — application code cannot accidentally skip the geometry update.
- The trigger must handle NULL lat/lng by setting `geom = NULL` (graceful degradation).

**Alternatives considered**:
- **Application-managed geometry**: Risk of inconsistency if any code path forgets to compute `geom`. Violates data-layer integrity principle.
- **Generated column (PostgreSQL 12+)**: Cannot use `ST_SetSRID(ST_MakePoint(...))` in a generated column expression (not immutable in PostgreSQL's view). Triggers are the standard pattern for PostGIS auto-population.

**Trigger implementation pattern**:
```sql
CREATE OR REPLACE FUNCTION inventory.trg_station_geom()
RETURNS TRIGGER AS $$
BEGIN
  IF NEW.latitude IS NOT NULL AND NEW.longitude IS NOT NULL THEN
    NEW.geom := ST_SetSRID(ST_MakePoint(NEW.longitude, NEW.latitude), 4326);
  ELSE
    NEW.geom := NULL;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

### R-003: Partner Delete Guard Trigger

**Decision**: Use a `BEFORE UPDATE` trigger on `inventory.partner` that checks for active stations before allowing `deleted_at` to be set.

**Rationale**:
- The constitution mandates enforcement at the data-access layer. A trigger is the strongest data-layer guarantee.
- Application logic (Sprint 5) provides user-friendly error messages; the trigger is the safety net that cannot be bypassed.
- The trigger fires only when `deleted_at` transitions from NULL to non-NULL (soft delete attempt), not on every update.

**Trigger implementation pattern**:
```sql
CREATE OR REPLACE FUNCTION inventory.trg_partner_delete_guard()
RETURNS TRIGGER AS $$
BEGIN
  IF OLD.deleted_at IS NULL AND NEW.deleted_at IS NOT NULL THEN
    IF EXISTS (
      SELECT 1 FROM inventory.station
      WHERE partner_id = NEW.id
        AND is_live = true
        AND deleted_at IS NULL
    ) THEN
      RAISE EXCEPTION 'ACTIVE_STATIONS_EXIST'
        USING HINT = 'Cannot soft-delete partner with active stations';
    END IF;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

### R-004: Analytics Monthly Partitioning Strategy

**Decision**: Use PostgreSQL declarative partitioning by range on `occurred_at`. Pre-create 12 monthly partitions (`raw_event_2026_01` through `raw_event_2026_12`) in the migration.

**Rationale**:
- Declarative partitioning (PostgreSQL 10+) is simpler and better optimized than inheritance-based partitioning.
- Pre-creating 12 partitions ensures the system works immediately without runtime partition creation.
- Sprint 14 (Analytics Writer) will add a partition maintenance function for ongoing auto-creation.
- A default partition (`raw_event_default`) catches any out-of-range dates, preventing insert failures.

**Alternatives considered**:
- **pg_partman extension**: Adds a dependency and operational complexity. Overkill for pre-creating 12 fixed partitions.
- **No default partition**: Risk of silent insert failures for dates outside pre-created range. A default partition is safer.

### R-005: Idempotent Migration Pattern

**Decision**: Use `IF NOT EXISTS` / `IF EXISTS` guards on all DDL statements. Each migration checks before creating.

**Rationale**:
- The spec requires migrations to be re-runnable without errors.
- `sqlx-cli` tracks applied migrations in `_sqlx_migrations` table, so normally re-running is a no-op. However, the `IF NOT EXISTS` pattern provides defense-in-depth for manual `psql` execution or disaster recovery.
- For constraint/index creation that lacks `IF NOT EXISTS` in older PostgreSQL versions, wrap in `DO $$ ... EXCEPTION ... $$` blocks.

**Alternatives considered**:
- **Rely solely on sqlx-cli tracking**: Fragile if someone runs migrations manually via `psql` (common in debugging/ops).
- **Drop-and-recreate on each run**: Destructive; loses data. Unacceptable.

### R-006: ULID Generation in PostgreSQL

**Decision**: Use the `ulid` PostgreSQL extension or a PL/pgSQL function to generate ULIDs in seed data. Application code (Rust) will generate ULIDs at runtime.

**Rationale**:
- Seed data needs valid ULIDs. A PostgreSQL-side generator keeps the seed script self-contained.
- At runtime, Rust services will use a Rust ULID crate (`ulid` or `uuid7`) — this is a Sprint 5+ concern.
- The `ulid` extension (`CREATE EXTENSION ulid`) is available from PGXN. If installation is undesirable, a pure SQL function using `gen_random_bytes()` can produce compliant ULIDs.

**Alternatives considered**:
- **Hard-coded ULIDs in seed**: Works but makes the seed script rigid and hard to extend.
- **UUIDv4 instead of ULID**: Rejected — the constitution mandates ULID+prefix, and ULIDs are time-sortable (important for indexed primary keys).

### R-007: Visible Stations View

**Decision**: Create `inventory.visible_stations` as a simple SQL VIEW joining `inventory.station` with the visibility filter.

**Rationale**:
- Encapsulates the visibility rule in one place — services don't duplicate the 4-condition filter.
- A view (not materialized) is always consistent with the underlying table; no staleness risk.
- GIST indexes on the underlying `station` table are used when the view is queried with spatial predicates (PostgreSQL optimizes through views).

**Implementation**:
```sql
CREATE VIEW inventory.visible_stations AS
SELECT *
FROM inventory.station
WHERE is_live = true
  AND deleted_at IS NULL
  AND status = 'active'
  AND is_public = true;
```

## Summary of Decisions

| ID | Decision | Key Rationale |
|----|----------|--------------|
| R-001 | sqlx-cli with raw SQL files | Standard Rust migration tool; full PostGIS control |
| R-002 | PL/pgSQL trigger for geom | Ensures lat/lng ↔ geom consistency at data layer |
| R-003 | BEFORE UPDATE trigger for partner delete guard | Constitution-mandated data-layer enforcement |
| R-004 | Declarative partitioning + 12 pre-created partitions + default partition | Safe, simple, immediately functional |
| R-005 | IF NOT EXISTS guards on all DDL | Idempotent for both sqlx-cli and manual psql |
| R-006 | ULID via PostgreSQL function for seed data | Self-contained seed; runtime uses Rust ULID crate |
| R-007 | Simple SQL VIEW for visible_stations | Single source of truth for visibility rule; index-transparent |
