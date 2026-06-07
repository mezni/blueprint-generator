# Research: Driver Web App

**Branch**: `009-driver-web-app` | **Date**: 2026-06-03

## Overview

Research to resolve technical unknowns for the Driver Web App implementation. Covers driver-service API contracts, Keycloak auth integration, Leaflet marker clustering patterns, and React Query best practices for optimistic updates.

---

## 1. Driver-Service API Contracts

### Decision

Use the existing driver-service REST API with standard `SuccessEnvelope`/`ItemEnvelope`/`ErrorEnvelope` response shapes. All endpoints are at `/api/v1/driver/*` behind the Traefik proxy.

### Available Endpoints

| Method | URL Pattern | Auth | Purpose |
|--------|-------------|------|---------|
| GET | `/api/v1/driver/stations` | Optional | List stations by bounding area (lat/lng/radius_km) |
| GET | `/api/v1/driver/stations/{id}` | Optional | Single station detail with chargers |
| GET | `/api/v1/driver/stations/search` | Optional | Search stations by text query |
| POST | `/api/v1/driver/favorites/{id}` | Required | Add favorite (201 Created) |
| DELETE | `/api/v1/driver/favorites/{id}` | Required | Remove favorite |
| GET | `/api/v1/driver/favorites` | Required | List favorite station IDs (`string[]`) |
| POST | `/api/v1/driver/reviews` | Required | Create review |
| PATCH | `/api/v1/driver/reviews/{id}` | Required | Update own review |
| DELETE | `/api/v1/driver/reviews/{id}` | Required | Delete own review (soft) |
| GET | `/api/v1/driver/reviews` | Required | List own reviews (paginated) |
| GET | `/api/v1/driver/me` | Required | Get driver profile |
| PATCH | `/api/v1/driver/me` | Required | Update driver profile |
| POST | `/api/v1/clickstream/events` | Optional | Emit clickstream event (stub) |

### Key Findings

1. **Spatial queries use lat/lng/radius_km**, not bounding box. The `bbox` query parameter is accepted by the route but the repository layer uses `ST_DWithin` with `lat`/`lng`/`radius_km`. The frontend should compute the center of the viewport and use a radius proportional to zoom level.

2. **Search endpoint only implements `q` text search**. The `city`, `connector_type`, and `availability` filter parameters are accepted by the route handler but NOT yet wired into the SQL query. The frontend should pass them as query params for future-proofing, but filtering will only work client-side or be added in a future sprint.

3. **Favorites API returns `string[]` of station IDs**. To display favorited stations on the map, the app needs the full station objects. Approach: map the IDs against already-loaded stations in the React Query cache, or fetch via the list endpoint with radius if needed.

4. **Reviews API is user-scoped**. `GET /api/v1/driver/reviews` returns only the current user's reviews. There is no public endpoint to fetch all reviews for a station. Station detail includes `review_summary` (`average_rating` + `total_reviews`). The app should show the review summary from the station detail, and only let the user see/manage their own review.

5. **Clickstream endpoint is a stub** (Sprint 13). The app should still POST events to `/api/v1/clickstream/events` using `EventEnvelope` format. Failures are silently ignored (fire-and-forget).

6. **Response envelopes**: All success responses use `{ success: true, data, meta }`. Errors use `{ success: false, error: { code, message, details } }`. Canonical error codes include `UNAUTHENTICATED`, `TOKEN_EXPIRED`, `NOT_FOUND`, `ALREADY_EXISTS`, `VALIDATION_FAILED`.

### Alternatives Considered

- **Bounding box queries**: Considered using `bbox` parameter directly, but backend uses radius-based spatial queries. Frontend will compute radius from viewport bounds.
- **Separate API client per domain**: Considered one client per resource (station client, favorite client). Rejected in favor of a single `ApiClient` instance with typed methods, matching the existing pattern in `api-client` package.

---

## 2. Keycloak Auth Integration

### Decision

Use `keycloak-js` library with Authorization Code flow + PKCE, initialized with `onLoad: check-sso` for progressive authentication.

### Configuration

| Setting | Value |
|---------|-------|
| Keycloak URL | `http://localhost/auth` (via Traefik proxy) |
| Realm | `bornemap` |
| Client ID | `bornemap-api` |
| Client type | Public (dev: `bornemap-realm.json`) |
| PKCE method | S256 |
| Token lifespan | 15 minutes (realm config) |

### Integration Pattern

1. **Init**: `keycloak.init({ onLoad: 'check-sso', silentCheckSsoRedirectUri, pkceMethod: 'S256' })` — checks for existing session without redirect
2. **Login**: `keycloak.login()` — redirects to Keycloak login page; returns to SPA after auth
3. **Token**: `keycloak.token` — available after init; auto-refreshed by keycloak-js
4. **Logout**: `keycloak.logout()` — clears Keycloak session
5. **User info**: `keycloak.tokenParsed` — contains `sub` (Keycloak user ID), `email`, `preferred_username`, `realm_access.roles`
6. **Silent refresh**: Requires `public/silent-check-sso.html` in `apps/driver-web/public/`

### Update to `@bornemap/auth-client`

The auth-client package will be updated to:
- Create and initialize a singleton `Keycloak` instance
- `getToken()` — calls `keycloak.updateToken(5)` (min 5s validity) then returns `keycloak.token`
- `login(provider?)` — calls `keycloak.login({ idpHint: provider })` (optional social provider hint)
- `logout()` — calls `keycloak.logout()`
- Expose React context: `AuthProvider` wrapping children with auth state, `useAuth()` hook returning `{ isAuthenticated, user, login, logout, getToken }`

### Alternatives Considered

- **Popup flow**: Considered popup-based login for less disruption. Rejected because popup blockers increasingly block cross-origin popups, and redirect flow is the standard for keycloak-js SPA.
- **Direct Keycloak server call**: Considered calling the OIDC token endpoint directly with PKCE. Rejected because keycloak-js handles token refresh, session management, and iframe-based silent check-sso.
- **Custom JWT handling**: Considered implementing the auth flow manually without keycloak-js. Rejected as unnecessary complexity — keycloak-js is the maintained, standard library for Keycloak SPA integration.

---

## 3. Leaflet Marker Clustering

### Decision

Use vanilla Leaflet (not react-leaflet) with `leaflet.markercluster` plugin for clustered station markers.

### Current State

- MapContainer uses **vanilla Leaflet** (`L.map(...)` in useEffect) — NOT react-leaflet
- Leaflet version: `1.9.4`
- `@types/leaflet`: `^1.9.21`
- No `leaflet.markercluster` or its types installed
- Map exposes `L.Map` instance via `onMount` callback

### Integration Pattern

1. Install `leaflet.markercluster` and `@types/leaflet.markercluster`
2. Import `leaflet.markercluster` CSS (from the package)
3. Create `L.MarkerClusterGroup` and add to map via `map.addLayer(markerClusterGroup)`
4. Add/remove markers by calling `markerClusterGroup.addLayer(marker)` or `clearLayers()`
5. When viewport changes, clear cluster group and re-add markers from new API data
6. Optionally create a `StationMarkers` component that takes the `L.Map` instance and manages cluster lifecycle

### Alternatives Considered

- **react-leaflet**: Considered switching to react-leaflet (v4) for declarative map components. Rejected because the existing MapContainer component uses vanilla Leaflet and works well. Switching would require rewriting the component and re-benchmarking mount performance.
- **Supercluster**: Considered Supercluster (a more modern clustering library). Rejected because leaflet.markercluster is the standard companion for Leaflet, is well-documented, and handles all required clustering behaviors.

---

## 4. React Query Patterns

### Decision

Use `@tanstack/react-query` v5 for all server state management with the following patterns:

- **Queries**: `useQuery` for reads (stations, detail, search, favorites, reviews)
- **Mutations**: `useMutation` with `onMutate` for optimistic updates (favorites toggle)
- **Cache invalidation**: `queryClient.invalidateQueries` after mutations
- **Query keys**: Hierarchical keys like `['stations', 'list', bbox]`, `['stations', 'detail', id]`, `['favorites', userId]`

### Optimistic Update Pattern for Favorites

```typescript
const useFavoriteToggle = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ stationId, isFavorited }: { stationId: string; isFavorited: boolean }) =>
      isFavorited
        ? apiClient.delete(`/api/v1/driver/favorites/${stationId}`)
        : apiClient.post(`/api/v1/driver/favorites/${stationId}`),
    onMutate: async ({ stationId, isFavorited }) => {
      await queryClient.cancelQueries({ queryKey: ['favorites'] });
      const previous = queryClient.getQueryData(['favorites']);
      queryClient.setQueryData(['favorites'], (old: string[] | undefined) =>
        isFavorited
          ? (old ?? []).filter((id) => id !== stationId)
          : [...(old ?? []), stationId]
      );
      return { previous };
    },
    onError: (err, vars, context) => {
      queryClient.setQueryData(['favorites'], context?.previous);
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ['favorites'] });
    },
  });
};
```

### Alternatives Considered

- **Redux Toolkit**: Considered for global state including auth and UI state. Rejected because React Query handles all server state, and auth state can be managed via React Context (auth-client). Redux would add unnecessary boilerplate.
- **Zustand**: Considered lightweight alternative for UI state (search query, panel open/close). Rejected because these are local component states managed by useState/useReducer; they don't need global state management.

---

## 5. React Router Setup

### Decision

Use `react-router` v7 with a simple flat route structure — no nested layouts needed since the app is a single-page map with overlays.

### Route Structure

| Path | Component | Auth |
|------|-----------|------|
| `/` | `MapView` (full-screen map) | None |
| `*` | `NotFound` | None |

The single-route design reflects the map-first UX. All overlays (detail panel, search, auth modal) are conditionally rendered within the `MapView` — they do not change routes.

### Alternatives Considered

- **Hash router**: Considered for simpler deployment without server-side route handling. Rejected in favor of BrowserRouter, which is standard for Vite SPAs with a single catch-all route.
- **No router**: Considered skipping react-router entirely since there's only one page. Rejected because react-router is a declared dependency in the plan, provides `NotFound` handling, and future sprints may add routes (e.g., profile page, settings).

---

## 6. Dependency List for Installation

| Package | Version | Purpose | Install Target |
|---------|---------|---------|---------------|
| `@tanstack/react-query` | ^5.x | Server state management | `apps/driver-web` |
| `react-router` | ^7.x | Client-side routing | `apps/driver-web` |
| `leaflet.markercluster` | ^1.5.x | Map marker clustering | `apps/driver-web` |
| `@types/leaflet.markercluster` | ^1.5.x | Types for marker cluster | `apps/driver-web` (dev) |
| `keycloak-js` | ^26.x | Keycloak auth adapter | `packages/auth-client` |
| `@bornemap/api-client` | workspace | HTTP client with JWT | `apps/driver-web` |
| `@bornemap/auth-client` | workspace | Auth hooks + context | `apps/driver-web` |
| `@bornemap/event-taxonomy` | workspace | Event names + envelope | `apps/driver-web` |
