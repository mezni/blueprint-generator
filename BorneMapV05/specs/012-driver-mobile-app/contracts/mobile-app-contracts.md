# Mobile App Contracts

**Feature**: Driver Mobile App (Sprint 12)
**Date**: 2026-06-04
**Purpose**: Define all API and data contracts that the mobile app consumes or emits

---

## Table of Contents

1. [API Contracts](#api-contracts)
   - [Driver Service Endpoints](#driver-service-endpoints)
   - [Clickstream Service Events](#clickstream-service-events)
   - [Auth Keycloak Protocol](#auth-keycloak-protocol)
2. [Local Storage Contracts](#local-storage-contracts)
3. [Event Contracts](#event-contracts)

---

## API Contracts

### Driver Service Endpoints

The mobile app consumes the following REST API endpoints from the driver-service:

#### 1. GET /api/v1/driver/stations

**Purpose**: Discover stations within a geographic area.

**Query Parameters**:
```typescript
interface StationsQueryParams {
  lat: number;           // User latitude (required)
  lng: number;           // User longitude (required)
  radius_km: number;     // Search radius in kilometers (default: 10, max: 50)
  bbox?: string;         // Bounding box "minLon,minLat,maxLon,maxLat"
  connector_type?: 'CCS' | 'Type2' | 'CHAdeMO';
  availability?: 'available' | 'limited' | 'unavailable';
  page?: number;         // Page number (default: 1)
  size?: number;         // Items per page (default: 20, max: 100)
}
```

**Response** (Success Envelope):
```typescript
interface StationsResponse {
  success: true;
  data: {
    stations: Station[];
    meta: {
      page: number;
      size: number;
      total: number;
      total_pages: number;
      has_next: boolean;
      has_prev: boolean;
    };
  };
  meta: {
    total_count: number;
  };
}
```

**Error Envelope** (Standard):
```typescript
interface ErrorResponse {
  success: false;
  error: {
    code: string;         // Error code from constitution
    message: string;      // Human-readable message
    details?: Record<string, unknown>;
  };
}
```

**Usage**: Mobile app queries stations based on user location and viewport debounced (300-500ms)

---

#### 2. GET /api/v1/driver/stations/{station_id}

**Purpose**: Get detailed information about a specific station.

**Path Parameters**:
```typescript
interface StationIdParams {
  station_id: string;    // "STN-<ULID>"
}
```

**Response** (Success Envelope):
```typescript
interface StationDetailResponse {
  success: true;
  data: {
    station: {
      id: string;
      name: string;
      description: string | null;
      latitude: number;
      longitude: number;
      city: string | null;
      country: string | null;
      status: 'active' | 'inactive' | 'maintenance' | 'draft';
      is_live: boolean;
      is_public: boolean;
      partner_id: string;
      partner_name: string;
      chargers: Charger[];
      created_at: string;
      deleted_at: string | null;
    };
    distance_km?: number;  // Distance from user if location available
    geom?: {
      lat: number;
      lng: number;
    };
  };
  meta: Record<string, never>;
}
```

**Usage**: Mobile app shows detailed view when user taps station marker

---

#### 3. GET /api/v1/driver/stations/search

**Purpose**: Search stations by query parameters.

**Query Parameters**:
```typescript
interface StationSearchParams {
  q: string;             // Search query (name, city, etc.)
  city?: string;         // City filter
  connector_type?: 'CCS' | 'Type2' | 'CHAdeMO';
  availability?: 'available' | 'limited' | 'unavailable';
  page?: number;
  size?: number;
}
```

**Response**: Same format as `/api/v1/driver/stations`

**Usage**: Mobile app shows search results screen

---

#### 4. POST /api/v1/driver/favorites/{station_id}

**Purpose**: Add a station to user's favorites.

**Path Parameters**:
```typescript
interface FavoriteIdParams {
  station_id: string;    // "STN-<ULID>"
}
```

**Headers**:
```typescript
interface FavoriteHeaders {
  Authorization: string; // Bearer <JWT token>
}
```

**Response** (Success Envelope):
```typescript
interface FavoriteResponse {
  success: true;
  data: {
    user_id: string;
    station_id: string;
    created_at: string;
  };
  meta: Record<string, never>;
}
```

**Error Codes**: `UNAUTHENTICATED`, `INSUFFICIENT_ROLE`, `ALREADY_EXISTS`

**Usage**: Mobile app uses optimistic UI update, then syncs with server

---

#### 5. DELETE /api/v1/driver/favorites/{station_id}

**Purpose**: Remove a station from user's favorites.

**Path Parameters**:
```typescript
interface FavoriteIdParams {
  station_id: string;    // "STN-<ULID>"
}
```

**Headers**:
```typescript
interface FavoriteHeaders {
  Authorization: string; // Bearer <JWT token>
}
```

**Response** (Success Envelope):
```typescript
interface FavoriteDeleteResponse {
  success: true;
  data: {
    user_id: string;
    station_id: string;
  };
  meta: Record<string, never>;
}
```

**Usage**: Mobile app uses optimistic UI update, then syncs with server

---

#### 6. GET /api/v1/driver/favorites

**Purpose**: Get user's favorites list.

**Headers**:
```typescript
interface FavoritesHeaders {
  Authorization: string; // Bearer <JWT token>
}
```

**Query Parameters**:
```typescript
interface FavoritesQueryParams {
  page?: number;
  size?: number;
}
```

**Response** (Success Envelope):
```typescript
interface FavoritesResponse {
  success: true;
  data: {
    favorites: Favorite[];
    meta: {
      page: number;
      size: number;
      total: number;
      total_pages: number;
      has_next: boolean;
      has_prev: boolean;
    };
  };
  meta: {
    total_count: number;
  };
}
```

**Usage**: Mobile app shows favorites screen with paginated list

---

#### 7. POST /api/v1/driver/reviews

**Purpose**: Submit a review for a station.

**Headers**:
```typescript
interface ReviewHeaders {
  Authorization: string; // Bearer <JWT token>
}
```

**Request Body**:
```typescript
interface ReviewSubmitRequest {
  station_id: string;    // "STN-<ULID>"
  rating: number;        // 1 to 5
  comment?: string;      // Optional comment (max 1000 chars)
}
```

**Response** (Success Envelope):
```typescript
interface ReviewSubmitResponse {
  success: true;
  data: {
    id: string;
    user_id: string;
    station_id: string;
    rating: number;
    comment: string | null;
    status: 'published';
    created_at: string;
  };
  meta: Record<string, never>;
}
```

**Error Codes**: `UNAUTHENTICATED`, `INSUFFICIENT_ROLE`, `ALREADY_EXISTS`, `VALIDATION_FAILED`

**Usage**: Mobile app shows review submission form, optimistic UI update

---

#### 8. PATCH /api/v1/driver/reviews/{review_id}

**Purpose**: Update an existing review (owner only).

**Path Parameters**:
```typescript
interface ReviewIdParams {
  review_id: string;     // "REV-<ULID>"
}
```

**Headers**:
```typescript
interface ReviewHeaders {
  Authorization: string; // Bearer <JWT token>
}
```

**Request Body**:
```typescript
interface ReviewUpdateRequest {
  rating?: number;       // 1 to 5
  comment?: string;      // Optional comment (max 1000 chars)
}
```

**Usage**: Owner can modify their review (future feature)

---

#### 9. DELETE /api/v1/driver/reviews/{review_id}

**Purpose**: Soft delete a review.

**Path Parameters**:
```typescript
interface ReviewIdParams {
  review_id: string;     // "REV-<ULID>"
}
```

**Headers**:
```typescript
interface ReviewHeaders {
  Authorization: string; // Bearer <JWT token>
}
```

**Response**: Same as POST /api/v1/driver/reviews

**Usage**: Owner can delete their review (future feature)

---

#### 10. GET /api/v1/driver/me

**Purpose**: Get current authenticated user's profile.

**Headers**:
```typescript
interface MeHeaders {
  Authorization: string; // Bearer <JWT token>
}
```

**Response** (Success Envelope):
```typescript
interface MeResponse {
  success: true;
  data: {
    id: string;
    keycloak_user_id: string;
    email: string | null;
    status: 'active' | 'disabled';
    last_login_at: string;
    partner_membership?: {
      user_id: string;
      partner_id: string;
      role: 'owner' | 'manager' | 'operator' | 'viewer';
    };
  };
  meta: Record<string, never>;
}
```

**Usage**: Mobile app shows user profile screen

---

#### 11. PATCH /api/v1/driver/me

**Purpose**: Update user profile.

**Headers**:
```typescript
interface MeHeaders {
  Authorization: string; // Bearer <JWT token>
}
```

**Request Body**:
```typescript
interface MeUpdateRequest {
  display_name?: string;
  avatar_url?: string;
  preferred_language?: string;
  preferences?: Record<string, unknown>;
}
```

**Usage**: Mobile app shows profile edit screen (future feature)

---

### Clickstream Service Events

The mobile app emits events to the clickstream-service via the existing API client:

#### Event Contract

```typescript
interface ClickstreamEvent {
  event_id: string;              // Unique event ID ("CLK-<ULID>")
  event_version: number;         // Current version (1)
  schema_namespace: 'clickstream';
  event_name: string;            // Event name from taxonomy
  occurred_at: string;           // ISO 8601 timestamp
  ingested_at: string;           // ISO 8601 timestamp (server ingestion time)
  channel: 'driver_mobile';
  session_id: string;            // Session identifier
  correlation_id?: string;       // Optional correlation ID
  anonymous_id?: string;         // Optional anonymous user ID
  user_id?: string;              // Optional user ID if authenticated
  actor_role?: 'registered_driver' | 'partner' | 'admin' | 'anonymous';
  path?: string;                 // Current page path
  payload: Record<string, unknown>;  // Event-specific data
  metadata?: Record<string, unknown>; // Additional context
}
```

#### Events Emitted by Mobile App

1. **Navigation Events**:
   ```typescript
   {
     event_name: 'page.viewed',
     payload: { page_name: 'map' }
   }
   {
     event_name: 'map.loaded',
     payload: {}
   }
   {
     event_name: 'map.viewport_changed',
     payload: { bbox: string, zoom_level: number }
   }
   ```

2. **Discovery Events**:
   ```typescript
   {
     event_name: 'search.performed',
     payload: { query: string, filter_type: string }
   }
   {
     event_name: 'stations.nearby.viewed',
     payload: { station_count: number }
   }
   {
     event_name: 'filter.applied',
     payload: { connector_type?: string, availability?: string }
   }
   ```

3. **Station Events**:
   ```typescript
   {
     event_name: 'station.marker_clicked',
     payload: { station_id: string }
   }
   {
     event_name: 'station.opened',
     payload: { station_id: string }
   }
   {
     event_name: 'charger.opened',
     payload: { station_id: string, charger_id: string }
   }
   ```

4. **Favorites Events**:
   ```typescript
   {
     event_name: 'favorite_station.added',
     payload: { station_id: string }
   }
   {
     event_name: 'favorite_station.removed',
     payload: { station_id: string }
   }
   ```

5. **Reviews Events**:
   ```typescript
   {
     event_name: 'review.submitted',
     payload: { station_id: string, rating: number }
   }
   ```

6. **Auth Events**:
   ```typescript
   {
     event_name: 'auth.started',
     payload: {}
   }
   {
     event_name: 'auth.succeeded',
     payload: {}
   }
   {
     event_name: 'auth.failed',
     payload: { error: string }
   }
   ```

7. **Partner Events**:
   ```typescript
   {
     event_name: 'partner_station.created',
     payload: { station_id: string }
   }
   {
     event_name: 'partner_station.updated',
     payload: { station_id: string }
   }
   {
     event_name: 'partner_availability.updated',
     payload: { station_id: string, availability: string }
   }
   ```

8. **Failure Events**:
   ```typescript
   {
     event_name: 'search.failed',
     payload: { error: string }
   }
   {
     event_name: 'station.load_failed',
     payload: { station_id: string, error: string }
   }
   ```

---

### Auth Keycloak Protocol

The mobile app authenticates via Keycloak OAuth2:

#### Flow

1. **OAuth2 Authorization Code Flow**:
   - Redirect user to Keycloak login page
   - User authenticates with credentials or social provider
   - Keycloak redirects back with authorization code
   - Mobile app exchanges code for access token

2. **Token Refresh**:
   - Access tokens expire (default: 5 minutes)
   - Mobile app automatically refreshes using refresh token
   - No user interaction required

#### Token Structure

```typescript
interface KeycloakToken {
  access_token: string;   // JWT access token (5 min expiry)
  refresh_token: string;  // JWT refresh token (30 day expiry)
  token_type: string;     // "Bearer"
  expires_in: number;     // Access token lifetime in seconds
}
```

#### JWT Claims

```typescript
interface KeycloakJWT {
  sub: string;                      // User ID (keycloak_user_id)
  exp: number;                      // Token expiration timestamp
  iat: number;                      // Token issued at timestamp
  aud: string;                      // Audience (bornemap-api)
  iss: string;                      // Issuer (Keycloak URL)
  realm_access: {
    roles: string[];                // User roles
  };
  preferred_username?: string;      // Email or username
}
```

#### Token Storage

- **Storage**: Expo Secure Store (AES-256 encrypted)
- **Protection**: PIN/biometric lock optional layer
- **Cleanup**: Automatic clear on logout

---

## Local Storage Contracts

### AsyncStorage Keys

```typescript
// Favorite station IDs (JSON array of strings)
const STORAGE_KEY_FAVORITES = 'favorites';

// Auth token (encrypted)
const STORAGE_KEY_AUTH_TOKEN = 'auth_token';

// Refresh token (encrypted)
const STORAGE_KEY_REFRESH_TOKEN = 'refresh_token';

// User profile (JSON object)
const STORAGE_KEY_USER_PROFILE = 'user_profile';

// Offline queue (JSON array of pending API operations)
const STORAGE_KEY_OFFLINE_QUEUE = 'offline_queue';
```

---

## Event Contracts

### Event Emission Utility

```typescript
interface EventPayload {
  event_id?: string;  // Auto-generated if not provided
  session_id?: string; // Auto-generated if not provided
  path?: string;      // Current route if available
}

function emitEvent(
  eventName: string,
  payload: EventPayload
): void;
```

### Offline Event Queue

```typescript
interface QueuedEvent {
  event_name: string;
  payload: EventPayload;
  timestamp: number;   // Unix timestamp (ms)
  retry_count: number;
  max_retries: 10;
}
```

---

## Contract Summary

| Contract Type | Count | Purpose |
|---------------|-------|---------|
| API Endpoints | 11 | All data operations |
| Clickstream Events | 18 | User behavior tracking |
| Local Storage Keys | 5 | Offline data persistence |
| Event Queue | 1 | Offline event synchronization |

**Total Contracts**: 35
