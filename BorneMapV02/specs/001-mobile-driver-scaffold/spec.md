# Feature Specification: Mobile Driver App Scaffold

**Feature Branch**: `001-mobile-driver-scaffold`

**Created**: 2026-05-27

**Status**: Draft

**Input**: User description: "Scaffold minimal directory architecture for BorneMap mobile driver app with offline map engine, CI/CD pipeline, and troubleshooting workflow"

## Clarifications

### Session 2026-05-27

- Q: How should the map behave when default provider tiles are unavailable offline? → A: Silent grey/empty area — no error surfaced; viewport and marker remain functional
- Q: Should the map viewport allow free pan/zoom or be locked to the Tunis center? → A: Interactive (default) — full pan, zoom, and gesture support enabled
- Q: What should happen if the map component fails to initialize entirely? → A: Error fallback screen — catch the error, show a text fallback with error description, debug overlay remains visible

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Project Scaffolding & CI Pipeline (Priority: P1)

A developer initializes the project repository, sets up the standard directory layout, and configures a CI pipeline that validates the frontend builds without errors on every push to main or develop.

**Why this priority**: Project structure and CI must exist before any code can be safely contributed. This is the foundation that enables all future development.

**Independent Test**: Clone the repo on a fresh machine, run `npm ci && npx expo export --platform web`, and verify the build succeeds with zero errors.

**Acceptance Scenarios**:

1. **Given** an empty repository, **When** the developer runs `mkdir -p apps/mobile-driver` and creates the standard layout, **Then** the directory structure matches `.github/workflows/ci.yml`, `apps/mobile-driver/App.js`, and `apps/mobile-driver/src/screens/`
2. **Given** a CI pipeline configuration, **When** a push is made to `main` or `develop`, **Then** GitHub Actions triggers the `frontend-test` job
3. **Given** a CI pipeline run, **When** `npx expo export --platform web` executes, **Then** the build completes with zero warnings treated as errors

---

### User Story 2 - Offline Map Baseline (Priority: P2)

A driver opens the mobile app and sees a map centered on Tunis, Tunisia, with a marker at the core baseline coordinate. The map renders entirely offline with no network requests or backend dependency.

**Why this priority**: The offline map is the core visual output of the mobile driver app. Rendering it without network dependencies validates the UI isolation principle and provides a stable diagnostic baseline.

**Independent Test**: Launch the app with airplane mode enabled and confirm the map viewport renders centered over Tunis with the debug overlay visible.

**Acceptance Scenarios**:

1. **Given** the app is launched, **When** the map screen loads, **Then** the initial region centers on latitude `36.8065` and longitude `10.1815` with appropriate zoom deltas
2. **Given** no network connectivity, **When** the map renders, **Then** no network requests are made and no error boundaries are triggered
3. **Given** a marker at the Tunis coordinate, **When** the map viewport loads, **Then** the marker displays with title "Tunis Core Baseline" and a description referencing Phase 1

---

### User Story 3 - Debug Diagnostics Overlay (Priority: P3)

A developer troubleshooting mobile client rendering sees a persistent debug overlay indicating sandbox mode, confirming the app is running in its offline isolation state without relying on backend services.

**Why this priority**: The debug overlay accelerates UI diagnostics by providing immediate visual feedback about the app runtime mode, reducing time spent checking network adapter configurations.

**Independent Test**: Launch the app and visually confirm a white overlay card in the top-left portion of the screen shows "BorneMap Sandbox Mode" and "Tunisia Map Layer Rendered Offline".

**Acceptance Scenarios**:

1. **Given** the app is running, **When** the map screen renders, **Then** a debug overlay with a white background is positioned at the top of the screen
2. **Given** the debug overlay is visible, **When** a developer reads its content, **Then** it displays "BorneMap Sandbox Mode" as bold text and "Tunisia Map Layer Rendered Offline" as subtitle text
3. **Given** the app transitions to any state, **When** the overlay remains, **Then** it stays non-interactive and does not block map gestures

---

### Edge Cases

- **No network at first launch**: Map tiles may not render from remote tile servers if using default provider — the app must show a silent grey/empty tile area without crash or error UI; the viewport and marker remain functional
- **Multiple fast relaunches**: Rapid app restarts (start/stop/start) must not cause map rendering artifacts or memory leaks
- **Device rotation**: Map viewport must correctly handle screen orientation changes without losing the centered Tunis coordinate
- **Expo Go sandbox limits**: Certain native modules may not be available in Expo Go — failure to load must surface a descriptive message rather than a silent crash
- **Map component initialization failure**: If the map component fails entirely (missing API key, missing Google Play Services), the app MUST display a text fallback screen with an error description; the debug overlay MUST remain visible

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Project repository MUST follow the directory layout: `.github/workflows/ci.yml`, `apps/mobile-driver/App.js`, `apps/mobile-driver/src/screens/MapScreen.js`
- **FR-002**: CI pipeline MUST validate the mobile driver build on every push and pull request targeting `main` or `develop` branches
- **FR-003**: Mobile app MUST render a map viewport centered on Tunis, Tunisia (latitude `36.8065`, longitude `10.1815`) with initial zoom deltas of `0.12` and `0.06`; the viewport MUST support full pan, zoom, and gesture interaction
- **FR-004**: Map viewport MUST render without making network requests or depending on any backend service
- **FR-005**: A marker MUST be placed at the Tunis center coordinate with title "Tunis Core Baseline" and an informative description
- **FR-006**: A debug overlay MUST display "BorneMap Sandbox Mode" prominently at the top of the screen when the map is visible
- **FR-007**: A subtitle line "Tunisia Map Layer Rendered Offline" MUST appear below the sandbox mode label
- **FR-008**: The debug overlay MUST have a semi-transparent white background, rounded corners, and drop shadow for visibility over map tiles
- **FR-009**: Application MUST launch and render without a backend or API service running
- **FR-010**: If the map component fails to initialize, the app MUST display a fallback screen with an error description and MUST keep the debug overlay visible; the app MUST NOT crash

### Key Entities

- **Map Viewport**: Represents the geographic window centered on Tunis with configurable latitude/longitude deltas for zoom control; supports full pan, zoom, and gesture interaction
- **Map Marker**: A pinned location at the Tunis core coordinate displaying title and description metadata
- **Debug Overlay**: A heads-up display element showing sandbox mode status for diagnostic purposes

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: CI pipeline completes the build verification step in under 3 minutes from trigger on a standard GitHub Actions runner
- **SC-002**: Map viewport renders the Tunis center coordinate on first launch within 5 seconds of app start
- **SC-003**: Application launches and renders the map without any network requests observable in network debugging tools
- **SC-004**: All three user stories are independently testable without any backend infrastructure
- **SC-005**: A developer can complete a clean install (`rm -rf node_modules .expo && npm install && npm run start:tunnel`) and see the map within 2 minutes of starting the install

## Assumptions

- The mobile app targets Expo SDK 51 with React Native 0.74.1 as specified in the locked technology stack
- Node.js v24.16.0 and npm v11.13.0 are available on the CI runner and developer machine
- Map tile rendering uses the default provider (Apple Maps on iOS, Google Maps on Android) — offline behavior depends on the platform's pre-cached tiles
- The `.github/workflows/ci.yml` CI pipeline does not require a backend environment to validate the frontend build
- Expo Go on a physical device is the primary testing target; iOS Simulator and Android Emulator are secondary
