# Data Model: Sprint 1 — OSM Data & Station Discovery

**Date**: 2026-06-05 | **Version**: 1.0 | **Status**: Complete

## Overview

This document defines all entities, relationships, validation rules, and state transitions for Sprint 1. It serves as the canonical reference for domain models, database schema design, and API contracts.

---

## Entity Catalog

### 1. Station (`inventory.station`)

**Purpose**: Charging station location managed by a Partner; represents physical infrastructure.

**Identity**: 16-character prefixed NanoID `STN-[A-Z0-9]{13}`

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | VARCHAR(16) | ❌ | PK, `STN-*` pattern | NanoID generated at insert |
| `partner_id` | VARCHAR(16) | ❌ | FK → `inventory.partner(id)` | Owner of this station |
| `name` | VARCHAR(255) | ❌ | NOT NULL, unique per partner | e.g., "Tunis Central Station" |
| `address` | TEXT | ✅ | Nullable | Street address if available |
| `latitude` | DECIMAL(10, 8) | ❌ | NOT NULL, -90 to 90 | WGS84 coordinates |
| `longitude` | DECIMAL(11, 8) | ❌ | NOT NULL, -180 to 180 | WGS84 coordinates |
| `osm_node_id` | BIGINT | ✅ | FK → `gis.osm_nodes(id)` (soft) | Reference to source OSM data |
| `availability_status` | ENUM('available', 'unavailable', 'unknown') | ❌ | DEFAULT 'unknown' | Real-time availability |
| `capacity` | INT | ✅ | > 0 if set | Total number of chargers at station |
| `created_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | Immutable |
| `updated_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW(), updated on each change | Tracks last modification |
| `deleted_at` | TIMESTAMPTZ | ✅ | Soft delete marker | Non-null = station is inactive |

**Relationships**:
- `partner_id` → `inventory.partner` (Many-to-One): Each station owned by exactly one partner
- Reverse: One partner owns many stations
- `osm_node_id` → `gis.osm_nodes` (Soft reference): Source OSM POI (informational only)
- One-to-Many: Station → Many `inventory.charger` records
- One-to-Many: Station → Many `users.favorite` records
- One-to-One (derived): Station ↔ `gis.station_locations` (GIS projection)

**Lifecycle**:

```
[Created] → [Available|Unavailable] → [Soft Deleted] → [Archived (post-MVP)]
```

1. **Created**: Partner creates via API; `deleted_at = NULL`, `availability_status = 'unknown'`
2. **Available/Unavailable**: Partner updates status via `/api/v1/partner/stations/{id}` 
3. **Soft Deleted**: Partner soft-deletes via API; `deleted_at = NOW()`; station no longer appears in public discovery
4. **Archived**: Post-MVP; hard delete after retention period (e.g., 90 days)

**Validation Rules**:
- `name`: 3-255 characters, non-empty
- `latitude`: -90.0 to 90.0 (inclusive)
- `longitude`: -180.0 to 180.0 (inclusive)
- `partner_id`: Must exist in `inventory.partner`
- `capacity`: If set, must be > 0
- Uniqueness: (`partner_id`, `name`) tuple must be unique (no duplicate names per partner)

**Query Patterns**:
```sql
-- Public discovery (no auth)
SELECT id, name, address, latitude, longitude
FROM inventory.station
WHERE deleted_at IS NULL AND ST_DWithin(
    ST_SetSRID(ST_Point(longitude, latitude), 4326)::geography,
    ST_SetSRID(ST_Point(?, ?), 4326)::geography,
    ?
)
ORDER BY ST_Distance(...) ASC;

-- Partner view (scoped to partner_id from JWT)
SELECT id, name, availability_status, capacity, created_at, updated_at
FROM inventory.station
WHERE partner_id = ? AND deleted_at IS NULL;

-- Admin view (unfiltered)
SELECT * FROM inventory.station;
```

**Notes**:
- Coordinates stored as DECIMAL (precise, indexable); converted to PostGIS geometry in `gis.station_locations`
- Soft delete allows recovery; station can be undeleted by setting `deleted_at = NULL`
- `availability_status` is real-time data managed by partner; NOT derived from charger counts

---

### 2. Charger (`inventory.charger`)

**Purpose**: Individual charging port at a station with specific connector type and power rating.

**Identity**: 16-character prefixed NanoID `CHG-[A-Z0-9]{13}`

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | VARCHAR(16) | ❌ | PK, `CHG-*` pattern | NanoID generated at insert |
| `station_id` | VARCHAR(16) | ❌ | FK → `inventory.station(id)` | Parent station |
| `connector_type` | ENUM ('chademo', 'type2', 'tesla_us', 'gb_t') | ❌ | NOT NULL | Charge connector standard |
| `power_kw` | DECIMAL(5, 2) | ❌ | > 0 | Charging power in kilowatts |
| `status` | ENUM ('available', 'in_use', 'maintenance', 'offline') | ❌ | DEFAULT 'available' | Current port status |
| `created_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | Immutable |
| `updated_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW(), updated on each change | |
| `deleted_at` | TIMESTAMPTZ | ✅ | Soft delete marker | |

**Relationships**:
- `station_id` → `inventory.station` (Many-to-One): Each charger belongs to one station
- One-to-Many: Charger → Many `analytics.usage_event` records

**Lifecycle**:
```
[Created] → [Available|In Use|Maintenance|Offline] → [Soft Deleted]
```

**Validation Rules**:
- `connector_type`: Must be one of the defined ENUM values
- `power_kw`: Must be > 0 and ≤ 350 kW (practical limit for chargers)
- `station_id`: Must exist in `inventory.station` and NOT be soft-deleted

**Notes**:
- Status is updated in real-time by partner via API
- Not exposed in public discovery; discovered implicitly when browsing station details (future feature, P2+)
- Soft delete allows recovery; removed charger can be re-enabled

---

### 3. Partner (`inventory.partner`)

**Purpose**: Business entity that owns and manages one or more charging stations.

**Identity**: 16-character prefixed NanoID `PRT-[A-Z0-9]{13}`

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | VARCHAR(16) | ❌ | PK, `PRT-*` pattern | NanoID generated at insert |
| `name` | VARCHAR(255) | ❌ | NOT NULL | Organization name |
| `email` | VARCHAR(255) | ❌ | UNIQUE, NOT NULL | Primary contact email |
| `phone` | VARCHAR(20) | ✅ | Nullable | Contact phone |
| `country` | VARCHAR(2) | ❌ | ISO 3166-1 alpha-2 (e.g., 'TN') | Primary operating country |
| `status` | ENUM ('active', 'inactive', 'suspended') | ❌ | DEFAULT 'active' | Partner account status |
| `created_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | Immutable |
| `updated_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | |
| `deleted_at` | TIMESTAMPTZ | ✅ | Soft delete marker | |

**Relationships**:
- One-to-Many: Partner → Many `inventory.station` records
- One-to-Many: Partner → Many `users.user` records (partner users)

**Validation Rules**:
- `name`: 3-255 characters
- `email`: Valid RFC 5322 email format; unique across all partners
- `country`: Valid ISO 3166-1 alpha-2 code
- At least one active partner user must exist before stations can be created

**Notes**:
- Once created, cannot be moved to different country (immutable constraint planned for future)
- Soft delete prevents further station additions but doesn't delete existing stations
- No hard delete in MVP; soft-deleted partners can be archived post-MVP

---

### 4. User (`users.user`)

**Purpose**: Authenticated user of the system; either a registered driver or a partner user.

**Identity**: 16-character prefixed NanoID `USR-[A-Z0-9]{13}`

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | VARCHAR(16) | ❌ | PK, `USR-*` pattern | NanoID generated at insert |
| `keycloak_id` | UUID | ❌ | UNIQUE, NOT NULL | External ID from Keycloak |
| `email` | VARCHAR(255) | ❌ | NOT NULL | From Keycloak; may not be unique across users |
| `name` | VARCHAR(255) | ✅ | Nullable | Full name from Keycloak profile |
| `role` | ENUM ('registered_driver', 'partner', 'admin') | ❌ | NOT NULL | User type (strict set) |
| `partner_id` | VARCHAR(16) | ✅ | FK → `inventory.partner(id)` | Non-null only if role = 'partner' |
| `created_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | Immutable |
| `updated_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | |
| `deleted_at` | TIMESTAMPTZ | ✅ | Soft delete marker | |

**Relationships**:
- `keycloak_id`: One-to-One with Keycloak user (informational only)
- `partner_id` → `inventory.partner` (Many-to-One): Partner users belong to exactly one partner
- One-to-Many: User → Many `users.favorite` records
- One-to-Many: User → Many `users.review` records

**Validation Rules**:
- `keycloak_id`: Must be valid UUID format; must exist in Keycloak
- `email`: Valid email format
- `role`: Must be exactly one of the three defined roles
- If `role = 'partner'`: `partner_id` must be non-null and exist in `inventory.partner`
- If `role = 'registered_driver'` or `'admin'`: `partner_id` must be NULL
- Constraint: No more than one partner user per partner (enforced at business logic layer)

**Lifecycle**:
```
[Created in Keycloak] → [Synced to users.user] → [Soft Deleted] → [Archived post-MVP]
```

1. User authenticates via Keycloak
2. JWT contains `sub` (user ID), `email`, and `partner_id` (if applicable)
3. On first login, middleware upserts user into `users.user` table
4. Soft delete: Set `deleted_at` (user cannot login, but history preserved)

**Notes**:
- User record synced from Keycloak on first authenticated request
- Keycloak is source of truth for identity; `users.user` is cached copy
- Partner users are strictly scoped to one organization; enforced at API layer

---

### 5. Favorite (`users.favorite`)

**Purpose**: User's saved charging station; enables registered drivers to bookmark stations for quick access.

**Identity**: 16-character prefixed NanoID `FAV-[A-Z0-9]{13}`

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | VARCHAR(16) | ❌ | PK, `FAV-*` pattern | NanoID generated at insert |
| `user_id` | VARCHAR(16) | ❌ | FK → `users.user(id)` | User who saved the favorite |
| `station_id` | VARCHAR(16) | ❌ | FK → `inventory.station(id)` | Favorited station |
| `created_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | Immutable |

**Constraints**:
- UNIQUE(`user_id`, `station_id`): A user can favorite a station at most once
- Composite PK or UNIQUE index recommended for performance

**Relationships**:
- `user_id` → `users.user` (Many-to-One)
- `station_id` → `inventory.station` (Many-to-One)

**Lifecycle**:
```
[Created] → [Deleted (hard delete)]
```

1. User creates favorite; `created_at = NOW()`
2. User removes favorite; row is hard-deleted (no soft delete for ephemeral data)
3. If station is soft-deleted, favorite remains but station is hidden from UI

**Validation Rules**:
- `user_id`: Must exist in `users.user` and NOT be soft-deleted
- `station_id`: Must exist in `inventory.station`
- `user_id` must have role `registered_driver` (only drivers can create favorites)

**Query Patterns**:
```sql
-- Get user's favorites (with station details)
SELECT f.id, s.id, s.name, s.address, s.latitude, s.longitude
FROM users.favorite f
JOIN inventory.station s ON f.station_id = s.id
WHERE f.user_id = ? AND s.deleted_at IS NULL
ORDER BY f.created_at DESC;

-- Check if user has favorited a station
SELECT COUNT(*) 
FROM users.favorite 
WHERE user_id = ? AND station_id = ?;
```

**Notes**:
- No soft delete; hard delete is immediate and permanent
- Favorite can point to a soft-deleted station (but UI hides it)
- No audit trail needed (ephemeral, user-driven)

---

### 6. Review (`users.review`)

**Purpose**: User's rating and comment on a station (P2 feature, included in data model for completeness).

**Identity**: 16-character prefixed NanoID `REV-[A-Z0-9]{13}`

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | VARCHAR(16) | ❌ | PK, `REV-*` pattern | NanoID generated at insert |
| `user_id` | VARCHAR(16) | ❌ | FK → `users.user(id)` | Author of review |
| `station_id` | VARCHAR(16) | ❌ | FK → `inventory.station(id)` | Reviewed station |
| `rating` | SMALLINT | ❌ | CHECK (1-5) | Star rating (1-5) |
| `comment` | TEXT | ✅ | Nullable, max 1000 chars | Freeform review text |
| `created_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | Immutable |
| `updated_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | |
| `deleted_at` | TIMESTAMPTZ | ✅ | Soft delete marker | User can remove review |

**Constraints**:
- UNIQUE(`user_id`, `station_id`): User can review a station at most once (can update)

**Relationships**:
- `user_id` → `users.user` (Many-to-One)
- `station_id` → `inventory.station` (Many-to-One)

**Lifecycle**:
```
[Created] → [Updated (optional)] → [Soft Deleted (optional)] → [Archived post-MVP]
```

**Validation Rules**:
- `rating`: Must be 1, 2, 3, 4, or 5
- `comment`: If provided, 1-1000 characters
- `user_id`: Must exist in `users.user` and have role `registered_driver`

**Notes**:
- Soft delete allows removal without losing history for analytics
- Update operation only affects `comment` and `updated_at` (not `rating` or `created_at`)
- Visible in public discovery only if not soft-deleted (P2 feature)

---

## Geospatial (GIS) Entities

### 7. OSM Ways (`gis.osm_ways`)

**Purpose**: Roads and boundaries from OpenStreetMap; imported during data load phase.

**Source**: OpenStreetMap (via osm2pgsql)

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | BIGINT | ❌ | PK | OSM way ID |
| `name` | VARCHAR(255) | ✅ | Nullable | Road or boundary name |
| `type` | VARCHAR(50) | ✅ | Nullable | OSM way type (highway, boundary, etc.) |
| `geom` | GEOMETRY(LineString, 4326) | ❌ | NOT NULL | Road geometry (WGS84) |
| `tags` | JSONB | ✅ | Nullable | Full OSM tags |

**Indexes**:
- `CREATE INDEX idx_osm_ways_geom ON gis.osm_ways USING GIST(geom)`

**Notes**:
- Read-only; updated only during OSM import refresh
- Not used in Sprint 1 discovery queries (informational only)
- Available for routing/mapping (future features, P2+)

---

### 8. OSM Nodes (`gis.osm_nodes`)

**Purpose**: Points of interest from OpenStreetMap; used to enrich station locations.

**Source**: OpenStreetMap (via osm2pgsql)

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | BIGINT | ❌ | PK | OSM node ID |
| `name` | VARCHAR(255) | ✅ | Nullable | POI name |
| `amenity` | VARCHAR(50) | ✅ | Nullable | OSM amenity type |
| `geom` | GEOMETRY(Point, 4326) | ❌ | NOT NULL | POI geometry (WGS84) |
| `tags` | JSONB | ✅ | Nullable | Full OSM tags |

**Indexes**:
- `CREATE INDEX idx_osm_nodes_geom ON gis.osm_nodes USING GIST(geom)`

**Notes**:
- Read-only; updated during OSM import refresh
- Linked to `inventory.station` via `osm_node_id` (soft reference)
- Useful for enriching station data with nearby amenities (future)

---

### 9. Station Locations (GIS Projection) (`gis.station_locations`)

**Purpose**: Derived spatial projection of `inventory.station` records; enables fast geospatial queries.

**Source**: Asynchronously synced from `inventory.station` by GIS Sync Worker

**Fields**:

| Field | Type | Nullable | Constraints | Notes |
|-------|------|----------|-------------|-------|
| `id` | VARCHAR(16) | ❌ | PK, FK → `inventory.station(id)` | Unique per station |
| `station_id` | VARCHAR(16) | ❌ | | Denormalized for clarity |
| `name` | VARCHAR(255) | ❌ | NOT NULL | Copied from station.name |
| `partner_id` | VARCHAR(16) | ❌ | | Copied from station.partner_id |
| `geom` | GEOMETRY(Point, 4326) | ❌ | NOT NULL | ST_Point(longitude, latitude) |
| `synced_at` | TIMESTAMPTZ | ❌ | DEFAULT NOW() | Last sync timestamp |
| `deleted_at` | TIMESTAMPTZ | ✅ | Nullable | Marks deleted stations in GIS layer |

**Indexes**:
- PK: `id`
- GIST: `CREATE INDEX idx_station_locations_geom ON gis.station_locations USING GIST(geom)`
- For soft-delete filtering: `CREATE INDEX idx_station_locations_deleted ON gis.station_locations(deleted_at)`

**Constraints**:
- FK: `station_id` → `inventory.station(id)` (soft ref; worker handles cascade)
- Unique: `station_id` (one GIS record per station)

**Sync Logic** (GIS Sync Worker):

```
FOR EACH unprocessed event IN inventory.station_outbox:
  IF event.type = 'created' OR 'updated':
    UPSERT gis.station_locations
      id = event.station_id,
      geom = ST_SetSRID(ST_Point(station.longitude, station.latitude), 4326),
      synced_at = NOW(),
      deleted_at = NULL (clear if was previously soft-deleted)
  ELSE IF event.type = 'deleted':
    UPDATE gis.station_locations SET deleted_at = NOW()
  
  MARK event AS processed_at = NOW()
```

**Query Pattern**:

```sql
-- Find nearby stations (public discovery)
SELECT id, name, 
       ST_Y(geom) as latitude,
       ST_X(geom) as longitude
FROM gis.station_locations
WHERE ST_DWithin(
    geom,
    ST_SetSRID(ST_Point(?, ?), 4326)::geography,
    ? -- radius in meters
)
AND deleted_at IS NULL
ORDER BY ST_Distance(geom, ST_SetSRID(ST_Point(?, ?), 4326)::geography) ASC
LIMIT 100;
```

**Notes**:
- NOT source of truth; `inventory.station` is canonical
- GIS failures (e.g., sync worker down) do NOT block station updates
- Eventually consistent; 5-minute SLA for new/updated stations to appear
- Hard delete of `inventory.station` is not in MVP scope; GIS records soft-deleted only

---

## Schema Layout

```sql
-- INVENTORY SCHEMA (business data - source of truth)
SCHEMA inventory {
  TABLE station (
    id PK, partner_id FK, name, address, 
    latitude, longitude, osm_node_id,
    availability_status, capacity,
    created_at, updated_at, deleted_at
  );
  
  TABLE charger (
    id PK, station_id FK, connector_type,
    power_kw, status, created_at, updated_at, deleted_at
  );
  
  TABLE partner (
    id PK, name, email UNIQUE, phone,
    country, status, created_at, updated_at, deleted_at
  );
  
  TABLE station_outbox (
    id PK, station_id, event_type,
    payload JSONB, created_at, processed_at
  );
}

-- USERS SCHEMA (user-centric data)
SCHEMA users {
  TABLE user (
    id PK, keycloak_id UNIQUE, email,
    name, role, partner_id FK, created_at, updated_at, deleted_at
  );
  
  TABLE favorite (
    id PK, user_id FK, station_id FK,
    created_at, UNIQUE(user_id, station_id)
  );
  
  TABLE review (
    id PK, user_id FK, station_id FK,
    rating CHECK(1-5), comment, created_at, updated_at, deleted_at,
    UNIQUE(user_id, station_id)
  );
}

-- GIS SCHEMA (geospatial projections - derived, non-canonical)
SCHEMA gis {
  TABLE osm_ways (
    id PK, name, type, geom LINESTRING, tags JSONB
  );
  
  TABLE osm_nodes (
    id PK, name, amenity, geom POINT, tags JSONB
  );
  
  TABLE station_locations (
    id PK FK(inventory.station), name, partner_id, 
    geom POINT, synced_at, deleted_at
  );
}
```

---

## Validation & State Constraints

### Cross-Entity Constraints

1. **Station can only belong to active partners**:
   ```sql
   ALTER TABLE inventory.station
   ADD CONSTRAINT fk_station_active_partner
   FOREIGN KEY (partner_id) REFERENCES inventory.partner(id)
   WHERE partner.deleted_at IS NULL;
   ```

2. **User favorites only for active stations**:
   - Application layer enforces: on `POST /api/v1/favorites`, verify `station.deleted_at IS NULL`
   - On `GET /api/v1/favorites`, filter out soft-deleted stations

3. **Partner users must belong to a partner**:
   ```sql
   ALTER TABLE users.user
   ADD CONSTRAINT ck_partner_user_has_partner
   CHECK (
     (role != 'partner' AND partner_id IS NULL) OR
     (role = 'partner' AND partner_id IS NOT NULL)
   );
   ```

4. **Station coordinates must be valid**:
   - Validated in Rust domain layer before insert
   - Database constraint: `-90 <= latitude <= 90`, `-180 <= longitude <= 180`

---

## Summary

**Entities (9 total)**:
1. ✅ Station (inventory, with soft delete)
2. ✅ Charger (inventory, with soft delete)
3. ✅ Partner (inventory, with soft delete)
4. ✅ User (users, with soft delete)
5. ✅ Favorite (users, hard delete)
6. ✅ Review (users, soft delete for audit)
7. ✅ OSM Ways (gis, read-only)
8. ✅ OSM Nodes (gis, read-only)
9. ✅ Station Locations (gis, derived projection)

**Key Design Decisions**:
- ✅ Soft delete for business-critical entities (stations, users) except favorites (ephemeral)
- ✅ GIS is derived projection only; does NOT block station updates
- ✅ Partner scope enforced at API layer (JWT-driven)
- ✅ Favorite: hard delete (no audit trail needed)
- ✅ Outbox pattern for async GIS sync
- ✅ 16-char NanoID identifiers with entity-specific prefixes

**Ready for API contract definition and task breakdown.**
