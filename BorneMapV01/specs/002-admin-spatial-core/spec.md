# Feature Specification: Admin Spatial Core Validation

**Feature Branch**: `004-admin-spatial-core`

**Created**: 2026-05-25

**Status**: Draft

**Input**: User description: "phase 1 from docs/plan.md"

## Clarifications

### Session 2026-05-25

- Q: Are editor and viewer roles in scope for Phase 1, or just admin? → A: Admin only. Role-based access control for editor/viewer is deferred to a later phase.
- Q: How many stations and chargers are expected in this phase? → A: Under 500 stations. No pagination required; simple query patterns suffice.
- Q: What distance defines "nearby" for spatial search? → A: 20 km default search radius.
- Q: What prevents duplicate stations? → A: Station name unique per operator. Two stations cannot share the same name if they belong to the same operator.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Station Management on Map (Priority: P1)

As an admin, I want to add, edit, and remove EV charging stations directly on the interactive map so that the station database stays accurate and up to date.

**Why this priority**: Station data is the core product asset. Without the ability to manage stations on the map, the platform has no value for drivers.

**Independent Test**: An admin can log into the portal, see the full-screen map with existing stations shown as markers, click an empty area to add a new station with coordinates, fill in the details, and see the marker appear immediately.

**Acceptance Scenarios**:

1. **Given** the admin portal is loaded, **When** the admin views the map, **Then** all active stations are shown as custom green markers at their precise locations.
2. **Given** the map is displayed, **When** the admin clicks a location on the map, **Then** a form opens allowing entry of station name, operator, address, plug types, and speed.
3. **Given** the admin submits a new station form, **When** the data is saved, **Then** a new green marker appears at the selected location and the station appears in the station list.
4. **Given** a station marker exists on the map, **When** the admin clicks it, **Then** a panel displays station details with edit and delete options.
5. **Given** the admin edits a station, **When** changes are saved, **Then** the marker and list entry update immediately without page reload.
6. **Given** the admin deletes a station, **When** confirmed, **Then** the marker is removed from the map and the station is removed from the list.

---

### User Story 2 — Spatial Search & Discovery (Priority: P1)

As an admin, I want to search for stations near a specific location so that I can quickly inspect stations in a given area.

**Why this priority**: Spatial search is the primary discovery mechanism for the platform and must work correctly from day one.

**Independent Test**: An admin drags the map to a new area, and the station list updates to show only stations within the visible map area.

**Acceptance Scenarios**:

1. **Given** the map is displayed at a certain zoom level, **When** the admin pans or zooms, **Then** the station list filteres to show only stations within the visible bounding box.
2. **Given** the admin types an address or coordinates in the search field, **When** they submit the search, **Then** the map centers on that location and shows nearby stations.
3. **Given** a search returns multiple nearby stations, **When** the results are displayed, **Then** they are sorted by distance from the search center.

---

### User Story 3 — Admin Authentication (Priority: P1)

As an admin, I want to be authenticated when performing administrative actions so that only authorized personnel can modify station data.

**Why this priority**: Data integrity requires that only verified admins can create, edit, or delete stations.

**Independent Test**: An unauthenticated visitor can see the map but cannot add, edit, or delete stations; clicking any admin action prompts for authentication.

**Acceptance Scenarios**:

1. **Given** an unauthenticated visitor views the portal, **When** they try to add or edit a station, **Then** they are prompted to authenticate.
2. **Given** an authenticated admin performs an action, **When** the request is sent, **Then** it includes valid authentication credentials.
3. **Given** an admin is authenticated, **When** they perform CRUD operations, **Then** the system verifies their identity and authorizes the action.
4. **Given** authentication credentials are invalid or expired, **When** an admin action is attempted, **Then** the system returns an authorization error.

---

### User Story 4 — Stations Data Table (Priority: P2)

As an admin, I want to see all stations in a sortable, filterable table so that I can quickly browse and manage station records without navigating the map.

**Why this priority**: A data table provides a complementary view to the map for batch operations and data review.

**Independent Test**: An admin clicks a toggle to switch from map view to table view, sees all stations listed with key fields, and can search/filter by name or status.

**Acceptance Scenarios**:

1. **Given** the admin is on the stations page, **When** they switch to table view, **Then** all stations are displayed in a table with columns for name, operator, address, plug types, speed, and status.
2. **Given** the table is displayed, **When** the admin types in a search box, **Then** the table filters to show only matching stations in real time.
3. **Given** the table is displayed, **When** the admin clicks a column header, **Then** the table sorts by that column (ascending/descending toggle).

---

### Edge Cases

- What happens when the tile CDN is unreachable? → Map shows a warning banner and falls back to OSM tiles.
- What happens when a station has no coordinates? → Station appears in the data table but not on the map, with a visual indicator.
- What happens when two admins edit the same station simultaneously? → Last save wins; the earlier admin's changes appear stale after a page refresh.
- What happens when a station marker is placed at sea/inaccessible area? → The system accepts the coordinates but the marker shows; validation is administrative review.
- What happens when a search returns zero results? → The map displays a "No stations found in this area" message and the station list is empty.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Admins MUST be able to view a full-screen interactive map with all active stations displayed as custom markers at their geographic coordinates.
- **FR-002**: Admins MUST be able to add a new station by clicking a location on the map and filling in a form with name, operator, address, plug types, and speed.
- **FR-003**: Admins MUST be able to click an existing marker to view station details and access edit/delete actions.
- **FR-004**: Adding, editing, or deleting a station MUST update both the map markers and the station list immediately without requiring a page reload.
- **FR-005**: Deleting a station MUST require explicit confirmation before proceeding.
- **FR-006**: The station list MUST automatically filter to show only stations within the current map viewport as the admin pans and zooms.
- **FR-007**: Admins MUST be able to search by address or coordinates to center the map on a specific location and show nearby stations.
- **FR-008**: Nearby search results MUST be sorted by distance from the search center or map center.
- **FR-009**: All administrative CRUD operations MUST require valid authentication.
- **FR-010**: Unauthenticated visitors MUST be able to view the map and stations but MUST NOT be able to add, edit, or delete.
- **FR-011**: Authentication credentials MUST be validated on every administrative request.
- **FR-012**: Admins MUST be able to switch between map view and an alternative table view of all stations.
- **FR-013**: The table view MUST support real-time search filtering by station name.
- **FR-014**: The table view MUST support sorting by any visible column.
- **FR-015**: Coordinates saved through the admin portal MUST appear on the map within the localized bounding box from which they were entered.

### Key Entities *(include if feature involves data)*

- **Station**: A physical EV charging location with name, operator, address, geographic coordinates, plug types, maximum speed, and active/inactive status. Each station belongs to an optional partner organization.
- **Charger**: An individual charging connector at a station, identified by connector type (CCS, Type2, CHAdeMO, GBT), power rating in kW, and operational status.
- **Partner**: An organization that owns or operates stations, with contact details.
- **Admin User**: An authorized person with credentials who can manage stations, identified as admin role with active status.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: An admin can add a station on the map and see it appear as a marker within 3 seconds of saving.
- **SC-002**: An admin can complete a full station add workflow (click map → fill form → save) in under 60 seconds.
- **SC-003**: Nearby station search returns results within 2 seconds for any viewport within Tunisia.
- **SC-004**: The map renders at 60 frames per second during pan and zoom on target devices.
- **SC-005**: At least 2 real admins can successfully complete the station management workflow in a validation session without assistance.

## Assumptions

- Admins have modern browsers with JavaScript enabled and stable internet connectivity.
- The base map tile provider (CartoDB Positron) is reachable with a fallback to OSM.
- Geographic coordinates are entered in standard decimal degrees format.
- Stations within Tunisia are the initial focus; coordinate bounds validation covers typical Tunisian geography.
- The authentication mechanism is already established (from Phase 0 foundation work).
- Station and charger data models from Phase 0 are already in place and will be reused.
- Mobile driver app features are explicitly out of scope for this phase.
