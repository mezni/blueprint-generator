# Data Model: Admin Service MVP

**Branch**: `005-admin-service-mvp` | **Date**: 2026-06-02

This document describes the application-level data model — the Rust types, their relationships, and validation rules. The database schema already exists from Sprint 4 (migrations 0000-0017). A new migration (0018) adds the `idempotency_key` table.

## New Database Table

### inventory.idempotency_key (Migration 0018)

| Column | Type | Constraints | Notes |
|--------|------|-------------|-------|
| id | TEXT | PK | `EVT-<ULID>` (reuses event prefix for idempotency tracking) |
| key | TEXT | UNIQUE NOT NULL | Client-provided `Idempotency-Key` header value |
| station_id | TEXT | FK → `inventory.station.id` NOT NULL | The station created under this key |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | Used for TTL cleanup (24h) |

Indexes: `UNIQUE(key)`, `BTREE(created_at)` (for TTL cleanup queries).

## Application Models (Rust Types)

### Partner

```rust
struct PartnerRow {
    id: String,           // PRT-<ULID>
    name: String,
    partner_type: String, // "business" | "private"
    status: String,       // "active" | "suspended"
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Option<String>,
    updated_by: Option<String>,
    deleted_at: Option<DateTime<Utc>>,
}
```

**Validation rules**:
- `name`: required, 1-255 chars
- `partner_type`: one of `["business", "private"]`
- `status`: one of `["active", "suspended"]`
- Soft delete: `deleted_at` set, never hard delete
- Delete blocked if active stations exist (trigger + app-level check)

**State transitions**:
```
status: active ↔ suspended
  - active → suspended: admin action
  - suspended → active: admin action
  - soft delete: only when no active stations (ACTIVE_STATIONS_EXIST error otherwise)
```

### Station

```rust
struct StationRow {
    id: String,           // STN-<ULID>
    partner_id: String,   // FK → partner, always from membership
    name: String,
    description: Option<String>,
    latitude: f64,
    longitude: f64,
    status: String,       // "draft" | "active" | "inactive" | "maintenance"
    is_live: bool,
    is_public: bool,
    city: Option<String>,
    country: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Option<String>,
    updated_by: Option<String>,
    deleted_at: Option<DateTime<Utc>>,
}
```

**Validation rules**:
- `latitude`: -90.0 to 90.0 (FR-023)
- `longitude`: -180.0 to 180.0 (FR-023)
- `status` lifecycle enforced (FR-024): `draft → active → inactive → maintenance → active`
- `partner_id`: NEVER from client, always from `CurrentUser.partner_id` (partner API) or from existing record (admin API)
- Soft delete only (FR-005, FR-016)
- GIS outbox row inserted on every create/update/soft-delete (FR-018)
- Optimistic concurrency via `updated_at` / `If-Match` ETag (FR-028)

**Status lifecycle** (application-enforced):
```
draft → active        (partner or admin sets is_live=true, is_public=true)
active → inactive     (partner or admin)
active → maintenance  (partner or admin)
inactive → active     (partner or admin)
maintenance → active  (partner or admin)
```

Invalid transitions return `INVALID_STATE_TRANSITION`. Example: `draft → inactive` is invalid (must go through `active` first).

### Charger

```rust
struct ChargerRow {
    id: String,           // CHG-<ULID>
    station_id: String,   // FK → station
    charger_type: String, // "CCS" | "Type2" | "CHAdeMO"
    power_kw: Option<f64>,
    status: String,       // "available" | "offline" | "fault"
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Option<String>,
    updated_by: Option<String>,
    deleted_at: Option<DateTime<Utc>>,
}
```

**Validation rules**:
- `station_id`: must exist and belong to the authenticated partner (partner API)
- `charger_type`: one of `["CCS", "Type2", "CHAdeMO"]`
- `status`: one of `["available", "offline", "fault"]`
- `power_kw`: if provided, must be > 0
- Soft delete only
- Partner scoping inherited from parent station

### StationAvailability

```rust
struct StationAvailabilityRow {
    id: String,
    station_id: String,   // FK → station
    status: String,       // "available" | "limited" | "unavailable"
    source: String,       // "manual_partner" | "system_sync" | "admin"
    updated_at: DateTime<Utc>,
}
```

**Validation rules**:
- `status`: one of `["available", "limited", "unavailable"]`
- `source`: set to `"manual_partner"` when updated via partner API, `"admin"` when via admin API
- Upsert behavior: if availability row exists for station, update it; otherwise insert

### UserAccount

```rust
struct UserAccountRow {
    id: String,              // USR-<ULID>
    keycloak_user_id: String,
    email: Option<String>,
    status: String,          // "active" | "disabled"
    created_at: DateTime<Utc>,
    last_login_at: Option<DateTime<Utc>>,
}
```

**Validation rules**:
- `keycloak_user_id`: unique, matches JWT `sub`
- Admin list endpoint only (no mutation by admin-service in this sprint)

### PartnerMembership

```rust
struct PartnerMembershipRow {
    user_id: String,      // FK → user_account, UNIQUE
    partner_id: String,   // FK → partner
    role: String,         // "owner" | "manager" | "operator" | "viewer"
}
```

**Validation rules**:
- Strict 1:1 mapping (UNIQUE on `user_id`)
- Used to derive `partner_id` for partner-scoped requests

### StationReview

```rust
struct ReviewRow {
    id: String,           // REV-<ULID>
    user_id: String,
    station_id: String,
    rating: i32,          // 1-5
    comment: Option<String>,
    status: String,       // "published" | "hidden" | "flagged" | "deleted"
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

**Validation rules**:
- Admin can only moderate status (PATCH `/admin/reviews/{id}/status`)
- Status transitions (admin moderation): `published → hidden`, `published → flagged`, `flagged → hidden`, `hidden → published`, `any → deleted`
- Review uses `status='deleted'` for logical delete (NOT `deleted_at` column)

### GisSyncQueue (Outbox)

```rust
struct GisSyncQueueRow {
    id: String,           // EVT-<ULID>
    entity_type: String,  // "station"
    entity_id: String,
    operation: String,    // "insert" | "update" | "delete"
    payload: Option<serde_json::Value>,
    status: String,       // "pending"
    created_at: DateTime<Utc>,
    processed_at: Option<DateTime<Utc>>,
}
```

**Rules**:
- Always inserted with `status = 'pending'`
- Inserted within the same transaction as the station mutation
- Payload: optional JSON with changed fields (for future worker use)

## Relationships

```
Partner 1──N Station 1──N Charger
                 │
                 ├──1 StationAvailability
                 │
                 └──N StationReview

UserAccount 1──1 PartnerMembership ──1 Partner
UserAccount 1──N StationReview
UserAccount 1──N FavoriteStation

Station ──→ GisSyncQueue (outbox on mutation)
Station ──→ IdempotencyKey (on creation)
```

## DTOs (Request/Response)

### CreateStationRequest
```json
{
  "name": "string (required, 1-255)",
  "description": "string (optional)",
  "latitude": "number (required, -90 to 90)",
  "longitude": "number (required, -180 to 180)",
  "status": "string (optional, default: 'draft')",
  "is_live": "boolean (optional, default: false)",
  "is_public": "boolean (optional, default: false)",
  "city": "string (optional)",
  "country": "string (optional)"
}
```

### UpdateStationRequest
```json
{
  "name": "string (optional)",
  "description": "string (optional)",
  "latitude": "number (optional)",
  "longitude": "number (optional)",
  "status": "string (optional)",
  "is_live": "boolean (optional)",
  "is_public": "boolean (optional)",
  "city": "string (optional)",
  "country": "string (optional)"
}
```

### CreatePartnerRequest
```json
{
  "name": "string (required, 1-255)",
  "type": "string (required, 'business' | 'private')",
  "status": "string (optional, default: 'active')"
}
```

### UpdatePartnerRequest
```json
{
  "name": "string (optional)",
  "type": "string (optional)",
  "status": "string (optional)"
}
```

### CreateChargerRequest
```json
{
  "station_id": "string (required, STN-*)",
  "type": "string (required, 'CCS' | 'Type2' | 'CHAdeMO')",
  "power_kw": "number (optional, > 0)",
  "status": "string (optional, default: 'available')"
}
```

### UpdateChargerRequest
```json
{
  "type": "string (optional)",
  "power_kw": "number (optional)",
  "status": "string (optional)"
}
```

### UpdateAvailabilityRequest
```json
{
  "status": "string (required, 'available' | 'limited' | 'unavailable')"
}
```

### ModerateReviewRequest
```json
{
  "status": "string (required, 'published' | 'hidden' | 'flagged' | 'deleted')"
}
```
