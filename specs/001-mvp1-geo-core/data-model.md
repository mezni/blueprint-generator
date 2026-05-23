# Phase 1 — Data Model: MVP 1 Geo Core

**Branch**: `001-mvp1-geo-core` | **Date**: 2026-05-23 | **Plan**: [`plan.md`](./plan.md) | **Spec**: [`spec.md`](./spec.md)

This file is the canonical data model for MVP 1. Migrations under `backend/services/station-service/migrations/` must produce exactly this state. No table or column is permitted outside what is defined here.

---

## 1. Schemas

Two schemas in MVP 1:

| Schema | Purpose |
|---|---|
| `station_domain` | Companies, stations, chargers. Sole owner of spatial data. |
| `public` | Default schema; reserved for SQLx introspection metadata only. **No application tables.** |

`identity_domain` is **not** created in MVP 1 (admin auth is in-memory from the env allowlist; no user persistence).

---

## 2. Entities (ERD-style)

```text
station_domain.companies (1) ───< (N) station_domain.stations (1) ───< (N) station_domain.chargers
```

- `stations.company_id` REFERENCES `companies.id`.
- `chargers.station_id` REFERENCES `stations.id` with `ON DELETE CASCADE` (a hard-deleted station drops its chargers; soft-deleted stations preserve them).

---

## 3. Tables

### 3.1 `station_domain.companies`

| Column | Type | Constraints | Notes |
|---|---|---|---|
| `id` | `UUID` | PK, default `gen_random_uuid()` | Newtype `CompanyId` in Rust. |
| `name` | `TEXT` | NOT NULL, `CHECK (char_length(name) BETWEEN 1 AND 200)` | Operator display name. |
| `is_test` | `BOOLEAN` | NOT NULL, default `FALSE` | Marks synthetic seed rows. |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, default `NOW()` | |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, default `NOW()` | Touched by app on update. |
| `deleted_at` | `TIMESTAMPTZ` | NULL | Soft delete. |

**Indexes**: PK only. (No reads filter by company name in MVP 1.)

### 3.2 `station_domain.stations`

| Column | Type | Constraints | Notes |
|---|---|---|---|
| `id` | `UUID` | PK, default `gen_random_uuid()` | Newtype `StationId`. |
| `company_id` | `UUID` | NOT NULL, REFERENCES `companies(id)` | RESTRICT on delete. |
| `name` | `TEXT` | NOT NULL, `CHECK (char_length(name) BETWEEN 1 AND 200)` | |
| `address` | `TEXT` | NOT NULL, `CHECK (char_length(address) BETWEEN 1 AND 500)` | Single-line postal address. |
| `location` | `GEOGRAPHY(Point, 4326)` | NOT NULL | Canonical `[lng, lat]` point. |
| `is_active` | `BOOLEAN` | NOT NULL, default `TRUE` | Operator-asserted activity flag. |
| `under_maintenance` | `BOOLEAN` | NOT NULL, default `FALSE` | Operator-asserted maintenance flag. |
| `opening_hours_osm` | `TEXT` | NULL, `CHECK (char_length(opening_hours_osm) <= 500)` | OSM `opening_hours` string. Server-side parser validates beyond length. |
| `is_test` | `BOOLEAN` | NOT NULL, default `FALSE` | |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, default `NOW()` | |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, default `NOW()` | |
| `deleted_at` | `TIMESTAMPTZ` | NULL | Soft delete. |

**Indexes**:
- `stations_location_gix`: `USING GIST (location)` — required by FR-005 / Principle I.
- `stations_company_idx`: `(company_id)` — supports admin queries grouped by company.
- `stations_active_partial_idx`: `(id) WHERE deleted_at IS NULL AND is_active = TRUE` — small win for the public read path; optional but recommended.

**Validation (enforced at the service layer before persistence)**:
- `opening_hours_osm` is parsed by the `opening_hours` crate; failures return 422 + RFC-7807.
- `location` is constructed from `[lng, lat]`; reject if longitude ∉ [-180, 180] or latitude ∉ [-90, 90].

### 3.3 `station_domain.chargers`

| Column | Type | Constraints | Notes |
|---|---|---|---|
| `id` | `UUID` | PK, default `gen_random_uuid()` | Newtype `ChargerId`. |
| `station_id` | `UUID` | NOT NULL, REFERENCES `stations(id)` ON DELETE CASCADE | |
| `connector` | `TEXT` | NOT NULL, `CHECK (connector IN ('Type2','CCS','CHAdeMO','Type2_Tethered'))` | Closed enum (R-006, FR-018). |
| `power_kw` | `NUMERIC(6,2)` | NOT NULL, `CHECK (power_kw > 0 AND power_kw <= 600)` | Reasonable physical bounds. |
| `is_active` | `BOOLEAN` | NOT NULL, default `TRUE` | |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, default `NOW()` | |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, default `NOW()` | |
| `deleted_at` | `TIMESTAMPTZ` | NULL | Soft delete. |

**Indexes**:
- `chargers_station_idx`: `(station_id)`.

---

## 4. Lifecycle / State Transitions

### Station

```text
created  ──set under_maintenance──►  maintenance
   │                                      │
   │                                      └──unset under_maintenance──► created
   │
   └──set is_active=false──►  inactive  ──set is_active=true──►  created
   │
   └──admin soft-delete──►  deleted (deleted_at IS NOT NULL)
                                  │
                                  └── (admin restore is OUT OF SCOPE in MVP 1)
```

`pinColor(marker)` derives:

- `BorneMapMapStyles.pins.activeGreen` when `is_active AND NOT under_maintenance AND deleted_at IS NULL`.
- `BorneMapMapStyles.pins.inactiveRed` otherwise.

Soft-deleted stations never reach the client (filtered by `deleted_at IS NULL` in the repository).

### Charger

No explicit state machine — `is_active` toggles; soft delete is the only terminal state. Connector and power are immutable after creation in MVP 1 (admin must delete + recreate to change). Documented as a deliberate constraint to reduce edit-form complexity.

---

## 5. Identity / Uniqueness Rules

- `companies.id`, `stations.id`, `chargers.id` are UUID v4 generated by Postgres.
- No business-key uniqueness in MVP 1 (two stations may share a name; admin's responsibility to disambiguate). Adding `(company_id, name)` UNIQUE is deferred to MVP 2 once operator workflows surface.
- The seed migration uses deterministic UUIDs (UUID v5 in a fixed namespace) so re-running the seed produces stable IDs for tests.

---

## 6. Soft Delete & `is_test` Semantics

| Flag | Effect on public reads | Effect on admin reads |
|---|---|---|
| `deleted_at IS NOT NULL` | Excluded | Included only when `?include_deleted=true` is set on the admin endpoint (MVP 1 includes this query param on `GET /api/v1/admin/stations`). |
| `is_test = TRUE` | Excluded when env `BORNEMAP_HIDE_TEST_ROWS=true` (default in prod); included otherwise. | Included by default; filterable via `?include_test=false`. |

---

## 7. Canonical SQL: Viewport Read

```sql
-- Quantized bbox in west=$1, south=$2, east=$3, north=$4
SELECT
    s.id                                  AS "id: StationId",
    s.name,
    ST_X(s.location::geometry)            AS "lng!: f64",
    ST_Y(s.location::geometry)            AS "lat!: f64",
    s.is_active,
    s.under_maintenance
FROM station_domain.stations s
WHERE s.deleted_at IS NULL
  AND ST_DWithin(
        s.location,
        ST_MakeEnvelope($1, $2, $3, $4, 4326)::geography,
        0
      )
ORDER BY s.id
LIMIT 5000;
```

## 8. Canonical SQL: Full Detail Read

```sql
SELECT
    s.id                       AS "id: StationId",
    s.company_id               AS "company_id: CompanyId",
    c.name                     AS company_name,
    s.name,
    s.address,
    ST_X(s.location::geometry) AS "lng!: f64",
    ST_Y(s.location::geometry) AS "lat!: f64",
    s.is_active,
    s.under_maintenance,
    s.opening_hours_osm,
    s.created_at,
    s.updated_at
FROM station_domain.stations s
JOIN station_domain.companies c ON c.id = s.company_id
WHERE s.id = $1
  AND s.deleted_at IS NULL;
```

A second query loads chargers:

```sql
SELECT
    ch.id        AS "id: ChargerId",
    ch.connector,
    ch.power_kw,
    ch.is_active
FROM station_domain.chargers ch
WHERE ch.station_id = $1
  AND ch.deleted_at IS NULL
ORDER BY ch.id;
```

## 9. Canonical SQL: Admin Create

```sql
WITH inserted AS (
    INSERT INTO station_domain.stations
        (company_id, name, address, location, is_active, under_maintenance, opening_hours_osm)
    VALUES
        ($1, $2, $3, ST_MakePoint($4, $5)::geography, $6, $7, $8)
    RETURNING id
)
SELECT id AS "id: StationId" FROM inserted;
```

`$4 = lng`, `$5 = lat` — the `[lng, lat]` order is enforced by the parameter binding in the repository.

## 9a. Canonical SQL: Admin Create Chargers (alongside station)

When `AdminStationCreate.chargers` is non-empty, the repository inserts each charger in a batch after the station row:

```sql
INSERT INTO station_domain.chargers
    (station_id, connector, power_kw, is_active)
VALUES
    ($1, $2, $3, $4);
```

`$1 = station_id` (from the RETURNING id of the station insert), `$2 = connector` (validated by service layer against CHECK constraint values), `$3 = power_kw`, `$4 = is_active` (default `TRUE`).

Multiple chargers are inserted in a single `sqlx::query!` call within the same transaction as the station insert.

## 10. Canonical SQL: Admin Update / Soft Delete

Patch is partial; the repository builds the `SET` clause from non-null DTO fields. Soft delete:

```sql
UPDATE station_domain.stations
SET deleted_at = NOW(),
    updated_at = NOW()
WHERE id = $1
  AND deleted_at IS NULL
RETURNING id;
```

---

## 11. Migration Plan (filenames are authoritative)

| # | Filename | Purpose |
|---|---|---|
| 1 | `20260523_0001_init_companies.sql` | Create schema `station_domain`; create `companies` table + PK. |
| 2 | `20260523_0002_init_stations.sql` | Create `stations` table, FK to `companies`, GiST index, partial active index. |
| 3 | `20260523_0003_init_chargers.sql` | Create `chargers` table, FK to `stations`, station index. |
| 4 | `20260523_0004_seed_synthetic.sql` | Insert 1 demo company + 500 stations + 1–4 chargers each, all `is_test=TRUE`. Deterministic UUIDs. |

Each migration is reversible by a corresponding `*.down.sql` if SQLx-migrate's reversible mode is enabled; MVP 1 uses forward-only migrations (the constitution's CI gate verifies idempotency from an empty DB).

---

## 12. DTO Mapping (Rust ↔ TypeScript)

| Rust (`station_domain::models`) | OpenAPI schema | TypeScript (generated) |
|---|---|---|
| `StationMarker` | `StationMarker` | `StationMarker` |
| `StationDetail` | `StationDetail` | `StationDetail` |
| `Charger` | `Charger` | `Charger` |
| `Company` | `Company` (only `id`, `name`) | `Company` |
| `BboxQuery` (request) | `BboxQuery` | `BboxQuery` |
| `AdminStationCreate` | `AdminStationCreate` | `AdminStationCreate` |
| `AdminStationPatch` | `AdminStationPatch` | `AdminStationPatch` |

`coord` in every DTO is `[number, number]` ordered `[lng, lat]`; the OpenAPI schema marks it as `type: array, items: number, minItems: 2, maxItems: 2`.

---

## 13. Out of Scope for MVP 1 (Recorded Here to Prevent Drift)

- User / driver tables — MVP 2.
- Reviews / favorites — MVP 2.
- Avatars — MVP 3.
- Outbox / event log — MVP 3.
- Redis cache layer — MVP 4.
- Keycloak / invitations — MVP 5.
- Per-service databases — MVP 6.

If any of the above start to appear in MVP 1 PRs, the gate fails on Principle IV (Modular Monolith Until Justified) and Principle XII (Non-Goals).
