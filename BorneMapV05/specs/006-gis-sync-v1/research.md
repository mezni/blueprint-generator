# Research: GIS Sync System v1

## Geometry Update Strategy

- **Decision**: Use `ST_SetSRID(ST_MakePoint($2, $1), 4326)` directly in UPDATE statement
- **Rationale**: Single SQL expression, no intermediate computation, leverages PostGIS native functions. The geometry is stored as `GEOGRAPHY(Point, 4326)` per the existing schema.
- **Alternatives considered**: Computing GeoJSON in Rust and using `ST_GeomFromGeoJSON` — adds serialization overhead with no benefit.

## Idempotency Approach

- **Decision**: UPDATE is naturally idempotent — running `SET geom = ST_SetSRID(ST_MakePoint(lng, lat), 4326)` with the same lat/lng values always produces the same geometry. For `delete`, `SET geom = NULL` is also idempotent.
- **Rationale**: No UPSERT needed since there is exactly one station row per entity_id. The UPDATE targets a specific station by primary key. Replaying the same operation yields bit-identical geometry.
- **Alternatives considered**: Using `INSERT ... ON CONFLICT DO UPDATE` — unnecessary since stations already exist.

## Concurrent Batch Processing

- **Decision**: Process rows in parallel within each batch using `tokio::join_all` or `futures::stream::FuturesUnordered`
- **Rationale**: Each row targets a different station (different `entity_id`), so no row-level conflicts. Parallel processing maximizes throughput within the poll interval.
- **Alternatives considered**: Sequential processing (too slow for large batches), tokio::spawn per row (overkill, no benefit over join_all).

## Stale Processing Row Recovery

- **Decision**: On startup, reset any rows in `processing` status older than `GIS_WORKER_STALE_PROCESSING_TIMEOUT_MS` back to `pending`
- **Rationale**: A crash during processing leaves rows stuck in `processing`. The timeout prevents infinite stuck rows while giving in-flight operations time to complete.
- **Alternatives considered**: Resetting all `processing` rows on startup (risks double-processing of rows that were actually in-flight), using advisory locks (too complex for v1).

## OSM Tunisia Import

- **Decision**: One-time CLI command using `osm2pgsql` (external tool) or `ogr2ogr` to download and import the Geofabrik PBF extract
- **Rationale**: OSM tools (osm2pgsql, ogr2ogr) are mature, well-tested, and handle the complex OSM data model. Writing a custom OSM parser in Rust is not worthwhile for a one-time import.
- **Source URL**: `https://download.geofabrik.de/africa/tunisia-latest.osm.pbf`
- **Target tables**: `gis.osm_roads`, `gis.osm_admin_boundaries` (simplified schema, not full OSM model)
- **Alternatives considered**: Overpass API queries (slow, rate-limited, no bulk support); bundled PBF in repo (large binary, versioning nightmare).

## Retry/Backoff Strategy

- **Decision**: Exponential backoff with jitter: `base_delay * 2^attempt + random(0, base_delay)`, capped at `GIS_WORKER_MAX_RETRIES` (default 3)
- **Rationale**: Standard exponential backoff prevents thundering herd on transient failures. Jitter prevents synchronized retry storms.
- **Alternatives considered**: Fixed retry interval (too aggressive for DB outages), no retry (unreliable).

## Feature Flag Integration

- **Decision**: Check `FF_ENABLE_GIS_SYNC` env var at startup; if `false`, log a message and exit immediately
- **Rationale**: Simple, fail-fast approach. No runtime feature flag system needed for v1.
- **Alternatives considered**: Dynamic feature flag polling (over-engineered for v1).

## Configuration Reference

All config values with defaults:

| Variable | Default | Description |
|----------|---------|-------------|
| `GIS_WORKER_PORT` | 8084 | HTTP health endpoint port |
| `GIS_WORKER_POLL_INTERVAL_MS` | 5000 | Poll loop sleep interval |
| `GIS_WORKER_BATCH_SIZE` | 50 | Max rows per batch |
| `GIS_WORKER_MAX_RETRIES` | 3 | Max retry attempts before dead_letter |
| `GIS_WORKER_RETRY_BASE_DELAY_MS` | 1000 | Base delay for exponential backoff |
| `GIS_WORKER_STALE_PROCESSING_TIMEOUT_MS` | 30000 | Age threshold for stale `processing` rows |
| `GIS_DEFAULT_SRID` | 4326 | Coordinate system SRID |
| `FF_ENABLE_GIS_SYNC` | true | Feature flag to disable GIS sync |
| `PLATFORM_DB_*` | — | Database connection config (from common-db) |
