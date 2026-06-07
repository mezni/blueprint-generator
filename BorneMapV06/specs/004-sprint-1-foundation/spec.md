# Feature Specification: Sprint 1 — OSM Data & Station Discovery

**Feature Branch**: `004-sprint-1-foundation`

**Created**: 2026-06-05

**Status**: Draft

**Input**: From docs/10-delivery/mvp01 specify sprint 1

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Public Driver Discovers Nearby Charging Stations (Priority: P1)

A public driver opens the map and wants to find the nearest charging stations without creating an account. The system loads OpenStreetMap data, calculates proximity, and displays available stations sorted by distance.

**Why this priority**: Core MVP feature — enables users to find charging stations immediately, which is the primary value proposition. No authentication required means maximum accessibility.

**Independent Test**: Developer can run `GET /api/v1/stations/nearby?lat=36.8&lng=10.1&radius=5000` and receive a list of 10+ stations sorted by distance with names, addresses, and real-world OSM data populated in the database.

**Acceptance Scenarios**:

1. **Given** stations exist in the database with OSM coordinates, **When** user requests nearby stations within 5km radius, **Then** system returns only stations within that radius sorted by distance
2. **Given** no stations exist in the requested radius, **When** user requests nearby stations, **Then** system returns empty list (not error)
3. **Given** user request with invalid coordinates, **When** request is processed, **Then** system rejects with clear error message (e.g., "invalid latitude: 100")

---

### User Story 2 - OSM Data Imported & Station Locations Synced (Priority: P1)

System ingests OpenStreetMap data for Tunisia (roads, boundaries, amenity points) and synchronizes real-world charging station locations into the GIS layer. This enables map rendering and proximity calculations.

**Why this priority**: Blocking dependency for all discovery features. Without OSM data and station locations, no geographic queries work. Must complete before any public-facing features launch.

**Independent Test**: DBA can verify that `gis.osm_ways` contains 10,000+ road records, `gis.station_locations` contains 50+ station geometries, and a test proximity query returns correct results.

**Acceptance Scenarios**:

1. **Given** OSM data import script runs, **When** data is loaded into `gis` schema, **Then** all tables are populated with valid geom geometries and spatial indexes are created
2. **Given** inventory.station records exist, **When** GIS Sync Worker processes the outbox, **Then** corresponding entries appear in `gis.station_locations` within 5 minutes
3. **Given** a station location is updated, **When** the sync worker runs, **Then** the GIS layer reflects the change without blocking the station update

---

### User Story 3 - Registered Driver Creates Favorites & Views Reviews (Priority: P2)

A registered driver authenticates via Keycloak, browses stations, creates favorites, and views ratings and reviews from other users. Favorites are persisted to their profile.

**Why this priority**: High user engagement feature but depends on authentication. Deferred to P2 to unblock P1 public discovery first.

**Independent Test**: Developer can authenticate as a registered driver, call `POST /api/v1/favorites` with a station ID, then call `GET /api/v1/favorites` and see the saved station in the list.

**Acceptance Scenarios**:

1. **Given** user is authenticated and a station exists, **When** user creates a favorite, **Then** the station is added to user's favorites list
2. **Given** user has favorites, **When** user views their favorites page, **Then** all favorited stations display with current availability and reviews
3. **Given** user removes a favorite, **When** the removal completes, **Then** the station no longer appears in the favorites list

---

### User Story 4 - Partner Views Their Own Stations (Priority: P2)

A partner (business user) authenticates, accesses their dashboard, and sees a list of only their own stations with availability and occupancy metrics. Partners CANNOT see other partners' stations.

**Why this priority**: Partner onboarding feature. Enables partners to manage their infrastructure but depends on Keycloak role-based access. Deferred to P2.

**Independent Test**: Partner A logs in and calls `GET /api/v1/partner/stations`, receives only their own stations. Partner B logs in and cannot see Partner A's stations.

**Acceptance Scenarios**:

1. **Given** authenticated partner user, **When** user requests their stations, **Then** system returns ONLY stations they own, filtered at API layer
2. **Given** partner owns multiple stations, **When** user views dashboard, **Then** availability and occupancy are displayed for each station
3. **Given** partner attempts to view another partner's station, **When** request is processed, **Then** system rejects with 403 Forbidden

---

### Edge Cases

- What happens when OSM import fails partially (e.g., 80% complete)? → GIS layer degrades gracefully; public discovery still works with partial data. Non-critical errors logged, critical errors block until resolved.
- How does system handle a station with invalid coordinates in inventory? → Station validation rejects invalid coords at insert time. GIS sync skips records with NULL geometry.
- What if user searches outside Tunisia (or in area with no OSM data)? → Query returns empty list, not error. Empty state message guides user.
- How does system recover if GIS sync worker crashes? → Outbox persists failed changes; worker restarts and retries. GIS failures do NOT block station updates.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST import OpenStreetMap data (ways, nodes, relations) for Tunisia into `gis` schema
- **FR-002**: System MUST create spatial indexes (GIST) on all geometry columns for fast proximity queries
- **FR-003**: System MUST expose `/api/v1/stations/nearby?lat=...&lng=...&radius=...` endpoint returning stations sorted by distance
- **FR-004**: System MUST calculate great-circle distance using Haversine formula (embedded in ev-geo crate)
- **FR-005**: System MUST validate input coordinates (lat: -90 to 90, lng: -180 to 180) and reject invalid requests
- **FR-006**: System MUST populate `gis.station_locations` by syncing `inventory.station` records asynchronously
- **FR-007**: GIS Sync Worker MUST read from an outbox table and process station location changes without blocking station updates
- **FR-008**: System MUST return empty list (not error) when no stations found in radius
- **FR-009**: System MUST enforce partner scope at API layer — partners see ONLY their own stations in all endpoints
- **FR-010**: System MUST authenticate requests to protected endpoints via Keycloak JWT (except public GET /api/v1/stations/nearby)
- **FR-011**: System MUST persist user favorites to `users.favorite` table with foreign key to `inventory.station`
- **FR-012**: System MUST return 403 Forbidden if user attempts to access another user's favorites or data
- **FR-013**: System MUST expose partner dashboard at `/api/v1/partner/stations` returning only partner's stations with availability metrics
- **FR-014**: System MUST validate all API input (coordinates, IDs, user scope) and return clear error messages

### Key Entities

- **Station**: Charging station location; identified by `STN-*` prefix ID; has name, address, coordinates, soft-delete flag, partner owner
- **Charger**: Individual charging port at a station; identified by `CHG-*` prefix ID; has connector type, power rating, status (available/in_use/maintenance/offline)
- **Partner**: Business entity that owns stations; identified by `PRT-*` prefix ID; has name, contact info
- **User**: Driver or partner user; identified by `USR-*` prefix ID; has Keycloak external ID, role (registered_driver/partner/admin), profile data
- **Favorite**: User's saved station; links `User` → `Station` with timestamp
- **Review**: User's rating and comment on a station; identified by `REV-*` prefix ID; has rating (1-5), comment text, timestamp
- **OSM Way**: Road or boundary from OpenStreetMap; has geometry (LineString), tags (name, type)
- **OSM Node**: Point of interest from OpenStreetMap; has geometry (Point), tags (amenity, name)
- **Station Location** (GIS): Derived spatial record linking `inventory.station` to `gis` geometry; updated asynchronously; NOT source of truth

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Public drivers can discover stations within 5km in under 500ms; system supports 100 concurrent nearby searches
- **SC-002**: 95% of nearby station queries return results within 1 second; median response time under 300ms
- **SC-003**: OSM data import completes for Tunisia in under 10 minutes; all 10,000+ ways and 50,000+ nodes indexed
- **SC-004**: GIS sync worker processes station location changes within 5 minutes of station creation/update
- **SC-005**: System handles 1000 concurrent authenticated users without performance degradation
- **SC-006**: Partner isolation enforced: partner A cannot see partner B's stations in any API response
- **SC-007**: Favorites functionality has zero data loss; all user-saved stations persist and display correctly
- **SC-008**: System rejects 100% of invalid coordinate inputs with clear error messages
- **SC-009**: OSM data fully covers all major roads and administrative boundaries in Tunisia (visual inspection confirms)

## Assumptions

- **Data availability**: OpenStreetMap data for Tunisia is current and accessible via osm2pgsql or similar ingestion tools
- **GIS schema exists**: `gis` schema is already created with all required tables (osm_ways, osm_nodes, roads, boundaries, station_locations) per Sprint 0 migrations
- **Keycloak integration exists**: Keycloak service is deployed and reachable; JWT validation is implemented in driver-service middleware
- **Spatial indexing**: PostGIS extensions (postgis, postgis_topology) are already enabled on platform_db per Sprint 0
- **Outbox pattern**: An outbox table (`inventory.station_outbox` or similar) exists to track changes for async sync; implementation deferred to implementation planning
- **Distance calculation**: Haversine function from `ev-geo` crate is used; no external geo library required
- **Public access allowed**: Nearby stations endpoint `/api/v1/stations/nearby` is publicly accessible without Keycloak auth; all other endpoints require JWT
- **Soft deletes only**: Inactive/deleted stations are marked with `deleted_at` timestamp, never hard-deleted; discovery queries filter by `deleted_at IS NULL`
- **Partner scope mandatory**: Every partner-related query is scoped to authenticated user's partner organization; enforced at infrastructure layer via JWT claims
- **GIS async failures non-blocking**: If GIS sync fails, station updates proceed normally; GIS layer is eventually consistent, not critical path
