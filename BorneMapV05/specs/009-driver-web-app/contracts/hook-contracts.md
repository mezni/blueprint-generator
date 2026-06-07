# Hook Contracts: Driver Web App

**Branch**: `009-driver-web-app` | **Date**: 2026-06-03

## Overview

React Query hook contracts for all data fetching and mutations. Each hook uses `@bornemap/api-client` for HTTP calls and follows standard React Query v5 patterns.

---

## Station Hooks

### `useStationMarkers`

```typescript
interface StationListParams {
  lat: number;
  lng: number;
  radiusKm: number;
  connectorType?: ConnectorType;
  availability?: StationAvailability;
}

function useStationMarkers(params: StationListParams): {
  stations: StationListItem[];
  isLoading: boolean;
  isError: boolean;
  error: Error | null;
}
```

**Query key**: `['stations', 'list', params]`

**Behavior**: Fetches stations within radius of center point. Re-fetches when params change (debounced via 500ms viewport throttle). Cancels in-flight requests on new query.

**API call**: `GET /api/v1/driver/stations` with query params.

---

### `useStationDetail`

```typescript
function useStationDetail(stationId: string | null): {
  station: StationDetail | undefined;
  isLoading: boolean;
  isError: boolean;
  error: Error | null;
  refetch: () => void;
}
```

**Query key**: `['stations', 'detail', stationId]`

**Behavior**: Fetches single station detail. Disabled when `stationId` is null (no selection). Caches for session duration. Provides `refetch` for retry on error.

**API call**: `GET /api/v1/driver/stations/{id}`

---

### `useSearch`

```typescript
function useSearch(query: SearchQuery): {
  results: StationListItem[];
  isLoading: boolean;
  isError: boolean;
  error: Error | null;
  totalResults: number;
}
```

**Query key**: `['stations', 'search', query]`

**Behavior**: Debounced (300ms) text search. Cancels in-flight requests on new query. Returns empty array when query is empty.

**API call**: `GET /api/v1/driver/stations/search`

**Note**: Only `q` (text) parameter is wired in backend; `city`, `connector_type`, `availability` are accepted but not yet filtered. Pass them for future-proofing.

---

## Favorite Hooks

### `useFavorites`

```typescript
function useFavorites(): {
  favoriteIds: string[];
  isLoading: boolean;
  isError: boolean;
}
```

**Query key**: `['favorites', 'list', userId]`

**Behavior**: Returns array of favorited station IDs. Only available when authenticated. Returns empty array when anonymous.

**API call**: `GET /api/v1/driver/favorites`

---

### `useFavoriteToggle`

```typescript
function useFavoriteToggle(): {
  toggle: (stationId: string, isCurrentlyFavorited: boolean) => void;
  isPending: boolean;
}
```

**Behavior**: Optimistic mutation. Updates local cache immediately, rolls back on error. Shows pending state during API call.

**API calls**:
- Add: `POST /api/v1/driver/favorites/{id}`
- Remove: `DELETE /api/v1/driver/favorites/{id}`

---

## Review Hooks

### `useReviews`

```typescript
function useReviews(stationId: string): {
  userReview: Review | null;
  isLoading: boolean;
  isError: boolean;
}
```

**Query key**: `['reviews', 'user']`

**Behavior**: Fetches current user's reviews and filters for the given `stationId`. Returns `null` if user hasn't reviewed this station.

**API call**: `GET /api/v1/driver/reviews` (user-scoped, returns all user reviews)

---

### `useReviewMutation`

```typescript
function useReviewMutation(): {
  create: (data: ReviewCreate) => Promise<Review>;
  update: (id: string, data: ReviewUpdate) => Promise<Review>;
  remove: (id: string) => Promise<void>;
  isPending: boolean;
}
```

**Behavior**: Create, update, and delete reviews. Invalidates `['reviews']` cache on success.

**API calls**:
- Create: `POST /api/v1/driver/reviews`
- Update: `PATCH /api/v1/driver/reviews/{id}`
- Delete: `DELETE /api/v1/driver/reviews/{id}`

---

## Auth Hooks

### `useAuth`

```typescript
function useAuth(): AuthState & {
  isGated: boolean;              // True if auth modal should be shown
  setGatedAction: (action: () => Promise<void>) => void;
  executeGatedAction: <T>(action: () => Promise<T>) => Promise<T>;
}
```

**Behavior**: Wraps auth-client's Keycloak integration. `executeGatedAction` checks auth, shows modal if needed, then runs the action.

---

## Clickstream Hook

### `useClickstream`

```typescript
interface ClickstreamOptions {
  debounceMs?: number;           // For viewport events
  enabled?: boolean;             // Allow disabling events
}

function useClickstream(options?: ClickstreamOptions): {
  emit: (eventName: EventName, payload?: Record<string, unknown>) => void;
}
```

**Behavior**: Fire-and-forget event emission. Generates event envelope with session/correlation IDs. Silently ignores failures. No return value.

**API call**: `POST /api/v1/clickstream/events`
