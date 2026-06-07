# Partner API Contract

**Base Path**: `/api/v1/partner`
**Auth**: Bearer JWT (partner role required)
**Standard Envelope**:
- Success: `{ "success": true, "data": {...}, "meta": {...} }`
- Error: `{ "success": false, "error": { "code": "STRING", "message": "STRING" } }`

---

## GET /api/v1/partner/me

Returns the authenticated partner's profile and membership info.

**Auth**: partner role required

**Response 200**:
```json
{
  "success": true,
  "data": {
    "user_id": "USR-01HXYZ",
    "email": "partner@example.com",
    "partner_id": "PRT-01HABC",
    "partner_name": "Acme Charging",
    "membership_role": "owner"
  },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403)

---

## GET /api/v1/partner/stations

List stations owned by the authenticated partner.

**Auth**: partner role required

**Query Parameters**:
| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number (1-based) |
| size | integer | 20 | Items per page (1-100) |
| include_deleted | boolean | false | Include soft-deleted stations |
| status | string | - | Filter by status |

**Response 200**:
```json
{
  "success": true,
  "data": [
    {
      "id": "STN-01HXYZ",
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
      "updated_at": "2026-06-01T00:00:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "size": 20,
    "total": 1,
    "total_pages": 1,
    "has_next": false,
    "has_prev": false
  }
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403)

---

## POST /api/v1/partner/stations

Create a station under the authenticated partner's ownership.

**Auth**: partner role required

**Headers**:
| Header | Required | Description |
|--------|----------|-------------|
| Idempotency-Key | yes | Unique key to prevent duplicate creation |

**Request Body**:
```json
{
  "name": "New Station",
  "description": "Optional description",
  "latitude": 36.8065,
  "longitude": 10.1815,
  "status": "draft",
  "is_live": false,
  "is_public": false,
  "city": "Tunis",
  "country": "TN"
}
```

**Response 201**:
```json
{
  "success": true,
  "data": {
    "id": "STN-01HNEW",
    "partner_id": "PRT-01HABC",
    "name": "New Station",
    "description": "Optional description",
    "latitude": 36.8065,
    "longitude": 10.1815,
    "status": "draft",
    "is_live": false,
    "is_public": false,
    "city": "Tunis",
    "country": "TN",
    "created_at": "2026-06-02T12:00:00Z",
    "updated_at": "2026-06-02T12:00:00Z"
  },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `INVALID_COORDINATES` (400), `VALIDATION_FAILED` (400), `FORBIDDEN` (403, partner suspended)

---

## PATCH /api/v1/partner/stations/{id}

Update a station owned by the authenticated partner.

**Auth**: partner role required

**Headers**:
| Header | Required | Description |
|--------|----------|-------------|
| If-Match | yes | ETag with current `updated_at` value |

**Request Body**: Partial update (only included fields are modified)
```json
{
  "name": "Updated Name",
  "status": "active",
  "is_live": true
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

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404), `PARTNER_SCOPE_VIOLATION` (403), `INVALID_COORDINATES` (400), `INVALID_STATE_TRANSITION` (400), `CONCURRENT_MODIFICATION` (409)

---

## DELETE /api/v1/partner/stations/{id}

Soft-delete a station owned by the authenticated partner.

**Auth**: partner role required

**Response 200**:
```json
{
  "success": true,
  "data": { "...station with deleted_at set..." },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404), `PARTNER_SCOPE_VIOLATION` (403)

---

## GET /api/v1/partner/chargers

List chargers at stations owned by the authenticated partner.

**Auth**: partner role required

**Query Parameters**:
| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number |
| size | integer | 20 | Items per page (1-100) |
| station_id | string | - | Filter by station |

**Response 200**:
```json
{
  "success": true,
  "data": [
    {
      "id": "CHG-01HXYZ",
      "station_id": "STN-01HABC",
      "type": "CCS",
      "power_kw": 50.0,
      "status": "available",
      "created_at": "2026-06-01T00:00:00Z",
      "updated_at": "2026-06-01T00:00:00Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total": 1, "total_pages": 1, "has_next": false, "has_prev": false }
}
```

---

## POST /api/v1/partner/chargers

Create a charger at a station owned by the authenticated partner.

**Auth**: partner role required

**Request Body**:
```json
{
  "station_id": "STN-01HABC",
  "type": "CCS",
  "power_kw": 50.0,
  "status": "available"
}
```

**Response 201**:
```json
{
  "success": true,
  "data": {
    "id": "CHG-01HNEW",
    "station_id": "STN-01HABC",
    "type": "CCS",
    "power_kw": 50.0,
    "status": "available",
    "created_at": "2026-06-02T12:00:00Z",
    "updated_at": "2026-06-02T12:00:00Z"
  },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `PARTNER_SCOPE_VIOLATION` (403, station not owned), `NOT_FOUND` (404, station doesn't exist), `VALIDATION_FAILED` (400)

---

## PATCH /api/v1/partner/chargers/{id}

Update a charger at a station owned by the authenticated partner.

**Auth**: partner role required

**Headers**:
| Header | Required | Description |
|--------|----------|-------------|
| If-Match | yes | ETag with current `updated_at` value |

**Request Body**: Partial update
```json
{
  "status": "offline",
  "power_kw": 75.0
}
```

**Response 200**:
```json
{
  "success": true,
  "data": { "...updated charger..." },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404), `PARTNER_SCOPE_VIOLATION` (403), `CONCURRENT_MODIFICATION` (409)

---

## PATCH /api/v1/partner/stations/{id}/availability

Update station availability for a station owned by the authenticated partner.

**Auth**: partner role required

**Request Body**:
```json
{
  "status": "limited"
}
```

**Response 200**:
```json
{
  "success": true,
  "data": {
    "id": "...",
    "station_id": "STN-01HABC",
    "status": "limited",
    "source": "manual_partner",
    "updated_at": "2026-06-02T12:00:00Z"
  },
  "meta": {}
}
```

**Errors**: `UNAUTHENTICATED` (401), `INSUFFICIENT_ROLE` (403), `NOT_FOUND` (404), `PARTNER_SCOPE_VIOLATION` (403), `VALIDATION_FAILED` (400)
