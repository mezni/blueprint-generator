# Feature Specification: Cross-Platform UI Synchronization

- **Branch:** `006-cross-platform-ui`
- **Created:** 2026-05-28
- **Status:** Draft
- **Specifier:** dali

---

## Clarifications

### Session 2026-05-28
- Q: Station detail panel/sheet interaction model — what states and gestures should be supported? → A: Desktop: fixed-height, non-draggable panel, dismiss via X or outside click. Mobile: bottom sheet with peek (120px) and expanded (70% height) states, draggable, swipe-to-dismiss.
- Q: Shared API contract approach? → A: Reference an OpenAPI 3.0 schema file under `contracts/` in the feature directory; base path `/api/v1/`, three endpoint names: search, station-detail, filter-sync.
- Q: Data volume & rendering scale — max stations in a single viewport? → A: Moderate scale — ≤200 stations per viewport. Clustering recommended; virtual rendering if exceeding threshold.
- Q: Accessibility baseline? → A: WCAG 2.1 AA — screen reader labels on all interactive map elements, 44pt minimum touch targets on mobile, full keyboard navigation on desktop.
- Q: Cross-platform filter sync mechanism? → A: Poll-based — client fetches `GET /api/v1/filters?session_id=` on app foreground/resume and every 60s while active. Server-timestamped, last-writer-wins.

---

## User Scenarios & Testing

### P1 — Unified Map Layout Parity
> **As a** user,  
> **I want** the desktop web map and the mobile app map to use the same layout primitives (header, map viewport, bottom sheet/panel, zoom controls),  
> **so that** the experience is consistent across devices and transport modes (car, bicycle, pedestrian).

- **Why P1:** Cross-platform layout parity is the core value of this feature. Without it, users switching between desktop and mobile face a fractured experience.
- **Independent test:**
  1. Open MapPortal on desktop (1920×1080) — verify header, full-height map viewport, zoom controls, bottom panel, and floating action button are visible.
  2. Open MapScreen on mobile (390×844) — verify compact header, full-height mapport, floating zoom controls, bottom sheet, and FAB are visible.
  3. Compare layout structure — both should expose: navigation bar/header → map viewport → search/filter tool → zoom controls → station detail area → FAB.
- **Acceptance:**
  - Given a desktop browser at 1920×1080, when MapPortal renders, then all six layout zones are present in the defined order.
  - Given a mobile device at 390×844, when MapScreen renders, then all six layout zones are present in the defined order (with mobile-adapted sizes).
  - Given a responsive test, when viewport width transitions from 1440px to 375px, then the layout adjusts via a breakpoint-aware strategy (e.g., panel→sheet, absolute→floating controls) without losing any zone.

### P1 — Cross-Platform Navigation Consistency
> **As a** user,  
> **I want** the navigation bar on desktop and the bottom tab bar on mobile to expose the same four destinations (Map, Explore, Saved, Profile),  
> **so that** I can navigate intuitively regardless of device.

- **Why P1:** Navigation is the primary wayfinding mechanism; inconsistency here disorients users.
- **Independent test:**
  1. On desktop, click each nav item — verify route/state change.
  2. On mobile, tap each tab item — verify route/state change.
  3. Compare active-state indicators (underline on desktop, filled icon on mobile).
- **Acceptance:**
  - Given the desktop NavBar, when rendered, then it contains exactly four items: Map, Explore, Saved, Profile.
  - Given the mobile BottomTabBar, when rendered, then it contains exactly four items: Map, Explore, Saved, Profile.
  - Given any nav/tab click, when the destination changes, then the corresponding active indicator updates on the same device and (if applicable) in the shared session.

### P1 — Map Component & Interaction Parity
> **As a** user,  
> **I want** the map tiles, charger markers, and cluster behavior to be identical on both platforms,  
> **so that** I can rely on the same visual information and interaction model everywhere.

- **Why P1:** The map is the central artifact; visual and behavioral differences cause user errors and erode trust.
- **Independent test:**
  1. Load the same map region on both platforms — compare tile source, marker density, clustering threshold, and marker icon appearance.
  2. Interact (pan, zoom, tap marker) on desktop — verify behavior matches mobile.
- **Acceptance:**
  - Given the same center and zoom on both platforms, when rendered, then tile layer URLs, marker icons (charging stations), and cluster algorithms produce visually identical output.
  - Given a marker tap on either platform, when the detail view opens, then the same station ID, name, charger count, and status is shown.

### P2 — Search & Filter Parity
> **As a** user,  
> **I want** the search bar and filter controls to work identically on desktop and mobile,  
> **so that** I can refine results with the same effort on any device.

- **Why P2:** Search/filter is a core workflow but used less frequently than map browsing; functional parity is required, but UX polish (e.g., mobile keyboard handling) can be deferred.
- **Acceptance:**
  - Given a search query on desktop, when submitted, then results match the same query on mobile.
  - Given an active filter set (e.g., "Type 2 only") on one platform, when the other platform resumes to foreground or after a 60s poll interval, then filters are synchronized via `GET /api/v1/filters`.

### P2 — Zoom Control Parity
> **As a** user,  
> **I want** zoom-in/zoom-out and locate-me controls to be accessible on both platforms,  
> **so that** I can adjust the viewport regardless of device.

- **Acceptance:**
  - Given the desktop map, when rendered, then zoom controls are in the bottom-right corner as an inline group.
  - Given the mobile map, when rendered, then zoom controls are in the bottom-right corner as floating action buttons over the map.

### P2 — Station Detail Sheet Parity
> **As a** user,  
> **I want** tapping a station marker to open a detail view — a bottom panel on desktop and a bottom sheet on mobile — with identical information,  
> **so that** I can evaluate a charging station consistently on any device.

- **Acceptance:**
  - Given a station marker tap on desktop, when the bottom panel opens, then it shows: station name, address, available/total chargers, connector types, status indicator, and a "Navigate" CTA.
  - Given a station marker tap on mobile, when the bottom sheet opens, then it shows the same six fields in the same order.
  - Given the mobile bottom sheet, when in peek state (120px), then station name and status are visible; when dragged to expanded state (70% height), then all six fields are visible.
  - Given the mobile bottom sheet, when the user swipes down past the peek threshold, then the sheet dismisses completely.
  - Given the desktop bottom panel, when the user clicks outside or presses X, then the panel closes.

### P2 — Shared Analytics Events
> **As a** product manager,  
> **I want** the desktop web app and the mobile app to emit identical clickstream event payloads for map interactions,  
> **so that** the analytics pipeline deduplicates and aggregates events from both platforms in a single schema.

- **Acceptance:**
  - Given a marker tap on desktop, when the analytics event fires, then its schema matches the marker tap event on mobile (same event name, same required fields, same optional fields pattern).
  - Given a zoom action on either platform, when the analytics event fires, then `zoom_level` and `viewport_center` are populated in both.

---

## Edge Cases

- **Network failure during search:** Both platforms should show an inline error message and a retry button; search state is preserved so the user can retry without re-typing.
- **Empty search results:** Both platforms show a "No stations found" illustration with a suggestion to widen the area.
- **GPS unavailable (mobile) / location denied (desktop):** The locate-me button should be disabled with a tooltip explaining the missing permission.
- **Station detail loading failure:** The detail panel/sheet shows a skeleton placeholder for 300 ms; if still loading after 2 s, show a static summary (name + address) with a "Retry" link.
- **Rapid marker taps:** Debounce detail-open events with a 500 ms window to prevent sheet flickering.
- **Mobile sheet drag conflict:** When the sheet is expanded and the map is visible behind it, map pan gestures should not conflict with sheet scroll/drag; the sheet captures vertical drag within its bounds.
- **Desktop panel on narrow viewport:** Below 500 px viewport height, the bottom panel collapses to a minimized bar showing only station name and status; tap the bar to expand.
- **Keyboard navigation (desktop):** All interactive elements (search, filters, zoom, FAB, panel close) must be reachable via Tab in a logical order. Escape closes the panel. Enter/Space activates buttons. Map markers are grouped into a single tab-stop per cluster; individual markers in expanded clusters are navigable via arrow keys.
- **Screen reader labels (both):** All markers, buttons, and controls must have `aria-label` equivalents. Map tiles and decorative elements are marked `aria-hidden="true"`. Dynamic region updates (e.g., search results loading) use `aria-live="polite"`.
- **Touch target size (mobile):** All tappable elements have a minimum 44×44 pt hit area. Closely packed controls (zoom ±, FAB) are spaced to meet this constraint.
- **Cross-platform filter sync conflict:** If filters are modified on both platforms simultaneously, the last-writer-wins strategy applies (server-timestamped filter state).

---

## Out of Scope

- Offline map tile caching (separate feature).
- Turn-by-turn navigation inside the app (defers to Google Maps / Waze via URL scheme).
- User authentication / saved-stations sync beyond what the shared API provides.
- Push notifications for charger availability changes.
- Dark mode / theme customization.
- Tablet-specific layout adaptations (phablet+ is treated as desktop).

---

## Requirements

### Functional Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-001 | Desktop web shall render a NavBar with four items: **Map**, **Explore**, **Saved**, **Profile**; the active item is indicated by an underline. | P1 |
| FR-002 | Mobile app shall render a BottomTabBar with four items: **Map**, **Explore**, **Saved**, **Profile**; the active item has a filled icon. | P1 |
| FR-003 | Desktop web shall render a MapPortal layout: full-height viewport, top panel (search + filters) overlaid on map, bottom panel for station details, floating zoom controls (bottom-right), floating action button (bottom-center). | P1 |
| FR-004 | Mobile app shall render a MapScreen layout: compact header, full-height MapView, bottom sheet for station details, floating zoom controls (bottom-right), floating action button (bottom-center). | P1 |
| FR-005 | Both platforms shall use the same tile layer URL and marker/cluster rendering configuration for a given map region. | P1 |
| FR-006 | Both platforms shall emit identical clickstream event payloads for marker-tap, search-submit, filter-change, zoom-in, zoom-out, and locate-me actions, conforming to the shared schema defined in `contracts/api.yaml`. | P2 |
| FR-007 | Desktop search bar and mobile search bar shall call `GET /api/v1/search` and return identical results for the same input, per `contracts/api.yaml`. | P2 |
| FR-008 | Desktop filter controls and mobile filter controls shall call `GET/PUT /api/v1/filters` and produce identical query parameters when the same filter set is active, per `contracts/api.yaml`. | P2 |
| FR-009 | Desktop station detail (bottom panel) and mobile station detail (bottom sheet) shall display the same six fields: station name, address, available/total chargers, connector types, status indicator, "Navigate" CTA. | P2 |
| FR-010 | Desktop zoom controls (inline group, bottom-right) and mobile zoom controls (floating buttons, bottom-right) shall perform the same zoom and locate-me operations. | P2 |

### Key Entities

| Entity | Field | Type | Notes |
|---|---|---|---|
| `ClickstreamEvent` | `event_name` | string | One of: `marker_tap`, `search_submit`, `filter_change`, `zoom_in`, `zoom_out`, `locate_me` |
| `ClickstreamEvent` | `platform` | string | `desktop_web` or `mobile_app` |
| `ClickstreamEvent` | `session_id` | uuid | Generated on app start; shared across page views until expiry |
| `ClickstreamEvent` | `timestamp` | datetime | ISO-8601 UTC |
| `ClickstreamEvent` | `properties` | JSON | Variable payload depending on event_name (e.g., `station_id` for `marker_tap`, `query` for `search_submit`) |
| `StationDetail` | `station_name`, `address`, `available_chargers`, `total_chargers`, `connector_types[]`, `status`, `navigate_url` | — | Shared response shape consumed by both platforms |

---

## Success Criteria

| ID | Criterion | Target |
|---|---|---|
| SC-001 | Desktop and mobile render all six layout zones (nav, map, search/filter, zoom, detail, FAB) on first load. | 100% of test runs |
| SC-002 | Marker-tap on either platform opens the detail view with all six station fields populated. | ≤ 500 ms p95 |
| SC-003 | Identical search query on both platforms returns identical result set. | 100% match |
| SC-004 | Clickstream events from both platforms are accepted by the shared analytics endpoint without schema validation errors. | 100% of events |
| SC-005 | Layout passes responsive breakpoint test across 375 px, 768 px, 1024 px, and 1440 px without overlapping or clipped elements. | Lighthouse "No layout shift" |
| SC-006 | Filter state set on one platform is reflected on the other within 60s via poll-based sync (last-writer-wins). | Verified via integration test |

---

## Assumptions

1. **Breakpoint strategy**: 375–767 px = mobile layout; 768+ = desktop layout. No tablet-specific layout is required.
2. **Tech stack**: Desktop uses React with Leaflet (via react-leaflet); mobile uses React Native with `react-native-maps` (Google Maps on iOS / Android). Both share a common API gateway at `/api/v1/`.
3. **API contract**: All shared endpoints are defined in `contracts/api.yaml` (OpenAPI 3.0). The server validates requests against this schema; clients must adhere to it.
4. **Analytics backend**: The shared clickstream schema is enforced by a JSON Schema validator on the ingest side; events that fail validation are dropped to a dead-letter queue.
5. **Network**: Both platforms assume a stable internet connection for map tiles, search, and detail data.
6. **Data volume**: Maximum of ~200 stations per viewport at city level zoom. Clustering is recommended; platforms may switch to virtual rendering if density exceeds threshold.
7. **Session management**: `session_id` is generated client-side and persisted in localStorage (web) or AsyncStorage (mobile). It does not require user authentication.
