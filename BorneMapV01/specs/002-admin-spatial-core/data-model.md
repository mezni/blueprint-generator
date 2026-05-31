# Data Model: Admin Spatial Core Validation

## Persistent Entities

### stations (extended from Phase 0)

| Column | Type | Constraints |
|--------|------|-------------|
| id | SERIAL | PRIMARY KEY |
| partner_id | INTEGER | FK → partners(id), NULLABLE |
| name | VARCHAR(255) | NOT NULL |
| operator | VARCHAR(255) | NULLABLE |
| address | VARCHAR(500) | NULLABLE |
| location | GEOGRAPHY(Point, 4326) | NULLABLE, GiST index enabled |
| plug_types | TEXT[] | NULLABLE |
| speed_kw | FLOAT | NULLABLE |
| is_active | BOOLEAN | NOT NULL, DEFAULT true |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |

**Uniqueness**: Station name unique per operator (application-layer enforced).
**Spatial Index**: `idx_stations_location` — GiST index on `location` column.

### API Extensions (Phase 1)

| Method | Path | Description | Auth |
|--------|------|-------------|------|
| GET | /api/v1/stations/nearby?lat={lat}&lng={lng}&radius_m={m} | Find active stations within radius, sorted by distance | Public |
| GET | /api/v1/stations?sw_lat={}&sw_lng={}&ne_lat={}&ne_lng={} | Viewport-filtered station list | Public |
| POST | /api/v1/stations | Create a new station with spatial coordinates | Admin |
| PATCH | /api/v1/stations/{id} | Update station fields | Admin |
| DELETE | /api/v1/stations/{id} | Delete a station | Admin |

### Spatial Query Patterns

**Nearby search (ST_DWithin + ST_Distance):**
```sql
SELECT *, ST_Distance(location, ST_MakePoint(:lng, :lat)::geography) AS dist
FROM stations
WHERE ST_DWithin(location, ST_MakePoint(:lng, :lat)::geography, :radius_m)
  AND is_active = true
ORDER BY dist ASC;
```

**Viewport filter (bounds-derived ST_DWithin):**
```sql
SELECT *
FROM stations
WHERE ST_DWithin(
  location,
  ST_MakePoint(:center_lng, :center_lat)::geography,
  :diagonal_radius_m
)
  AND is_active = true
ORDER BY id;
```

### CoordinatePoint Schema

Input validation for spatial coordinates:

| Field | Type | Validation |
|-------|------|------------|
| coordinates | [float, float] | [longitude, latitude], lon ∈ [-180,180], lat ∈ [-90,90] |
| type | "Point" | Literal "Point" |

### Mock JWT Payload

| Claim | Type | Description |
|-------|------|-------------|
| sub | string | User ID |
| username | string | Admin username |
| roles | string[] | Role list (e.g., ["admin"]) |
| iat | number | Issued-at timestamp |
| exp | number | Expiration timestamp |

Header: `X-Mock-Token: base64({...})`
