# Contracts: Core Database Schema

**Feature**: Sprint 4 — Core Database Schema  
**Date**: 2026-06-02

## Overview

This sprint produces database-level contracts only (no API endpoints). The contracts define:

1. **Schema boundaries** — which service owns which schema
2. **Cross-schema references** — FK relationships that cross schema boundaries
3. **Trigger contracts** — database triggers that enforce business rules
4. **View contracts** — the `visible_stations` view as a shared query interface

## Schema Ownership Contract

| Schema | Database | Owner Service | Other Services (Read) |
|--------|----------|---------------|----------------------|
| inventory | platform_db | admin-service | driver-service (read), gis-worker (outbox read) |
| users | platform_db | admin-service | driver-service (read) |
| gis | platform_db | gis-worker | admin-service (write outbox rows) |
| analytics | analytics_db | analytics-writer | clickstream-service (write via writer) |

**Rules**:
- No cross-schema writes without crossing a service boundary
- `admin-service` writes to `inventory.*` and `users.*`
- `admin-service` inserts into `gis.sync_queue` (outbox pattern) on station mutation
- `gis-worker` reads from `gis.sync_queue` and updates GIS state
- `driver-service` reads from `inventory.*` and `users.*` (never writes)

## Cross-Schema Reference Contract

| From | To | Type | Rule |
|------|----|------|------|
| inventory.station.partner_id | inventory.partner.id | FK | Station must reference existing partner |
| inventory.charger.station_id | inventory.station.id | FK | Charger must reference existing station |
| inventory.station_availability.station_id | inventory.station.id | FK | Availability must reference existing station |
| users.partner_membership.partner_id | inventory.partner.id | FK (cross-schema) | Membership must reference existing partner |
| users.partner_membership.user_id | users.user_account.id | FK | Membership must reference existing user |
| users.favorite_station.user_id | users.user_account.id | FK | Favorite must reference existing user |
| users.favorite_station.station_id | inventory.station.id | FK (cross-schema) | Favorite must reference existing station |
| users.station_review.user_id | users.user_account.id | FK | Review must reference existing user |
| users.station_review.station_id | inventory.station.id | FK (cross-schema) | Review must reference existing station |

**Cross-schema FK rule**: FKs from `users` to `inventory` are permitted because both schemas are in the same `platform_db`. No cross-database FKs.

## Trigger Contracts

### trg_station_geom (inventory.station)

| Property | Value |
|----------|-------|
| Event | BEFORE INSERT OR UPDATE on inventory.station |
| Condition | latitude OR longitude changed (or new row) |
| Action | Set `geom = ST_SetSRID(ST_MakePoint(longitude, latitude), 4326)`; NULL if lat/lng NULL |
| Guarantees | geom always consistent with lat/lng |
| Failure mode | None (deterministic computation) |

### trg_partner_delete_guard (inventory.partner)

| Property | Value |
|----------|-------|
| Event | BEFORE UPDATE on inventory.partner |
| Condition | `deleted_at` transitioning from NULL to non-NULL |
| Action | Check for active stations (is_live=true, deleted_at IS NULL) under this partner; if found, RAISE EXCEPTION 'ACTIVE_STATIONS_EXIST' |
| Guarantees | Partner with active stations cannot be soft-deleted |
| Failure mode | Exception with code ACTIVE_STATIONS_EXIST and hint message |

## View Contracts

### inventory.visible_stations

| Property | Value |
|----------|-------|
| Type | SQL VIEW (not materialized) |
| Source | inventory.station |
| Filter | is_live = true AND deleted_at IS NULL AND status = 'active' AND is_public = true |
| Columns | All station columns (SELECT *) |
| Index usage | Queries on this view use underlying station indexes (GIST, BTREE) |
| Consumers | driver-service, admin-service read queries |

## Migration Ordering Contract

Migrations MUST be applied in sequential numeric order. No parallel or out-of-order execution.

| Range | Domain | Dependencies |
|-------|--------|-------------|
| 0001-0006 | inventory schema (partner → station → charger → availability → view) | PostGIS enabled |
| 0007-0012 | users schema (user_account → profile → membership → favorites → reviews) | inventory schema |
| 0013-0014 | gis schema (sync_queue) | None |
| 0015 | Triggers (geom, partner delete guard) | inventory schema complete |
| 0016 | Seed data | All schemas complete |
| 0017 | Smoke test (verification) | All schemas + seed data |

Analytics migrations run independently against `analytics_db`:

| Range | Domain |
|-------|--------|
| 0001 | analytics schema |
| 0002 | raw_event (partitioned parent) |
| 0003 | raw_event partitions (2026-01 through 2026-12 + default) |
| 0004 | event_dead_letter |
