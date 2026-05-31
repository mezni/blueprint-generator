# Feature Specification: Admin Dashboard

**Feature Branch**: `007-admin-dashboard`

**Created**: 2026-05-29

**Status**: Draft

**Input**: User description: "Add admin dashboard as the third frontend application alongside existing web-driver and mobile-driver, with cross-platform UI blueprint matrix defining layout behavior for all three apps. All backend integrations, telemetry processing, database connections, and message-broker syncs are explicitly out of scope — everything runs on static client-side mock data."

## User Scenarios & Testing

### User Story 1 - Admin Console Overview Dashboard (Priority: P1)

An admin logs into the BorneMap sandbox master console and sees a navigation sidebar with collapsible entity menus, a top status bar, and a main content area showing a vitals summary matrix with partner counts, station counts, and mock telemetry indicators.

**Why this priority**: The overview dashboard is the entry point for all admin users and establishes the core layout (sidebar + top bar + content canvas) that all other tabs depend on.

**Independent Test**: Can be fully tested by loading the admin dashboard URL and verifying that the overview tab renders the vitals summary matrix with three metric cards.

**Acceptance Scenarios**:

1. **Given** the admin dashboard is loaded, **When** the page renders, **Then** a left navigation sidebar with collapsible menu items is visible
2. **Given** the overview tab is active, **When** the page renders, **Then** three metric cards are displayed showing PARTNERS (148), STATIONS (1,240), and MOCK TELEMETRY HITS (Offline)
3. **Given** the ENTITIES menu is expanded by default, **When** the page loads, **Then** nested PARTNERS and STATIONS links are visible under ENTITIES

---

### User Story 2 - Partners and Stations Data Tables (Priority: P1)

An admin navigates to the PARTNERS or STATIONS tab and sees a high-density data table with sortable column headers, badge status chips, and mock registry data rendered from in-memory arrays.

**Why this priority**: Viewing and browsing entity data is the core admin task; data tables replace the map canvas used in the other two applications.

**Independent Test**: Can be fully tested by clicking the PARTNERS or STATIONS navigation items and verifying that tabular data renders with correct rows and status badges.

**Acceptance Scenarios**:

1. **Given** the admin clicks PARTNERS in the sidebar, **When** the partners view renders, **Then** a table displays with columns for ID, BRAND ENTITY NAME, HUBS, and STATUS
2. **Given** the admin clicks STATIONS in the sidebar, **When** the stations view renders, **Then** a table displays with columns for HUB ID, NAME DESIGNATION, ZONAL PLACEMENT, and STATUS
3. **Given** a partner row is rendered, **When** the status cell is Active, **Then** it displays a green indicator dot and "Active" text

---

### User Story 3 - Desktop Web Map Portal (Priority: P2)

A desktop user visits the web-driver application and sees a full-screen interactive map with station markers, a top navigation bar, a centered search overlay with filter pills, circular zoom buttons, and a bottom popover detail card on marker click.

**Why this priority**: The desktop web portal already exists conceptually but needs to be aligned with the new cross-platform UI blueprint layout behaviors.

**Independent Test**: Can be fully tested by opening the web-driver URL and verifying that the map, navbar, search overlay, zoom buttons, and detail card all render and interact.

**Acceptance Scenarios**:

1. **Given** the web-driver loads, **When** the page renders, **Then** a fixed top navigation bar with ABOUT, APP, MAP, CONTACT links and a REGISTER NOW button is visible
2. **Given** the map is rendered, **When** the user clicks a station marker, **Then** a bottom-center popover detail card appears with station name, status, provider, and charger list
3. **Given** the zoom column is displayed, **When** the user clicks the + or - button, **Then** the map zooms in or out accordingly

---

### User Story 4 - Mobile Native Map Screen (Priority: P2)

A mobile user opens the mobile-driver app and sees a full-screen native MapView with station markers, a floating header with brand logo and registration capsule, a search row, circular zoom buttons, and a draggable bottom sheet for station details.

**Why this priority**: Mobile parity with desktop web ensures a consistent cross-platform experience for end users.

**Independent Test**: Can be fully tested by launching the mobile app on a device or simulator and verifying that the map, header, zoom controls, and bottom sheet render.

**Acceptance Scenarios**:

1. **Given** the mobile app loads, **When** the screen renders, **Then** a full-screen MapView with OpenStreetMap tiles is displayed centered on Tunisia
2. **Given** a station marker is visible, **When** the user taps it, **Then** a bottom sheet slides up covering 35% of the screen showing station details
3. **Given** the zoom controls are displayed, **When** the user taps the + or - button, **Then** the map camera zooms in or out

---

### Edge Cases

- What happens when no stations match the current filter? The data table and map show zero results with an appropriate empty state.
- How does the system handle a missing navigation target? The detail card/sheet simply omits the Navigate button when no navigation URL is present.
- How does the admin sidebar behave when ENTITIES is collapsed? The nested PARTNERS and STATIONS links are hidden; clicking ENTITIES again re-expands them.

## Requirements

### Functional Requirements

- **FR-001**: Admin dashboard MUST render a left sidebar with collapsible ENTITIES menu containing PARTNERS and STATIONS nested links
- **FR-002**: Admin dashboard MUST render a top status bar showing the sandbox master console title and a MOCK ENGINE ACTIVE badge
- **FR-003**: Admin dashboard MUST render an overview tab with three metric cards: Partners, Stations, and Mock Telemetry Hits
- **FR-004**: Admin dashboard MUST render a partners data table with columns for ID, Brand Entity Name, Hubs, and Status, with inline text search that filters rows in real time against the mock array
- **FR-005**: Admin dashboard MUST render a stations data table with columns for Hub ID, Name Designation, Zonal Placement, and Status, with inline text search that filters rows in real time against the mock array
- **FR-006**: All three applications (admin-dashboard, web-driver, mobile-driver) MUST operate entirely on static client-side mock data with zero backend integration
- **FR-007**: Web-driver MUST render a full-screen interactive map with a top navigation bar, centered search overlay, filter pills, and circular zoom buttons
- **FR-008**: Web-driver MUST open a bottom-center popover detail card when a station marker is clicked, showing station name, status, provider, and charger list
- **FR-009**: Mobile-driver MUST render a full-screen native MapView with a floating header, search row, and circular zoom controls
- **FR-010**: Mobile-driver MUST open a bottom sheet covering 35% of the screen when a station marker is tapped
- **FR-011**: ALL tabs in the admin sidebar beyond overview/partners/stations MUST display a fallback box indicating the view is mock-rendered but backend pipelines are excluded

### Key Entities

- **Partner**: A brand entity (e.g., TotalEnergies Tunisia, Shell Tunisia) with an ID, name, hub count, and status. Linked to stations through the station's partner reference.
- **Station**: A charging hub location with an ID, name, latitude/longitude coordinates, a partner reference, and a list of chargers. Each charger has a plug type, power output, and status.
- **Charger**: An individual charging unit within a station, characterized by plug type (CCS2, Type2), power output (kW), and operational status (Available, Occupied).

## Success Criteria

### Measurable Outcomes

- **SC-001**: Admin dashboard loads and renders the full layout (sidebar + top bar + overview) in under 3 seconds on a standard desktop browser
- **SC-002**: Data tables for partners and stations render with correct mock data rows on the first attempt — no empty tables or loading spinners
- **SC-003**: Desktop web-driver map and mobile-driver map both render the same mock stations at the same coordinates, demonstrating cross-platform parity
- **SC-004**: All interactive elements (navigation links, buttons, map markers, zoom controls, detail card close) respond to user input with no console errors or uncaught exceptions
- **SC-005**: Switching between all admin tabs (overview, partners, stations, users, analytics, settings, logs) completes in under 100ms

## Assumptions

- All three applications operate in a sandbox/mock mode — no real backend, telemetry, or database connections are required
- Static mock data arrays are defined inline within each component and are sufficient for UI development and testing
- The admin dashboard is a standalone web application (apps/admin-dashboard) separate from web-driver and mobile-driver
- Maps use standard tile layers with OpenStreetMap imagery on both platforms
- The cross-platform blueprint matrix governs layout differences while maintaining consistent component behavior across all three applications
- Existing web-driver and mobile-driver components are rewritten or replaced to match the new blueprint specification

## Clarifications

### Session 2026-05-29

- Q: Terminology normalization — Station vs Hub vs Node → A: "Station" is canonical term. "Hub" and "Node" are permitted as UI synonyms in table headers and display labels only.
- Q: Admin data table loading behavior — should tables simulate a delay? → A: Render instantly from in-memory arrays with no artificial loading simulation.
- Q: Should admin data tables include inline text search/filter? → A: Yes — add a text input above each table that filters rows in real time from the mock array, matching the blueprint matrix spec.
- Q: Map tile failure handling during sandbox mode? → A: Out of scope for mock sandbox. Tile loading errors are not handled in this phase.
- Q: Quantify "completes instantly" in SC-005? → A: Target is < 100ms for tab switching.
