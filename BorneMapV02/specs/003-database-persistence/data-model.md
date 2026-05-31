# Data Model: Database Persistence

## Entity-Relationship Overview

```
Partner (1) ────< (N) Station (1) ────< (N) Charger
```

- A **Partner** owns zero or more **Stations**
- A **Station** belongs to exactly one **Partner**
- A **Station** has one or more **Chargers**
- A **Charger** belongs to exactly one **Station**

## Entities

### Partner

Represents a charging network operator or property owner.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | VARCHAR(12) | PK, pattern `^prt-[a-f0-9]{8}$` | Unique identifier |
| `name` | VARCHAR(255) | NOT NULL | Display name |
| `type` | ENUM('Private', 'Business') | NOT NULL | Classification |
| `contact_email` | VARCHAR(255) | NOT NULL | Operator contact |
| `is_live` | BOOLEAN | NOT NULL DEFAULT false | Master visibility flag |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Creation timestamp |

### Station

Represents a physical EV charging location.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | VARCHAR(12) | PK, pattern `^stn-[a-f0-9]{8}$` | Unique identifier |
| `partner_id` | VARCHAR(12) | FK → partners(id), ON DELETE RESTRICT | Owning partner |
| `name` | VARCHAR(255) | NOT NULL | Display name |
| `geom` | GEOGRAPHY(Point, 4326) | NOT NULL | Geospatial position |
| `status` | VARCHAR(50) | NOT NULL DEFAULT 'Available' | Operational status |
| `is_live` | BOOLEAN | NOT NULL DEFAULT false | Visibility for driver queries |
| `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Last update timestamp |

**Indexes**:
- `idx_stations_geom` — GiST index on `geom` (spatial queries)
- `idx_stations_partner_id` — B-tree on `partner_id` (JOIN performance)
- `idx_stations_is_live` — B-tree on `is_live` (visibility filtering)

### Charger

Represents an individual charging connector at a station.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | VARCHAR(12) | PK, pattern `^chg-[a-f0-9]{8}$` | Unique identifier |
| `station_id` | VARCHAR(12) | FK → stations(id), ON DELETE CASCADE | Parent station |
| `plug_type` | VARCHAR(50) | NOT NULL | Connector standard (e.g., CCS2, Type2) |
| `power_output` | INT | NOT NULL | Output in kW |
| `status` | VARCHAR(50) | NOT NULL DEFAULT 'Available' | Operational status |
| `is_live` | BOOLEAN | NOT NULL DEFAULT false | Visibility flag |
| `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Last update timestamp |

**Indexes**:
- `idx_chargers_station_id` — B-tree on `station_id` (JOIN performance)

## Validation Rules

1. All IDs must match their respective regex patterns (`prt-`, `stn-`, `chg-` + 8 hex chars)
2. `power_output` must be positive integer (minimum 1 kW)
3. `latitude` range: -90 to 90 (enforced by PostGIS check constraint)
4. `longitude` range: -180 to 180 (enforced by PostGIS)
5. Station `status` values: 'Available' or 'Occupied' (application-level enum)
6. Charger `status` values: 'Available' or 'Occupied'
7. Deleting a partner is restricted if it has stations (`ON DELETE RESTRICT`)
8. Deleting a station cascades to its chargers (`ON DELETE CASCADE`)

## API Response Shape

### GET /api/v1/stations/nearby

```json
[
  {
    "id": "stn-e3b0c442",
    "name": "LES BERGES DU LAC 2 HUB",
    "partner": {
      "id": "prt-a1b2c3d4",
      "name": "TotalEnergies Tunisia",
      "type": "Business"
    },
    "latitude": 36.8324,
    "longitude": 10.2321,
    "status": "Available",
    "chargers": [
      {
        "id": "chg-7b2a19f4",
        "plug_type": "CCS2",
        "power_output": 120,
        "status": "Available"
      }
    ],
    "is_live": false,
    "updated_at": "2026-05-28T09:41:00Z"
  }
]
```

### GET /health

```json
{
  "status": "ok",
  "database": "connected"
}
```
