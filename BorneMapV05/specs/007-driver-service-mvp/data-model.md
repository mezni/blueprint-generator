# Data Model: Driver Service MVP

**Date**: 2026-06-02

## Overview

The driver service reads from and writes to `platform_db` tables defined in Sprints 4 and 5. This document describes the entities from the driver-service perspective — which fields are exposed to driver users and which internal fields are filtered.

---

## Entity: Station (read-only for drivers)

**Source**: `inventory.station`

| Field | Type | Exposed? | Notes |
|-------|------|----------|-------|
| id | TEXT PK (STN-ULID) | ✅ | Primary identifier |
| name | TEXT | ✅ | Station display name |
| description | TEXT? | ✅ | Optional description |
| latitude | DOUBLE PRECISION | ✅ | Decimal degrees |
| longitude | DOUBLE PRECISION | ✅ | Decimal degrees |
| city | TEXT? | ✅ | City name |
| country | TEXT? | ✅ | Country name |
| status | TEXT | ❌ | Internal (always 'active' due to visibility filter) |
| is_live | BOOLEAN | ❌ | Internal (always true due to visibility filter) |
| is_public | BOOLEAN | ❌ | Internal (always true due to visibility filter) |
| geom | GEOGRAPHY(Point,4326) | ✅ (as GeoPoint) | Derived as `{"lat": X, "lng": Y}` |
| partner_id | TEXT FK | ❌ | Never exposed to driver users |
| created_at | TIMESTAMPTZ | ❌ | Internal audit |
| updated_at | TIMESTAMPTZ | ❌ | Internal audit |
| deleted_at | TIMESTAMPTZ? | ❌ | Internal (filtered WHERE NULL) |

**Visibility rule** (all must be true):
- `is_live = true`
- `deleted_at IS NULL`
- `status = 'active'`
- `is_public = true`

**Computed fields** (added in API response):
- `distance_km: f64?` — Great-circle distance from query point (PostGIS ST_Distance)
- `geom: GeoPoint { lat, lng }` — Always returned for map rendering

---

## Entity: Charger (read-only for drivers)

**Source**: `inventory.charger`

| Field | Type | Exposed? | Notes |
|-------|------|----------|-------|
| id | TEXT PK (CHG-ULID) | ✅ | |
| station_id | TEXT FK | ✅ | Links to parent station |
| type | TEXT | ✅ | CCS, Type2, or CHAdeMO |
| power_kw | NUMERIC? | ✅ | Rated power in kW |
| status | TEXT | ✅ | available, offline, or fault |
| created_at | TIMESTAMPTZ | ❌ | Internal audit |
| updated_at | TIMESTAMPTZ | ❌ | Internal audit |
| deleted_at | TIMESTAMPTZ? | ❌ | Filtered WHERE NULL |

**Filter**: `deleted_at IS NULL`

---

## Entity: Station Availability (read-only for drivers)

**Source**: `inventory.station_availability`

| Field | Type | Exposed? | Notes |
|-------|------|----------|-------|
| id | TEXT PK | ❌ | Internal |
| station_id | TEXT FK | ❌ | Used as join key |
| status | TEXT | ✅ | available, limited, or unavailable |
| source | TEXT | ❌ | Internal (manual_partner, system_sync, admin) |
| updated_at | TIMESTAMPTZ | ❌ | Internal |

**Note**: Returns `null` if no availability record exists for the station. Only the most recent record per station is used (`ORDER BY updated_at DESC LIMIT 1`).

---

## Entity: Favorite Station (driver-owned)

**Source**: `users.favorite_station`

| Field | Type | Notes |
|-------|------|-------|
| user_id | TEXT FK → `users.user_account.id` | Composite PK |
| station_id | TEXT FK → `inventory.station.id` | Composite PK |
| created_at | TIMESTAMPTZ | Auto-set on insert |

**Constraints**:
- `PRIMARY KEY (user_id, station_id)` — one favorite per user per station
- `ON CONFLICT DO NOTHING` — idempotent add

**Operations**: Add, Remove (DELETE), List by user_id

---

## Entity: Station Review (driver-owned)

**Source**: `users.station_review`

| Field | Type | Notes |
|-------|------|-------|
| id | TEXT PK (REV-ULID) | Auto-generated |
| user_id | TEXT FK | Owner (set from auth context) |
| station_id | TEXT FK | Target station |
| rating | INTEGER | 1–5, validated at application layer |
| comment | TEXT? | Optional free text |
| status | TEXT | published (default) or deleted |
| created_at | TIMESTAMPTZ | Auto-set |
| updated_at | TIMESTAMPTZ | Updated on PATCH |

**Constraints**:
- `UNIQUE (user_id, station_id)` — one review per user per station
- `CHECK (rating >= 1 AND rating <= 5)`
- Owner-only modify: application enforces `user_id` match
- Soft delete: status transitions to `deleted`, row not removed
- Deleted reviews are excluded from review summary aggregations (`WHERE status = 'published'`)

**Lifecycle**:
```
submitted → published → deleted (by owner)
```

---

## Entity: User Profile (driver-owned)

**Source**: `users.user_account` + `users.user_profile`

| Field | Table | Type | Notes |
|-------|-------|------|-------|
| user_id | user_account | TEXT PK (USR-ULID) | From auth provisioning |
| email | user_account | TEXT? | From Keycloak |
| display_name | user_profile | TEXT? | Optional, settable by driver |
| avatar_url | user_profile | TEXT? | Optional |
| preferred_language | user_profile | TEXT? | e.g., "fr", "ar" |
| preferences | user_profile | JSONB? | Arbitrary driver preferences |
| created_at | user_account | TIMESTAMPTZ | Account creation |
| last_login_at | user_account | TIMESTAMPTZ? | Last login timestamp |

**Note**: `users.user_profile` row may not exist for a user until they first PATCH their profile. The GET endpoint returns `null` for profile fields if no profile row exists.
