# Data Model: Sprint 11 — Admin Dashboard

## Overview

The admin dashboard is a frontend-only view into existing platform data. No new backend entities are introduced. The data model documented here describes the entities as presented in admin views, not the canonical database schema (which is in `docs/EXECUTION_PLAN.md` §4).

All entities use ULID+prefix IDs, soft delete (`deleted_at`), and audit fields (`created_at`, `updated_at`, `created_by`, `updated_by`).

## Entities

### Partner

Canonical source: `inventory.partner` (platform_db)

| Field | Type | Admin Display | Editable? |
|-------|------|---------------|-----------|
| id | TEXT (PRT-ULID) | Yes | No |
| name | TEXT | Yes | Yes |
| type | TEXT (business \| private) | Yes | Yes |
| status | TEXT (active \| suspended) | Yes | Yes |
| station_count | INTEGER (derived) | Yes (computed) | No |
| created_at | TIMESTAMPTZ | Yes | No |
| deleted_at | TIMESTAMPTZ NULL | Yes (soft-delete indicator) | Via delete action |

**Admin-specific rules**:
- Delete is blocked if `active stations exist` (backend-enforced via `ACTIVE_STATIONS_EXIST` error code)
- Delete performs soft delete (sets `deleted_at`)
- Suspending a partner should make their stations non-operational

### Station

Canonical source: `inventory.station` (platform_db)

| Field | Type | Admin Display | Editable? |
|-------|------|---------------|-----------|
| id | TEXT (STN-ULID) | Yes | No |
| partner_id | TEXT (FK→partner) | Yes (partner name) | No |
| name | TEXT | Yes | Yes |
| description | TEXT | Yes | Yes |
| latitude | DOUBLE PRECISION | Yes | Yes (with confirmation) |
| longitude | DOUBLE PRECISION | Yes | Yes (with confirmation) |
| status | TEXT (active \| inactive \| maintenance \| draft) | Yes | Yes |
| is_live | BOOLEAN | Yes | Yes |
| is_public | BOOLEAN | Yes | Yes |
| city | TEXT | Yes | No (current scope) |
| country | TEXT | Yes | No (current scope) |
| charger_count | INTEGER (derived) | Yes (inline column) | No |
| created_at | TIMESTAMPTZ | Yes | No |
| deleted_at | TIMESTAMPTZ NULL | Yes (toggle to show) | Via soft-delete |

**Admin-specific rules**:
- Global access to all stations across all partners
- Soft-delete hides station from default view; toggle to show deleted
- Changing coordinates triggers GIS resync via existing outbox pattern
- Inline charger list expands on row click

### Charger

Canonical source: `inventory.charger` (platform_db)

| Field | Type | Admin Display | Editable? |
|-------|------|---------------|-----------|
| id | TEXT (CHG-ULID) | Yes | No |
| station_id | TEXT (FK→station) | Yes (in context) | No |
| type | TEXT (CCS \| Type2 \| CHAdeMO) | Yes | No (read-only in admin) |
| power_kw | NUMERIC | Yes | No (read-only in admin) |
| status | TEXT (available \| offline \| fault) | Yes | No (read-only in admin) |

**Admin-specific rules**:
- Chargers are displayed as read-only detail within station views
- No separate charger management in admin dashboard

### Review

Canonical source: `users.station_review` (platform_db)

| Field | Type | Admin Display | Editable? |
|-------|------|---------------|-----------|
| id | TEXT (REV-ULID) | Yes | No |
| user_id | TEXT (FK→user_account) | Yes | No |
| station_id | TEXT (FK→station) | Yes | No |
| rating | INT (1-5) | Yes | No |
| comment | TEXT | Yes (truncated preview) | No |
| status | TEXT (published \| hidden \| flagged \| deleted) | Yes | Yes (moderation) |
| created_at | TIMESTAMPTZ | Yes | No |

**Admin-specific rules**:
- Admin can change review status following lifecycle: submitted → published → flagged → hidden → deleted
- Invalid transitions are rejected by backend

### User Account

Canonical source: `users.user_account` + `users.user_profile` + `users.partner_membership` (platform_db)

| Field | Type | Admin Display | Editable? |
|-------|------|---------------|-----------|
| id | TEXT (USR-ULID) | Yes | No |
| email | TEXT | Yes | No (read-only) |
| status | TEXT (active \| disabled) | Yes | No (read-only) |
| role | TEXT (derived from partner_membership + Keycloak) | Yes | No (read-only) |
| display_name | TEXT (from user_profile) | Yes | No (read-only) |
| last_login_at | TIMESTAMPTZ | Yes | No (read-only) |

**Admin-specific rules**:
- Read-only view; no user management actions in this sprint

## State Transitions

### Review Moderation Lifecycle

```
submitted → published
submitted → flagged
published → flagged
flagged → hidden
hidden → (no further transitions in current scope)
flagged → deleted (soft-delete)
```

Invalid transitions (e.g., hidden → published) return `REVIEW_STATE_INVALID` error.

### Station Lifecycle

```
draft → active → inactive → deleted (soft-delete)
draft → active → maintenance → active
```

Admin can set any valid status. Deleting a station is always soft delete.

### Partner Lifecycle

```
active → suspended
active → deleted (soft-delete) — blocked if active stations exist
```

## API Endpoints Consumed

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/api/v1/admin/overview` | Dashboard metric counts |
| GET | `/api/v1/admin/partners` | List partners (paginated) |
| POST | `/api/v1/admin/partners` | Create partner |
| PATCH | `/api/v1/admin/partners/{id}` | Update partner |
| DELETE | `/api/v1/admin/partners/{id}` | Soft-delete partner |
| GET | `/api/v1/admin/stations` | List stations (paginated) |
| PATCH | `/api/v1/admin/stations/{id}` | Update station |
| DELETE | `/api/v1/admin/stations/{id}` | Soft-delete station |
| GET | `/api/v1/admin/reviews` | List reviews (paginated) |
| PATCH | `/api/v1/admin/reviews/{id}/status` | Moderate review status |
| GET | `/api/v1/admin/users` | List users (paginated) |

All endpoints require `admin` role and return standard envelopes (`success`/`error`).
