# Research: Cross-Platform UI Synchronization

## Decisions

### Decision 1: Testing Framework — Jest + React Native Testing Library

- **Decision**: Jest (pre-configured with Expo SDK 54) + React Native Testing Library for component tests. Detox for optional mobile E2E (deferred).
- **Rationale**: Expo SDK 54 ships with Jest pre-configured via `jest-expo`. React Native Testing Library (`@testing-library/react-native`) provides lightweight component-level assertions compatible with both `.native.js` and `.web.js` platform files. Adding a separate E2E framework (Detox, Playwright) is premature until CI demonstrates flakiness or coverage gaps.
- **Alternatives considered**:
  - **Detox**: Full E2E for mobile. High maintenance overhead; no existing CI device farm configured.
  - **Playwright**: Web-only E2E. Not relevant for the native mobile target.
  - **No tests**: Current state. Adds regression risk for a feature spanning two platforms.

### Decision 2: Navigation Architecture — React Navigation (Stack + BottomTab)

- **Decision**: React Navigation v7 with a bottom tab navigator wrapping the existing `MapScreen` plus placeholder screens for Explore, Saved, Profile. On desktop web, a custom `NavBar` component (CSS-based horizontal bar) synchronizes with the same navigation state.
- **Rationale**: React Navigation is the de facto standard for React Native navigation and integrates with Expo. The bottom tabs map 1:1 to the FR-002 requirement. For desktop web, wrapping navigation state in a shared context lets both the mobile BottomTabBar and desktop NavBar read/write the same state.
- **Alternatives considered**:
  - **Expo Router**: File-based routing; less flexible for tab bar customization.
  - **Single-screen with internal state**: Avoids navigation library dependency but makes it harder to add screen-level code-splitting later.

### Decision 3: Filter Sync Storage — Server-side session store (in-memory, api-service)

- **Decision**: The `PUT /api/v1/filters` endpoint stores filter state in a per-session in-memory `HashMap<SessionId, FilterState>` within `api-service` (behind `web::Data<Mutex<HashMap>>`). No database persistence — filters are ephemeral and not needed across service restarts.
- **Rationale**: Filter state is transient (no historical value), low cardinality (one entry per active session), and does not require durability. An in-memory store avoids a new database dependency (constitution Principle I — no Redis until validated). Poll-based sync (60s interval) is tolerant of service restarts (client re-fetches on next poll).
- **Alternatives considered**:
  - **PostgreSQL table**: Overkill for ephemeral filter state. Adds schema migration overhead.
  - **Redis**: Barred by Principle I (Validation Before Optimization).
  - **Client-side only**: No cross-platform sync; violates SC-006.

### Decision 4: Clickstream Integration — Reuse existing `/api/v1/analytics/connect` endpoint

- **Decision**: New UI interaction events (marker_tap, search_submit, filter_change, zoom_in, zoom_out, locate_me) are sent to the existing `POST /api/v1/analytics/connect` endpoint. The event payload is mapped from the new `ClickstreamEvent` schema to the existing `AnalyticsEvent` shape at the client layer.
- **Rationale**: Avoids creating a second analytics ingest path. The existing pipeline (api-service → RabbitMQ → analytics-service → MongoDB) already handles asynchronous event ingestion. Adding new event types does not change the infrastructure.
- **Alternatives considered**:
  - **New analytics endpoint**: Unnecessary duplication — the existing pipeline is generic enough.
  - **Client-side batch-and-flush**: Adds complexity (offline queue, retry logic) with no current requirement for offline analytics.

### Decision 5: Chart / Icon Assets — Inline SVG (desktop) + Vector icons (mobile)

- **Decision**: Desktop web NavBar uses inline SVG icons (no external icon library dependency). Mobile BottomTabBar uses `@expo/vector-icons` (already available in Expo SDK 54).
- **Rationale**: Desktop web already bundles Leaflet; adding an icon library adds bundle weight. Inline SVGs are lightweight and tree-shakable. Mobile already has `@expo/vector-icons` from the Expo SDK — no new dependency.
- **Alternatives considered**:
  - **lucide-react** (desktop): Adds ~30KB min+gzip. Not justified for 4 nav icons.
  - **react-native-vector-icons** (mobile): Already included transitively via Expo; no explicit install needed.

## Compliance Review

| Constitution Rule | Status | Notes |
|---|---|---|
| Principle I: Validation Before Optimization | Compliant | No new infrastructure added. Filter sync uses in-memory store in existing api-service. |
| Principle II: Stack LOCKED | Compliant | Uses existing React Native / Expo stack. No new languages or frameworks. |
| Principle III: API & Service Architecture | Compliant | New endpoints (`/api/v1/search`, `/api/v1/stations/{id}`, `/api/v1/filters`) follow `/api/v1` prefix convention. |
| Principle III: Service Isolation | Compliant | New routes added to existing `api-service` domain modules (locate module extended with search/filter routes). |
| Principle IV: nanouuid IDs | Compliant | Station IDs already use `stn-` prefix. New entities (events) use existing patterns. |
| Principle V: Docker Compose | Compliant | No new infrastructure services required. |
| Principle V: /spec and /docs sync | Compliant | All artifacts under `specs/006-cross-platform-ui/`. |
