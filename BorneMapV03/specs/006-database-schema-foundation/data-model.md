# Data Model: Database & Schema Foundation

## Schema Overview

```
borne_map (database)
├── inventory           # Admin Service (sole writer)
│   ├── partner         # Charging network partner
│   ├── station         # Physical charging station (authoritative geometry)
│   ├── charger         # Individual charging point at a station
│   └── review          # Driver reviews of stations
├── users               # Driver Service + Admin Service (writers)
│   ├── user_account    # Application user (1:1 with Keycloak)
│   └── favorite        # Driver's saved stations
├── gis                 # GIS Sync Worker (sole writer)
│   ├── station_enrichment  # GIS-computed enrichment data
│   └── geo_boundary     # Geographic boundary data
└── analytics           # Clickstream Service + workers (sole writers)
    └── raw_event       # Append-only time-partitioned events
```

---

## Schema: `inventory`

Owned by: `admin_service` role
Writable by: Admin Service
Readable by: Admin Service, GIS Sync Worker (read-only)

### partner

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | VARCHAR(26) | PK | `PRT-` + nanoid(21) |
| name | VARCHAR(255) | NOT NULL | Legal business name |
| email | VARCHAR(255) | NOT NULL, UNIQUE | Contact email |
| phone | VARCHAR(50) | | Contact phone |
| website | VARCHAR(500) | | Business website |
| status | VARCHAR(32) | NOT NULL, DEFAULT 'active' | active, suspended, inactive |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| deleted_at | TIMESTAMPTZ | | Soft delete |

**Constraints**:
- `status IN ('active', 'suspended', 'inactive')`
- `deleted_at IS NULL OR deleted_at > created_at`

**Indexes**:
- `idx_partner_status ON inventory.partner (status)`

---

### station

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | VARCHAR(26) | PK | `STN-` + nanoid(21) |
| partner_id | VARCHAR(26) | NOT NULL, FK → inventory.partner.id | Owning partner |
| name | VARCHAR(255) | | Display name |
| address | TEXT | | Street address |
| location | GEOGRAPHY(Point, 4326) | NOT NULL | Authoritative geometry |
| status | VARCHAR(32) | NOT NULL, DEFAULT 'active' | active, maintenance, offline, removed |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| deleted_at | TIMESTAMPTZ | | Soft delete |

**Constraints**:
- `status IN ('active', 'maintenance', 'offline', 'removed')`
- `deleted_at IS NULL OR deleted_at > created_at`

**Indexes**:
- `idx_station_location ON inventory.station USING GIST (location)`
- `idx_station_partner ON inventory.station (partner_id)`
- `idx_station_status ON inventory.station (status)`

---

### charger

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | VARCHAR(26) | PK | `CHG-` + nanoid(21) |
| station_id | VARCHAR(26) | NOT NULL, FK → inventory.station.id | Parent station |
| connector_type | VARCHAR(50) | NOT NULL | CCS, CHAdeMO, Type2, etc. |
| power_kw | DECIMAL(6,1) | NOT NULL, CHECK (power_kw > 0) | Max power output |
| status | VARCHAR(32) | NOT NULL, DEFAULT 'available' | available, occupied, fault, offline |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| deleted_at | TIMESTAMPTZ | | Soft delete |

**Constraints**:
- `connector_type IN ('CCS', 'CHAdeMO', 'Type2', 'GB/T', 'Tesla')`
- `status IN ('available', 'occupied', 'fault', 'offline')`

**Indexes**:
- `idx_charger_station ON inventory.charger (station_id)`
- `idx_charger_status ON inventory.charger (status)`

---

### review

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | VARCHAR(26) | PK | `REV-` + nanoid(21) |
| station_id | VARCHAR(26) | NOT NULL, FK → inventory.station.id | Reviewed station |
| user_id | VARCHAR(26) | NOT NULL, FK → users.user_account.id | Review author |
| rating | SMALLINT | NOT NULL, CHECK (rating >= 1 AND rating <= 5) | 1-5 stars |
| comment | TEXT | | Free-text review |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| deleted_at | TIMESTAMPTZ | | Soft delete |

**Constraints**:
- UNIQUE(station_id, user_id) — one review per user per station

**Indexes**:
- `idx_review_station ON inventory.review (station_id)`
- `idx_review_user ON inventory.review (user_id)`

---

## Schema: `users`

Owned by: `driver_service` role
Writable by: Driver Service, Admin Service
Readable by: all authenticated services

### user_account

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | VARCHAR(26) | PK | `USR-` + nanoid(21) |
| keycloak_id | VARCHAR(255) | NOT NULL, UNIQUE | Keycloak `sub` claim (1:1 mapping) |
| email | VARCHAR(255) | | Cached from Keycloak |
| display_name | VARCHAR(255) | | User-facing name |
| status | VARCHAR(32) | NOT NULL, DEFAULT 'active' | active, suspended, orphaned |
| last_login_at | TIMESTAMPTZ | | Last authentication timestamp |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| deleted_at | TIMESTAMPTZ | | Soft delete |

**Constraints**:
- `status IN ('active', 'suspended', 'orphaned')`

**Indexes**:
- `idx_user_keycloak ON users.user_account (keycloak_id)`
- `idx_user_status ON users.user_account (status)`

**Note**: Roles are NOT stored in the DB — extracted from Keycloak JWT on each request.

---

### favorite

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PK, DEFAULT gen_random_uuid() | Internal ID |
| user_id | VARCHAR(26) | NOT NULL, FK → users.user_account.id | User who favorited |
| station_id | VARCHAR(26) | NOT NULL, FK → inventory.station.id | Favorited station |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |

**Constraints**:
- UNIQUE(user_id, station_id)

**Indexes**:
- `idx_favorite_user ON users.favorite (user_id)`
- `idx_favorite_station ON users.favorite (station_id)`

---

## Schema: `gis`

Owned by: `gis_worker` role
Writable by: GIS Sync Worker
Readable by: Admin Service, Driver Service (for enriched queries)

### station_enrichment

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| station_id | VARCHAR(26) | PK, FK → inventory.station.id | References authoritative station |
| address_components | JSONB | | Parsed address (street, city, postal_code, country) |
| coordinates_4326 | GEOGRAPHY(Point, 4326) | | Copy of station.location for query isolation |
| elevation_m | DECIMAL(8,2) | | Elevation from DEM |
| timezone | VARCHAR(64) | | IANA timezone |
| nearby_pois | JSONB | | Nearby points of interest (cafes, shops) |
| computed_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Last enrichment timestamp |

**Indexes**:
- `idx_enrichment_coords ON gis.station_enrichment USING GIST (coordinates_4326)`

---

### geo_boundary

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PK, DEFAULT gen_random_uuid() | Internal ID |
| name | VARCHAR(255) | NOT NULL | Boundary name (e.g., "Paris", "Île-de-France") |
| boundary | GEOGRAPHY(Polygon, 4326) | NOT NULL | Polygon boundary |
| boundary_type | VARCHAR(32) | NOT NULL | city, region, country, service_area |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |

**Indexes**:
- `idx_boundary_geom ON gis.geo_boundary USING GIST (boundary)`

---

## Schema: `analytics`

Owned by: `clickstream_service` + `analytics_worker` roles
Writable by: Clickstream Service, analytics workers
Readable by: Admin Service (for reporting)

### raw_event

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PK, DEFAULT gen_random_uuid() | Event ID |
| event_type | VARCHAR(64) | NOT NULL | e.g., station_search, charger_selected, auth_attempt |
| payload | JSONB | NOT NULL | Event-specific data |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Partition key |

**Partitioning**: Daily range partitioning on `created_at`.
**Constraints**: Append-only — no UPDATE or DELETE (except partition management).

**Indexes** (per partition):
- `idx_raw_event_type ON analytics.raw_event (event_type)`
- `idx_raw_event_created ON analytics.raw_event (created_at)`

---

## Entity Relationship Diagram

```
inventory.partner ──1:N── inventory.station ──1:N── inventory.charger
                              │
                              │ 1:N
                              │
                         inventory.review ──N:1── users.user_account
                              │
                              │ 1:1 (GIS derived)
                              │
                        gis.station_enrichment

users.user_account ──1:N── users.favorite ──N:1── inventory.station

analytics.raw_event (independent, references no other tables)
```

## Cross-Schema Data Flow (Read-Only)

| Reader Schema | Can Read From |
|--------------|---------------|
| inventory | itself |
| users | itself, inventory.station (favorites) |
| gis | inventory.station (for enrichment) |
| analytics | itself |

## ID Prefix Convention

All application entities use prefix-format IDs:

| Prefix | Table(s) |
|--------|----------|
| `USR-` | user_account |
| `PRT-` | partner |
| `STN-` | station |
| `CHG-` | charger |
| `REV-` | review |
