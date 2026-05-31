# API Contract: Stations

## Get Nearby Stations

Returns a list of EV charging stations near the default Tunis area with charger details.

### Request

```
GET /api/v1/stations/nearby
```

**Headers:**

| Header | Value | Required |
|--------|-------|----------|
| Accept | application/json | no |

**Query Parameters:** None in v1 (returns all mock stations)

### Response: 200 OK

```json
[
  {
    "id": "stn-e3b0c442",
    "name": "LES BERGES DU LAC 2 HUB",
    "provider_id": "prv-k9x2m47a",
    "provider_name": "TotalEnergies Tunisia",
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
    "updated_at": "2026-05-27T13:00:00Z"
  }
]
```

**Response Schema:**

| Field | Type | Description |
|-------|------|-------------|
| id | string | Station ID (format: `stn-xxxxxxxx`) |
| name | string | Station display name |
| provider_id | string | Provider ID (format: `prv-xxxxxxxx`) |
| provider_name | string | Provider display name |
| latitude | number | WGS 84 latitude |
| longitude | number | WGS 84 longitude |
| status | string | "Available" or "Occupied" |
| chargers | Charger[] | Array of charger objects |
| updated_at | string | ISO 8601 timestamp |

**Charger Object:**

| Field | Type | Description |
|-------|------|-------------|
| id | string | Charger ID (format: `chg-xxxxxxxx`) |
| plug_type | string | Connector standard (e.g., "CCS2") |
| power_output | number | Power in kW |
| status | string | "Available" or "Occupied" |

### Response: Error

| Status | Body | Description |
|--------|------|-------------|
| 500 | Internal Server Error | Server-side failure |
| N/A | Connection refused | Backend not running |

### Example Usage

```bash
curl http://localhost:8080/api/v1/stations/nearby
```

### Version History

| Version | Date | Description |
|---------|------|-------------|
| 1.0.0 | 2026-05-27 | Initial release — returns all mock stations |
