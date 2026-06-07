# Feature Specification: GIS Sync System v1

**Feature Branch**: `006-gis-sync-v1`

**Created**: 2026-06-02

**Status**: Draft

**Input**: User description: "Sprint 6 — GIS Sync System v1: implement gis-worker — poll/consume gis.sync_queue (or RabbitMQ), process states pending → processing → done|failed → dead_letter. Convert station lat/lng to geom. OSM Tunisia import (basic). Idempotent + replay-safe + retry/backoff."

## Clarifications

### Session 2026-06-02

- Q: What concurrency model should the worker use for batch processing? → A: Parallel — process all rows within a batch concurrently for maximum throughput; idempotency handles any ordering concerns.
- Q: How should OSM Tunisia data be sourced? → A: Download from Geofabrik (`https://download.geofabrik.de/africa/tunisia-latest.osm.pbf`) at migration/init time via a one-time script.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Station Geometry Auto-Syncs on Mutation (Priority: P1)

When a station is created or updated through the admin or partner API, the GIS worker picks up the outbox row and converts the station's lat/lng into a PostGIS geometry point, making it immediately queryable via spatial queries.

**Why this priority**: Spatial query results are the foundation for map-based station discovery. Without geometry sync, the driver service bbox/radius queries return no results.

**Independent Test**: Create a station via the partner API, wait for the GIS worker poll cycle, then verify the station's `geom` column is populated with a valid Point(4326) geometry. Bbox queries around the station's coordinates return it.

**Acceptance Scenarios**:

1. **Given** a station exists with valid latitude and longitude, **When** its outbox row status transitions to `done`, **Then** `inventory.station.geom` is populated with `ST_SetSRID(ST_MakePoint(lng, lat), 4326)`
2. **Given** a station's geometry is synced, **When** a driver service bbox query runs, **Then** the station is returned in results (using GIST index scan)
3. **Given** a station is updated with new coordinates, **When** the GIS worker processes the `update` outbox row, **Then** the `geom` column reflects the new lat/lng

---

### User Story 2 - Idempotent and Replay-Safe Processing (Priority: P1)

The GIS worker can safely replay the same outbox row multiple times without producing incorrect or duplicate state. This ensures resilience against worker crashes, network failures, and manual replay operations.

**Why this priority**: The outbox pattern with at-least-once delivery guarantees that every row is processed; idempotency prevents data corruption on replay. Without this, recovery from failures would require manual intervention.

**Independent Test**: Process an outbox row, then process it again. The station geometry remains identical after the second run. No duplicate rows are created in GIS tables.

**Acceptance Scenarios**:

1. **Given** a processed outbox row (`status = done`), **When** the worker processes it again (e.g., status reset to `pending` manually), **Then** the station geometry is unchanged and no orphan records are created
2. **Given** a `delete` outbox row, **When** processed once, **Then** the station geometry is cleared/nullified; **When** processed again, **Then** the operation is a no-op (station already has no geometry)
3. **Given** a worker crash mid-processing, **When** the worker restarts and encounters a `processing` row (via startup recovery), **Then** it resumes or re-processes without corruption

---

### User Story 3 - Failed Processing with Retry and Dead-Letter (Priority: P2)

When the GIS worker encounters a transient error (e.g., invalid coordinates, DB connection loss), it retries with backoff and eventually moves the row to a dead-letter queue for manual inspection.

**Why this priority**: Ensures system resilience and observability. Without graceful failure handling, bad data would block the queue indefinitely or be silently dropped.

**Independent Test**: Submit an outbox row with invalid lat/lng, observe the worker retry cycle, then verify the row lands in dead-letter after exhausting retries.

**Acceptance Scenarios**:

1. **Given** an outbox row with out-of-range coordinates (lat > 90 or lng > 180), **When** the worker processes it, **Then** it transitions to `failed` with error details; after max retries it becomes `dead_letter`
2. **Given** a transient DB failure, **When** the worker retries, **Then** it uses exponential backoff (`GIS_WORKER_RETRY_BASE_DELAY_MS`, `GIS_WORKER_MAX_RETRIES`)
3. **Given** a row in `dead_letter` status, **When** inspected, **Then** the `payload` column contains the original outbox data and worker can log the error reason

---

### User Story 4 - Basic OSM Tunisia Base Layer (Priority: P3)

The GIS database includes a basic OSM import for Tunisia (roads, administrative boundaries, points of interest) to provide geographic context for station markers on the map.

**Why this priority**: Enhances map UX by showing roads and landmarks alongside stations. Lower priority because stations are discoverable via coordinates alone.

**Independent Test**: After OSM import, run a bbox query over Tunis and verify that roads and administrative boundaries are returned from the `gis` schema.

**Acceptance Scenarios**:

1. **Given** the OSM Tunisia import has run, **When** a bbox query covers Tunis, **Then** road geometries are returned from the OSM layer
2. **Given** the OSM import, **When** a station is created with coordinates in Tunis, **Then** the station geometry exists alongside the OSM base layer in the same coordinate system (SRID 4326)

---

### Edge Cases

- What happens when a station has `NULL` lat/lng? (Must skip geometry generation and move to `failed` with `INVALID_COORDINATES`)
- What happens when the worker restarts with rows stuck in `processing`? (Worker must treat them as eligible for retry after a configurable `GIS_WORKER_STALE_PROCESSING_TIMEOUT_MS`)
- What happens when no outbox rows exist? (Worker must sleep for `GIS_WORKER_POLL_INTERVAL_MS` and retry)
- What happens when the outbox row references a soft-deleted station? (Worker must still process the geometry update or clear it, depending on operation)
- What happens if the OSM import is very large? (Must use batch insert with progress logging; import runs as a one-time migration/CLI command, not in the main poll loop)
- What happens when both RabbitMQ and direct DB poll are configured? (DB poll is the fallback if RabbitMQ is unavailable)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a `gis-worker` binary that polls `gis.sync_queue` for rows with `status = 'pending'` on a configurable interval (`GIS_WORKER_POLL_INTERVAL_MS`, default 5000)
- **FR-002**: Worker MUST process outbox rows in parallel within each batch, using a configurable batch size (`GIS_WORKER_BATCH_SIZE`, default 50); rows are ordered by `created_at` for fair scheduling but processed concurrently
- **FR-003**: For `insert` and `update` operations with valid lat/lng, worker MUST compute `geom = ST_SetSRID(ST_MakePoint(lng, lat), 4326)` and update `inventory.station.geom` for the matching station
- **FR-004**: For `delete` operations, worker MUST set `inventory.station.geom = NULL` for the matching station
- **FR-005**: Worker MUST transition outbox rows: `pending → processing → done` on success; `pending → processing → failed` on error; `failed → dead_letter` after exhausting retries
- **FR-006**: Worker MUST implement exponential backoff with configurable base delay (`GIS_WORKER_RETRY_BASE_DELAY_MS`, default 1000) and max retries (`GIS_WORKER_MAX_RETRIES`, default 3)
- **FR-007**: Worker MUST be idempotent — processing the same outbox row multiple times MUST produce identical station geometry state
- **FR-008**: Worker MUST handle stale `processing` rows on startup — rows in `processing` for longer than `GIS_WORKER_STALE_PROCESSING_TIMEOUT_MS` (default 30000) are reset to `pending` for retry
- **FR-009**: Worker MUST validate coordinates — lat MUST be in [-90, 90], lng in [-180, 180]; invalid coordinates set status to `failed` with `INVALID_COORDINATES`
- **FR-010**: System MUST provide a one-time CLI command or migration script that downloads Tunisia OSM data from Geofabrik (`https://download.geofabrik.de/africa/tunisia-latest.osm.pbf`) and imports it into `gis.osm_*` tables (roads, admin boundaries)
- **FR-011**: OSM import MUST use SRID 4326 (WGS 84) to match station geometry
- **FR-012**: Worker MUST log each state transition with outbox row id, entity_id, operation, and error details on failure
- **FR-013**: Worker SHOULD support optional RabbitMQ consumption as an alternative to DB polling (deferred to future sprint). For v1, DB polling is the sole consumption mechanism. If `RABBITMQ_QUEUE_GIS_SYNC` is configured in v1, the worker logs a warning that RabbitMQ mode is not yet implemented and falls back to DB polling.
- **FR-014**: Worker MUST use the standard `common-db` PgPool factory and the same `platform_db` connection config as other services
- **FR-015**: Worker MUST respect the `FF_ENABLE_GIS_SYNC` feature flag — if false, worker exits immediately with a log message

### Key Entities

- **Outbox Row** (`gis.sync_queue`): A row representing a pending geometry sync operation. Contains entity_type, entity_id, operation (insert/update/delete), payload (JSONB), and lifecycle status.
- **Station Geometry** (`inventory.station.geom`): A PostGIS GEOGRAPHY(Point, 4326) column that stores the spatial representation of a station's coordinates. This is the authoritative GIS layer.
- **OSM Base Layer** (`gis.osm_*`): Imported OpenStreetMap data for Tunisia providing geographic context (roads, administrative boundaries), separate from station geometries.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A station created via the partner API has its `geom` column populated within `GIS_WORKER_POLL_INTERVAL_MS + 1s` of creation
- **SC-002**: Replaying the same outbox row twice results in identical station geometry (verified by comparing `ST_AsGeoJSON(geom)` before and after replay)
- **SC-003**: All outbox rows eventually reach `done` status for valid coordinates; invalid coordinate rows reach `dead_letter`
- **SC-004**: Bbox queries on `inventory.station.geom` use GIST index (verified via `EXPLAIN ANALYZE`)
- **SC-005**: OSM import runs successfully and Tunisia roads/admin boundaries are queryable from the `gis` schema
- **SC-006**: Worker starts and stops gracefully; stale `processing` rows on restart are handled without manual intervention
- **SC-007**: Worker respects `FF_ENABLE_GIS_SYNC=false` by exiting immediately

## Assumptions

- Sprint 5 (Admin Service MVP) migrations are complete — `gis.sync_queue` table exists and is populated on station mutations
- Sprint 4 (Core DB Schema) migrations are complete — `inventory.station.geom` column exists with GIST index
- PostGIS extension is enabled in `platform_db` (from Sprint 4)
- RabbitMQ is available in the compose stack (from Sprint 2), but DB polling is the primary consumption mechanism for v1
- The `gis-worker` binary skeleton exists from Sprint 1; the service does not need a public HTTP port (internal only)
- OSM Tunisia data can be obtained from a public URL or bundled extract; the import is a one-time operation, not part of the poll loop
- The worker runs in the same Docker network as `platform_db` and (optionally) RabbitMQ
- Idempotency is achieved by making the geometry update an UPSERT or targeted UPDATE that produces the same result regardless of how many times it runs
