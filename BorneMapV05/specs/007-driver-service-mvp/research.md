# Research: Driver Service MVP

**Date**: 2026-06-02

## Overview

No open technical questions required research — all decisions follow established patterns from the existing monorepo (admin-service pattern, common crates, database schema). This document records the key design decisions for reference.

## Architecture Decisions

### Decision: Axum with State-based dependency injection

**Decision**: Use `axum::Router` with `State<PgPool>` for DB access, `Extension<CurrentUser>` for auth context. Follow the exact pattern from admin-service.

**Rationale**: Consistent with existing service pattern. Axum's type-safe extractors and layered middleware compose well for the public + authenticated route split.

**Alternatives considered**: None — project convention.

---

### Decision: Optional auth middleware for public discovery endpoints

**Decision**: Apply `optional_auth_middleware` to station discovery routes so authenticated users get distance-from-location via their current position (passed as query params), while anonymous users still get full results.

**Rationale**: The spec requires station detail to include `distance_km` when lat/lng is provided. This is driven by query params, not auth state. Optional auth allows future enhancement (e.g., personalizing distance from user's saved home location) without breaking public access.

**Alternatives considered**: Separate public vs authenticated route trees for discovery — overengineered for MVP.

---

### Decision: PostGIS ST_DWithin for spatial filtering

**Decision**: Use `ST_DWithin(geom, ST_SetSRID(ST_MakePoint($lng, $lat), 4326)::geography, $radius_meters)` for radius queries with GIST index scan.

**Rationale**: PostGIS ST_DWithin with geography cast handles the Haversine distance calculation correctly, uses the existing GIST index on `inventory.station.geom`, and matches the pattern established in sprint 5/6.

**Alternatives considered**: Manual Haversine formula — slower, no index usage. Bounding box approximation — less accurate at high latitudes.

---

### Decision: Visibility filter at query level, not application level

**Decision**: Enforce `is_live=true AND deleted_at IS NULL AND status='active' AND is_public=true` directly in SQL WHERE clauses.

**Rationale**: Database-level filtering prevents accidentally leaking non-visible stations due to application bugs. The existing `inventory.visible_stations` view codifies this rule, but direct WHERE clauses are used for visibility and testability.

**Alternatives considered**: Application-level filter after fetch — less secure. RLS (Row-Level Security) — overengineered for visibility-only filtering.

---

### Decision: One review per user per station via DB UNIQUE constraint

**Decision**: Enforce the single-review constraint at the database level (`CONSTRAINT uq_station_review_user_station UNIQUE (user_id, station_id)`) and catch the constraint violation in the application layer.

**Rationale**: DB-level enforcement prevents race conditions from concurrent requests. Application-level validation alone could allow duplicates under concurrent load.

**Alternatives considered**: Application-level check-then-insert — race condition window. ON CONFLICT DO NOTHING — silent failure, no error feedback to user.

---

### Decision: Profile upsert pattern

**Decision**: Use `SELECT` then `INSERT` or `UPDATE` for user profile operations (no native UPSERT since `users.user_profile` has no unique constraint beyond the PK).

**Rationale**: The profile table is optional (row may not exist on first PATCH). Two-query approach is clean and readable for MVP.

**Alternatives considered**: `INSERT ... ON CONFLICT DO UPDATE` — would require adding a unique constraint. Single `INSERT` with error handling — less clean.
