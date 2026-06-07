# Phase 1: Data Model — Sprint 10 Partner Dashboard

## Entities (Frontend Types)

### Station

```
Station {
  station_id: string         // Format: STN-{ULID}
  partner_id: string         // Format: PRT-{ULID}
  name: string               // Human-readable name
  address: string | null     // Street address
  latitude: f64              // -90 to 90
  longitude: f64             // -180 to 180
  status: StationStatus      // 'active' | 'inactive' | 'maintenance' | 'draft'
  availability_status: StationAvailabilityStatus  // 'available' | 'limited' | 'unavailable'
  created_at: string         // RFC 3339
  updated_at: string         // RFC 3339
}
```

### StationCreate

```
StationCreate {
  name: string               // Required
  address?: string           // Optional
  latitude: f64              // Required, validated on backend
  longitude: f64             // Required, validated on backend
}
```

### StationUpdate

```
StationUpdate {
  name?: string
  address?: string
  latitude?: f64
  longitude?: f64
  status?: StationStatus
  availability_status?: StationAvailabilityStatus
}
```

### Charger

```
Charger {
  charger_id: string         // Format: CHG-{ULID}
  station_id: string         // Parent station
  charger_type: ChargerType  // 'CCS' | 'Type2' | 'CHAdeMO'
  power_kw: f64              // Power output in kW
  status: ChargerStatus      // 'available' | 'offline' | 'fault'
  created_at: string
  updated_at: string
}
```

### ChargerCreate

```
ChargerCreate {
  station_id: string
  charger_type: ChargerType
  power_kw: f64
  status: ChargerStatus
}
```

### ChargerUpdate

```
ChargerUpdate {
  charger_type?: ChargerType
  power_kw?: f64
  status?: ChargerStatus
}
```

### Profile

```
Profile {
  user_id: string
  email: string | null
  partner_id: string | null
  partner_name: string | null
  membership_role: string | null  // 'owner' | 'manager' | 'operator' | 'viewer'
}
```

### Availability

```
Availability {
  station_id: string
  availability_status: StationAvailabilityStatus
  source: string             // 'manual_partner' | 'system_sync' | 'admin'
  updated_at: string
}
```

## API Response Envelopes

### Success (paginated list)

```
SuccessEnvelope<T> {
  success: true
  data: T[]
  meta: PaginationMeta
}
```

### Success (single item)

```
ItemEnvelope<T> {
  success: true
  data: T
  meta: {}                   // Empty object
}
```

### Error

```
ErrorEnvelope {
  success: false
  error: {
    code: string             // Error code constant
    message: string
    details?: Record<string, unknown>
  }
}
```

### PaginationMeta

```
PaginationMeta {
  page: number
  size: number
  total: number
  total_pages: number
  has_next: boolean
  has_prev: boolean
}
```

## State Transitions

### Station Status
```
draft → active
active → inactive | maintenance
inactive → active | maintenance
maintenance → active
```
Status can remain unchanged (self-transition allowed).

### Availability
```
available → limited | unavailable
limited → available | unavailable
unavailable → available | limited
```
No restrictions — any-to-any transitions allowed.

## Optimistic Concurrency

- **Station updates**: require `If-Match` header with current `updated_at` (RFC 3339)
- **Charger updates**: require `If-Match` header with current `updated_at` (RFC 3339)
- **Station creation**: requires `Idempotency-Key` header (UUID v4)

## Error Codes (Partner-Facing)

| Code | Meaning |
|------|---------|
| `UNAUTHENTICATED` | No valid JWT token |
| `FORBIDDEN` | Token valid but insufficient role |
| `PARTNER_SCOPE_VIOLATION` | Partner accessing another partner's data |
| `NOT_FOUND` | Resource doesn't exist |
| `ALREADY_EXISTS` | Idempotency key collision |
| `VALIDATION_FAILED` | Invalid input data |
| `INVALID_COORDINATES` | Lat/lng out of range |
| `INVALID_STATE_TRANSITION` | Illegal status change |
| `CONCURRENT_MODIFICATION` | `If-Match` etag mismatch |
| `ACTIVE_STATIONS_EXIST` | Cannot delete partner with active stations |
