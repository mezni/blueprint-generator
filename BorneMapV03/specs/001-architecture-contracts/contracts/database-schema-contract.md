# Database Schema Contract

## Purpose

Define the four PostgreSQL schemas, their table structures, constraints, and
ownership rules. The database is a single PostgreSQL 16+ instance with PostGIS.

## Version

1.0.0

## Schema Overview

| Schema | Owner | Purpose | Tables |
|--------|-------|---------|--------|
| `inventory` | Admin Service | Transactional business data | `partner`, `station`, `charger`, `station_availability` |
| `users` | Driver + Admin | Identity-linked user data | `user_account`, `user_profile`, `partner_membership`, `favorite_station`, `station_review` |
| `gis` | GIS Worker | Geospatial enrichment | `roads`, `boundaries`, `station_geospatial_view` |
| `analytics` | Clickstream Service | Events + aggregations | `raw_event`, `daily_event_count`, `station_daily_metric`, `search_daily_metric` |

## `inventory` Schema

| Table | Key Columns | Constraints |
|-------|-------------|-------------|
| `partner` | `id` (PRT-), `name`, `type` (business/private) | PK on `id` |
| `station` | `id` (STN-), `partner_id`, `geom` (POINT), `name`, `status` (active/inactive) | PK on `id`, FK → `partner(id)`, GIST index on `geom` |
| `charger` | `id` (CHG-), `station_id`, `power_kw`, `connector_type` | PK on `id`, FK → `station(id)` |
| `station_availability` | `station_id`, `available` (boolean), `updated_at` | FK → `station(id)`, updated by partner |

## `users` Schema

| Table | Key Columns | Constraints |
|-------|-------------|-------------|
| `user_account` | `id` (USR-), `keycloak_user_id`, `email`, `role` | PK on `id`, UNIQUE on `keycloak_user_id`, UNIQUE on `email` |
| `user_profile` | `user_id`, `display_name`, `phone`, `avatar_url` | PK on `user_id`, FK → `user_account(id)` |
| `partner_membership` | `user_id`, `partner_id` | PK composite (`user_id`, `partner_id`), FK → both tables, UNIQUE per user |
| `favorite_station` | `user_id`, `station_id`, `created_at` | PK composite (`user_id`, `station_id`) |
| `station_review` | `id` (REV-), `user_id`, `station_id`, `rating`, `body`, `soft_deleted` | PK on `id`, UNIQUE (`user_id`, `station_id`), FK → both |

## `gis` Schema

| Table | Description | Source |
|-------|-------------|--------|
| `roads` | OSM road geometry | OSM Tunisia batch import |
| `boundaries` | Administrative boundaries | OSM Tunisia batch import |
| `station_geospatial_view` | Materialized view of station locations with enriched spatial context | Derived from `inventory.station` + spatial joins |

## `analytics` Schema

| Table | Description | Partitioning |
|-------|-------------|--------------|
| `raw_event` | Immutable event log | BY RANGE (`timestamp`), monthly partitions |
| `daily_event_count` | Aggregated event counts per type per day | BY RANGE (`date`) |
| `station_daily_metric` | Per-station daily metrics | BY RANGE (`date`) |
| `search_daily_metric` | Search query aggregations | BY RANGE (`date`) |

## General Rules

- All table PKs use prefixed NanoID except composite PKs
- Soft delete for: `station`, `user_account`, `station_review`
- Migrations: only Admin Service writes `inventory` migrations; all migrations
  versioned and backward-compatible
- Partitioning: `raw_event` MUST be time-partitioned; other tables MAY be
  partitioned when volume warrants
