# Feature Specification: Driver Web App

**Feature Branch**: `009-driver-web-app`

**Created**: 2026-06-03

**Status**: Draft

**Input**: User description: "Implement the full driver web application with map-first discovery, station detail, search/filter, favorites, reviews, progressive auth, and clickstream events"

## User Scenarios & Testing

### User Story 1 — Map-first Station Discovery (Priority: P1)

A driver opens the web app and sees a full-screen interactive map with station markers clustered by viewport. The map loads the driver's default region (Tunisia) and displays visible stations within the current viewport. As the driver pans and zooms, the map refreshes station markers via spatial queries.

**Why this priority**: The map is the primary interface — without it, no discovery is possible. Every other feature depends on the user first finding stations.

**Independent Test**: A page that renders an interactive map with clustered station markers that update when the viewport changes. Can be tested by zooming/pamming and verifying new markers appear.

**Acceptance Scenarios**:

1. **Given** the map has loaded, **When** the user pans or zooms, **Then** station markers refetch based on the new viewport within 500ms debounce
2. **Given** visible stations in the viewport, **When** zoomed out, **Then** markers are clustered to avoid visual clutter
3. **Given** no stations in the current viewport, **When** the map loads, **Then** an empty-state message displays
4. **Given** the driver is unauthenticated, **When** the map loads, **Then** public stations are displayed without requiring login
5. **Given** a populated viewport, **When** station markers render, **Then** each marker shows station name and availability indicator on hover

---

### User Story 1b — Map Interaction States (Priority: P1)

The map transitions through three states: Idle (initial load), Viewport Active (bbox query + clustering active as user pans/zooms), and Station Selected (detail panel visible). Each state has a distinct loading treatment.

**Why this priority**: Clear state management prevents confusing UX like stale markers or multiple simultaneous loading indicators.

**Independent Test**: An observer can identify which state the map is in by checking visible UI elements (loading skeletons, detail panel, marker presence).

**Acceptance Scenarios**:

1. **Given** the app loads, **When** in Idle state, **Then** a full-screen skeleton is shown while the initial map tiles and station data load
2. **Given** the map is interactive, **When** the user pans/zooms, **Then** viewport transitions to Active state with spinner only on the data layer (map tiles remain visible)
3. **Given** a station is selected, **When** in Station Selected state, **Then** the detail panel slides in and map centers on the station
4. **Given** the detail panel is open, **When** the user closes it, **Then** the map returns to Viewport Active state

---

### User Story 2 — Station Details & Search (Priority: P1)

A driver selects a station marker and sees a detail panel with station information: name, description, address, charger types (CCS/Type2/CHAdeMO) with power ratings, real-time availability status, and distance from current location. The driver can also search stations by name, city, or connector type.

**Why this priority**: Drivers need to evaluate stations before navigating to them. Search provides an alternative discovery path.

**Independent Test**: A detail panel opens when clicking a marker. Search returns filtered results. Can be validated by comparing displayed data against known station values.

**Acceptance Scenarios**:

1. **Given** a station marker is selected, **When** the detail panel opens, **Then** it displays name, description, charger types with power, availability, and distance
2. **Given** the search input, **When** the user types a query, **Then** results appear after a 300ms debounce
3. **Given** search filters (connector type, availability), **When** applied, **Then** results are filtered accordingly
4. **Given** the detail panel is open, **When** the user clicks outside, **Then** the panel closes
5. **Given** a station with no chargers, **When** the detail panel renders, **Then** it shows a "No chargers available" message

### User Story 3 — Progressive Authentication (Priority: P2)

A driver browses the map, views station details, and uses search without any authentication. Only when the driver attempts to favorite a station or submit a review is a login modal presented. After successful login, the gated action completes seamlessly.

**Why this priority**: The platform must be immediately useful to unauthenticated visitors. Authentication is a friction point that should only appear when necessary.

**Independent Test**: An anonymous user can complete the full discovery flow (map → select → details) without seeing a login prompt. Attempting to favorite triggers a login modal, and after login the favorite is created.

**Acceptance Scenarios**:

1. **Given** an anonymous driver, **When** browsing the map, **Then** no authentication prompt appears
2. **Given** an anonymous driver, **When** tapping the favorite button, **Then** a login modal appears
3. **Given** a successfully logged-in user, **When** the login modal closes, **Then** the gated action (favorite/review) completes without requiring the user to repeat the action
4. **Given** a logged-in driver, **When** viewing the map, **Then** already-favorited stations show a filled heart icon
5. **Given** login fails, **When** the modal displays an error, **Then** the user can retry or dismiss and continue browsing

---

### User Story 4 — Favorites & Reviews (Priority: P2)

A registered driver can favorite stations for quick access and submit reviews (rating + comment) for stations they have visited. Reviews are editable by the owner and support moderation states. Favorites are toggled with a single tap.

**Why this priority**: Favorites and reviews are the primary engagement features for returning drivers. They are gated behind authentication per US3.

**Independent Test**: A driver can add/remove a favorite and submit/edit a review. The review appears on the station detail. These are independent of map state.

**Acceptance Scenarios**:

1. **Given** a registered driver, **When** favoriting a station, **Then** the station is marked as favorited and can be filtered via the inline favorites toggle
2. **Given** a favorited station, **When** unfavoriting, **Then** the station is removed from favorites
3. **Given** a registered driver, **When** submitting a review with rating 1-5 and comment, **Then** the review appears on the station's detail panel
4. **Given** an existing review, **When** the owner edits it, **Then** the updated content replaces the previous
5. **Given** an existing review, **When** the owner deletes it, **Then** the review is removed from the station
6. **Given** a station with no reviews, **When** viewing details, **Then** a "No reviews yet" message is shown
7. **Given** a driver who already reviewed a station, **When** attempting to review again, **Then** the system shows an error that only one review per station is allowed

---

### User Story 5 — Clickstream Events (Priority: P3)

Every meaningful user interaction emits a clickstream event to the analytics pipeline via the api-client: page views, map loads, viewport changes, marker clicks, station opens, searches, filters, favorites, reviews, and auth events.

**Why this priority**: Analytics data is essential for understanding usage patterns and improving the product, but the app functions without it.

**Independent Test**: A test observer can verify that each listed interaction produces the expected event name and payload without blocking the user experience.

**Acceptance Scenarios**:

1. **Given** any page load, **When** the page renders, **Then** a `page.viewed` event is emitted
2. **Given** the map, **When** it finishes loading, **Then** a `map.loaded` event is emitted
3. **Given** the map, **When** the viewport changes after debounce, **Then** a `map.viewport_changed` event is emitted
4. **Given** a search, **When** results are returned, **Then** a `search.performed` event is emitted
5. **Given** a station marker clicked, **When** the detail opens, **Then** a `station.marker_clicked` event is emitted
6. **Given** a favorite toggle, **When** completed, **Then** a `favorite_station.added` or `favorite_station.removed` event is emitted
7. **Given** a review submission, **When** completed, **Then** a `review.submitted` event is emitted
8. **Given** an auth flow, **When** started/succeeded/failed, **Then** the corresponding `auth.*` event is emitted
9. **Given** any event emission failure, **When** the API call errors, **Then** the user experience is unaffected (fire-and-forget)

---

### Edge Cases

- What happens when the user pans outside Tunisia and no stations exist in that region? Empty-state message on the map layer.
- How does the map handle rapid panning/zooming? Viewport queries are debounced by 500ms; in-flight requests are cancelled when a new viewport query fires.
- What happens if the station detail API fails? An inline error message with a retry button replaces the detail content.
- How does the login modal handle a network error during authentication? Error message displayed within the modal; user can retry or close the modal.
- What happens when a user with a slow connection loads the app? Skeleton screens render for the initial map load; subsequent data fetches show spinners only on the affected section.
- How does the app handle expired JWTs during a gated action? The login modal re-appears, and after re-authentication, the action retries automatically.

## Requirements

### Functional Requirements

- **FR-001**: The app MUST display an interactive map with clustered station markers that update based on viewport
- **FR-002**: Users MUST be able to pan and zoom the map with viewport-driven station refetch (500ms debounce)
- **FR-003**: The app MUST show station details (name, description, chargers, availability, distance) when a marker is selected
- **FR-004**: Users MUST be able to search stations by name, city, and filter by connector type and availability
- **FR-005**: Unauthenticated users MUST be able to browse the map, view station details, and use search
- **FR-006**: Authentication MUST only be required for gated actions (favorite, review)
- **FR-007**: The login modal MUST appear when a gated action is attempted by an anonymous user
- **FR-008**: After successful login, the gated action MUST complete automatically
- **FR-009**: Registered drivers MUST be able to add and remove favorite stations
- **FR-010**: Registered drivers MUST be able to submit, edit, and delete their own reviews (rating 1-5 + comment)
- **FR-011**: The app MUST enforce one review per user per station
- **FR-012**: The app MUST emit clickstream events for all listed interactions without blocking the user
- **FR-013**: The app MUST render using skeleton loading patterns during initial data fetches
- **FR-014**: The app MUST cancel in-flight requests when superseded by a new viewport query
- **FR-015**: The app MUST display empty-state messages when no stations or reviews are available

### Key Entities

- **Station**: A charging location with geographic position, name, description, address metadata, and operational status. Has associated chargers and availability.
- **Station Marker**: A map annotation representing a station's geographic position with visual status indicator.
- **Station Detail**: A panel showing full station information including charger list, availability, reviews summary, and distance.
- **Favorite**: A user's bookmark for a station, toggleable on/off.
- **Review**: A user's rating (1-5) and optional comment for a station. One per user per station. Supports owner edit/delete and admin moderation.
- **Search Query**: A user's text input with optional filters (connector type, availability) producing filtered station results.
- **Auth Modal**: A modal dialog that presents Keycloak login when an anonymous user attempts a gated action.

## Success Criteria

### Measurable Outcomes

- **SC-001**: An unauthenticated user can discover a station (map → select → view details) in under 3 seconds on a standard connection
- **SC-002**: Map viewport transitions feel responsive — marker updates appear within 1 second of pan/zoom stop
- **SC-003**: 100% of gated actions (favorite, review) are preceded by an auth prompt when the user is anonymous
- **SC-004**: After login, the gated action completes without requiring the user to repeat the trigger action
- **SC-005**: Search returns results within 500ms of debounce completion
- **SC-006**: All clickstream events listed in the acceptance scenarios are emitted with correct event names and payloads
- **SC-007**: The app renders correctly in both LTR (French) and RTL (Arabic) layouts using the design system foundation
- **SC-008**: Page load shows skeleton screens within 200ms, replaced by content within 3 seconds on a standard connection

## Assumptions

- The `@bornemap/design-tokens` package and all 5 component primitives (Button, Input, Card, Modal, MapContainer) from Sprint 8 are available
- The `@bornemap/api-client` package provides the HTTP client with JWT handling, pagination helpers, and envelope parsing
- The `@bornemap/auth-client` package provides Keycloak login/logout modal and JWT management
- The `@bornemap/event-taxonomy` package defines the canonical event names and envelope from the execution plan
- The driver-service backend APIs are fully operational (Sprint 7)
- Station data has been seeded for the Tunisia region
- "Standard connection" in success criteria means ≥10 Mbps throughput, ≤50ms latency to the server
- React Query is used for all server state management (caching, refetching, optimistic updates)
- The app targets both Arabic (RTL) and French (LTR) users with the design system's RTL foundation
- Map clustering uses Leaflet-compatible clustering (e.g., Leaflet.markercluster)
- Loading patterns follow skeleton screens for initial content and minimal spinners for subsequent updates
- Station detail panel appears as a **side panel** (260–400px) shrinking the map when a marker is selected
- Favorites are accessed via an **inline map filter** toggle (no dedicated favorites page)
- Search is a **collapsible overlay** triggered by a search icon/button in the header
