# Data Model: Core Database Schema

**Feature**: Sprint 4 — Core Database Schema  
**Date**: 2026-06-02

## Database Topology

```
┌─────────────────────────────────────────────────────────────┐
│                      PostgreSQL Instance                      │
│                                                               │
│  ┌──────────────┐  ┌──────────────────┐  ┌────────────────┐ │
│  │ keycloak_db  │  │   platform_db    │  │ analytics_db   │ │
│  │ (Keycloak    │  │   (PostGIS on)   │  │                │ │
│  │  owned)      │  │                  │  │  analytics     │ │
│  │              │  │  inventory       │  │   raw_event    │ │
│  │              │  │  users           │  │   event_dead_  │ │
│  │              │  │  gis             │  │   letter       │ │
│  └──────────────┘  └──────────────────┘  └────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Entity Relationship Diagram

```
inventory.partner ─────────┐
    │                      │
    │ 1:N                  │
    ▼                      │
inventory.station ─────────┤
    │                      │
    │ 1:N                  │ owner via station
    ▼                      │
inventory.charger          │
                           │
inventory.station_         │
  availability             │
    │                      │
    └──────────────────────┘

users.user_account ────────┐
    │                      │
    ├── 1:1 ── users.user_profile
    │                      │
    ├── 1:1 ── users.partner_membership ──► inventory.partner
    │                      │
    ├── M:N ── users.favorite_station ────► inventory.station
    │                      │
    └── 1:N ── users.station_review ─────► inventory.station

gis.sync_queue (outbox, independent)

analytics.raw_event (partitioned, independent)
analytics.event_dead_letter (independent)
```

## Schema: inventory

### inventory.partner

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| id | TEXT | NO | | PK, pattern `PRT-<ULID>` |
| name | TEXT | NO | | |
| type | TEXT | NO | | CHECK IN ('business', 'private') |
| status | TEXT | NO | 'active' | CHECK IN ('active', 'suspended') |
| created_at | TIMESTAMPTZ | NO | NOW() | |
| updated_at | TIMESTAMPTZ | NO | NOW() | |
| created_by | TEXT | NO | | |
| updated_by | TEXT | NO | | |
| deleted_at | TIMESTAMPTZ | YES | NULL | Soft delete marker |

**Indexes**: BTREE(id), BTREE(status)

**Triggers**: `trg_partner_delete_guard` — BEFORE UPDATE, blocks `deleted_at` if active stations exist

### inventory.station

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| id | TEXT | NO | | PK, pattern `STN-<ULID>` |
| partner_id | TEXT | NO | | FK → inventory.partner(id) |
| name | TEXT | NO | | |
| description | TEXT | YES | NULL | |
| latitude | DOUBLE PRECISION | NO | | CHECK (-90, 90) |
| longitude | DOUBLE PRECISION | NO | | CHECK (-180, 180) |
| geom | GEOGRAPHY(Point, 4326) | YES | NULL | Auto-populated by trigger |
| status | TEXT | NO | 'draft' | CHECK IN ('draft', 'active', 'inactive', 'maintenance') |
| is_live | BOOLEAN | NO | FALSE | |
| is_public | BOOLEAN | NO | FALSE | |
| city | TEXT | YES | NULL | |
| country | TEXT | YES | NULL | |
| created_at | TIMESTAMPTZ | NO | NOW() | |
| updated_at | TIMESTAMPTZ | NO | NOW() | |
| created_by | TEXT | NO | | |
| updated_by | TEXT | NO | | |
| deleted_at | TIMESTAMPTZ | YES | NULL | Soft delete marker |

**Indexes**: GIST(geom), BTREE(partner_id), BTREE(status), BTREE(is_live, is_public), BTREE(city)

**Triggers**: `trg_station_geom` — BEFORE INSERT OR UPDATE, sets `geom = ST_SetSRID(ST_MakePoint(lng, lat), 4326)`

**Visibility rule**: `is_live = true AND deleted_at IS NULL AND status = 'active' AND is_public = true`

### inventory.charger

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| id | TEXT | NO | | PK, pattern `CHG-<ULID>` |
| station_id | TEXT | NO | | FK → inventory.station(id) |
| type | TEXT | NO | | CHECK IN ('CCS', 'Type2', 'CHAdeMO') |
| power_kw | NUMERIC | YES | NULL | |
| status | TEXT | NO | 'available' | CHECK IN ('available', 'offline', 'fault') |
| created_at | TIMESTAMPTZ | NO | NOW() | |
| updated_at | TIMESTAMPTZ | NO | NOW() | |
| created_by | TEXT | NO | | |
| updated_by | TEXT | NO | | |
| deleted_at | TIMESTAMPTZ | YES | NULL | Soft delete marker |

**Indexes**: BTREE(station_id), BTREE(status)

### inventory.station_availability

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| id | TEXT | NO | | PK |
| station_id | TEXT | NO | | FK → inventory.station(id) |
| status | TEXT | NO | | CHECK IN ('available', 'limited', 'unavailable') |
| source | TEXT | NO | | CHECK IN ('manual_partner', 'system_sync', 'admin') |
| updated_at | TIMESTAMPTZ | NO | NOW() | |

**Indexes**: BTREE(station_id)

**Note**: Operational projection only — not authoritative. Full audit fields (`created_by`, `updated_by`, `deleted_at`) not required per data model spec; this table is replaced/overwritten rather than soft-deleted.

### inventory.visible_stations (VIEW)

```sql
CREATE VIEW inventory.visible_stations AS
SELECT *
FROM inventory.station
WHERE is_live = true
  AND deleted_at IS NULL
  AND status = 'active'
  AND is_public = true;
```

## Schema: users

### users.user_account

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| id | TEXT | NO | | PK, pattern `USR-<ULID>` |
| keycloak_user_id | TEXT | NO | | UNIQUE |
| email | TEXT | YES | NULL | |
| status | TEXT | NO | 'active' | CHECK IN ('active', 'disabled') |
| created_at | TIMESTAMPTZ | NO | NOW() | |
| last_login_at | TIMESTAMPTZ | YES | NULL | |

**Indexes**: UNIQUE(keycloak_user_id)

**Note**: Not a mutable business entity (created by auth flow, not user action). Only `created_at` and `last_login_at` — no `updated_at`/`created_by`/`updated_by`/`deleted_at` per canonical data model.

### users.user_profile

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| user_id | TEXT | NO | | PK, FK → users.user_account(id) |
| display_name | TEXT | YES | NULL | |
| avatar_url | TEXT | YES | NULL | |
| preferred_language | TEXT | YES | NULL | |
| preferences | JSONB | YES | NULL | |

**Note**: Optional, safe to delete. No audit fields per canonical data model.

### users.partner_membership

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| user_id | TEXT | NO | | PK, FK → users.user_account(id), UNIQUE |
| partner_id | TEXT | NO | | FK → inventory.partner(id) |
| role | TEXT | NO | | CHECK IN ('owner', 'manager', 'operator', 'viewer') |

**Constraints**: UNIQUE(user_id) — strict 1:1. A user can belong to at most one partner.

### users.favorite_station

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| user_id | TEXT | NO | | PK component, FK → users.user_account(id) |
| station_id | TEXT | NO | | PK component, FK → inventory.station(id) |
| created_at | TIMESTAMPTZ | NO | NOW() | |

**Primary Key**: (user_id, station_id) — composite

### users.station_review

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| id | TEXT | NO | | PK, pattern `REV-<ULID>` |
| user_id | TEXT | NO | | FK → users.user_account(id) |
| station_id | TEXT | NO | | FK → inventory.station(id) |
| rating | INTEGER | NO | | CHECK (1–5) |
| comment | TEXT | YES | NULL | |
| status | TEXT | NO | 'published' | CHECK IN ('published', 'hidden', 'flagged', 'deleted') |
| created_at | TIMESTAMPTZ | NO | NOW() | |
| updated_at | TIMESTAMPTZ | NO | NOW() | |

**Constraints**: UNIQUE(user_id, station_id) — one review per user per station

**Indexes**: BTREE(station_id), BTREE(user_id)

## Schema: gis

### gis.sync_queue

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| id | TEXT | NO | | PK |
| entity_type | TEXT | NO | | CHECK IN ('station', 'charger') |
| entity_id | TEXT | NO | | |
| operation | TEXT | NO | | CHECK IN ('insert', 'update', 'delete') |
| payload | JSONB | YES | NULL | |
| status | TEXT | NO | 'pending' | CHECK IN ('pending', 'processing', 'done', 'failed', 'dead_letter') |
| created_at | TIMESTAMPTZ | NO | NOW() | |
| processed_at | TIMESTAMPTZ | YES | NULL | |

**Indexes**: BTREE(status), BTREE(entity_type, entity_id)

**Note**: Outbox table. Queue-specific fields (created_at, processed_at) rather than full audit set.

## Schema: analytics

### analytics.raw_event (Partitioned)

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| event_id | TEXT | NO | | Dedup key |
| event_name | TEXT | NO | | |
| session_id | TEXT | NO | | |
| user_id | TEXT | YES | NULL | |
| anonymous_id | TEXT | YES | NULL | |
| actor_role | TEXT | YES | NULL | |
| occurred_at | TIMESTAMPTZ | NO | | Partition key |
| ingested_at | TIMESTAMPTZ | NO | NOW() | |
| path | TEXT | YES | NULL | |
| payload | JSONB | YES | NULL | |
| metadata | JSONB | YES | NULL | |

**Partitioning**: RANGE by `occurred_at`, monthly partitions: `raw_event_2026_01` through `raw_event_2026_12`, plus `raw_event_default`

**Indexes** (per partition): BTREE(event_name, occurred_at), BTREE(user_id), BTREE(session_id)

### analytics.event_dead_letter

| Column | Type | Nullable | Default | Constraints |
|--------|------|----------|---------|-------------|
| id | TEXT | NO | | PK |
| event_id | TEXT | NO | | |
| event_name | TEXT | YES | NULL | |
| error_code | TEXT | YES | NULL | |
| error_message | TEXT | YES | NULL | |
| raw_payload | JSONB | YES | NULL | |
| created_at | TIMESTAMPTZ | NO | NOW() | |

## State Transitions

### Station Lifecycle
```
draft → active → inactive → (soft-deleted via deleted_at)
                 → maintenance → active
```
Enforced at application level (Sprint 5). DB only constrains valid status values via CHECK.

### Review Moderation Lifecycle
```
submitted → published → flagged → hidden → (soft-deleted via status='deleted')
                                → published (unflag)
```
Enforced at application level (Sprint 7). DB only constrains valid status values via CHECK.

### GIS Sync Queue State Machine
```
pending → processing → done
                    → failed → dead_letter
```
Managed by gis-worker (Sprint 6). DB constrains valid states via CHECK.

## Audit Fields Summary

| Table | created_at | updated_at | created_by | updated_by | deleted_at |
|-------|-----------|-----------|-----------|-----------|-----------|
| inventory.partner | YES | YES | YES | YES | YES |
| inventory.station | YES | YES | YES | YES | YES |
| inventory.charger | YES | YES | YES | YES | YES |
| inventory.station_availability | NO | YES | NO | NO | NO |
| users.user_account | YES | NO | NO | NO | NO |
| users.user_profile | NO | NO | NO | NO | NO |
| users.partner_membership | NO | NO | NO | NO | NO |
| users.favorite_station | YES | NO | NO | NO | NO |
| users.station_review | YES | YES | NO | NO | NO |
| gis.sync_queue | YES | NO | NO | NO | NO |
| analytics.raw_event | NO | NO | NO | NO | NO |
| analytics.event_dead_letter | YES | NO | NO | NO | NO |

**Rationale**: Audit fields (created_by, updated_by, deleted_at) are applied to **mutable business entities** that are subject to soft-delete and operational modification. Operational/queue tables and lightweight join tables use minimal timestamps.
