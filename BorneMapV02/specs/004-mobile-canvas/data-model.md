# Data Model: Mobile Canvas

**Date**: 2026-05-28 | **Source**: [spec.md](./spec.md)

## Entities

### Partner

Represents an organization that operates charging stations.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | VARCHAR(12) | PK, CHECK `^prt-[a-f0-9]{8}$` | Nanouuid identifier |
| name | VARCHAR(255) | NOT NULL | Organization name |
| type | partner_classification | NOT NULL | Enum: 'Private' or 'Business' |
| contact_email | VARCHAR(255) | NOT NULL | Business contact email |
| is_live | BOOLEAN | NOT NULL, DEFAULT false | Visibility flag for staging |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Record creation timestamp |

### Station

A physical EV charging location owned by a Partner.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | VARCHAR(12) | PK, CHECK `^stn-[a-f0-9]{8}$` | Nanouuid identifier |
| partner_id | VARCHAR(12) | FK → partners(id), ON DELETE RESTRICT | Owning partner |
| name | VARCHAR(255) | NOT NULL | Display name |
| geom | GEOGRAPHY(Point, 4326) | NOT NULL | Spatial location |
| status | VARCHAR(50) | NOT NULL, DEFAULT 'Available' | Current operational state |
| is_live | BOOLEAN | NOT NULL, DEFAULT false | Visibility flag for staging |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Last update timestamp |

**Status Lifecycle**: Available ↔ Occupied, Available → Offline → Available, Available → Maintenance → Available

### Charger

An individual charging unit at a Station.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | VARCHAR(12) | PK, CHECK `^chg-[a-f0-9]{8}$` | Nanouuid identifier |
| station_id | VARCHAR(12) | FK → stations(id), ON DELETE CASCADE | Parent station |
| plug_type | VARCHAR(50) | NOT NULL | Connector standard (e.g., CCS2, CHAdeMO, Type2) |
| power_output | INT | NOT NULL, CHECK ≥ 1 | Power rating in kW |
| status | VARCHAR(50) | NOT NULL, DEFAULT 'Available' | Current operational state |
| is_live | BOOLEAN | NOT NULL, DEFAULT false | Visibility flag for staging |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Last update timestamp |

**Status Lifecycle**: Same as Station (Available, Occupied, Offline, Maintenance)

## Relationships

```
Partner 1───* Station 1───* Charger
  │                        │
  │                        └─ plug_type: VARCHAR(50)
  │                        └─ power_output: INT (kW)
  │
  └─ type: partner_classification
  └─ contact_email: VARCHAR(255)

Station:
  └─ geom: GEOGRAPHY(Point, 4326) — spatial index via GiST
  └─ status: lifecycle-managed string
```

## Constraints Summary

| Constraint | Scope | Rule |
|------------|-------|------|
| Partner ID pattern | partners.id | `^prt-[a-f0-9]{8}$` |
| Station ID pattern | stations.id | `^stn-[a-f0-9]{8}$` |
| Charger ID pattern | chargers.id | `^chg-[a-f0-9]{8}$` |
| Partner FK | stations.partner_id | REFERENCES partners(id) ON DELETE RESTRICT |
| Station FK | chargers.station_id | REFERENCES stations(id) ON DELETE CASCADE |
| Power output | chargers.power_output | ≥ 1 kW |
| Classification | partners.type | 'Private' or 'Business' |

## Indexes

| Index | Table | Column(s) | Type |
|-------|-------|-----------|------|
| idx_stations_geom | stations | geom | GiST |
| idx_stations_partner_id | stations | partner_id | B-tree |
| idx_stations_is_live | stations | is_live | B-tree |
| idx_chargers_station_id | chargers | station_id | B-tree |
