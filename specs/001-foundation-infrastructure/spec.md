# Feature Specification: Foundation & Infrastructure Loop

**Feature Branch**: `003-foundation-infrastructure`

**Created**: 2026-05-24

**Status**: Draft

**Input**: User description: "read from docs/plan.md phase 0"

## Clarifications

### Session 2026-05-24

- Q: What level of visual polish should placeholders have? → A: Polished skeleton with design tokens applied (accent green, border-radius, shadows, map tiles fully rendering).
- Q: What should happen when map tile CDN is unreachable? → A: Show a warning banner and fall back to OSM default tiles. CartoDB Positron is primary.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Local Development Environment (Priority: P1)

As a developer on the BorneMap team, I want to run the full stack (backend server
with spatial database) on my local machine so that I can write, test, and verify
code before submitting changes.

**Why this priority**: Every subsequent feature depends on a working local
development loop. Without this, no other team member can contribute code.

**Independent Test**: A new team member can clone the repository, run the setup
commands, and see the backend health endpoint respond successfully within 5
minutes.

**Acceptance Scenarios**:

1. **Given** a clean clone of the repository, **When** the developer runs the
   project setup commands, **Then** the backend application starts and the
   `/health/live` endpoint returns a 200 status.
2. **Given** the backend is running, **When** the developer checks
   `/health/ready`, **Then** the endpoint confirms the spatial database is
   connected.
3. **Given** the backend is running, **When** a developer edits a source file
   and saves, **Then** the server hot-reloads the change within 5 seconds.

---

### User Story 2 — Automated Quality Gates (Priority: P1)

As a project maintainer, I want every pull request to be automatically checked
for code formatting, linting, type safety, and build correctness so that
standards are enforced without manual review overhead.

**Why this priority**: Catching issues at PR time prevents downstream defects,
ensures consistent code quality from day one, and frees reviewers to focus on
logic rather than style.

**Independent Test**: Open a pull request with a deliberate formatting
violation; the CI pipeline fails and reports the specific issue within 5
minutes.

**Acceptance Scenarios**:

1. **Given** a pull request is opened, **When** the CI workflow triggers,
   **Then** it runs linting, formatting checks, type checking, and build steps.
2. **Given** the code passes all quality checks, **When** the CI workflow
   completes, **Then** the PR is marked as passing.
3. **Given** the code has a lint or type error, **When** the CI workflow runs,
   **Then** the error is reported clearly in the workflow output and the PR is
   marked as failing.

---

### User Story 3 — Admin Portal Scaffold (Priority: P2)

As an administrator, I want the web admin portal to render with its base layout
and map tile integration so that the team can start building admin interfaces
on top of a solid visual foundation.

**Why this priority**: The admin portal is the primary tool for managing
station data. Starting with a visual scaffold ensures the UI direction is
established before feature work begins.

**Independent Test**: Navigate to the admin portal URL and see a full-screen
map with CartoDB Positron tiles and floating panel placeholders.

**Acceptance Scenarios**:

1. **Given** the admin portal is running, **When** a user opens it in a
   browser, **Then** the page displays a full-screen map with CartoDB Positron
   tiles.
2. **Given** the map is displayed, **When** the user pans and zooms, **Then**
   the map responds smoothly without errors.
3. **Given** the map is displayed, **When** the user inspects the page, **Then**
   floating panel containers for search, station list, and station form are
   present with design tokens applied (green accents, rounded corners,
   backdrop blur, clean drop-shadows).

---

### User Story 4 — Mobile App Placeholder (Priority: P3)

As a driver, I want to see that the mobile app launches and displays a
responsive screen so that the mobile development workflow is validated and
ready for feature work.

**Why this priority**: The mobile app is a key delivery channel. Confirming
Expo Go can compile and serve a basic screen de-risks the mobile development
pipeline before significant investment.

**Independent Test**: Install the Expo Go app on a physical device, scan the
QR code, and see a responsive placeholder screen with the app shell visible
(status bar area, safe area layout, accent-colored elements).

**Acceptance Scenarios**:

1. **Given** the Expo development server is running, **When** a developer scans
   the QR code with Expo Go on a physical device, **Then** the app loads and
   displays a placeholder message.
2. **Given** the app is running in Expo Go, **When** the developer inspects the
   screen, **Then** the placeholder includes an app shell with safe area
   margins and accent-colored elements matching the design tokens.
3. **Given** the app is running in Expo Go, **When** the developer edits a
   source file and saves, **Then** the app hot-reloads with the change.

---

### Edge Cases

- **Docker unavailable**: If Docker or Docker Compose is not installed, the
  setup script must print a clear error message with installation instructions.
- **Port conflict**: If ports 8000 (backend) or 5432 (database) are already in
  use, the setup must log a conflict warning and suggest alternatives.
- **CI timeout**: If CI workflow exceeds 10 minutes, it should time out
  gracefully and report the timeout in the log.
- **Mobile device not available**: Developers can use the Expo Go iOS/Android
  simulator as fallback for local testing.
- **Map tile CDN unreachable**: If CartoDB Positron tile CDN fails to load,
  the map shows a warning banner and falls back to OpenStreetMap default tiles.
- **Dependency install failure**: If `pnpm install`, `npm install`, `pip install`,
  or Docker image pull fails, the setup process must print the specific error and
  suggest corrective action (network check, disk space, version requirements).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Developers MUST be able to start the backend application with a
  single command and see it respond to health checks.
- **FR-002**: The backend MUST connect to a local spatial database and report
  database connectivity via a readiness endpoint.
- **FR-003**: The backend MUST support hot-reloading so that code changes
  take effect without manual restart.
- **FR-004**: Every pull request MUST trigger an automated CI workflow that
  checks code formatting, linting, type safety, and build correctness.
- **FR-005**: The CI workflow MUST report pass/fail status on the pull
  request within 10 minutes.
- **FR-006**: The admin portal MUST render a full-screen map using CartoDB
  Positron tiles as the base layer, with an OpenStreetMap fallback and a
  visible warning if the primary tile provider is unreachable.
- **FR-007**: The admin portal layout MUST include floating panel containers
  for search, station list, and station form elements styled with the project
  design tokens (green accents, rounded-2xl, backdrop blur, drop-shadows).
- **FR-008**: The mobile app MUST launch in Expo Go on both iOS and Android
  devices and display a responsive placeholder screen.
- **FR-009**: Both the admin portal and mobile app MUST support hot-reload
  during development.

### Key Entities *(include if feature involves data)*

- **No persistent data entities in this phase.** Phase 0 establishes the
  development infrastructure only. Data models begin in Phase 1.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A new developer can go from git clone to a running backend
  health check in under 5 minutes (excluding dependency download time).
- **SC-002**: CI pipeline completes within 10 minutes and reports clear
  pass/fail status.
- **SC-003**: Admin portal loads and displays map tiles within 5 seconds on a
  local development machine.
- **SC-004**: Mobile app placeholder renders on both iOS and Android devices
  via Expo Go within 10 seconds.

## Assumptions

- Developers have Docker and Docker Compose installed on their machines.
- Developers have Node.js 20+ and Python 3.12+ installed locally.
- The project uses GitHub for hosting and GitHub Actions for CI.
- Docker images for PostGIS and Python are publicly available and do not
  require authentication.
- Expo Go is installed on at least one physical device or simulator for
  mobile testing.
- The admin portal will be served via Vite's development server on port 5173.
- The backend will be served via Uvicorn on port 8000 with hot-reload enabled.
