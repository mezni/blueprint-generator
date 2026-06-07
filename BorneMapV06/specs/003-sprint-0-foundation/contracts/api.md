# API Contracts: Sprint 0 Foundation

**Date**: 2026-06-05  
**Plan**: [plan.md](../plan.md)  
**Scope**: Public endpoints and interface contracts (Sprint 0 stubs only; real implementations in Sprint 2)

---

## Overview

Sprint 0 establishes the **public API foundation** with one fully functional endpoint and stubs for four discovery endpoints. These contracts define the interfaces; implementations come in Sprint 2.

**Authentication**: Sprint 0 endpoints require NO authentication (public access).  
**Base URL**: `http://localhost:8000` (development)  
**Content-Type**: All responses are `application/json`

---

## Endpoints

### 1. Health Check (Implemented Sprint 0)

**Path**: `GET /health`

**Description**: Service health status. Used by Docker health checks and monitoring.

**Request**:
```http
GET /health HTTP/1.1
Host: localhost:8000
```

**Response (200 OK)**:
```json
{
  "status": "ok"
}
```

**Implementation**: Implemented in Sprint 0 (required for Docker compose health checks).

---

### 2. Nearby Stations (Stub Sprint 0, Implementation Sprint 2)

**Path**: `GET /stations/nearby`

**Description**: Find charging stations within a given radius of a location.

**Query Parameters**:
| Parameter | Type | Required | Default | Constraints |
|-----------|------|----------|---------|------------|
| `lat` | float | Yes | — | -90.0 to 90.0 |
| `lng` | float | Yes | — | -180.0 to 180.0 |
| `radius_km` | float | No | 10 | > 0 |
| `limit` | integer | No | 20 | 1-100 |

**Request Example**:
```http
GET /stations/nearby?lat=36.806389&lng=10.181667&radius_km=25&limit=50
```

**Response (200 OK)**:
```json
{
  "stations": [
    {
      "id": "STN-a1b2c3d4e5f6g7h8",
      "name": "Tunis Central Hub",
      "address": "123 Avenue Habib Bourguiba, Tunis",
      "latitude": 36.806389,
      "longitude": 10.181667,
      "distance_m": 1250,
      "charger_count": 8,
      "available_count": 5
    },
    {
      "id": "STN-z9y8x7w6v5u4t3s2",
      "name": "Sfax Station 1",
      "address": "456 Route Aéroport, Sfax",
      "latitude": 34.740833,
      "longitude": 10.761111,
      "distance_m": 18500,
      "charger_count": 4,
      "available_count": 2
    }
  ],
  "query": {
    "latitude": 36.806389,
    "longitude": 10.181667,
    "radius_km": 25
  }
}
```

**Response (400 Bad Request)**:
```json
{
  "error": "invalid_params",
  "message": "latitude must be between -90 and 90"
}
```

**Response (503 Service Unavailable)** (database error):
```json
{
  "error": "database_error",
  "message": "Failed to query stations"
}
```

**Status Codes**:
- `200 OK` — Request successful, results returned
- `400 Bad Request` — Invalid parameters (lat/lng out of range, invalid types)
- `503 Service Unavailable` — Database or service error

**Implementation Status**: Stub in Sprint 0; real implementation in Sprint 2.

---

### 3. Map Markers (Stub Sprint 0, Implementation Sprint 2)

**Path**: `GET /stations/markers`

**Description**: Lightweight marker data for map rendering. Returns minimal payload (ID, location, availability count).

**Query Parameters**:
| Parameter | Type | Required | Constraints |
|-----------|------|----------|------------|
| `bbox` | string | Yes | Format: `min_lat,min_lng,max_lat,max_lng` |

**Request Example**:
```http
GET /stations/markers?bbox=33.5,8.5,37.5,12.5
```

**Response (200 OK)**:
```json
{
  "markers": [
    {
      "id": "STN-a1b2c3d4e5f6g7h8",
      "latitude": 36.806389,
      "longitude": 10.181667,
      "available_count": 5
    },
    {
      "id": "STN-z9y8x7w6v5u4t3s2",
      "latitude": 34.740833,
      "longitude": 10.761111,
      "available_count": 2
    }
  ]
}
```

**Response (400 Bad Request)**:
```json
{
  "error": "invalid_bbox",
  "message": "bbox must be: min_lat,min_lng,max_lat,max_lng"
}
```

**Status Codes**:
- `200 OK` — Request successful
- `400 Bad Request` — Invalid bbox format
- `503 Service Unavailable` — Database error

**Implementation Status**: Stub in Sprint 0; real implementation in Sprint 2.

---

### 4. Search Stations (Stub Sprint 0, Implementation Sprint 2)

**Path**: `GET /stations/search`

**Description**: Full-text search on station name/address with optional filters.

**Query Parameters**:
| Parameter | Type | Required | Default | Constraints |
|-----------|------|----------|---------|------------|
| `q` | string | Yes | — | Min 2 chars, max 255 |
| `lat` | float | No | — | -90 to 90 (for distance sorting) |
| `lng` | float | No | — | -180 to 180 |
| `connector_type` | string | No | — | CCS2, Type2, TeslaSupercharger |
| `min_power_kw` | float | No | — | > 0 |
| `limit` | integer | No | 20 | 1-100 |
| `offset` | integer | No | 0 | ≥ 0 |

**Request Example**:
```http
GET /stations/search?q=tunis&limit=10&offset=0&connector_type=CCS2&min_power_kw=50
```

**Response (200 OK)**:
```json
{
  "stations": [
    {
      "id": "STN-a1b2c3d4e5f6g7h8",
      "name": "Tunis Central Hub",
      "address": "123 Avenue Habib Bourguiba, Tunis",
      "latitude": 36.806389,
      "longitude": 10.181667,
      "distance_m": null,
      "charger_count": 8,
      "available_count": 5
    }
  ],
  "pagination": {
    "offset": 0,
    "limit": 10,
    "total": 1,
    "has_more": false
  }
}
```

**Response (400 Bad Request)**:
```json
{
  "error": "invalid_query",
  "message": "search query must be at least 2 characters"
}
```

**Status Codes**:
- `200 OK` — Request successful
- `400 Bad Request` — Invalid query or parameters
- `503 Service Unavailable` — Database error

**Implementation Status**: Stub in Sprint 0; real implementation in Sprint 2.

---

### 5. Station Detail (Stub Sprint 0, Implementation Sprint 2)

**Path**: `GET /stations/{id}`

**Description**: Complete details for a single station including charger list and ratings.

**Path Parameters**:
| Parameter | Type | Required | Constraints |
|-----------|------|----------|------------|
| `id` | string | Yes | Valid STN- prefixed NanoID |

**Request Example**:
```http
GET /stations/STN-a1b2c3d4e5f6g7h8
```

**Response (200 OK)**:
```json
{
  "station": {
    "id": "STN-a1b2c3d4e5f6g7h8",
    "name": "Tunis Central Hub",
    "address": "123 Avenue Habib Bourguiba, Tunis",
    "latitude": 36.806389,
    "longitude": 10.181667,
    "chargers": [
      {
        "id": "CHG-x1x2x3x4x5x6x7x8",
        "connector_type": "CCS2",
        "power_kw": 150,
        "status": "available"
      },
      {
        "id": "CHG-y1y2y3y4y5y6y7y8",
        "connector_type": "Type2",
        "power_kw": 22,
        "status": "in_use"
      }
    ],
    "rating": {
      "average": 4.5,
      "review_count": 42
    }
  }
}
```

**Response (404 Not Found)**:
```json
{
  "error": "station_not_found",
  "message": "Station with ID STN-notexistent not found"
}
```

**Response (400 Bad Request)**:
```json
{
  "error": "invalid_id",
  "message": "ID must be a valid STN- prefixed NanoID"
}
```

**Status Codes**:
- `200 OK` — Request successful
- `400 Bad Request` — Invalid station ID format
- `404 Not Found` — Station does not exist
- `503 Service Unavailable` — Database error

**Implementation Status**: Stub in Sprint 0; real implementation in Sprint 2.

---

## Error Response Format

All error responses follow this structure:

```json
{
  "error": "error_code",
  "message": "Human-readable error message",
  "details": {
    "field": "additional context"
  }
}
```

**Common Error Codes**:
- `invalid_params` — One or more query parameters are invalid
- `invalid_body` — Request body is malformed
- `database_error` — Database operation failed
- `not_found` — Resource not found (404)
- `unauthorized` — Missing or invalid authentication (401) — not used Sprint 0
- `forbidden` — User lacks permission (403) — not used Sprint 0
- `internal_error` — Unexpected server error (500)

---

## Response Codes Summary

| Code | Meaning | Scenarios |
|------|---------|-----------|
| 200 | OK | Request succeeded |
| 400 | Bad Request | Invalid parameters, malformed request |
| 404 | Not Found | Resource doesn't exist |
| 500 | Internal Server Error | Unhandled exception |
| 503 | Service Unavailable | Database down, service starting up |

---

## Data Types & Validation

### Station Summary
```typescript
{
  id: string,              // STN-xxxxxxxxxxxxxxxx (16 chars)
  name: string,            // Non-empty, max 255 chars
  address: string,         // Optional, max 1000 chars
  latitude: number,        // -90.0 to 90.0 (7 decimal places)
  longitude: number,       // -180.0 to 180.0 (7 decimal places)
  distance_m: number | null,     // Distance from query point (meters)
  charger_count: integer,  // Total chargers at station
  available_count: integer // Chargers with status='available'
}
```

### Charger
```typescript
{
  id: string,              // CHG-xxxxxxxxxxxxxxxx (16 chars)
  connector_type: string,  // CCS2, Type2, TeslaSupercharger, etc.
  power_kw: number | null, // NULL if unknown
  status: string           // available, in_use, maintenance, offline
}
```

### Rating
```typescript
{
  average: number | null,  // 0-5 star rating (null if no reviews)
  review_count: integer    // Total reviews for this station
}
```

---

## Pagination

Paginated endpoints use `offset` and `limit` parameters:

```json
{
  "stations": [...],
  "pagination": {
    "offset": 0,
    "limit": 20,
    "total": 150,
    "has_more": true
  }
}
```

- `offset`: 0-indexed starting position
- `limit`: Number of results per page (1-100)
- `total`: Total available results
- `has_more`: Whether more results exist beyond this page

---

## Geospatial Coordinates

All lat/lng coordinates use **WGS84** (EPSG:4326):
- Latitude range: -90.0 (South Pole) to 90.0 (North Pole)
- Longitude range: -180.0 (Dateline) to 180.0 (Dateline)
- Precision: Up to 7 decimal places ≈ 1.1 cm accuracy

**Bounding Box (bbox) format**: `min_lat,min_lng,max_lat,max_lng`

Example (Tunisia):
```
bbox=33.5,8.5,37.5,12.5
```

---

## Rate Limiting (Future)

Sprint 0 has NO rate limiting. Rate limiting will be added in future sprints.

---

## Versioning (Future)

Sprint 0 API is v1 (implicit). Future breaking changes will use URL versioning (`/v2/stations/...`).

---

## Related Documentation

- **Data Model**: [data-model.md](../data-model.md)
- **Docker Contract**: [docker-compose.md](docker-compose.md)
- **Implementation Plan**: [../plan.md](../plan.md)
