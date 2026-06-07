# Data Model: Monorepo + Tooling Foundation

> **Note**: Sprint 1 is a build-tooling sprint. The "data model" here refers to the shared type schemas that become compile-time contracts between backend and frontend. No runtime data is stored or processed yet.

## 1. API Envelope (Success)

```typescript
// Shared across all API responses
interface SuccessEnvelope<T> {
  success: true;
  data: T;
  meta: PaginationMeta;
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

## 2. API Envelope (Error)

```typescript
interface ErrorEnvelope {
  success: false;
  error: {
    code: ErrorCode;
    message: string;
    details?: Record<string, unknown>;
  };
}
```

## 3. Canonical Error Codes

```typescript
type ErrorCode =
  // Auth
  | 'UNAUTHENTICATED'
  | 'FORBIDDEN'
  | 'TOKEN_EXPIRED'
  // RBAC / Ownership
  | 'PARTNER_SCOPE_VIOLATION'
  | 'INSUFFICIENT_ROLE'
  // Resource
  | 'NOT_FOUND'
  | 'ALREADY_EXISTS'
  | 'SOFT_DELETED'
  // Validation
  | 'VALIDATION_FAILED'
  | 'INVALID_COORDINATES'
  | 'INVALID_STATE_TRANSITION'
  // Business
  | 'ACTIVE_STATIONS_EXIST'
  | 'REVIEW_STATE_INVALID';
```

## 4. Event Envelope (Clickstream)

```typescript
interface EventEnvelope {
  event_id: string;          // CLK-<ULID>
  event_version: number;     // 1
  schema_namespace: 'clickstream';
  event_name: string;        // <domain>.<action>
  occurred_at: string;       // ISO 8601
  ingested_at: string;       // ISO 8601
  channel: Channel;
  session_id: string;
  correlation_id?: string;
  anonymous_id?: string;     // Required if no user_id
  user_id?: string;          // Nullable (anonymous)
  actor_role?: string;       // Derived from JWT or 'anonymous'
  path?: string;
  payload: Record<string, unknown>;
  metadata: Record<string, unknown>;
}

type Channel = 'driver_web' | 'driver_mobile' | 'partner_dashboard' | 'admin_dashboard';
```

## 5. ID Prefix Convention

```typescript
type EntityPrefix =
  | 'USR'  // User accounts
  | 'PRT'  // Partners
  | 'STN'  // Stations
  | 'CHG'  // Chargers
  | 'REV'  // Reviews
  | 'EVT'  // Analytics events
  | 'CLK'  // Clickstream event IDs
  | 'SESS' // Sessions
  | 'ANON' // Anonymous users
  ;
```

## 6. Role Enum

```typescript
type Role = 'registered_driver' | 'partner' | 'admin';
```

## 7. Station Status (for reference)

```typescript
type StationStatus = 'active' | 'inactive' | 'maintenance' | 'draft';
type StationAvailabilityStatus = 'available' | 'limited' | 'unavailable';
type PartnerStatus = 'active' | 'suspended';
type ChargerStatus = 'available' | 'offline' | 'fault';
type ChargerType = 'CCS' | 'Type2' | 'CHAdeMO';
type ReviewStatus = 'published' | 'hidden' | 'flagged' | 'deleted';
type PartnerRole = 'owner' | 'manager' | 'operator' | 'viewer';
type GISQueueStatus = 'pending' | 'processing' | 'done' | 'failed' | 'dead_letter';
type AvailabilitySource = 'manual_partner' | 'system_sync' | 'admin';
```
