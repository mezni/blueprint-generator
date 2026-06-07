# Data Model: GIS Sync System v1

## Existing Tables (consumed/modified by gis-worker)

### `gis.sync_queue` (outbox — consumed)

Created by migration 0014 in Sprint 4. gis-worker reads and writes this table.

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK | Event ULID (EVT- prefix) |
| entity_type | TEXT | `station` | `charger` |
| entity_id | TEXT | FK to the entity (e.g., `STN-xxx`) |
| operation | TEXT | `insert` | `update` | `delete` |
| payload | JSONB NULL | Optional extra data |
| status | TEXT | `pending` | `processing` | `done` | `failed` | `dead_letter` |
| created_at | TIMESTAMPTZ | Row creation time |
| processed_at | TIMESTAMPTZ NULL | When processing completed |

**Worker interactions:**
- Read: `SELECT * FROM gis.sync_queue WHERE status = 'pending' ORDER BY created_at LIMIT <batch_size>`
- Update: `UPDATE gis.sync_queue SET status = 'processing' WHERE id = <id> AND status = 'pending'`
- Update: `UPDATE gis.sync_queue SET status = 'done', processed_at = NOW() WHERE id = <id>`
- Update: `UPDATE gis.sync_queue SET status = 'failed' WHERE id = <id>`
- Update: `UPDATE gis.sync_queue SET status = 'dead_letter' WHERE id = <id>`
- Startup recovery: `UPDATE gis.sync_queue SET status = 'pending' WHERE status = 'processing' AND created_at < NOW() - INTERVAL '<timeout_ms> milliseconds'`

### `inventory.station` (geom column — updated)

Created by migration 0003 in Sprint 4. gis-worker updates the `geom` column.

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK | `STN-<ULID>` |
| latitude | DOUBLE PRECISION | Source lat |
| longitude | DOUBLE PRECISION | Source lng |
| geom | GEOGRAPHY(Point, 4326) NULL | Computed by gis-worker |
| ... | ... | Other columns (name, status, etc.) — not touched by gis-worker |

**Worker interactions:**
- Insert/update: `UPDATE inventory.station SET geom = ST_SetSRID(ST_MakePoint($2, $1), 4326) WHERE id = $3`
- Delete: `UPDATE inventory.station SET geom = NULL WHERE id = $1`

## New Tables (created by gis-worker migrations)

### `gis.osm_roads`

OSM road data for Tunisia, imported via one-time CLI script.

| Column | Type | Notes |
|--------|------|-------|
| osm_id | BIGINT | OSM node/way ID |
| name | TEXT | Road name |
| highway | TEXT | Road type (motorway, primary, secondary, etc.) |
| geom | GEOMETRY(MultiLineString, 4326) | Road geometry |
| tags | JSONB | Additional OSM tags |

Indexes: `GIST(geom)`, `BTREE(highway)`.

### `gis.osm_admin_boundaries`

OSM administrative boundary data for Tunisia (governorates, delegations).

| Column | Type | Notes |
|--------|------|-------|
| osm_id | BIGINT | OSM relation ID |
| name | TEXT | Boundary name |
| admin_level | INTEGER | 4=governorate, 6=delegation |
| geom | GEOMETRY(MultiPolygon, 4326) | Boundary geometry |
| tags | JSONB | Additional OSM tags |

Indexes: `GIST(geom)`, `BTREE(admin_level)`.

### `gis.osm_pois` (optional)

OSM points of interest (optional — imported if available).

| Column | Type | Notes |
|--------|------|-------|
| osm_id | BIGINT | OSM node ID |
| name | TEXT | POI name |
| amenity | TEXT | Type (parking, fuel, etc.) |
| geom | GEOMETRY(Point, 4326) | POI location |
| tags | JSONB | Additional OSM tags |

Indexes: `GIST(geom)`, `BTREE(amenity)`.

## State Machine

```
                +-- transient error --+
                |                     |
                v                     |
  +---------+  +------------+  +------+----+  +------------+
  | pending |→ | processing |→ |  done     |  |            |
  +---------+  +------------+  +-----------+  |            |
       |            |                         |            |
       |            |  +--------+             |  dead_letter
       |            +→ | failed |→ ... (retry)|            |
       |               +--------+    max      +------------+
       |                    |        retries
       +--- startup --------+
       recovery (stale rows)
```

- `pending`: Ready for processing (initial state set by admin-service)
- `processing`: Being processed by worker (set atomically via UPDATE with status check)
- `done`: Successfully processed (station geometry updated)
- `failed`: Transient error occurred; eligible for retry
- `dead_letter`: Max retries exhausted; requires manual inspection
