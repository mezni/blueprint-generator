# Feature Specification: MVP 1 — Geo Core (Read-Only Map)

**Feature Branch**: `001-mvp1-geo-core`
**Created**: 2026-05-23
**Status**: Draft
**Input**: User description: "Deliver a read-only viewport-driven map of EV chargers in Tunisia on web and mobile, with an admin portal that supports full station CRUD. Mock JWT auth; no reviews or driver accounts yet."

> Read this first: [`docs/plan.md`](../../docs/plan.md) (MVP 1 section), [`docs/constitution.md`](../../docs/constitution.md), [`docs/architecture.md`](../../docs/architecture.md).

## Clarifications

### Session 2026-05-23

- Q: Which URL scheme should the "Navigate" action use to hand off to the OS map provider? → A: Universal Google Maps web URL — `https://www.google.com/maps/dir/?api=1&destination={lat},{lng}`. The OS routes to the installed native map app where available and falls back to a browser tab otherwise. Single cross-platform implementation; honors NG-5 (no in-app routing).
- Q: How should station opening hours be modeled? → A: OSM `opening_hours` string stored in a `TEXT` column (`opening_hours_osm`). Server validates with an OSM-spec parser; admin UI accepts the raw string with inline validation. Renders on clients via an OSM `opening_hours` parser library. Special values: `"24/7"` for always-open; `NULL` for unknown.
- Q: Which connector types are valid for MVP 1 chargers? → A: Closed enum enforced by a Postgres `CHECK` constraint with exactly four values: `Type2`, `CCS`, `CHAdeMO`, `Type2_Tethered`. Admin UI presents these as a dropdown; the same constants drive future filter pills in MVP 2 ("CCS", "Fast 50kW+"). Adding a new connector type later requires a migration.
- Q: How are MVP 1 admin accounts established and authenticated? → A: Env-driven allowlist. The backend reads `MOCK_ADMIN_USERNAMES` (comma-separated). `POST /api/v1/auth/mock-login` accepts `{username, role}`: if `role="admin"` AND username is in the allowlist, an admin JWT is issued; if `role="admin"` AND username is NOT in the allowlist, the response is HTTP 403; if `role="driver"`, the request is rejected with HTTP 400 because MVP 1 has no driver accounts. Allowlists are per-environment; staging uses a small fixed list rotated for the validation cohort.
- Q: Which devices and OS versions define the MVP 1 performance target and compatibility floor? → A: Performance reference devices for SC-002 (60 FPS pan/zoom): **Android — Samsung Galaxy A33 5G** (Android 12, 6 GB RAM, mid-2022 mid-range); **iOS — iPhone 11** (iOS 15+). Compatibility floors (below which the app refuses to launch with a clear "Update your OS" screen): **Android 10 (API 29)** and **iOS 15**. The validation cohort MUST include at least one tester on each performance reference device and at least one tester on the compatibility floor for each platform.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Driver discovers nearest charger (Priority: P1)

A driver opens the web or mobile app, the map auto-centers on Tunisia, and they pan/zoom to see chargers as colored pins. Tapping a pin opens a detail view with chargers, opening hours, address. Tapping "Navigate" deep-links to the OS map app.

**Why this priority**: This is the entire reason the platform exists. Without it, MVP 1 has no product.

**Independent Test**: A driver can locate the nearest charger to a chosen address and view its details in under 30 seconds, on both web and mobile, using only the deployed staging build.

**Acceptance Scenarios**:

1. **Given** the map is loaded centered on Tunis, **When** the driver pans to Sousse, **Then** chargers in the Sousse viewport render within 1 s after pan settles.
2. **Given** a marker is visible, **When** the driver taps/clicks it, **Then** a detail panel (web) or bottom sheet (mobile) shows name, address, charger count, opening hours.
3. **Given** the detail view is open, **When** the driver taps "Navigate", **Then** the client opens `https://www.google.com/maps/dir/?api=1&destination={lat},{lng}`, and the OS routes that URL to its installed native map app (or to a browser tab if none is installed).
4. **Given** the driver is offline, **When** they open the app, **Then** they see a clear offline state with the last-cached viewport.

---

### User Story 2 — Admin manages station data (Priority: P1)

An administrator logs into the admin portal, lists stations in a searchable/sortable table, creates a new station with `[lng, lat]` coordinates, edits an existing station, and soft-deletes a station. Soft-deleted stations disappear from the public map.

**Why this priority**: Data must be maintainable from day one. Without it, the catalog cannot grow.

**Independent Test**: A new admin can create a station with valid coordinates, see it appear on the map within 60 s (after their next viewport fetch), then soft-delete it and confirm it disappears.

**Acceptance Scenarios**:

1. **Given** the admin is logged in, **When** they open `/admin/stations`, **Then** they see a paginated table of all non-deleted stations.
2. **Given** the admin opens "Add Station", **When** they submit valid data (`name`, `address`, `[lng, lat]`, `company`), **Then** the station persists and is queryable via `GET /api/v1/stations`.
3. **Given** the admin edits a station's `under_maintenance` flag to `true`, **When** the change is saved, **Then** the pin color flips from green to red on the next viewport refresh.
4. **Given** the admin soft-deletes a station, **When** the deletion succeeds, **Then** the station no longer appears in public viewport queries but remains in admin-only "Show deleted" view.

---

### User Story 3 — Mock authentication for admins (Priority: P1 by dependency, P2 by user-facing impact)

An admin enters a username at `/login`. If the username is on the environment's `MOCK_ADMIN_USERNAMES` allowlist, the system issues a mock JWT with `realm_access.roles = ["admin"]` carrying the canonical claim shape (`sub`, `preferred_username`, `realm_access.roles`, `iat`, `exp`). All admin endpoints require this token. No driver accounts exist yet.

**Why this priority**: Required to gate admin endpoints, but uses dev-only mock crypto (frozen claim shape for forward compatibility with MVP 5 Keycloak).

**Independent Test**: An allowlisted admin can log in, receive a JWT whose decoded payload matches the canonical claim shape exactly, and use it as a Bearer token on `POST /api/v1/admin/stations` successfully; a non-allowlisted username receives HTTP 403.

**Acceptance Scenarios**:

1. **Given** `MOCK_ADMIN_USERNAMES="alice,bob"` is configured AND the admin submits `{username: "alice", role: "admin"}`, **When** they submit, **Then** they receive a JWT decodable with the dev secret, matching the canonical claim shape, with `realm_access.roles = ["admin"]`.
2. **Given** `MOCK_ADMIN_USERNAMES="alice,bob"` AND the admin submits `{username: "mallory", role: "admin"}`, **When** they submit, **Then** the response is HTTP 403 with RFC-7807 problem details and **no** JWT is issued.
3. **Given** any user submits `{username: "...", role: "driver"}`, **When** they submit, **Then** the response is HTTP 400 because MVP 1 has no driver accounts.
4. **Given** an expired token, **When** the admin calls any `/admin` endpoint, **Then** the response is HTTP 401 with RFC-7807 problem details.
5. **Given** a driver-role token (e.g., from a future MVP) is presented, **When** the holder calls any `/admin` endpoint, **Then** the response is HTTP 403.

---

### Edge Cases

- Viewport bbox crossing antimeridian or invalid (`west > east`): server returns `400` with a clear problem-detail message.
- Viewport contains > 5000 candidate markers: server caps result at `LIMIT 5000` and indicates truncation in response metadata.
- Empty viewport (no markers): server returns `200` with `markers: []` (not 404).
- Bbox parameters with > 4 decimals: server quantizes server-side defensively and returns the quantized bbox in the response.
- Concurrent admin edits to the same station: last-write-wins on `updated_at`; conflict semantics deferred to MVP 2.
- Slow network on mobile during initial load: map shows skeletons; first marker render must occur ≤ 2.5 s on 4G.
- Soft-deleted record reappears via cache: client must refetch after admin invalidates cache (cache invalidation is server-side from MVP 4 onward; for MVP 1 a 60 s `staleTime` is acceptable).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST expose `GET /api/v1/stations?bbox=west,south,east,north` returning marker summaries (`id`, `name`, `coord [lng, lat]`, `is_active`, `under_maintenance`).
- **FR-002**: System MUST expose `GET /api/v1/stations/{id}` returning full station detail (marker summary + chargers list + address + opening hours + company).
- **FR-003**: System MUST expose `POST /api/v1/admin/stations`, `PATCH /api/v1/admin/stations/{id}`, `DELETE /api/v1/admin/stations/{id}` (soft delete). Admin-role JWT required.
- **FR-004**: System MUST expose `POST /api/v1/auth/mock-login { username, role }` returning a mock JWT with the canonical claim shape. Authorization rules: if `role="admin"` AND `username ∈ MOCK_ADMIN_USERNAMES` → 200 + JWT with `realm_access.roles=["admin"]`; if `role="admin"` AND `username ∉ MOCK_ADMIN_USERNAMES` → 403; if `role="driver"` → 400 (no driver accounts in MVP 1); other values → 400.
- **FR-005**: System MUST persist stations using `GEOGRAPHY(Point, 4326)` with a GiST index named `stations_location_gix`.
- **FR-006**: System MUST execute viewport queries using `ST_DWithin(location, ST_MakeEnvelope(...)::geography, 0)`. `ST_Distance` MAY be used for ordering only.
- **FR-007**: System MUST quantize incoming bbox to 4 decimal places before query execution and return the quantized bbox in the response.
- **FR-008**: Frontend (web + mobile) MUST debounce viewport-triggered fetches by ≥ 300 ms.
- **FR-009**: Frontend (web + mobile) MUST clip off-viewport markers before render and cluster at > 15 markers within 40 px.
- **FR-010**: Mobile `<Marker>` components MUST set `tracksViewChanges={false}`.
- **FR-011**: System MUST seed the database with ≥ 500 synthetic stations covering Tunisia's bounding box before staging deployment.
- **FR-012**: System MUST expose `/health/live` and `/health/ready` returning 200 in a healthy state.
- **FR-013**: System MUST emit JSON logs with `trace_id` per request and a Prometheus `/metrics` endpoint.
- **FR-014**: Frontend HTTP types MUST be generated from `openapi.json` (no hand-rolled HTTP interfaces).
- **FR-015**: Soft delete MUST set `deleted_at = NOW()` and excluded rows from public reads via `WHERE deleted_at IS NULL`.
- **FR-016**: The "Navigate" action MUST open the URL `https://www.google.com/maps/dir/?api=1&destination={lat},{lng}` (lat/lng substituted from the station's `[lng, lat]` location). The client MUST NOT bundle a routing engine or implement turn-by-turn navigation (NG-5).
- **FR-017**: Stations MUST store opening hours as an OSM `opening_hours` spec string in `opening_hours_osm TEXT` (nullable). The backend MUST validate the value on create/update against the OSM `opening_hours` grammar and reject invalid strings with HTTP 422 + RFC-7807 problem details citing the parse error. The client MUST render the string using an OSM `opening_hours` parser library and MUST display "Hours unknown" when the column is NULL.
- **FR-018**: Chargers MUST persist a `connector` column constrained to exactly one of `Type2`, `CCS`, `CHAdeMO`, `Type2_Tethered` via a Postgres `CHECK` constraint. Admin create/update endpoints MUST reject any other value with HTTP 422 + RFC-7807. The same four constants MUST be exported from `@bornemap/api-client` for reuse by future filter pills.
- **FR-019**: The backend MUST read `MOCK_ADMIN_USERNAMES` (comma-separated, trimmed, case-sensitive) at startup and use it as the sole source of truth for "who is an admin" in MVP 1. The variable MUST be required in every environment that issues real tokens; an unset/empty value MUST cause the server to refuse to start (fail-closed). The value MUST NOT be logged.
- **FR-020**: The mobile app MUST detect the device OS version on launch. If the OS is below the compatibility floor (Android < 10 / iOS < 15), the app MUST show a non-dismissible "Update your OS to use BorneMap" screen and MUST NOT initialize the map or any authenticated state. The web app's Vite build MUST target browsers matching the last 2 stable versions of Chrome, Firefox, Safari, and Edge (browserslist).

### Key Entities

- **Company**: Operator owning one or more stations. Attributes: `id`, `name`, `is_test`.
- **Station**: A physical site hosting chargers. Attributes: `id`, `company_id`, `name`, `address`, `location (lng,lat)`, `is_active`, `under_maintenance`, `opening_hours_osm` (OSM `opening_hours`-spec string, e.g. `"Mo-Fr 08:00-20:00; Sa 09:00-13:00"` or `"24/7"`; nullable when unknown), `is_test`, audit fields.
- **Charger**: A connector at a station. Attributes: `id`, `station_id`, `connector` (closed enum: one of `Type2`, `CCS`, `CHAdeMO`, `Type2_Tethered`), `power_kw`, `is_active`.
- **AdminUser** (mock): Username, role. No database persistence in MVP 1; the set of valid admin usernames is sourced from the `MOCK_ADMIN_USERNAMES` env var at server start and held in memory for the lifetime of the process.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: P95 server latency for `GET /api/v1/stations` ≤ **200 ms** at 500 stations seeded with 50 concurrent viewport queries/s.
- **SC-002**: Map sustains ≥ **60 FPS** during sustained pan/zoom (10-second gesture) on both performance reference devices: **Samsung Galaxy A33 5G** (Android 12, 6 GB RAM) and **iPhone 11** (iOS 15+). Measured via the platform's built-in frame profiler during a recorded validation session.
- **SC-003**: Cold-start time to first marker render on 4G ≤ **2.5 s** (web), ≤ **3.5 s** (mobile).
- **SC-004**: ≥ **90%** of test-cohort drivers complete "find nearest charger to a given address" in ≤ 30 s.
- **SC-005**: ≥ **95%** of test-cohort drivers can open the detail view of a chosen pin.
- **SC-006**: ≥ **90%** of test-cohort admins create a new station in ≤ 2 min on first attempt.
- **SC-007**: Zero hand-written HTTP type definitions on the frontend (verified via grep on `frontend/`).
- **SC-008**: Every spatial column has a GiST index (verified by introspection script in CI).
- **SC-009**: No `unwrap` / `expect` / `panic!` in non-test Rust code (verified by clippy lint configuration).

### Proceed Criteria → MVP 2

All SC-001..SC-009 pass **and** the validation report `validation.md` records no P0 usability blockers across ≥ 5 driver interviews + ≥ 2 admin interviews.

## Assumptions

- Performance reference devices: Samsung Galaxy A33 5G (Android 12, 6 GB RAM, mid-2022) and iPhone 11 (iOS 15+).
- Compatibility floors: Android 10 (API 29) and iOS 15. Below these, the app launches a fullscreen "Update your OS to use BorneMap" view and exits any deeper navigation.
- Synthetic seed dataset is acceptable for performance baselines; real operator data lands incrementally during MVP 2.
- Mock JWT secret is environment-scoped and never deployed to a public environment with real users beyond the staged test cohort.
- Mail transport, RabbitMQ, MinIO, Redis, and Keycloak are NOT required at this MVP.
- Web frontend supports the last 2 stable versions of Chrome, Firefox, Safari, and Edge.
- Mobile frontend supports iOS 15+ and Android 10+ (API 29+).

## Constitutional Gates (must be green at PR time)

- [ ] PostGIS + GiST + `[lng, lat]` everywhere (Principle I).
- [ ] Backend authority for all filtering and validation (Principle II).
- [ ] OpenAPI is the single source of HTTP truth; generated clients only (Principle III).
- [ ] Modular monolith only; no premature splits (Principle IV).
- [ ] TDD with PostGIS Testcontainers (Principle V).
- [ ] SQLx compile-time verified; `.sqlx/` committed (Principle VI).
- [ ] Newtype IDs; no `unwrap` in non-test code; RFC-7807 errors (Principle VII).
- [ ] Map Interaction Runtime Rules R1–R7 enforced (Principle VIII).
- [ ] Status Projection Rule honored (Principle IX).
- [ ] Real-user validation report attached (Principle XI).
