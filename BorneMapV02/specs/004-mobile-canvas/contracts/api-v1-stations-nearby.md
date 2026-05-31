# API Contract: GET /api/v1/stations/nearby

**Version**: 1.0 | **Last Updated**: 2026-05-28

## Purpose

Returns charging stations within a geographic radius from a given coordinate, with optional staging visibility.

## Request

**Method**: `GET`

**Path**: `/api/v1/stations/nearby`

**Query Parameters**:

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| lat | number | yes | — | Latitude in WGS84 (e.g., 36.8065) |
| lng | number | yes | — | Longitude in WGS84 (e.g., 10.1815) |
| distance | number | no | 15000 | Search radius in meters |
| show_staged | boolean | no | false | Include stations with `is_live = false` |

**Example Request**:

```http
GET /api/v1/stations/nearby?lat=36.8065&lng=10.1815&show_staged=true HTTP/1.1
Host: localhost:8080
Accept: application/json
```

## Response

**Status**: `200 OK`

**Content-Type**: `application/json`

**Body**: Array of station objects:

```json
[
  {
    "id": "stn-e3b0c442",
    "name": "Tunis Station 1",
    "partner": {
      "id": "prt-a1b2c3d4",
      "name": "TotalEnergies Tunisia",
      "type": "Business"
    },
    "latitude": 36.8001,
    "longitude": 10.1887,
    "status": "Available",
    "chargers": [
      {
        "id": "chg-7b2a19f4",
        "plug_type": "CCS2",
        "power_output": 120,
        "status": "Available"
      }
    ],
    "is_live": false,
    "updated_at": "2026-05-28T18:21:53.087506Z"
  }
]
```

### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| id | string | Station nanouuid (`stn-` prefix) |
| name | string | Station display name |
| partner | object | Partner snapshot (id, name, type) |
| latitude | number | WGS84 latitude |
| longitude | number | WGS84 longitude |
| status | string | Station status (Available, Occupied, Offline, Maintenance) |
| chargers | array | List of charger objects at this station |
| is_live | boolean | Whether station is live (non-staged) |
| updated_at | string | ISO 8601 timestamp of last update |

### Charger Object

| Field | Type | Description |
|-------|------|-------------|
| id | string | Charger nanouuid (`chg-` prefix) |
| plug_type | string | Connector type (CCS2, CHAdeMO, Type2) |
| power_output | integer | Power rating in kW |
| status | string | Charger status (Available, Occupied, Offline, Maintenance) |

## Errors

| Status | Condition |
|--------|-----------|
| 400 Bad Request | Missing or invalid query parameters |
| 500 Internal Server Error | Database query failure |

**Error Response** (500):

```json
{
  "error": "internal server error"
}
```

## Status Lifecycle

Valid values for station and charger `status`:

```
Available ←→ Occupied
Available → Offline → Available
Available → Maintenance → Available
```
