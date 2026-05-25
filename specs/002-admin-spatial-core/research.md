# Research: Admin Spatial Core Validation

**Phase**: Phase 0 — Technology & Approach Research

## Decisions

### Decision: Station CRUD with Spatial Queries

**Decision**: Extend existing `stations/` feature domain with spatial CRUD endpoints and a dedicated nearby search endpoint using ST_DWithin.

**Rationale**: The existing stations feature domain already has the correct file structure (models, schemas, repository, service, router). Phase 1 adds the spatial dimension: a `GEOGRAPHY(Point, 4326)` column for location, GiST index for performance, and a `GET /api/v1/stations/nearby` endpoint accepting `lat`, `lng`, and `radius_m` query parameters. The nearby endpoint uses `ST_DWithin` for indexed filtering and `ST_Distance` for result ordering.

**Alternatives considered**:
- Single catch-all endpoint with query params: Rejected because spatial query patterns (viewport vs. radius) have different indexing and response requirements.
- PostGIS-only bounding box without ST_DWithin: Rejected because ST_DWithin leverages the GiST index for performance; raw bounding box comparisons do not.

### Decision: Mock JWT Authentication

**Decision**: Implement mock JWT (HS256) per constitution §7 Phase 1–2 strategy. Token payload decoded from `X-Mock-Token` header contains `sub`, `username`, `roles`, `iat`, `exp`.

**Rationale**: Matches the established auth roadmap. Quick to implement (PyJWT for verification, base64-encoded JSON payloads for development). Allows frontend to carry auth state without a full identity provider.

**Alternatives considered**:
- No auth (Phase 0 style): Rejected because US3 requires admin-only write operations.
- Session-based auth: Rejected because token-based auth aligns with the planned Keycloak migration path.

### Decision: Leaflet Map Click-to-Add Flow

**Decision**: Use Leaflet's `click` event to capture coordinates, open a floating form panel, and POST the station via React Query mutation.

**Rationale**: The existing Phase 0 map infrastructure already uses Leaflet + react-leaflet. Adding click handlers and popup components is the standard Leaflet pattern. React Query handles cache invalidation so the new marker appears immediately.

**Alternatives considered**:
- Separate form page: Rejected because the spec requires map-centric interaction (FR-001, FR-002).
- Custom draw plugin: Overengineered for <500 stations.

### Decision: Viewport Filtering via Bounding Box

**Decision**: The station list updates by sending the current map bounds (SW/NE lat/lng) to the backend, which queries stations within that bounding box using `ST_DWithin` with a box-derived radius.

**Rationale**: The constitution mandates `ST_DWithin` as the only spatial `WHERE` filter. Convert the viewport bounding box to a center-point + diagonal radius for the `ST_DWithin` call.

**Alternatives considered**:
- Client-side filtering after full fetch: Rejected by constitution rule — never frontend filter-after-fetch of server collections.
- Raw bounding box comparison without ST_DWithin: Violates constitution §8.

### Decision: Custom Green DivIcon Markers

**Decision**: Use Leaflet `L.divIcon` with inline SVG/CSS for green circular markers matching design tokens (`#22c55e`). No default Leaflet blue pins.

**Rationale**: The constitution explicitly bans default blue pins (§9). The existing Phase 0 `MapView.tsx` already references the pattern.

**Alternatives considered**:
- L.icon with PNG assets: More complex build pipeline, harder to theme.
- L.circleMarker: Lacks the visual polish of a custom divIcon with shadow and border.

### Decision: Station Name Unique Per Operator

**Decision**: Backend enforces `UNIQUE(name, operator)` constraint at the application layer (unique together validation in service). No conflated display name handling.

**Rationale**: Per spec clarification Q4. Prevents accidental duplicates while allowing different operators to use the same station name.

**Alternatives considered**:
- Database-level unique constraint on (name, operator): Deferred to future migration; application-layer enforcement is sufficient at <500 stations.
- Global unique name: Too restrictive — different partners may have stations with the same name in different regions.

### Decision: 20 km Default Search Radius

**Decision**: Nearby search defaults to 20 km radius when no explicit radius is provided. Configurable per request.

**Rationale**: Per spec clarification Q3. Covers a typical Tunisian metro area. User can override via query parameter.

**Alternatives considered**:
- 10 km: Too tight for rural areas with sparse station coverage.
- 50 km: Too broad, returns stations outside practical driving range for EV.
