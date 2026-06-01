## Epic Key

DB-EPIC-03

## Priority

Critical (System Backbone)

## Depends on
EPIC-01 — Platform Foundation
EPIC-02 — Identity & Access (Keycloak Realm Contract)
## Blocks

ALL backend services (Admin, Driver, GIS Worker, Clickstream, Reporting)

## 1. Purpose

This epic defines the complete persistence and identity integration model of the platform.

It establishes:

PostgreSQL + PostGIS as the single system of record
strict schema ownership boundaries
identity integration with Keycloak (external system)
event storage model for analytics
deterministic ID strategy
migration system governance

This is a hard architectural contract — not an implementation suggestion.

## 2. Core Architecture Decision
### System of Record Rule

PostgreSQL is the only system of record for business and analytical data.

Keycloak is:

- identity provider only
- NOT a system of record for business data

## 3. Schema Model (FINAL)

PostgreSQL is divided into 4 isolated domains + identity linkage contract:

### 3.1 inventory (BUSINESS TRUTH)
**Ownership**: Admin Service (ONLY writer)

**Purpose**: Canonical EV infrastructure model

**Tables**: `partner`, `station`, `charger`, `station_availability`

**Critical rules**:
- `station.location` is authoritative geometry
- GIS never writes here
- partner owns stations (1 → N)

### 3.2 users (APPLICATION IDENTITY + INTERACTIONS)
**Ownership**: Driver Service + Admin Service

**Purpose**: Application-level user model linked to Keycloak

**Tables (UPDATED with Keycloak integration)**:
```sql
user_account (
    user_id TEXT PRIMARY KEY,          -- USR_xxx internal ID
    keycloak_id TEXT UNIQUE NOT NULL,  -- Keycloak sub
    email TEXT UNIQUE NOT NULL,
    role TEXT NOT NULL,
    created_at TIMESTAMP
)
```
- `user_profile`
- `partner_membership`
- `favorite_station`
- `station_review`

**Critical rules**:
- Keycloak = authentication identity
- `user_account` = application identity
- mapping is 1:1 via `keycloak_id`

### 3.3 gis (DERIVED SPATIAL LAYER)
**Ownership**: GIS Sync Worker ONLY

**Purpose**: Spatial enrichment and OSM data

**Tables**: `roads`, `boundaries`, `station_geospatial_cache`, derived spatial indexes

**Critical rules**:
- NEVER source of truth for stations
- ALWAYS derived from `inventory.station`
- async sync only

### 3.4 analytics (EVENT + METRICS SYSTEM)
**Ownership**: Clickstream Service + Analytics workers

**Purpose**: Event ingestion + aggregation

**Tables**:
```sql
raw_event (
    event_id TEXT PRIMARY KEY,
    event_type TEXT,
    timestamp TIMESTAMP,
    session_id TEXT,
    user_id TEXT NULL,
    platform TEXT,
    payload JSONB
)
```
- `station_daily_metric`
- `search_daily_metric`
- `daily_event_count`

**Critical rules**:
- append-only `raw_event`
- partitioned by time
- no business writes allowed here

## 4. Identity Integration Layer (NEW — CRITICAL ADDITION)

This is the missing contract layer that connects Keycloak → PostgreSQL

### 4.1 Identity Mapping Contract
**Rule**: Every Keycloak user MUST map to exactly one `users.user_account`.

### 4.2 Mapping strategy
| System | Field |
|--------|-------|
| Keycloak | `sub` |
| `users.user_account` | `keycloak_id` |

### 4.3 First login provisioning

On first authentication:

1. Extract JWT `sub`
2. Search `user_account.keycloak_id`
3. If not found:
   - create `user_account`
   - assign role from JWT
4. If exists:
   - continue request

### 4.4 Role synchronization rule

Roles are sourced from Keycloak ONLY:
- `registered_driver`
- `partner`
- `admin`

Backend MUST NOT override roles independently.

## 5. ID STRATEGY (FINALIZED)

### Format
| Entity | Format |
|--------|--------|
| User | `USR_xxx` |
| Partner | `PRT_xxx` |
| Station | `STN_xxx` |
| Charger | `CHG_xxx` |
| Review | `REV_xxx` |

**Rules**:
- generated in application layer
- immutable
- globally unique
- never reused

## 6. MIGRATION SYSTEM (ENFORCED ARCHITECTURE)

**Source of truth**: `/services/admin-service/migrations/`

**Rules**:
- all schema changes go through migrations only
- no ad-hoc SQL in services
- migrations are versioned sequentially
- rollback scripts optional but recommended

**Ownership rules**:
| Schema | Migration Owner |
|--------|----------------|
| inventory | Admin Service |
| users | Admin + Driver (restricted) |
| gis | GIS Worker (controlled) |
| analytics | Clickstream pipeline |

## 7. POSTGIS STANDARDIZATION

**Global rules**:
- SRID = 4326
- geometry type = GEOGRAPHY(Point)
- all distance queries use PostGIS functions

**Required indexes**:
- `inventory.station`: GIST(location), index(partner_id)
- `gis`: GIST(all geometry columns)
- `analytics`: (event_type, timestamp), partition key index

## 8. PERFORMANCE CONTRACT

**Targets**:
- station search < 200ms
- nearby queries < 300ms
- analytics ingestion non-blocking

**Design constraints**:
- no cross-schema joins in write path
- GIS queries must use spatial indexes
- analytics must be append-only

## 9. SECURITY + INTEGRITY RULES

**Hard constraints**:
- Keycloak is ONLY identity system
- no password storage in PostgreSQL
- JWT must be validated in all services
- role enforcement is mandatory at API layer

**Data integrity rules**:
- inventory is system of truth
- users is interaction layer
- gis is derived layer
- analytics is append-only event layer

## 10. ACCEPTANCE CRITERIA

EPIC-03 is complete when:

- **Database structure**: 4 schemas created (inventory, users, gis, analytics), PostGIS enabled, indexing strategy implemented
- **Identity integration**: Keycloak mapping contract defined, user_account includes keycloak_id, first login provisioning defined
- **Migration system**: single migration source established, schema ownership enforced
- **Rules enforcement**: no cross-schema writes allowed, GIS is derived only, analytics is append-only

## 11. OUTPUT ARTIFACTS

```
/db/
  migrations/
    inventory/
    users/
    gis/
    analytics/
  schema/
    inventory.sql
    users.sql
    gis.sql
    analytics.sql
  identity/
    keycloak-realm-contract.md
    jwt-mapping-spec.md
  id-strategy.md
  migration-policy.md
  postgis-standards.md
```

## 12. FINAL ONE-SENTENCE SUMMARY

EPIC-03 defines the complete PostgreSQL + PostGIS system backbone with strict schema ownership, deterministic ID strategy, controlled migration system, and a formal Keycloak-to-application identity mapping contract that guarantees consistent authentication and data integrity across all services.
