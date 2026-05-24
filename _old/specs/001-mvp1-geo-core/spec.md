# Feature Specification: MVP 1 — Geo Core (Read-Only Map)

**Feature Branch**: `001-mvp1-geo-core`
**Created**: 2026-05-23
**Status**: Draft
**Input**: User description: "Deliver a read-only viewport-driven map of EV chargers in Tunisia on web and mobile, with an admin portal that supports full station CRUD. Mock JWT auth; no reviews or driver accounts yet."

> Read this first: [`docs/plan.md`](../../docs/plan.md) (MVP 1 section), [`docs/constitution.md`](../../docs/constitution.md), [`docs/architecture.md`](../../docs/architecture.md).

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Driver discovers nearest charger (Priority: P1)

A driver opens the web or mobile app, the map auto-centers on Tunisia, and they pan/zoom to see chargers as colored pins. Tapping a pin opens a detail view with chargers, opening hours, address. Tapping "Navigate" deep-links to the OS map app.

**Why this priority**: This is the entire reason the platform exists. Without it, MVP 1 has no product.

**Independent Test**: A driver can locate the nearest charger to a chosen address and view its details in under 30 seconds, on both web and mobile, using only the deployed staging build.

**Acceptance Scenarios**:

1. **Given** the map is loaded centered on Tunis, **When** the driver pans to Sousse, **Then** chargers in the Sousse viewport render within 1 s after pan settles.
2. **Given** a marker is visible, **When** the driver taps/clicks it, **Then** a detail panel (web) or bottom sheet (mobile) shows name, address, charger count, opening hours.
3. **Given** the detail view is open, **When** the driver taps "Navigate", **Then** the OS map app opens with the destination preset to the station's coordinates.
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

### User Story 3 — Mock authentication for admins (Priority: P2)

An admin enters a username and selects a role at `/login`. The system issues a mock JWT carrying the canonical claim shape (`sub`, `preferred_username`, `realm_access.roles`, `iat`, `exp`). All admin endpoints require this token. No driver accounts exist yet.

**Why this priority**: Required to gate admin endpoints, but uses dev-only mock crypto (frozen claim shape for forward compatibility with MVP 5 Keycloak).

**Independent Test**: An admin can log in, receive a JWT whose decoded payload matches the canonical claim shape exactly, and use it as a Bearer token on `POST /api/v1/admin/stations` successfully.

**Acceptance Scenarios**:

1. **Given** the admin enters `{username: "admin", role: "admin"}`, **When** they submit, **Then** they receive a JWT decodable with the dev secret matching the canonical claim shape.
2. **Given** an expired token, **When** the admin calls any `/admin` endpoint, **Then** the response is HTTP 401 with RFC-7807 problem details.
3. **Given** a driver-role token, **When** the holder calls any `/admin` endpoint, **Then** the response is HTTP 403.

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
- **FR-004**: System MUST expose `POST /api/v1/auth/mock-login { username, role }` returning a mock JWT with the canonical claim shape.
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

### Key Entities

- **Company**: Operator owning one or more stations. Attributes: `id`, `name`, `is_test`.
- **Station**: A physical site hosting chargers. Attributes: `id`, `company_id`, `name`, `address`, `location (lng,lat)`, `is_active`, `under_maintenance`, `opening_hours_json`, `is_test`, audit fields.
- **Charger**: A connector at a station. Attributes: `id`, `station_id`, `connector` (e.g., CCS, Type2, CHAdeMO), `power_kw`, `is_active`.
- **AdminUser** (mock): Username, role. No persistence in MVP 1 beyond a hardcoded seed list (admin only).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: P95 server latency for `GET /api/v1/stations` ≤ **200 ms** at 500 stations seeded with 50 concurrent viewport queries/s.
- **SC-002**: Map sustains ≥ **60 FPS** during pan/zoom on the reference mid-range Android device.
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

- Reference mobile device is mid-range Android with 4 GB RAM (2022 baseline).
- Synthetic seed dataset is acceptable for performance baselines; real operator data lands incrementally during MVP 2.
- Mock JWT secret is environment-scoped and never deployed to a public environment with real users beyond the staged test cohort.
- Mail transport, RabbitMQ, MinIO, Redis, and Keycloak are NOT required at this MVP.
- Frontends run only on modern browsers (last 2 versions of Chrome/Firefox/Safari/Edge) and iOS 15+ / Android 9+.

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
