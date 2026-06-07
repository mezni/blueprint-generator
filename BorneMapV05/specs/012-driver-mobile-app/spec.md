# Feature Specification: Driver Mobile App

**Feature Branch**: `012-driver-mobile-app`

**Created**: 2026-06-04

**Status**: Draft

**Input**: User description: "sprint 12 - Mobile App (Expo)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Map Discovery (Priority: P1)

A registered driver opens the mobile app and sees stations nearby on a map, with distance indicators and ability to filter by connector type and availability.

**Why this priority**: Core value proposition - discovery is the primary use case for the app. Without map discovery, users cannot find charging stations.

**Independent Test**: Open the app on a device with GPS enabled, navigate to a location, and verify that stations appear on the map with accurate distances and available filter options.

**Acceptance Scenarios**:

1. **Given** the user has location services enabled, **When** the app loads, **Then** the user sees a map centered on their current location showing nearby charging stations within the default radius
2. **Given** the user is viewing the map, **When** they pan/zoom, **Then** the map updates to show stations in the visible area
3. **Given** the user is on the map, **When** they select a station marker, **Then** the station details panel opens showing name, charger types, availability, and distance

---

### User Story 2 - Station Details (Priority: P1)

A user taps on a station marker to view detailed information including name, description, charger specifications, real-time availability, and nearby reviews.

**Why this priority**: Information discovery is essential for making charging decisions. Users need to see charger types, power output, and availability before visiting.

**Independent Test**: Navigate to a station on the map, tap the marker, and verify all station information displays correctly with accurate availability status.

**Acceptance Scenarios**:

1. **Given** a station is selected, **When** the details panel opens, **Then** the user sees the station name, description, address, and list of chargers with types and power
2. **Given** a station has chargers, **When** the user scrolls to the chargers section, **Then** each charger shows its type (CCS/Type2/CHAdeMO), power output (kW), and current status (available/offline/fault)
3. **Given** a station has reviews, **When** the user scrolls to the reviews section, **Then** they see review ratings (1-5 stars), comments, and review counts
4. **Given** a station has availability data, **When** the user views the station, **Then** they see real-time availability status (available/limited/unavailable) with source indicator

---

### User Story 3 - Favorites Management (Priority: P2)

A registered driver can mark stations as favorites for quick access and remove them when no longer needed.

**Why this priority**: Favorites improve UX by reducing search time for frequently visited stations, but discovery is still the primary use case.

**Independent Test**: Login as a registered driver, add a station to favorites, navigate away, return to the map, and verify the favorite appears in the favorites list.

**Acceptance Scenarios**:

1. **Given** a driver is viewing a station details, **When** they tap the favorite button, **Then** the station is added to their favorites with a confirmation visual
2. **Given** a station is in favorites, **When** the user taps the favorite button again, **Then** the station is removed from favorites
3. **Given** the driver is in the favorites view, **When** they tap a favorite station, **Then** the station details panel opens with that station selected

---

### User Story 4 - Reviews & Ratings (Priority: P2)

A registered driver can submit reviews for stations they have visited and view reviews from other users.

**Why this priority**: Reviews help users make informed decisions and build trust in the platform. However, discovery remains the primary feature.

**Independent Test**: Login as a registered driver, visit a station, submit a review with rating and comment, then navigate to the station details to view the submitted review.

**Acceptance Scenarios**:

1. **Given** a driver is viewing station details, **When** they scroll to the reviews section, **Then** they see existing reviews with ratings, comments, and timestamps
2. **Given** a registered driver, **When** they submit a review with rating (1-5) and optional comment, **Then** the review is saved and immediately visible
3. **Given** a review has been submitted, **When** the driver returns to the station details, **Then** their review appears at the top of the reviews list

---

### User Story 5 - Login Flow (Priority: P1)

A user can authenticate with Keycloak via OAuth2, supporting both password-based login and social login providers (Google/Facebook).

**Why this priority**: Authentication is mandatory for favorite and review functionality. Without login, users cannot personalize their experience.

**Independent Test**: Navigate to a gated action (favorites/reviews) without login, verify login modal appears, complete authentication, and return to the previous action.

**Acceptance Scenarios**:

1. **Given** a user is not logged in, **When** they attempt a gated action, **Then** a login modal opens prompting for credentials
2. **Given** the login modal is open, **When** the user provides valid credentials and submits, **Then** they are authenticated and the modal closes
3. **Given** the login modal is open, **When** the user clicks a social login provider (Google/Facebook), **Then** they are redirected to the OAuth provider
4. **Given** a user has completed OAuth login, **When** they return to the app, **Then** they remain authenticated and can access gated features

---

### User Story 6 - RTL Support (Priority: P3)

The mobile app supports Right-to-Left (RTL) languages, primarily Arabic, with full layout flipping and proper text direction.

**Why this priority**: RTL support is important for regional accessibility but discovery and authentication remain priority. This can be built incrementally.

**Independent Test**: Switch the app to Arabic language, verify all UI elements flip layout direction, text renders correctly, and interactive elements remain accessible.

**Acceptance Scenarios**:

1. **Given** the app language is set to Arabic, **When** the user navigates through the app, **Then** all UI layouts flip from LTR to RTL
2. **Given** RTL layout is active, **When** the user performs actions, **Then** touch targets, spacing, and content alignment remain consistent
3. **Given** Arabic text is displayed, **When** the user reads content, **Then** text direction is correct and no content is mirrored

---

### User Story 7 - Offline-Safe UI (Priority: P3)

The mobile app maintains a functional UI even when network connectivity is unavailable, with appropriate loading states and error handling.

**Why this priority**: Offline capability improves UX in areas with poor connectivity but is not critical for MVP. Can be implemented as an enhancement.

**Independent Test**: Enable airplane mode in the app, navigate through different screens, verify appropriate offline UI states, and test reconnecting to network.

**Acceptance Scenarios**:

1. **Given** the app is online, **When** the user performs an action, **Then** the action completes successfully
2. **Given** the user has network connectivity, **When** it is lost, **Then** the app switches to offline mode with appropriate messaging
3. **Given** the app is offline, **When** the user attempts an action requiring network, **Then** they receive a clear error message explaining the issue
4. **Given** the app is offline, **When** network is restored, **Then** the app automatically syncs queued actions and refreshes data

---

### Edge Cases

- What happens when GPS location is denied by the user? The app should show a clear prompt explaining why location is needed and provide a way to enable it in device settings
- How does the system handle network timeouts? The app should show retry options with appropriate error messages and prevent duplicate requests
- What happens when a station is deleted while the user has it favorited? The favorite should be automatically removed with a notification
- How does the app handle different screen sizes (mobile, tablet)? The app should be responsive and work across different viewport sizes
- What happens when the user rapidly pans/zooms the map? The app should implement debouncing (300-500ms) to prevent excessive API calls
- How does the app handle out-of-order GPS updates? The app should use the most recent location and provide visual feedback when location updates

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support map-based station discovery with filters for connector type and availability
- **FR-002**: System MUST display station distance from user location with accuracy to 0.1 km
- **FR-003**: System MUST show real-time availability status (available/limited/unavailable) for each station
- **FR-004**: System MUST allow registered drivers to mark stations as favorites with one-tap interaction
- **FR-005**: System MUST allow registered drivers to submit reviews with ratings (1-5) and optional comments
- **FR-006**: System MUST enforce that each user can submit only one review per station
- **FR-007**: System MUST authenticate users via Keycloak OAuth2 with support for Google and Facebook providers
- **FR-008**: System MUST maintain user session across app restarts with:
  - Automatic token refresh 5 minutes before expiration
  - Session persistence using encrypted Secure Store
  - Automatic logout after 7 days of inactivity
  - Ability to manually sign out from profile page
- **FR-009**: System MUST support Right-to-Left layout when language is set to Arabic
- **FR-010**: System MUST preserve UI state when network connectivity is lost and restored
- **FR-011**: System MUST implement map viewport debouncing (300-500ms) to prevent excessive API calls
- **FR-012**: System MUST show appropriate error messages for network failures and timeouts
- **FR-013**: System MUST hide GPS location prompts and only ask for permission when necessary
- **FR-014**: System MUST remove favorites for soft-deleted stations automatically
- **FR-015**: System MUST validate all form inputs (reviews, filters) before submission

### Key Entities

- **Station**: Physical charging station location with name, description, coordinates, charger types, power output, and real-time availability status
- **Charger**: Individual charging unit within a station with type (CCS/Type2/CHAdeMO), power rating (kW), and operational status
- **Favorite**: User's saved station with user_id and station_id, linked 1:1 per user-station combination
- **Review**: User-generated station evaluation with rating (1-5), comment, status (published/hidden/flagged/deleted), and timestamp
- **User Account**: Authentication identity linked to Keycloak with role (registered_driver/partner/admin)
- **Session**: Authenticated user session maintained through Keycloak with automatic token refresh

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can complete station discovery and details view within 10 seconds of app launch
- **SC-002**: Map interactions (pan/zoom) complete within 300ms on 4G connections
- **SC-003**: 95% of favorite and review actions complete successfully without requiring re-authentication
- **SC-004**: App maintains functional UI with appropriate states when network connectivity is interrupted
- **SC-005**: RTL layout conversion for Arabic completes in under 100ms with no layout breakage
- **SC-006**: 90% of stations shown on map have accurate availability data
- **SC-007**: Map viewport updates trigger no more than 2 API calls per 1 second of user interaction
- **SC-008**: Users can submit reviews within 30 seconds of selecting a station
- **SC-009**: System supports 10,000 concurrent active users on mobile platform
- **SC-010**: System processes 50 events/second baseline from mobile app traffic
- **SC-011**: System architecture designed for 3x growth to 30,000 concurrent users in next version
- **SC-012**: All API calls include unique request IDs for end-to-end tracing
- **SC-013**: App launches successfully and reports metrics within 30 seconds
- **SC-014**: Critical errors are logged with context and reported to remote logging service
- **SC-015**: Health check endpoint returns app state within 200ms
- **SC-016**: User data encrypted at rest with AES-256 for all local storage
- **SC-017**: 100% of sensitive API responses encrypted during transmission
- **SC-018**: App implements biometric/PIN lock with 5 failed attempts timeout
- **SC-019**: Privacy policy shown on first launch and confirmed by user
- **SC-020**: Data retention controls accessible from app settings

## Clarifications

### Session 2026-06-04

- Q: What are the target performance metrics for concurrent users and event throughput on the mobile platform? → A: MVP scale: 10,000 concurrent active users, 50 events/sec baseline, designed for 3x growth to 30,000 concurrent users in next version
- Q: What observability requirements should the mobile app have? → A: Full observability: Structured JSON logs with request IDs, client-side performance metrics (app launch time, map interaction latency), error telemetry to remote logging service, and health check endpoint exposing app state
- Q: What security and privacy requirements should the mobile app implement? → A: Comprehensive security with privacy protection: all sensitive data encrypted at rest (network and storage), PIN/biometric lock for app access, password manager integration prevention, clear privacy policy modal on first launch, and GDPR-compliant data retention settings

## Assumptions

- Target users have mobile devices with GPS capabilities and modern web browsers
- Users may have intermittent network connectivity in the coverage area
- Users will primarily use the app in Tunisia where the base layer is available
- Map center defaults to Tunis coordinates (36.8065, 10.1815) with default radius of 10km
- Max map radius for discovery is 50km
- RTL support is primarily for Arabic language with potential for French RTL layout
- Offline mode will cache previously loaded stations and map tiles
- Social login providers (Google/Facebook) are configured in Keycloak realm
- Station availability data comes from partner updates or system sync, not real-time sensor data
- Review submissions are immediately visible to the user who submitted them
- Favorites are not synced across devices - each device maintains its own favorites
- Map interactions should be debounced to prevent excessive API calls
- All API calls use the `/api/v1/driver/*` endpoints from the driver-service
- The app reuses existing `shared-types`, `api-client`, and `auth-client` packages
- The app uses Expo for React Native development
- Offline-safe UI means preserving the current view with loading indicators, not preventing navigation
