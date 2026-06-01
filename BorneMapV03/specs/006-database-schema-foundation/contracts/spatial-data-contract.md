# Contract: Spatial Data

## Status

**Adopted**: 2026-05-31
**Scope**: All geospatial data across all schemas
**Owner**: Platform Architecture

## Standards

| Property | Standard | Reason |
|----------|----------|--------|
| Coordinate system | WGS 84 (SRID 4326) | Universal GPS standard |
| Geometry type | GEOGRAPHY(Point) | Automatic Earth-curvature-aware distance in meters |
| Index | GIST | Efficient `ST_DWithin` and nearest-neighbor queries |
| Distance unit | Meters | Native to GEOGRAPHY type |

## Authoritative Source

`inventory.station.location` is the single source of truth for station geometry. The `gis.station_enrichment.coordinates_4326` field is a derived copy for query isolation — it MUST NOT be treated as authoritative.

## Column Definition

```sql
location GEOGRAPHY(Point, 4326) NOT NULL
```

## Index Definition

```sql
CREATE INDEX idx_station_location ON inventory.station USING GIST (location);
```

## Query Patterns

**Nearby stations** (within radius in meters):
```sql
SELECT id, name, location
FROM inventory.station
WHERE ST_DWithin(location, ST_SetSRID(ST_MakePoint(:lng, :lat), 4326)::geography, :radius_meters)
  AND status = 'active'
ORDER BY location <-> ST_SetSRID(ST_MakePoint(:lng, :lat), 4326)::geography
LIMIT :limit;
```

**Distance between two points** (in meters):
```sql
SELECT ST_Distance(
    ST_SetSRID(ST_MakePoint(:lng1, :lat1), 4326)::geography,
    ST_SetSRID(ST_MakePoint(:lng2, :lat2), 4326)::geography
);
```

## Validation Rules

1. Longitude: -180 to 180
2. Latitude: -90 to 90
3. Coordinates MUST be in WGS 84 decimal degrees — NOT projected, NOT degrees/minutes/seconds
4. NULL locations are FORBIDDEN on `inventory.station`
5. Location is immutable once set (no UPDATE of location after creation — create a new station record instead)

## Extension Requirement

All spatial functionality requires `postgis` and `postgis_topology` extensions:

```sql
CREATE EXTENSION IF NOT EXISTS postgis;
```
