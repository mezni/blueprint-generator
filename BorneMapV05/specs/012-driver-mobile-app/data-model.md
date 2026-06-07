# Data Model: Driver Mobile App

**Feature**: Driver Mobile App (Sprint 12)
**Date**: 2026-06-04
**Based On**: Feature specification and API contracts

## Overview

The Driver Mobile App consumes existing API data structures from the `platform_db` database through the driver-service. This document defines how these entities appear in the mobile app, including client-side caching strategies and state management.

---

## Core Entities

### 1. Station

**Purpose**: Represents a charging station with location, charger information, and availability status.

**Source**: API response from `/api/v1/driver/stations/{station_id}`

**Fields**:
```typescript
interface Station {
  id: string;              // "STN-<ULID>"
  name: string;            // Station name
  description: string | null;
  latitude: number;        // Decimal degrees
  longitude: number;       // Decimal degrees
  city: string | null;
  country: string | null;
  status: 'active' | 'inactive' | 'maintenance' | 'draft';
  is_live: boolean;        // Published status
  is_public: boolean;      // Visibility flag
  partner_id: string;      // Owner partner
  partner_name: string;    // Owner name
  chargers: Charger[];     // Array of chargers
  created_at: string;      // ISO 8601 timestamp
  deleted_at: string | null; // Soft delete timestamp
}
```

**Mobile App Behavior**:
- **Caching**: Stored in React Query cache with 10-minute TTL
- **Visibility**: Only shows stations where `is_live = true AND status = 'active' AND is_public = true`
- **Distance Calculation**: Calculated client-side using Haversine formula
- **Deleted Stations**: Automatically removed from favorites when soft-deleted

**Validation Rules**:
- `latitude`: -90 to 90 (decimal degrees)
- `longitude`: -180 to 180 (decimal degrees)
- `status`: Must be one of the 4 valid values
- `is_live`: Must be boolean
- `is_public`: Must be boolean

---

### 2. Charger

**Purpose**: Individual charging unit within a station.

**Source**: Nested in Station response from `/api/v1/driver/stations/{station_id}`

**Fields**:
```typescript
interface Charger {
  id: string;              // "CHG-<ULID>"
  station_id: string;      // Parent station ID
  type: 'CCS' | 'Type2' | 'CHAdeMO';
  power_kw: number;        // Power output in kilowatts
  status: 'available' | 'offline' | 'fault';
}
```

**Mobile App Behavior**:
- **Caching**: Nested in Station object cache
- **Visibility**: Always shown if station is visible
- **Status Updates**: Refreshed when station details re-fetch

**Validation Rules**:
- `type`: Must be one of the 3 connector types
- `power_kw`: Must be positive number
- `status`: Must be one of the 3 valid states

---

### 3. Favorite

**Purpose**: User's saved station for quick access.

**Source**: Local storage only (not an API endpoint)

**Fields**:
```typescript
interface Favorite {
  user_id: string;         // "USR-<ULID>"
  station_id: string;      // "STN-<ULID>"
  created_at: string;      // ISO 8601 timestamp
}
```

**Mobile App Behavior**:
- **Storage**: AsyncStorage with encryption (Secure Store)
- **Access**: Managed via custom `useFavorites` hook
- **Sync**: Syncs with server when network available
- **Cleanup**: Automatically removed when station is soft-deleted

**Validation Rules**:
- `user_id`: Unique identifier from authentication
- `station_id`: Must match existing station
- **Constraints**: One favorite per user per station (unique constraint)

**State Transitions**:
```
Favorite Created (via toggle) → Favorite Synced (API call)
```

---

### 4. Review

**Purpose**: User evaluation of a visited station.

**Source**: API response from `/api/v1/driver/stations/{station_id}`

**Fields**:
```typescript
interface Review {
  id: string;              // "REV-<ULID>"
  user_id: string;         // "USR-<ULID>"
  station_id: string;      // "STN-<ULID>"
  rating: number;          // 1 to 5
  comment: string | null;  // Optional comment
  status: 'published' | 'hidden' | 'flagged' | 'deleted';
  created_at: string;      // ISO 8601 timestamp
  updated_at: string | null;
}
```

**Mobile App Behavior**:
- **Caching**: Stored in React Query cache for station details
- **Visibility**: Shows only `published` status by default
- **Submission**: Creates new review, updates cache immediately (optimistic UI)
- **Constraints**: One review per user per station (unique constraint)

**Validation Rules**:
- `rating`: Must be integer between 1 and 5
- `status`: Must be one of the 4 valid states
- **Constraints**: `rating` must be 1-5, `comment` max 1000 characters

**State Transitions**:
```
Draft → Published (after submission) → (moderated states)
```

---

### 5. User Account (Client State)

**Purpose**: Current authenticated user information.

**Source**: Authentication token from Keycloak + User profile from API

**Fields**:
```typescript
interface UserAccount {
  id: string;              // "USR-<ULID>"
  keycloak_user_id: string; // JWT sub claim
  email: string | null;
  status: 'active' | 'disabled';
  last_login_at: string;   // ISO 8601 timestamp
  partner_membership?: {
    user_id: string;       // Same as user_id
    partner_id: string;    // "PRT-<ULID>"
    role: 'owner' | 'manager' | 'operator' | 'viewer';
  };
}
```

**Mobile App Behavior**:
- **Storage**: Secure Store for token, AsyncStorage for profile
- **Session**: Token refreshes automatically via Keycloak
- **Session State**: Managed via `useAuth` hook
- **Login**: Handles OAuth2 flow and token storage

**Validation Rules**:
- `keycloak_user_id`: Unique, never changes
- `status`: Must be one of the 2 valid values
- `partner_membership.role`: Must be one of the 4 valid roles

**Session Lifecycle**:
```
Login → Token Stored → Token Refreshed (auto) → Logout
```

---

### 6. Session (Client State)

**Purpose**: Active authentication session with Keycloak.

**Source**: Keycloak JWT token and session state

**Fields**:
```typescript
interface Session {
  token: string;           // JWT access token
  refresh_token: string;   // JWT refresh token
  expires_at: number;      // Timestamp when token expires
  authenticated: boolean;  // Session status
}
```

**Mobile App Behavior**:
- **Storage**: Secure Store for security
- **Token Refresh**: Automatic via Keycloak SDK
- **Validation**: JWT signature and audience validation
- **Cleanup**: Clear on logout

**Validation Rules**:
- `token`: Valid JWT format
- `expires_at`: Must be in the future
- **Constraints**: One active session per user

---

## Relationships

### Station ↔ Charger
```
Station (1) → (N) Charger
Station contains multiple chargers
```

### User ↔ Favorite
```
User (1) → (N) Favorite
One favorite per user per station (N:1)
```

### User ↔ Review
```
User (1) → (N) Review
One review per user per station (N:1)
```

### User ↔ PartnerMembership
```
User (1) → (1) PartnerMembership
One membership per user
```

---

## Caching Strategy

### React Query Cache

| Entity | Cache Key | TTL | Refresh Strategy |
|--------|-----------|-----|------------------|
| Stations (map discovery) | `['stations']` | 10 min | Stale-while-revalidate |
| Station Detail | `['station', id]` | 10 min | Stale-while-revalidate |
| Favorites | `['favorites']` | 5 min | Always fresh |
| Reviews | `['reviews', stationId]` | 10 min | Stale-while-revalidate |
| User Profile | `['user', id]` | 1 hour | Stale-while-revalidate |

### AsyncStorage (Persistent)

| Entity | Key | Purpose |
|--------|-----|---------|
| Favorites | `favorites` | Local favorites storage (syncs to server) |
| Auth Token | `auth_token` | Encrypted authentication token |
| User Profile | `user_profile` | Cached user information |

---

## State Management

### Server State (React Query)

- **Stations**: Use `useQuery` with `['stations']` cache key
- **Station Detail**: Use `useQuery` with `['station', stationId]` cache key
- **Favorites**: Use `useQuery` with `['favorites']` cache key
- **Reviews**: Use `useQuery` with `['reviews', stationId]` cache key
- **User Profile**: Use `useQuery` with `['user', userId]` cache key

### Client State (React Context)

- **Authentication**: `AuthContext` - auth state, login/logout functions
- **Favorites**: `FavoritesContext` - favorites list, add/remove functions

---

## Data Flow Diagram

```
User Action → UI Component → Custom Hook → React Query → API Client
                                                    ↓
                                                 API Response
                                                    ↓
                                            Cache Update + UI Refresh
```

---

## Validation Summary

| Field | Type | Validation Rule | Client-Side Check |
|-------|------|----------------|-------------------|
| `station.latitude` | number | -90 to 90 | Range validation |
| `station.longitude` | number | -180 to 180 | Range validation |
| `review.rating` | number | 1-5 | Range validation |
| `review.comment` | string | Max 1000 chars | Length validation |
| `favorite.station_id` | string | Match existing station | Existence check |
| `auth.token` | string | Valid JWT | Format validation |

---

## Mobile-Specific Considerations

### Offline Handling

- **Stations**: Cache stored in AsyncStorage, shows from cache while offline
- **Favorites**: Syncs when network restores (background task)
- **Reviews**: Optimistic UI update, syncs on network restore
- **User Profile**: Cache for offline display, refreshes when online

### Performance Optimizations

- **Debouncing**: Map viewport updates debounced (300-500ms)
- **Pagination**: Map discovery paginated (20 stations per page)
- **Caching**: All API responses cached with TTL
- **Lazy Loading**: Heavy components loaded on demand

### Security

- **Token Storage**: Encrypted with Expo Secure Store (AES-256)
- **PIN/Biometric Lock**: Optional security layer
- **Data Encryption**: All local storage encrypted
- **PIN Prevention**: Password manager integration prevention

---

## Data Model Summary

| Entity | Storage Type | API Source | Cache Strategy |
|--------|--------------|------------|----------------|
| Station | React Query + AsyncStorage | `/api/v1/driver/stations` | 10 min TTL |
| Charger | React Query (nested) | `/api/v1/driver/stations/{id}` | 10 min TTL |
| Favorite | AsyncStorage (encrypted) | Not an API endpoint | 5 min TTL |
| Review | React Query | `/api/v1/driver/stations/{id}` | 10 min TTL |
| User Account | Secure Store + React Query | Auth endpoint | 1 hour TTL |
| Session | Secure Store | Keycloak SDK | Token refresh |

**Total Entities**: 6 (3 API entities, 2 client entities, 1 session state)

**Data Model Complexity**: Low - All entities are flat structures, no complex relationships requiring joins
