# MVP01 — Public Discovery Vertical Slice

A 12-week delivery that builds the complete public discovery foundation: OSM
geospatial data, inventory schema, Driver Service public endpoints, and both
Driver Web + Mobile apps.

---

## Scope Boundary

### In Scope

- GIS schema tables (OSM import targets, roads, boundaries, station GIS layer)
- Inventory schema (partner, station, charger tables only)
- Driver Service — public endpoints only (nearby, markers, search, station detail)
- Driver Web App (map, list, station detail, search)
- Driver Mobile App (map, list, station detail, search)
- Shared `ui` package (tokens + core components needed by these screens only)
- Shared `api-client` (driver endpoints only)

### Out of Scope

- Auth, favorites, reviews, profile
- Admin Service, Clickstream Service
- Partner Dashboard, Admin Dashboard
- Analytics
- Keycloak

---

## Sprint 0 — Foundation (Week 1–2)

**Goal:** Everything compiles, runs, and connects before writing any real logic.

### Monorepo Setup

- Initialize the monorepo with the directory structure from the codebase document
- Set up Cargo workspace with `Cargo.toml` at root
- Set up pnpm workspace with `pnpm-workspace.yaml`
- Scaffold `services/driver-service` as an empty Actix-Web binary with Clean
  Architecture module directories (`domain/`, `application/`, `infrastructure/`,
  `interface/handlers/`, `interface/middleware/`)
- Scaffold `apps/driver-web` as an empty Vite + React app
- Scaffold `apps/driver-mobile` as an empty Expo app
- Stub `crates/ev-core` (domain layer — NanoID, enums, value objects)
- Stub `crates/ev-db` (infrastructure layer — pool, pagination)
- Stub `crates/ev-geo` (domain layer — LatLng, bbox, distance)
- Stub `packages/ui` and `packages/api-client` as empty packages

### Database Foundation

- Create `db/migrations/0001_extensions.sql` — enable PostGIS, uuid-ossp, pgcrypto
- Create `db/migrations/0002_inventory_schema.sql` — create inventory schema
- Create `db/migrations/0003_gis_schema.sql` — create gis schema
- Write a migration runner script or use sqlx-cli configured in the workspace

### Docker Compose Baseline

- Write `infra/compose/docker-compose.yml` with: PostgreSQL + PostGIS, Driver Service, pgAdmin (dev only)
- Verify Driver Service connects to PostgreSQL on startup
- Write `infra/env/.env.example`

### Done When

- `cargo build` succeeds for the whole workspace
- `pnpm install` succeeds
- `docker compose up` starts PostgreSQL and Driver Service
- Driver Service `/health` returns 200
- Migrations run cleanly on a fresh database

---

## Sprint 1 — OSM Schema + Data Import (Week 3–4)

**Goal:** OSM data lives in the `gis` schema and is queryable.

### GIS Schema Tables

Create `db/migrations/0004_gis_tables.sql`:

```sql
-- Raw OSM node/way/relation import targets
gis.osm_nodes        (osm_id, tags JSONB, geom GEOMETRY(Point, 4326))
gis.osm_ways         (osm_id, tags JSONB, geom GEOMETRY(LineString, 4326))

-- Derived layers
gis.roads            (id, osm_id, name, road_type, geom GEOMETRY(LineString, 4326))
gis.boundaries       (id, osm_id, name, admin_level, geom GEOMETRY(MultiPolygon, 4326))
gis.amenity_points   (id, osm_id, amenity_type, name, tags JSONB, geom GEOMETRY(Point, 4326))

-- Station GIS layer (populated by GIS Sync Worker later)
gis.station_locations (
    station_id      TEXT PRIMARY KEY,  -- references inventory STN-...
    geom            GEOMETRY(Point, 4326),
    snapped_road_id BIGINT,
    region_id       BIGINT,
    updated_at      TIMESTAMPTZ
)
```

- Add spatial indexes on all geom columns
- Add index on `osm_id` for all OSM tables

### OSM Import

- Download Tunisia OSM extract (`.osm.pbf`) from Geofabrik
- Use `osm2pgsql` to import into `gis.osm_nodes`, `gis.osm_ways`
- Write a shell script at `infra/osm/import.sh` that runs the import against the local Docker PostgreSQL
- Derive `gis.roads` from OSM ways where `highway` tag is present
- Derive `gis.boundaries` from OSM relations where `boundary=administrative`
- Derive `gis.amenity_points` from OSM nodes where `amenity` tag is present

### Inventory Schema Baseline

Create `db/migrations/0005_inventory_tables.sql`:

```sql
inventory.partner (
    id          TEXT PRIMARY KEY,  -- PRT-...
    name        TEXT NOT NULL,
    created_at  TIMESTAMPTZ
)

inventory.station (
    id           TEXT PRIMARY KEY,  -- STN-...
    partner_id   TEXT REFERENCES inventory.partner(id),
    name         TEXT NOT NULL,
    address      TEXT,
    latitude     NUMERIC(10,7) NOT NULL,
    longitude    NUMERIC(10,7) NOT NULL,
    created_at   TIMESTAMPTZ,
    updated_at   TIMESTAMPTZ
)

inventory.charger (
    id             TEXT PRIMARY KEY,  -- CHG-...
    station_id     TEXT REFERENCES inventory.station(id),
    connector_type TEXT NOT NULL,
    power_kw       NUMERIC(6,2),
    status         TEXT NOT NULL DEFAULT 'available',
    updated_at     TIMESTAMPTZ
)
```

- Add dev seeds: 2 partners, 10 stations across Tunisia with real coordinates, 2–3 chargers per station

### ev-core Crate

- Implement `ids.rs` — NanoID generation with prefix support for STN, CHG, PRT, USR, REV, EVT
- Implement `types.rs` — shared enums (ConnectorType, ChargerStatus)

### ev-geo Crate

- Implement `point.rs` — LatLng struct, conversion to/from PostGIS geometry
- Implement `bbox.rs` — bounding box struct
- Implement `distance.rs` — haversine distance calculation

### Done When

- All migrations run cleanly from zero
- OSM Tunisia data is importable via `import.sh`
- `gis.roads` and `gis.boundaries` are populated after import
- Dev seeds insert successfully
- `ev-core` and `ev-geo` crates compile with unit tests passing

---

## Sprint 2 — Driver Service Public Endpoints (Week 5–6)

**Goal:** All public discovery endpoints are working and tested.

### ev-db Crate

- Implement `pool.rs` — SQLx PgPool setup from environment (infrastructure
  layer use by all services)
- Implement `pagination.rs` — shared cursor/offset pagination structs

### Driver Service — Clean Architecture Layers

Scaffold the full layer structure inside `services/driver-service/src/`:

**Domain layer** (`domain/`):
- `station.rs` — Station entity, StationSummary, MarkerPoint, StationDetail
- `favorite.rs` — Favorite entity (stub — out of scope for MVP01)
- `review.rs` — Review entity (stub — out of scope for MVP01)
- Repository traits: `StationRepository`

**Application layer** (`application/`):
- `stations.rs` — use case functions: `get_nearby`, `get_markers`,
  `search_stations`, `get_station_detail`
- Each function receives a `&impl StationRepository` for dependency inversion

**Infrastructure layer** (`infrastructure/`):
- `db/pool.rs` — SQLx PgPool initialization
- `db/stations.rs` — `StationRepository` implementation with raw SQL queries
  via `sqlx::query_as!`

**Interface layer** (`interface/`):
- `handlers/stations.rs` — Actix-Web handler functions for each endpoint
- `middleware/logging.rs` — request logging via `tracing`
- `router.rs` — route definitions binding paths to handlers

**Cross-cutting**:
- `config.rs` — environment configuration struct
- `errors.rs` — typed `ServiceError` enum mapped to HTTP status codes
- `main.rs` — binary entrypoint wiring everything together

### Endpoint Handlers

**GET /stations/nearby**
- Query params: `lat`, `lng`, `radius_km` (default 10), `limit` (default 20)
- Logic: query `inventory.station` with PostGIS `ST_DWithin`
- Returns: list of `StationSummary { id, name, address, lat, lng, distance_m, charger_count, available_count }`

**GET /stations/markers**
- Query params: `bbox` (min_lat, min_lng, max_lat, max_lng)
- Logic: query `inventory.station` within bounding box
- Returns: lightweight list of `MarkerPoint { id, lat, lng, available_count }`
- Purpose: map rendering — minimal payload

**GET /stations/search**
- Query params: `q` (text), `lat`, `lng`, `connector_type`, `min_power_kw`, `limit`, `offset`
- Logic: text search on name/address + optional filters
- Returns: paginated list of `StationSummary`

**GET /stations/:id**
- Logic: fetch station + chargers + rating summary
- Returns: `StationDetail { id, name, address, lat, lng, chargers: [...], rating_avg, review_count }`

### Database Query Layer

- Write all queries in `db/stations.rs` using SQLx
- Use raw SQL with `sqlx::query_as!` macro — no ORM
- Write integration tests in `tests/integration/stations_test.rs` using a test database

### Done When

- All four endpoints return correct data against the seeded database
- Integration tests pass for each endpoint including edge cases (no results, invalid bbox, station not found)
- Error responses are consistent and typed

---

## Sprint 3 — Shared UI Package + Driver Web App (Week 7–8)

**Goal:** Driver Web App is functional for public discovery.

### packages/ui — Tokens and Core Components

- Export all design tokens using the **bright theme** (see `docs/06-frontend/bright-theme.md`):
  - Colors: `ev-bg` (#F8FAF6), `ev-surface` (#FFFFFF), `ev-green` (#007943), `ev-glow` (#00E676), etc.
  - Typography: Plus Jakarta Sans, Inter with scale 12-36px
  - Spacing: 4px base unit, comprehensive scale
- Build these components only (what the discovery screens need):
  - Button (primary [ev-green bg], secondary, ghost — all sizes)
  - Input (text, search with ev-border)
  - Badge (availability status with ev-green or ev-glow)
  - Skeleton (for loading states)
  - EmptyState (ev-bg canvas)
  - ErrorState (ev-muted text)
  - Toast (ev-surface card)

### packages/api-client — Driver Endpoints

- Implement typed API functions for all four Driver Service endpoints
- Use `fetch` with typed request/response shapes matching the service contracts
- Export from `driver/stations.ts`

### Driver Web App Screens

All screens use the **bright theme** with `ev-` color tokens and Plus Jakarta Sans typography.

**Map / Home screen:**
- Full-bleed map (Mapbox GL JS or Leaflet, `bg-ev-mapBg` background)
- Fetch markers on bbox change
- Station markers with `bg-ev-glow` for available, `bg-gray-400` for unavailable
- Click marker to open `bg-ev-surface` station summary card
- Filter panel with `bg-ev-green` primary buttons, `border-ev-border` inputs

**Station List screen:**
- Fetch nearby stations using geolocation API
- Render StationCard list with `bg-ev-surface` cards, `border-ev-border`
- Pull-to-refresh equivalent (reload button with `bg-ev-green`)
- Loading skeletons (`bg-gray-200`) while fetching
- Empty state with `bg-ev-bg` canvas, `text-ev-muted` placeholder

**Station Detail screen:**
- Fetch station by ID
- Show name, address, charger list in `bg-ev-surface` card
- Show average rating (read only, no auth needed) in `text-ev-green`
- Show map snippet centered on station

**Search screen:**
- Search input with `border-ev-border` and `placeholder-ev-muted`
- Filter bar with pill buttons (`bg-ev-surface`, `border-ev-border`)
- Paginated results with `bg-ev-surface` result cards
- Empty state with `text-ev-muted`

**i18n setup:**
- Configure i18n with Arabic and French
- Implement RTL layout switching for Arabic (bright theme colors unchanged)
- Translate all static strings in these four screens

### Done When

- All four screens render correctly with real data from Driver Service
- Arabic RTL layout works
- Loading, empty, and error states are handled on every screen
- Runs on Chrome, Firefox, Safari without visual regressions

---

## Sprint 4 — Driver Mobile App (Week 9–10)

**Goal:** Driver Mobile App covers the same public discovery scope as the web app.

### packages/ui — Mobile Token Export

- Add a `native/` export path to the ui package exporting tokens as React Native compatible values (no CSS variables, raw values only)

### Driver Mobile App Screens

All screens use the same **bright theme** colors and typography as web app (via shared `tailwind.config.js`).

**Map screen:**
- Use `react-native-maps` or `expo-maps` with `ev-mapBg` background
- Fetch markers on region change
- Station markers with `ev-glow` color for available, gray for unavailable
- Tap marker to open bottom sheet (white `ev-surface` with `ev-border`) with station summary

**Station List screen:**
- Use device location via `expo-location`
- Fetch nearby stations
- Render StationCard list with `ev-surface` backgrounds, `ev-border` dividers
- Pull to refresh with `ev-green` spinner
- Loading skeletons with gray backgrounds

**Station Detail screen:**
- Full scrollable screen with `ev-bg` background
- Station info card (`ev-surface`, `ev-border`)
- ChargerRow list with `text-ev-green` availability status
- Rating display with `text-ev-green` stars
- Map snippet with `ev-mapBg`

**Search screen:**
- Search input with `ev-border`, `placeholder-ev-muted`
- Filter controls (pill buttons with `ev-green` active state)
- Results list with `ev-surface` cards

**Navigation:**
- Bottom tab bar: Map, List, Search
- Active tab uses `text-ev-green`, inactive uses `text-ev-muted`
- Stack navigator for station detail screens

**i18n and RTL:**
- Mirror the web app language support (Arabic and French)
- RTL layout for Arabic using React Native's built-in RTL support
- Bright theme colors and layout are RTL-compatible (no special color changes)

### Done When

- All four screens run correctly on iOS simulator and Android emulator
- Real data from Driver Service is displayed
- Location permission request works
- Arabic RTL layout renders correctly
- No crashes on empty state or network error

---

## Sprint 5 — Fix, Integrate, Harden (Week 11–12)

**Goal:** The vertical slice is solid end to end.

### Integration Verification

- Full end-to-end test: OSM import → seeds → Driver Service → Driver Web App on a clean environment
- Repeat for Driver Mobile App
- Verify all four web screens against all four service endpoints with real data

### Fix Sweep

- Run all backend integration tests — fix any failures
- Run all frontend component tests — fix any failures
- Manual smoke test of every screen on web and mobile
- Check all error states are handled: network down, service 500, empty results, slow response
- Check RTL on all screens in Arabic
- Check all loading states render correctly

### Performance Baseline

- Measure `/stations/nearby` query time with 10 stations and 1000 stations in the database
- Add PostGIS index if query exceeds 100ms
- Measure marker endpoint with full Tunisia station set
- Check frontend bundle size — identify anything obviously oversized

### Documentation

- Write `docs/api/driver-service.md` with all four endpoints documented
- Write `docs/schema/inventory.md` and `docs/schema/gis.md` reflecting what was actually built
- Write `docs/onboarding.md` section for running this slice locally
- Update `infra/env/.env.example` to be complete

### Done When

- End-to-end smoke test passes on a clean environment with no manual fixes needed
- All Class A issues are resolved
- Documentation reflects reality

---

## What You Have After These 5 Sprints

A fully working public discovery slice:

- Tunisia OSM data in the `gis` schema
- Inventory schema with real station data
- Driver Service with four tested public endpoints
- Driver Web App with map, list, detail, search in Arabic and French
- Driver Mobile App with the same four screens
- Shared design tokens and core components
- Typed API client shared across both apps

This is your foundation. Every subsequent sprint builds on this without
rebuilding it.
