# GIS Schema

Schema: `gis` in `platform_db`

## Rules

- GIS contains derived spatial data only
- NOT a source of truth
- Updated asynchronously via GIS Sync Worker
- Failures do not block station updates

## Tables

### `station_location`

| Column | Type | Description |
|--------|------|-------------|
| station_id | TEXT | FK to inventory.station |
| geom | GEOGRAPHY(Point, 4326) | Spatial point for GIS queries |
| updated_at | TIMESTAMPTZ | Last sync timestamp |

### `station_cluster`

| Column | Type | Description |
|--------|------|-------------|
| cluster_id | TEXT | Cluster identifier |
| station_ids | TEXT[] | Array of station IDs in cluster |
| center | GEOGRAPHY(Point, 4326) | Cluster center point |
| count | INTEGER | Number of stations in cluster |
