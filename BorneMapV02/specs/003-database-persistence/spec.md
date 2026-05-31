# Feature Specification: Database Persistence & Spatial Query Engine

**Feature Branch**: `003-database-persistence`

**Created**: 2026-05-28

**Status**: Draft

**Input**: User description: "add database update on existing code"

## User Scenarios & Testing

### User Story 1 - Mobile Driver Sees Nearby Charging Stations (Priority: P1)

A driver opens the BorneMap mobile app while near Tunis and sees EV charging stations plotted on a map. Stations are fetched from a persistent database that remembers their locations, statuses, and charger details across restarts.

**Why this priority**: This is the core value proposition of the app. Without a persistent data source, the app cannot show stations after a backend restart, making it unusable for real drivers.

**Independent Test**: Can be fully tested by deploying the database, seeding it with known test coordinates, and verifying the map shows the seeded stations at correct positions with correct status colors.

**Acceptance Scenarios**:

1. **Given** the database is seeded with 50 stations across 5 Tunisian regions, **When** a driver opens the map centered on Tunis (36.8065, 10.1815), **Then** stations within 15 km are displayed as map markers with correct status colors (green for Available, red for Occupied, orange for staged/non-live)
2. **Given** a station is live and Available, **When** the driver taps its marker, **Then** a detail card shows the station name, partner name and type, all available chargers with plug type and power output, and an "Available" badge
3. **Given** a station is marked as `is_live = false` (staged/testing), **When** queries use the default (non-staged) mode, **Then** that station is hidden from results
4. **Given** the database is empty, **When** the driver queries nearby stations, **Then** the API returns an empty array (not an error)

---

### User Story 2 - Backend Operator Seeds Test Data (Priority: P2)

An operator runs a seeding script to populate the database with representative test data. The seed produces 5 partners and 50 stations distributed across Tunis, Ariana, Sousse, Sfax, and Gabès with realistic coordinates, varying statuses, and multiple charger configurations.

**Why this priority**: A well-structured seed is required for development, manual testing, and CI integration tests. Without it, every developer must manually create test data.

**Independent Test**: Can be tested by running the seeding script, then querying the API and verifying 50 stations are returned with correct structure (partner snapshots, chargers, is_live flags).

**Acceptance Scenarios**:

1. **Given** an empty database, **When** the seed script runs, **Then** 5 partner records and 50 station records (each with at least 1 charger) are created
2. **Given** seeded data, **When** a staged-mode query (`show_staged=true`) is made from Tunis center, **Then** all 50 stations appear in results
3. **Given** seeded data, **When** a default query (staged mode off) is made, **Then** only stations with `is_live = true` appear

---

### User Story 3 - OSM Importer Enriches Station Data (Priority: P3)

An operator runs an OpenStreetMap import script that extracts real-world charging station nodes from Tunisia's OSM data and inserts them into the stations table, supplementing the manually seeded test data.

**Why this priority**: Real-world data improves map accuracy and demonstrates the pipeline's ability to ingest from open data sources. Lower priority because it relies on external data availability and is not required for core app functionality.

**Independent Test**: Can be tested by running the OSM import against a fresh database, then checking that stations from the real world appear in API results.

**Acceptance Scenarios**:

1. **Given** the OSM import script runs with a valid Tunisia `.osm.pbf` file, **When** the script completes, **Then** any charging station nodes from OSM are inserted into the stations table with their names and coordinates
2. **Given** duplicate detection has been implemented, **When** the import runs twice, **Then** no duplicate station records are created

---

### Edge Cases

- What happens when the database connection is lost during a query? The API should return a 500 error with a log entry, and the mobile app should show its existing "unable to connect" error state with a retry button
- How does the system handle coordinates outside Tunisia? The spatial query returns empty results for areas with no stations within the search radius
- What if the database container is unreachable at startup? The backend should fail fast with a clear error message indicating the connection failure
- How are partner IDs validated? Partner IDs must follow the `prt-` prefix pattern; stations referencing non-existent partners should fail with a referential integrity error at the database level

## Requirements

### Functional Requirements

- **FR-001**: The backend MUST persist station, partner, and charger records in a spatial database so that data survives service restarts
- **FR-002**: The API MUST support querying nearby stations within a configurable distance (default 15 km) from given lat/lng coordinates
- **FR-003**: The API MUST respect the `is_live` flag — stations with `is_live = false` MUST be excluded from default queries unless `show_staged=true` is explicitly provided
- **FR-004**: Each station response MUST include a nested partner object with `id`, `name`, and `type` fields
- **FR-005**: Each station response MUST include a list of chargers with `id`, `plug_type`, `power_output`, and `status`
- **FR-006**: All records MUST use identifiers matching the pattern: partners start with `prt-`, stations with `stn-`, chargers with `chg-`, each followed by 8 hexadecimal characters
- **FR-007**: The mobile app MUST display an orange marker for stations with `is_live = false` to distinguish staged/test data from live stations
- **FR-008**: The mobile app StationCard MUST show a "STAGED TESTING" badge for non-live stations and display partner ownership details
- **FR-009**: A seeder script MUST populate the database with 5 partners and 50 stations across Tunisian regions with realistic data
- **FR-010**: The CI pipeline MUST include a PostGIS database service for backend integration tests
- **FR-011**: A local development environment MUST be provided (e.g., Docker Compose) to run the PostGIS database container
- **FR-012**: An admin API endpoint MUST allow updating station and charger status values manually; operators or automated scripts can call this endpoint to reflect real-world changes
- **FR-013**: The backend MUST expose a health check endpoint (`/health`) that returns database connectivity status
- **FR-014**: The backend MUST log every API request with method, path, status code, and response duration

### Key Entities

- **Partner**: A charging network operator or property owner (e.g., TotalEnergies, Ola Energy). Each partner has a unique `prt-` ID, a name, a classification type (Business or Private), a contact email, and an `is_live` flag for master visibility control
- **Station**: A physical location with one or more EV chargers. Each station has a unique `stn-` ID, belongs to one partner, has a geographic position (latitude/longitude), an operational status, an `is_live` flag for staged visibility, and a last-updated timestamp
- **Charger**: An individual charging connector at a station. Each charger has a unique `chg-` ID, belongs to one station, has a plug type (e.g., CCS2, Type2), a power output in kW, an operational status, and an `is_live` flag

## Success Criteria

### Measurable Outcomes

- **SC-001**: A developer can start the database container, run migrations, seed data, start the backend, and see stations on the map within 5 minutes of following the setup instructions
- **SC-002**: The API returns nearby station results in under 500ms for a dataset of 50 stations within a 15 km search radius
- **SC-003**: 100% of seeded records pass identifier pattern validation (`prt-`, `stn-`, `chg-` with 8 hex chars)
- **SC-004**: All stations not marked `is_live` are hidden from default queries with no false positives
- **SC-005**: The CI pipeline runs all backend tests against a real PostGIS database service without manual setup

## Clarifications

### Session 2026-05-28

- Q: How should the API be secured? → A: Fully open (no auth); security is handled at the network level
- Q: How should station and charger statuses be updated? → A: Manual via admin API endpoint (PATCH) for operators or scripts
- Q: What observability signals should the backend expose? → A: Request logging + a /health endpoint with database connectivity check
- Q: What is the expected production data volume? → A: Hundreds, not thousands (< 500 stations at launch, < 2,000 in first year)
- Q: Should rate limiting be applied to the open API? → A: No rate limiting in v1; architecture should support adding it later via middleware

## Assumptions

- The development team has Docker installed for local database container deployment
- The OSM import script is a one-time extraction tool; ongoing sync from OSM is out of scope for this feature
- The seeding script generates deterministic data so that CI tests produce reproducible results
- Network latency between API service and database is negligible in local/CI environments
- The frontend API client already supports the nested `partner` object structure; if not, it will be adapted
- All `is_live` flags default to `false` for safety — operators must explicitly opt in to make data visible
- The API is fully open (no authentication); network-level controls (firewall, VPN) are expected for production deployment
- Expected data volume is under 500 stations at launch, under 2,000 within the first year; simple GiST indexing is sufficient
- Rate limiting is deferred to a future iteration; the API architecture should allow rate-limiting middleware to be added without restructuring
