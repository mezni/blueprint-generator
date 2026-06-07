# Data Model: Sprint 0 Foundation

**Date**: 2026-06-05  
**Plan**: [plan.md](plan.md)  
**Scope**: Database schemas, entity relationships, and seed data structure

---

## Overview

Sprint 0 establishes three separate PostgreSQL databases per constitution:

1. **keycloak_db** — Identity management (Keycloak managed; not touched in Sprint 0)
2. **platform_db** — Business data (inventory, users, gis schemas)
3. **analytics_db** — Event streaming (reserved for future; not used in Sprint 0)

This document focuses on **platform_db** schema definitions for MVP01 public discovery.

---

## Database: platform_db

### Schema: inventory

Core business entities for charging infrastructure.

#### Table: partner

Partnership organization records.

```sql
CREATE TABLE inventory.partner (
    id              TEXT PRIMARY KEY,              -- PRT-xxxxxxxxxxxxxxxx (16-char NanoID)
    name            TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ,                   -- Soft delete
    
    CONSTRAINT partner_name_not_empty CHECK (name <> '')
);

CREATE INDEX idx_partner_deleted_at ON inventory.partner(deleted_at)
    WHERE deleted_at IS NULL;                      -- Partial index for active partners
```

**Fields**:
- `id`: Unique identifier with PRT- prefix
- `name`: Organization name
- `created_at`: Timestamp when partner was created
- `updated_at`: Timestamp of last update
- `deleted_at`: Soft delete marker; NULL = active, NOT NULL = inactive

**Relationships**:
- One-to-Many with `station` (partner → stations)

**Validation**:
- Name is required and non-empty
- All timestamps are required

---

#### Table: station

EV charging stations.

```sql
CREATE TABLE inventory.station (
    id              TEXT PRIMARY KEY,              -- STN-xxxxxxxxxxxxxxxx (16-char NanoID)
    partner_id      TEXT NOT NULL REFERENCES inventory.partner(id),
    name            TEXT NOT NULL,
    address         TEXT,
    latitude        NUMERIC(10,7) NOT NULL,        -- -90.0000000 to 90.0000000
    longitude       NUMERIC(10,7) NOT NULL,        -- -180.0000000 to 180.0000000
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ,                   -- Soft delete
    
    CONSTRAINT station_name_not_empty CHECK (name <> ''),
    CONSTRAINT station_lat_valid CHECK (latitude >= -90 AND latitude <= 90),
    CONSTRAINT station_lng_valid CHECK (longitude >= -180 AND longitude <= 180)
);

CREATE INDEX idx_station_partner_id ON inventory.station(partner_id);
CREATE INDEX idx_station_location ON inventory.station USING GIST (
    ST_MakePoint(longitude, latitude)::geography
);
CREATE INDEX idx_station_deleted_at ON inventory.station(deleted_at)
    WHERE deleted_at IS NULL;                      -- Partial index for active stations
```

**Fields**:
- `id`: Unique identifier with STN- prefix
- `partner_id`: Foreign key to partner (required)
- `name`: Station name or display label
- `address`: Street address or location description
- `latitude`: WGS84 latitude (-90 to 90)
- `longitude`: WGS84 longitude (-180 to 180)
- `created_at`: Timestamp when station was created
- `updated_at`: Timestamp of last update
- `deleted_at`: Soft delete marker; NULL = active, NOT NULL = inactive

**Relationships**:
- Many-to-One with `partner` (station → partner)
- One-to-Many with `charger` (station → chargers)
- One-to-One with `gis.station_locations` (for spatial data projection)

**Validation**:
- Name is required and non-empty
- Latitude must be between -90 and 90
- Longitude must be between -180 and 180
- Partner ID must exist in inventory.partner
- Inactive stations (deleted_at IS NOT NULL) are filtered from public queries

**Spatial Indexes**:
- GIST index on PostGIS point geometry for distance queries (`ST_DWithin`)
- Enables `/stations/nearby` to efficiently find stations within a radius

---

#### Table: charger

Individual charging ports/connectors at stations.

```sql
CREATE TABLE inventory.charger (
    id              TEXT PRIMARY KEY,              -- CHG-xxxxxxxxxxxxxxxx (16-char NanoID)
    station_id      TEXT NOT NULL REFERENCES inventory.station(id),
    connector_type  TEXT NOT NULL,                 -- CCS2, Type2, TeslaSupercharger, etc.
    power_kw        NUMERIC(6,2),                  -- NULL allowed for unknown
    status          TEXT NOT NULL DEFAULT 'available',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ,                   -- Soft delete
    
    CONSTRAINT charger_status_valid CHECK (
        status IN ('available', 'in_use', 'maintenance', 'offline')
    ),
    CONSTRAINT charger_power_positive CHECK (power_kw > 0 OR power_kw IS NULL)
);

CREATE INDEX idx_charger_station_id ON inventory.charger(station_id);
CREATE INDEX idx_charger_status ON inventory.charger(status)
    WHERE deleted_at IS NULL;                      -- Partial index for active chargers
CREATE INDEX idx_charger_deleted_at ON inventory.charger(deleted_at)
    WHERE deleted_at IS NULL;
```

**Fields**:
- `id`: Unique identifier with CHG- prefix
- `station_id`: Foreign key to station (required)
- `connector_type`: Type of connector (enum-like string)
- `power_kw`: Power output in kilowatts (NULL for unknown)
- `status`: Current operational status (enum)
- `created_at`: Timestamp when charger was created
- `updated_at`: Timestamp of last update
- `deleted_at`: Soft delete marker

**Relationships**:
- Many-to-One with `station` (charger → station)

**Validation**:
- Connector type is required (not empty)
- Status is restricted to predefined values
- Power must be positive or NULL
- Station ID must exist in inventory.station
- Inactive chargers are filtered from availability counts

**Status Values**:
- `available` — Ready for use, no active session
- `in_use` — Currently charging a vehicle
- `maintenance` — Out of service, maintenance in progress
- `offline` — Out of service, reason TBD

---

### Schema: gis

Geospatial projection layer (derived from OSM + inventory data).

#### Table: osm_nodes

Raw OpenStreetMap node data (populated by osm2pgsql, Sprint 1).

```sql
CREATE TABLE gis.osm_nodes (
    osm_id          BIGINT PRIMARY KEY,
    tags            JSONB,
    geom            GEOMETRY(Point, 4326),
    
    CONSTRAINT osm_nodes_geom_not_null CHECK (geom IS NOT NULL)
);

CREATE INDEX idx_osm_nodes_geom ON gis.osm_nodes USING GIST(geom);
CREATE INDEX idx_osm_nodes_tags ON gis.osm_nodes USING GIN(tags);
```

---

#### Table: osm_ways

Raw OpenStreetMap way data (populated by osm2pgsql, Sprint 1).

```sql
CREATE TABLE gis.osm_ways (
    osm_id          BIGINT PRIMARY KEY,
    tags            JSONB,
    geom            GEOMETRY(LineString, 4326),
    
    CONSTRAINT osm_ways_geom_not_null CHECK (geom IS NOT NULL)
);

CREATE INDEX idx_osm_ways_geom ON gis.osm_ways USING GIST(geom);
CREATE INDEX idx_osm_ways_tags ON gis.osm_ways USING GIN(tags);
```

---

#### Table: roads

Derived road network from OSM ways with `highway` tag.

```sql
CREATE TABLE gis.roads (
    id              BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    osm_id          BIGINT REFERENCES gis.osm_ways(osm_id),
    name            TEXT,
    road_type       TEXT,                          -- highway tag value
    geom            GEOMETRY(LineString, 4326),
    
    CONSTRAINT roads_geom_not_null CHECK (geom IS NOT NULL)
);

CREATE INDEX idx_roads_osm_id ON gis.roads(osm_id);
CREATE INDEX idx_roads_geom ON gis.roads USING GIST(geom);
CREATE INDEX idx_roads_road_type ON gis.roads(road_type);
```

---

#### Table: boundaries

Administrative boundaries from OSM relations.

```sql
CREATE TABLE gis.boundaries (
    id              BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    osm_id          BIGINT,
    name            TEXT,
    admin_level     INTEGER,                       -- OSM admin_level tag
    geom            GEOMETRY(MultiPolygon, 4326),
    
    CONSTRAINT boundaries_geom_not_null CHECK (geom IS NOT NULL)
);

CREATE INDEX idx_boundaries_osm_id ON gis.boundaries(osm_id);
CREATE INDEX idx_boundaries_geom ON gis.boundaries USING GIST(geom);
CREATE INDEX idx_boundaries_admin_level ON gis.boundaries(admin_level);
```

---

#### Table: amenity_points

Points of interest from OSM (populated Sprint 1, not used MVP01).

```sql
CREATE TABLE gis.amenity_points (
    id              BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    osm_id          BIGINT,
    amenity_type    TEXT,                          -- amenity tag value
    name            TEXT,
    tags            JSONB,
    geom            GEOMETRY(Point, 4326),
    
    CONSTRAINT amenity_points_geom_not_null CHECK (geom IS NOT NULL)
);

CREATE INDEX idx_amenity_points_osm_id ON gis.amenity_points(osm_id);
CREATE INDEX idx_amenity_points_geom ON gis.amenity_points USING GIST(geom);
CREATE INDEX idx_amenity_points_amenity_type ON gis.amenity_points(amenity_type);
```

---

#### Table: station_locations

Station spatial projection (populated by GIS Sync Worker, Sprint 2+).

```sql
CREATE TABLE gis.station_locations (
    station_id      TEXT PRIMARY KEY REFERENCES inventory.station(id),
    geom            GEOMETRY(Point, 4326) NOT NULL,
    snapped_road_id BIGINT REFERENCES gis.roads(id),
    region_id       BIGINT REFERENCES gis.boundaries(id),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT station_locations_geom_valid 
        CHECK (ST_IsValid(geom) AND GeometryType(geom) = 'POINT')
);

CREATE INDEX idx_station_locations_geom ON gis.station_locations USING GIST(geom);
```

**Purpose**: Spatial projection of `inventory.station` for efficient geospatial queries. Updated asynchronously by GIS Sync Worker; does not block station updates.

---

## Entity Relationships

```
┌─────────────────┐
│   partner       │
│   (PRT-...)     │
└────────┬────────┘
         │ (1:N)
         │ partner_id FK
         │
     ┌───▼────────────────┐
     │    station         │
     │    (STN-...)       │
     └──┬────────────┬────┘
        │ (1:N)      │
        │ station_id │
        │ FK         │
        │            │ (1:1)
        │            │ station_id PK
        │            │
    ┌───▼───┐  ┌─────▼──────────────┐
    │charger│  │station_locations   │
    │(CHG-)│  │(gis.projection)    │
    └───────┘  └────────────────────┘
```

---

## Indexes for Performance

### inventory.partner
- `idx_partner_deleted_at`: Partial index on deleted_at WHERE deleted_at IS NULL for quick active-only queries

### inventory.station
- `idx_station_partner_id`: Foreign key lookup (filter by partner)
- `idx_station_location`: GIST spatial index for distance queries (`ST_DWithin`)
- `idx_station_deleted_at`: Partial index for active-only queries

### inventory.charger
- `idx_charger_station_id`: Foreign key lookup (get chargers for station)
- `idx_charger_status`: Partial index for availability counts
- `idx_charger_deleted_at`: Partial index for active chargers

### gis.* (OSM tables)
- GIST indexes on all geometry columns for spatial queries
- GIN indexes on JSONB tags for filtering by tag values

---

## Constraints & Validation

All tables enforce:
- **Primary key uniqueness**: 16-char NanoID with prefix ensures global uniqueness
- **Referential integrity**: Foreign keys prevent orphaned records
- **Data quality**: CHECK constraints on coordinates, status enums, power ranges
- **Soft deletes**: All business entities include deleted_at; active filters applied in queries
- **Timestamps**: created_at and updated_at required on all mutable entities

---

## Sprint 0 Scope

Sprint 0 creates the **schema only** — no data seeds beyond the structure.

### Migrations Created
1. `0001_extensions.sql` — Enable PostGIS, uuid-ossp, pgcrypto
2. `0002_inventory_schema.sql` — Create inventory schema
3. `0003_gis_schema.sql` — Create gis schema (empty of data; osm2pgsql populates in Sprint 1)

### Not Included in Sprint 0
- OSM data import (Sprint 1)
- Dev seeds (2 partners, 10 stations, chargers) (Sprint 1)
- GIS Sync Worker (Sprint 2+)

---

## Sprint 1+ Continuation

**Sprint 1**:
- `0004_gis_tables.sql` — Create osm_*, roads, boundaries, amenity_points, station_locations tables
- `0005_inventory_tables.sql` — Create partner, station, charger tables (may be combined with 0002-0003)
- OSM import via osm2pgsql
- Dev seeds

**Sprint 2+**:
- Keycloak integration (keycloak_db)
- Analytics schema (analytics_db)
- GIS Sync Worker (populates station_locations)
