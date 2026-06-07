# Feature Specification: Driver Service MVP

**Feature Branch**: `007-driver-service-mvp`

**Created**: 2026-06-02

**Status**: Draft

**Input**: Sprint 7 — Driver Service MVP: implement all `/api/v1/driver/*` endpoints — station discovery (bbox/radius/detail/search), favorites, reviews (one per user/station), profile. Enforce station visibility rule; exclude soft-deleted; Tunisia fallback center; include `distance_km` + `geom`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Public Station Discovery via Map (Priority: P1)

A driver opens the map and sees charging stations near their current location, filtered by visibility rules (active, live, public, not deleted).

**Why this priority**: The map view is the primary entry point for all driver users. Without spatial discovery, the app provides no value.

**Independent Test**: Query `/api/v1/driver/stations?lat=36.8&lng=10.18&radius_km=10` and verify only visible stations are returned with `distance_km` and `geom` fields.

**Acceptance Scenarios**:

1. **Given** a driver is on the map view, **When** they pan/zoom, **Then** a bbox query returns only stations where `is_live = true`, `deleted_at IS NULL`, `status = 'active'`, `is_public = true`
2. **Given** a station that is not live, **When** a bbox query covers its coordinates, **Then** it is excluded from results
3. **Given** a driver taps a station marker, **When** the detail endpoint is called, **Then** chargers, availability, and review summary are returned
4. **Given** a driver searches for "Tunis", **When** the search endpoint returns results, **Then** matching visible stations are returned ordered by name

---

### User Story 2 - Registered Driver Favorites (Priority: P1)

A logged-in driver can favorite charging stations for quick access later.

**Why this priority**: Favorites are a core retention feature that encourages return visits.

**Independent Test**: Authenticate as `registered_driver`, POST `/api/v1/driver/favorites/STN-123`, then GET `/api/v1/driver/favorites` returns `["STN-123"]`.

**Acceptance Scenarios**:

1. **Given** an authenticated driver, **When** they POST a favorite on a station, **Then** the station is added to their favorites list
2. **Given** a favorited station, **When** they DELETE the favorite, **Then** the station is removed from their favorites
3. **Given** an unauthenticated request, **When** they POST a favorite, **Then** the request is rejected with `UNAUTHENTICATED`

---

### User Story 3 - Driver Reviews (Priority: P2)

A logged-in driver can submit a rating and comment for a station they visited.

**Why this priority**: Reviews build community trust and help other drivers choose stations.

**Independent Test**: Authenticate as `registered_driver`, POST `/api/v1/driver/reviews` with `{station_id, rating: 4, comment: "good"}`, then verify the review exists. A second POST with the same station_id is rejected.

**Acceptance Scenarios**:

1. **Given** an authenticated driver, **When** they submit a review with rating 1-5, **Then** the review is created with status `published`
2. **Given** a driver submits a second review for the same station, **Then** the request is rejected with `ALREADY_EXISTS`
3. **Given** a review the driver owns, **When** they PATCH it, **Then** the rating/comment is updated
4. **Given** a review the driver owns, **When** they DELETE it, **Then** the review status transitions to `deleted`
5. **Given** a review owned by another driver, **When** the first driver tries to modify it, **Then** the request is rejected with `FORBIDDEN`

---

### User Story 4 - Driver Profile (Priority: P2)

A logged-in driver can view and update their profile.

**Why this priority**: Profile management is a standard user expectation.

**Independent Test**: Authenticate as `registered_driver`, GET `/api/v1/driver/me` returns driver profile. PATCH `/api/v1/driver/me` with `{display_name: "John"}` updates the profile.

**Acceptance Scenarios**:

1. **Given** an authenticated driver, **When** they GET `/api/v1/driver/me`, **Then** their user_id, email, display_name, avatar_url, preferences are returned
2. **Given** a driver updates their display_name, **When** they GET their profile, **Then** the new display_name is reflected
3. **Given** an unauthenticated request, **When** they GET `/api/v1/driver/me`, **Then** the request is rejected with `UNAUTHENTICATED`

### Edge Cases

- What happens when no stations match the query? Return an empty list with pagination meta (`total: 0`)
- What happens when `lat`/`lng` are missing in a radius query? Fall back to Tunisia default center (36.8065, 10.1815) with 10km radius
- What happens when `radius_km > MAP_MAX_RADIUS_KM`? Clamp to 50km
- What happens when a station is deleted while a user has it favorited? The favorite record remains (referential integrity); the station won't appear in discovery
- What happens when a review is updated to `deleted`? The review is soft-deleted; it no longer appears in user listings but the constraint remains

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide `GET /api/v1/driver/stations` that accepts optional `lat`, `lng`, `radius_km`, `page`, `size` query params
- **FR-002**: Station list endpoints MUST filter by visibility: `is_live = true`, `deleted_at IS NULL`, `status = 'active'`, `is_public = true`
- **FR-003**: Radius-based queries MUST use PostGIS `ST_DWithin` with GIST index for spatial filtering; default center (36.8065, 10.1815) with 10km radius when lat/lng omitted
- **FR-004**: Radius MUST be clamped to `MAP_MAX_RADIUS_KM` (50km)
- **FR-005**: System MUST provide `GET /api/v1/driver/stations/{station_id}` that returns station detail including chargers, availability status, and review summary (average rating + count)
- **FR-006**: System MUST provide `GET /api/v1/driver/stations/search` that accepts `q` (text search on name/city/description) with ILIKE matching
- **FR-007**: All list endpoints MUST paginate with `page` and `size` params (default 20, max 100)
- **FR-008**: Station responses MUST include `distance_km` (when lat/lng provided) and `geom` as `{"lat": X, "lng": Y}`
- **FR-009**: System MUST provide `POST /api/v1/driver/favorites/{station_id}` to add a station to the authenticated user's favorites
- **FR-010**: System MUST provide `DELETE /api/v1/driver/favorites/{station_id}` to remove a station from favorites
- **FR-011**: System MUST provide `GET /api/v1/driver/favorites` to list all favorited station IDs for the authenticated user
- **FR-012**: System MUST provide `POST /api/v1/driver/reviews` with body `{station_id, rating (1-5), comment (optional)}`; rating MUST be validated as 1-5
- **FR-013**: System MUST enforce one review per user per station (`UNIQUE(user_id, station_id)` constraint)
- **FR-014**: System MUST provide `PATCH /api/v1/driver/reviews/{id}` and `DELETE /api/v1/driver/reviews/{id}` — ONLY the review owner can modify/delete
- **FR-015**: Review DELETE MUST be a soft delete (set status to `deleted`)
- **FR-016**: System MUST provide `GET /api/v1/driver/me` returning the authenticated driver's profile (user_id, email, display_name, avatar_url, preferred_language, preferences, created_at, last_login_at)
- **FR-017**: System MUST provide `PATCH /api/v1/driver/me` to update profile fields (display_name, avatar_url, preferred_language, preferences)
- **FR-018**: Review endpoints (favorites, reviews, profile) MUST require `registered_driver` role; unauthenticated requests receive `UNAUTHENTICATED`
- **FR-019**: All responses MUST use the standard envelope format — list endpoints use `{success, data, meta}` with pagination, single-item endpoints use `{success, data, meta: {}}`

### Key Entities

- **Station**: Charging station entity from `inventory.station` with PostGIS geometry, visibility flags, and spatial index
- **Charger**: Individual charging unit tied to a station, with type (CCS/Type2/CHAdeMO), power, and status
- **Favorite Station**: A bookmark linking a user to a station (composite PK); stored in `users.favorite_station`
- **Station Review**: User-submitted rating (1-5) and optional comment; exactly one per user per station; lifecycle: published → (deleted by owner) or moderated by admin
- **User Profile**: Optional extended profile data for drivers (display_name, avatar, language, preferences)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Bbox/radius queries return only visible stations and use GIST index scan (verified via `EXPLAIN ANALYZE`)
- **SC-002**: A duplicate review for the same station by the same user is rejected with `ALREADY_EXISTS` (409)
- **SC-003**: A non-owner attempting to modify a review is rejected with `FORBIDDEN` (403)
- **SC-004**: Station search returns results in under 200ms p95 on a seeded dataset of 10,000 stations across Tunisia
- **SC-005**: Favorites CRUD works correctly — add, list, remove, and verify persistence across requests
- **SC-006**: A user can complete the full driver journey: discover stations → view detail → favorite → review

## Assumptions

- Sprint 4 (Core DB Schema) migrations are complete — all tables and indexes exist
- Sprint 5 (Admin Service MVP) has seeded station and charger data in the database
- Sprint 6 (GIS Sync v1) has populated `inventory.station.geom` with valid PostGIS geometries
- The `users.favorite_station` and `users.station_review` tables exist from Sprint 4 migrations
- The `users.user_profile` table exists and supports optional profile fields
- The `common-auth` crate provides `optional_auth_middleware` for endpoints that benefit from optional user context (e.g., distance from user location)
- Station availability data may be sparse; the availability field returns `null` if no availability record exists
- PostGIS extension is enabled in `platform_db`
