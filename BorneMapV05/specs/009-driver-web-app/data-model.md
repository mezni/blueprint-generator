# Data Model: Driver Web App

**Branch**: `009-driver-web-app` | **Date**: 2026-06-03

## Overview

TypeScript types for the Driver Web App frontend data model. Derived from driver-service API contracts (`services/driver-service/src/models/`) and the standard envelope types defined in `crates/common-types/src/api.rs`.

---

## Envelopes

```typescript
// Standard response envelopes (from @bornemap/api-contracts)
interface SuccessEnvelope<T> {
  success: true;
  data: T[];
  meta: PaginationMeta;
}

interface ItemEnvelope<T> {
  success: true;
  data: T;
  meta: Record<string, never>;
}

interface ErrorEnvelope {
  success: false;
  error: {
    code: string;
    message: string;
    details: unknown | null;
  };
}

interface PaginationMeta {
  page: number;
  size: number;
  total: number;
  total_pages: number;
  has_next: boolean;
  has_prev: boolean;
}
```

---

## Station

```typescript
// Station list item (from GET /api/v1/driver/stations and /search)
interface StationListItem {
  id: string;                   // "STN-..."
  name: string;
  description: string | null;
  latitude: number;
  longitude: number;
  city: string | null;
  country: string | null;
  distance_km: number | null;   // only when lat/lng provided
  geom: { lat: number; lng: number };
  charger_types: ChargerTypeInfo[];
  availability: StationAvailability | null;
  review_summary: ReviewSummary | null;
}

// Station detail (from GET /api/v1/driver/stations/{id})
interface StationDetail {
  id: string;
  name: string;
  description: string | null;
  latitude: number;
  longitude: number;
  city: string | null;
  country: string | null;
  distance_km: number | null;
  geom: { lat: number; lng: number };
  chargers: Charger[];
  charger_types: ChargerTypeInfo[];
  availability: StationAvailability | null;
  review_summary: ReviewSummary | null;
}
```

### Station Fields

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `id` | `string` | Always | ULID with STN- prefix |
| `name` | `string` | Always | Display name |
| `description` | `string \| null` | Optional | Markdown text |
| `latitude` | `number` | Always | WGS84 decimal degrees |
| `longitude` | `number` | Always | WGS84 decimal degrees |
| `city` | `string \| null` | Optional | City name |
| `country` | `string \| null` | Optional | Country name |
| `distance_km` | `number \| null` | Optional | From user query point |
| `charger_types` | `ChargerTypeInfo[]` | Always | Distinct connector types |
| `chargers` | `Charger[]` | Detail only | Full charger list |
| `availability` | `StationAvailability \| null` | Optional | Current availability status |

### Station State Transitions

The station entity is managed by the backend. Frontend observes:
- `StationListItem` ← `GET /stations` (list/search)
- `StationDetail` ← `GET /stations/{id}` (detail)
- No create/update/delete operations — stations are managed by partners (admin dashboard)

---

## Charger

```typescript
interface Charger {
  id: string;                    // "CHG-..."
  station_id: string;            // "STN-..."
  connector_type: ConnectorType;
  power_kw: number | null;
  status: ChargerStatus;
  created_at: string;            // ISO 8601
  updated_at: string;            // ISO 8601
}

interface ChargerTypeInfo {
  connector_type: ConnectorType;
  power_kw: number | null;
  status: ChargerStatus;
}
```

### Enums

```typescript
type ConnectorType = "CCS" | "Type2" | "CHAdeMO";
type ChargerStatus = "available" | "offline" | "fault";
type StationAvailability = "available" | "limited" | "unavailable";
```

---

## Favorite

```typescript
// Favorite operations return the station ID
interface FavoriteResponse {
  station_id: string;
}

// Favorites list returns station ID strings only
// GET /api/v1/driver/favorites → string[]
type FavoriteStationIds = string[];
```

### Validation Rules

- **Toggle**: POST to add, DELETE to remove
- **Idempotency**: Duplicate POST is silently ignored (ON CONFLICT DO NOTHING)
- **Not found**: DELETE on non-existent favorite returns NOT_FOUND (404)
- **Auth required**: Requires `registered_driver` role

---

## Review

```typescript
interface Review {
  id: string;                    // "REV-..."
  user_id: string;               // "USR-..."
  station_id: string;            // "STN-..."
  rating: number;                // 1-5
  comment: string | null;
  status: ReviewStatus;
  created_at: string;            // ISO 8601
  updated_at: string;            // ISO 8601
}

type ReviewStatus = "published" | "hidden" | "flagged" | "deleted";

// Create review request body
interface ReviewCreate {
  station_id: string;
  rating: number;                // 1-5
  comment?: string;
}

// Update review request body (all fields optional)
interface ReviewUpdate {
  rating?: number;               // 1-5
  comment?: string;
}

// Review summary on station detail
interface ReviewSummary {
  average_rating: number | null;
  total_reviews: number;
}
```

### Validation Rules

| Rule | Enforcement |
|------|-------------|
| Rating 1-5 | Backend returns VALIDATION_FAILED if out of range |
| One review per user per station | Backend returns ALREADY_EXISTS on duplicate |
| Owner can edit own review | Backend enforces ownership via JWT user_id |
| Owner can delete own review | Backend soft-deletes (sets status='deleted') |
| Cannot update deleted review | Backend returns VALIDATION_FAILED |

### Review State Transitions

```
[Create] → published
[Update] → published (or hidden/flagged by admin)
[Delete] → deleted (soft, by owner)
           → deleted (soft, by admin moderation)
           → hidden/flagged (admin moderation only)
```

---

## Search

```typescript
// Search query parameters
interface SearchQuery {
  q?: string;                    // Text search (name, city, description)
  city?: string;                 // City filter (NOT YET WIRED in backend)
  connector_type?: ConnectorType; // Connector type filter (NOT YET WIRED)
  availability?: StationAvailability; // Availability filter (NOT YET WIRED)
  page?: number;
  size?: number;
}

// Search response
type SearchResponse = SuccessEnvelope<StationListItem>;
```

---

## Auth

```typescript
interface AuthUser {
  sub: string;                    // Keycloak user UUID
  email?: string;
  preferred_username?: string;
  roles: string[];                // realm_access.roles
  user_id?: string;               // USR-... (mapped from Keycloak)
}

interface AuthState {
  isAuthenticated: boolean;
  isInitialized: boolean;
  user: AuthUser | null;
  login: (provider?: string) => Promise<void>;
  logout: () => Promise<void>;
  getToken: () => Promise<string | null>;
}
```

---

## Clickstream Event

```typescript
// Event envelope (from @bornemap/event-taxonomy)
interface EventEnvelope {
  event_id: string;
  event_version: number;
  schema_namespace: string;
  event_name: string;
  occurred_at: string;           // ISO 8601
  ingested_at: string;           // ISO 8601
  channel: string;
  session_id: string;
  correlation_id?: string;
  anonymous_id?: string;
  user_id?: string;
  actor_role: string;
  path: string;
  payload: Record<string, unknown>;
  metadata: Record<string, unknown>;
}

type EventName =
  | "page.viewed"
  | "map.loaded"
  | "map.viewport_changed"
  | "search.performed"
  | "stations.nearby.viewed"
  | "filter.applied"
  | "station.marker_clicked"
  | "station.opened"
  | "charger.opened"
  | "favorite_station.added"
  | "favorite_station.removed"
  | "review.submitted"
  | "review.updated"
  | "auth.started"
  | "auth.succeeded"
  | "auth.failed"
  | "search.failed"
  | "station.load_failed";
```

---

## Driver Profile

```typescript
interface DriverProfile {
  user_id: string;               // "USR-..."
  email: string | null;
  display_name: string | null;
  avatar_url: string | null;
  preferred_language: string | null; // "fr" | "ar"
  preferences: Record<string, unknown> | null;
  created_at: string | null;     // ISO 8601
  last_login_at: string | null;  // ISO 8601
}

interface ProfileUpdate {
  display_name?: string;
  avatar_url?: string;
  preferred_language?: string;
  preferences?: Record<string, unknown>;
}
```

---

## React Query Key Structure

```typescript
const queryKeys = {
  stations: {
    all: ['stations'] as const,
    list: (params: StationListParams) => ['stations', 'list', params] as const,
    detail: (id: string) => ['stations', 'detail', id] as const,
    search: (query: SearchQuery) => ['stations', 'search', query] as const,
  },
  favorites: {
    all: ['favorites'] as const,
    list: (userId: string) => ['favorites', 'list', userId] as const,
  },
  reviews: {
    all: ['reviews'] as const,
    list: (stationId: string) => ['reviews', 'list', stationId] as const,
    userList: () => ['reviews', 'user'] as const,
  },
  profile: {
    all: ['profile'] as const,
    me: () => ['profile', 'me'] as const,
  },
};
```
