# API Contract: Stations v1

## GET /api/v1/stations/nearby

Returns EV charging stations within a given radius of a coordinate pair.

### Query Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `lat` | number | yes | — | Center latitude (WGS84) |
| `lng` | number | yes | — | Center longitude (WGS84) |
| `distance` | number | no | 15000 | Search radius in meters |
| `show_staged` | boolean | no | false | Include `is_live=false` stations |

### Example Request

```
GET /api/v1/stations/nearby?lat=36.8065&lng=10.1815&distance=15000
```

### Example Response (200 OK)

```json
[
  {
    "id": "stn-e3b0c442",
    "name": "LES BERGES DU LAC 2 HUB",
    "partner": {
      "id": "prt-a1b2c3d4",
      "name": "TotalEnergies Tunisia",
      "type": "Business"
    },
    "latitude": 36.8324,
    "longitude": 10.2321,
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
    "updated_at": "2026-05-28T09:41:00Z"
  }
]
```

### Error Responses

| Status | Body | When |
|--------|------|------|
| 400 | `{"error":"missing parameter: lat"}` | Required params missing |
| 500 | — | Database connection failure |

---

## PATCH /api/v1/stations/{id}/status

Updates the operational status of a station.

### Request Body

```json
{
  "status": "Occupied"
}
```

### Example Response (200 OK)

```json
{
  "id": "stn-e3b0c442",
  "status": "Occupied",
  "updated_at": "2026-05-28T10:00:00Z"
}
```

### Error Responses

| Status | Body | When |
|--------|------|------|
| 404 | `{"error":"station not found"}` | Invalid station ID |
| 400 | `{"error":"invalid status"}` | Status not in allowed values |

---

## GET /health

Returns the service health status.

### Example Response (200 OK)

```json
{
  "status": "ok",
  "database": "connected"
}
```

### Example Response (503 Service Unavailable)

```json
{
  "status": "degraded",
  "database": "disconnected"
}
```
