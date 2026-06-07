# API Contracts: Sprint 11 — Admin Dashboard

All admin endpoints are hosted by `admin-service` and accessed via Traefik at `/api/v1/admin/*`.

## Base URL

```
http://localhost/api/v1/admin
```

## Authentication

All endpoints require `Authorization: Bearer <JWT>` header with Keycloak-issued token containing the `admin` role.

## Standard Envelopes

**Success**:
```json
{
  "success": true,
  "data": {},
  "meta": {
    "page": 1,
    "size": 20,
    "total": 100,
    "total_pages": 5,
    "has_next": true,
    "has_prev": false
  }
}
```

**Error**:
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable message",
    "details": {}
  }
}
```

## Endpoints

### Dashboard Overview

```
GET /admin/overview
```

Response `data`:
```json
{
  "total_partners": 42,
  "total_stations": 156,
  "active_stations": 98,
  "pending_reviews": 12,
  "recent_activity": []
}
```

### Partners

```
GET /admin/partners?page=1&size=20&search=&status=
```

Response `data`:
```json
[
  {
    "id": "PRT-01ABCDEF",
    "name": "GreenCharge Tunisie",
    "type": "business",
    "status": "active",
    "station_count": 12,
    "created_at": "2026-01-15T10:00:00Z",
    "deleted_at": null
  }
]
```

```
POST /admin/partners
```
Request body:
```json
{
  "name": "New Partner",
  "type": "business",
  "status": "active"
}
```

Headers: `Idempotency-Key: <uuid>` (required)

```
PATCH /admin/partners/{id}
```
Request body (partial):
```json
{
  "name": "Updated Name",
  "status": "suspended"
}
```

```
DELETE /admin/partners/{id}
```

Response: `204 No Content` on success. Blocks with `409 ACTIVE_STATIONS_EXIST` if partner has active stations.

### Stations

```
GET /admin/stations?page=1&size=20&partner_id=&status=&city=&show_deleted=false
```

Response `data`:
```json
[
  {
    "id": "STN-01ABCDEF",
    "partner_id": "PRT-01ABCDEF",
    "partner_name": "GreenCharge Tunisie",
    "name": "Station Downtown",
    "status": "active",
    "is_live": true,
    "is_public": true,
    "city": "Tunis",
    "latitude": 36.8065,
    "longitude": 10.1815,
    "chargers": [
      {
        "id": "CHG-01ABCDEF",
        "type": "CCS",
        "power_kw": 150,
        "status": "available"
      }
    ],
    "created_at": "2026-01-15T10:00:00Z",
    "deleted_at": null
  }
]
```

Note: `chargers` array may be returned inline or fetched separately depending on backend implementation.

```
PATCH /admin/stations/{id}
```
Headers: `If-Match: <version>` (optional, for optimistic concurrency)
Request body (partial):
```json
{
  "status": "maintenance",
  "is_live": true,
  "is_public": true,
  "name": "Updated Station Name",
  "description": "Updated description",
  "latitude": 36.8070,
  "longitude": 10.1820
}
```

```
DELETE /admin/stations/{id}
```
Response: `204 No Content`

### Reviews

```
GET /admin/reviews?page=1&size=20&status=&station_id=
```

Response `data`:
```json
[
  {
    "id": "REV-01ABCDEF",
    "station_id": "STN-01ABCDEF",
    "station_name": "Station Downtown",
    "user_id": "USR-01ABCDEF",
    "user_email": "user@example.com",
    "rating": 4,
    "comment": "Great charging station!",
    "status": "published",
    "created_at": "2026-02-10T14:30:00Z"
  }
]
```

```
PATCH /admin/reviews/{id}/status
```
Request body:
```json
{
  "status": "hidden"
}
```

### Users

```
GET /admin/users?page=1&size=20&search=
```

Response `data`:
```json
[
  {
    "id": "USR-01ABCDEF",
    "email": "user@example.com",
    "status": "active",
    "role": "registered_driver",
    "display_name": "John Doe",
    "last_login_at": "2026-03-01T08:00:00Z"
  }
]
```

## Error Codes Specific to Admin

| Code | Meaning |
|------|---------|
| `ACTIVE_STATIONS_EXIST` | Cannot delete partner with active stations |
| `INSUFFICIENT_ROLE` | User lacks `admin` role |
| `REVIEW_STATE_INVALID` | Invalid review status transition |
| `INVALID_STATE_TRANSITION` | Invalid station lifecycle transition |

Standard error codes: `UNAUTHENTICATED`, `FORBIDDEN`, `TOKEN_EXPIRED`, `NOT_FOUND`, `VALIDATION_FAILED`, `CONFLICT`.
