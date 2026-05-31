# Feature Specification: Mobile Canvas

**Feature Branch**: `004-mobile-canvas`

**Created**: 2026-05-28

**Status**: Draft

**Input**: User description: "enhance interface Mobile Canvas layout"

## Clarifications

### Session 2026-05-28

- Q: Which areas are explicitly in scope for this feature? → A: Schema rename + frontend documentation alignment (FR-001–FR-009). CI pipeline is documented but not modified.
- Q: How should the ENUM rename be handled given existing data? → A: Destructive migration — drop and recreate schema, then reseed demo data (since data is test/development only).
- Q: How should the map screen handle loading and error states? → A: Loading spinner during fetch, persistent error banner with retry button on failure.
- Q: What are the valid status values and transition rules for stations and chargers? → A: Extended set: Available, Occupied, Offline, Maintenance. Transitions: Available ↔ Occupied, Available → Offline → Available, Available → Maintenance → Available.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Architecture Documentation Alignment (Priority: P1)

As a developer onboarding to the project, I want a clearly documented project tree that reflects the actual codebase structure, so I can understand the system boundaries and locate components quickly.

**Why this priority**: Navigation is the first barrier to contribution; undocumented structure causes confusion and errors during development.

**Independent Test**: Can be fully tested by verifying the documented directory tree matches every file and directory in the repository, and delivers a reliable map of the system.

**Acceptance Scenarios**:

1. **Given** the project documentation, **When** a developer reads the tree, **Then** every directory and file listed must correspond to an actual path in the repository.
2. **Given** the documented tree, **When** a new developer navigates to any listed path, **Then** the expected files must exist at that location.

---

### User Story 2 - Consistent Partner Classification Naming (Priority: P1)

As a database administrator, I want the partner type enum renamed from `partner_type` to `partner_classification` so the naming clearly communicates that it describes a classification category rather than a generic type.

**Why this priority**: Ambiguous naming in the schema leads to confusion about the enum's purpose; this is a renaming fix that must be consistent across all layers.

**Independent Test**: Can be fully tested by running the migration and verifying the enum name `partner_classification` exists with values `'Private'` and `'Business'`, and no references to `partner_type` remain in the schema.

**Acceptance Scenarios**:

1. **Given** the migration has been applied, **When** I query `SELECT typname FROM pg_type WHERE typname = 'partner_classification'`, **Then** the enum type must exist.
2. **Given** the migration, **When** I query the `partners` table schema, **Then** the `type` column must use `partner_classification` as its type.
3. **Given** the migration, **When** I check for references to `partner_type`, **Then** none should exist in the database schema.

---

### User Story 3 - Identifier Contract Enforcement (Priority: P2)

As a backend engineer, I want the shared data contracts for primary identifiers explicitly documented and enforced, so every record type follows a consistent naming pattern.

**Why this priority**: Consistent identifiers prevent integration errors and make debugging easier; this is enforced at the database level via CHECK constraints.

**Independent Test**: Can be fully tested by verifying the CHECK constraints on each table enforce the documented regex patterns.

**Acceptance Scenarios**:

1. **Given** the `partners` table, **When** I insert a row with an ID that does not match `^prt-[a-f0-9]{8}$`, **Then** the insert must be rejected.
2. **Given** the `stations` table, **When** I insert a row with an ID that does not match `^stn-[a-f0-9]{8}$`, **Then** the insert must be rejected.
3. **Given** the `chargers` table, **When** I insert a row with an ID that does not match `^chg-[a-f0-9]{8}$`, **Then** the insert must be rejected.

---

### User Story 4 - Frontend Map and Station Card UI (Priority: P2)

As a mobile driver, I want to see charging stations plotted on a map with a detail sheet that shows charger-level information, so I can find and evaluate available charging spots.

**Why this priority**: The map and detail views are the primary user-facing interface; without them the API data has no consumer value.

**Independent Test**: Can be fully tested by loading the app, verifying stations appear as markers on the map, and tapping a marker to display its charger details in the bottom sheet.

**Acceptance Scenarios**:

1. **Given** the app is loaded, **When** the map renders, **Then** station markers must appear at their geographic coordinates.
2. **Given** a station marker is visible, **When** the user taps it, **Then** a bottom sheet must slide up showing the station name, partner, status, and charger list.
3. **Given** a charger has status "Available", **When** it is displayed in the sheet, **Then** it must show a green "Available" indicator.

---

### Edge Cases

- What happens when the database has no stations within the search radius? The map should render with no markers and the bottom sheet should be hidden.
- How does the system handle a station with no chargers? The detail sheet should show a station header with an empty charger list and a message indicating no chargers are available.
- What happens when the PostGIS extension is not installed? The migration must fail with a clear error message.
- How does the map screen behave during API loading? A loading spinner is displayed centered on the map while the fetch is in progress.
- How does the map screen handle a network or API error? A persistent error banner appears at the top of the screen with a retry button; no station markers are shown until the request succeeds.

### Out of Scope

- CI pipeline implementation changes (pipeline configuration is documented and assumed to exist, but not modified by this feature)
- Authentication and authorization mechanisms (deferred to future iteration)
- Server-side API changes beyond schema alignment (existing API contract remains unchanged)
- Mobile platform native builds (validated via Expo web export proxy)
- Data migration scripts for existing production databases (schema rename handled via new migration)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a documented project directory tree that matches the actual repository structure.
- **FR-002**: Database migration MUST define a `partner_classification` ENUM type with values `'Private'` and `'Business'`.
- **FR-003**: Database migration MUST use `partner_classification` as the column type for `partners.type`.
- **FR-004**: Database migration MUST enforce CHECK constraints on `partners.id` matching `^prt-[a-f0-9]{8}$`.
- **FR-005**: Database migration MUST enforce CHECK constraints on `stations.id` matching `^stn-[a-f0-9]{8}$`.
- **FR-006**: Database migration MUST enforce CHECK constraints on `chargers.id` matching `^chg-[a-f0-9]{8}$`.
- **FR-007**: The frontend map view MUST render station markers at coordinates returned by the nearby stations API.
- **FR-008**: The frontend detail sheet MUST display station name, partner, status, and charger list when a marker is tapped.
- **FR-009**: The frontend MUST render charger-level information including plug type, power output, and availability status.
- **FR-010**: The CI pipeline configuration at `.github/workflows/ci.yml` is documented in the project tree; pipeline behavior is verified indirectly via T027 (cargo build) and T028 (expo export).

### Key Entities *(include if feature involves data)*

- **Partner**: An organization that operates charging stations. Identified by a `prt-` prefixed nanouuid. Has a classification of `Private` or `Business`.
- **Station**: A physical EV charging location owned by a Partner. Identified by a `stn-` prefixed nanouuid. Has a geographic point location stored as PostGIS GEOGRAPHY(Point, 4326). Status can be: Available, Occupied, Offline, or Maintenance.
- **Charger**: An individual charging unit at a Station. Identified by a `chg-` prefixed nanouuid. Has a plug type, power output in kW, and an availability status (same lifecycle as Station).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A new developer can understand the system architecture within 15 minutes of reading the documented directory tree and data contracts.
- **SC-002**: The `partner_classification` enum migration applies cleanly with zero errors.
- **SC-003**: All three identifier patterns are enforced at the database level, rejecting any non-conforming IDs.
- **SC-004**: The map renders with station markers within 3 seconds of app load on a standard network connection.
- **SC-005**: The CI pipeline completes both backend compilation and frontend web export within 5 minutes.
- **SC-006**: 100% of frontend test scenarios (marker tap, charger display, empty state) pass without errors.

## Assumptions

- The database migration is managed via SQL scripts applied manually or through a migration runner (not auto-migration).
- The frontend app has a stable internet connection and can reach the API server.
- "Standard network connection" means WiFi with ≥10 Mbps download throughput, measured via browser performance API.
- PostGIS extension is already available in the target PostgreSQL environment.
- The existing API contract (`GET /api/v1/stations/nearby`) remains the data source for the frontend map view.
- Mobile platform (iOS/Android) builds are validated via Expo's web export as a proxy for cross-platform compatibility.
- The CI pipeline uses GitHub Actions with the `postgis/postgis:15-3.3` service container.
