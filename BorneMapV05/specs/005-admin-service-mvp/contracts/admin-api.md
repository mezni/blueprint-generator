# Admin API Contract

**Base Path**: `/api/v1/admin`
**Auth**: Bearer JWT (admin role required)
**Standard Envelope**:
- Success: `{ "success": true, "data": {...}, "meta": {...} }`
- Error: `{ "success": false, "error": { "code": "STRING", "message": "STRING" } }`

---

## GET /api/v1/admin/users

List all users with pagination.

**Auth**: admin role required

**Query Parameters**:
| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number (1-based) |
| size | integer | 20 | Items per page (1-100) |
| status | string | - | Filter by status (active/disabled) |

**Response 200**:
```json
{
  "success": true,
  "data": [
    {
      "id": "USR-01HXYZ",
      "keycloak_user_id": "abc-123-def",
      "email": "user@example.com",
      "status": "active",
      "created_at": "2026-06-01T00:00:00Z",
      "last_login_at": "2026-06-02T10:00:00Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total": 1, "total_pages": 1, "has_next": false, "has_prev": false }
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403)

---

## GET /api/v1/admin/partners

List all partners with pagination.

**Auth**: admin role required

**Query Parameters**:
| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number |
| size | integer | 20 | Items per page (1-100) |
| include_deleted | boolean | false | Include soft-deleted partners |
| status | string | - | Filter by status (active/suspended) |

**Response 200**:
```json
{
  "success": true,
  "data": [
    {
      "id": "PRT-01HABC",
      "name": "Acme Charging",
      "type": "business",
      "status": "active",
      "created_at": "2026-06-01T00:00:00Z",
      "updated_at": "2026-06-01T00:00:00Z",
      "deleted_at": null
    }
  ],
  "meta": { "page": 1, "size": 20, "total": 1, "total_pages": 1, "has_next": false, "has_prev": false }
}
```

---

## POST /api/v1/admin/partners

Create a new partner.

**Auth**: admin role required

**Request Body**:
```json
{
  "name": "New Partner",
  "type": "business",
  "status": "active"
}
```

**Response 201**:
```json
{
  "success": true,
  "data": {
    "id": "PRT-01HNEW",
    "name": "New Partner",
    "type": "business",
    "status": "active",
    "created_at": "2026-06-02T12:00:00Z",
    "updated_at": "2026-06-02T12:00:00Z",
    "deleted_at": null
  },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `VALIDATION_FAILED` (400)

---

## PATCH /api/v1/admin/partners/{id}

Update a partner.

**Auth**: admin role required

**Headers**:
| Header | Required | Description |
|--------|----------|-------------|
| If-Match | yes | ETag with current `updated_at` value |

**Request Body**: Partial update
```json
{
  "name": "Updated Partner Name",
  "status": "suspended"
}
```

**Response 200**:
```json
{
  "success": true,
  "data": { "...updated partner..." },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404), `CONCURRENT_MODIFICATION` (409), `VALIDATION_FAILED` (400)

---

## DELETE /api/v1/admin/partners/{id}

Soft-delete a partner. Blocked if active stations exist.

**Auth**: admin role required

**Response 200**:
```json
{
  "success": true,
  "data": { "...partner with deleted_at set..." },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404), `ACTIVE_STATIONS_EXIST` (409)

---

## GET /api/v1/admin/stations

List all stations across all partners with pagination.

**Auth**: admin role required

**Query Parameters**:
| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number |
| size | integer | 20 | Items per page (1-100) |
| include_deleted | boolean | false | Include soft-deleted stations |
| status | string | - | Filter by status |
| partner_id | string | - | Filter by partner |

**Response 200**:
```json
{
  "success": true,
  "data": [
    {
      "id": "STN-01HXYZ",
      "partner_id": "PRT-01HABC",
      "name": "Station A",
      "description": "...",
      "latitude": 36.8065,
      "longitude": 10.1815,
      "status": "active",
      "is_live": true,
      "is_public": true,
      "city": "Tunis",
      "country": "TN",
      "created_at": "2026-06-01T00:00:00Z",
      "updated_at": "2026-06-01T00:00:00Z",
      "deleted_at": null
    }
  ],
  "meta": { "page": 1, "size": 20, "total": 1, "total_pages": 1, "has_next": false, "has_prev": false }
}
```

---

## PATCH /api/v1/admin/stations/{id}

Update any station (global scope). Triggers GIS outbox event.

**Auth**: admin role required

**Headers**:
| Header | Required | Description |
|--------|----------|-------------|
| If-Match | yes | ETag with current `updated_at` value |

**Request Body**: Partial update
```json
{
  "status": "maintenance",
  "is_live": false
}
```

**Response 200**:
```json
{
  "success": true,
  "data": { "...updated station..." },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404), `INVALID_COORDINATES` (400), `INVALID_STATE_TRANSITION` (400), `CONCURRENT_MODIFICATION` (409)

---

## DELETE /api/v1/admin/stations/{id}

Soft-delete any station (admin role required).

**Auth**: admin role required

**Response 200**:
```json
{
  "success": true,
  "data": { "...station with deleted_at set..." },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404)

---

## GET /api/v1/admin/reviews

List all reviews with pagination.

**Auth**: admin role required

**Query Parameters**:
| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number |
| size | integer | 20 | Items per page (1-100) |
| status | string | - | Filter by status (published/hidden/flagged/deleted) |
| station_id | string | - | Filter by station |

**Response 200**:
```json
{
  "success": true,
  "data": [
    {
      "id": "REV-01HXYZ",
      "user_id": "USR-01HABC",
      "station_id": "STN-01HDEF",
      "rating": 4,
      "comment": "Great station",
      "status": "published",
      "created_at": "2026-06-01T00:00:00Z",
      "updated_at": "2026-06-01T00:00:00Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total": 1, "total_pages": 1, "has_next": false, "has_prev": false }
}
```

---

## PATCH /api/v1/admin/reviews/{id}/status

Moderate review status.

**Auth**: admin role required

**Request Body**:
```json
{
  "status": "hidden"
}
```

**Valid transitions**:
- `published` → `hidden`, `flagged`
- `flagged` → `hidden`, `published`
- `hidden` → `published`
- `any` → `deleted`

**Response 200**:
```json
{
  "success": true,
  "data": { "...review with updated status..." },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404), `REVIEW_STATE_INVALID` (400)

---

## Canonical Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| UNAUTHENTICATED | 401 | No valid JWT provided |
| TOKEN_EXPIRED | 401 | JWT has expired |
| INSUFFICIENT_ROLE | 403 | User lacks required role |
| FORBIDDEN | 403 | Partner is suspended or action not allowed |
| PARTNER_SCOPE_VIOLATION | 403 | Partner attempting to access another partner's resource |
| NOT_FOUND | 404 | Resource does not exist |
| ACTIVE_STATIONS_EXIST | 409 | Cannot delete partner with active stations |
| CONCURRENT_MODIFICATION | 409 | Optimistic concurrency conflict (If-Match mismatch) |
| ALREADY_EXISTS | 409 | Duplicate resource |
| VALIDATION_FAILED | 400 | Request validation error |
| INVALID_COORDINATES | 400 | Latitude/longitude out of valid range |
| INVALID_STATE_TRANSITION | 400 | Status transition not allowed |
| REVIEW_STATE_INVALID | 400 | Review status transition not allowed |
