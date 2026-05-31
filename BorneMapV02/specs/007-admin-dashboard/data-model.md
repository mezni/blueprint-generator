# Data Model: Admin Dashboard

## Entities

### Partner
| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique identifier (mock: `prt-*` prefix; production: `prt-` + nanouuid) |
| name | string | Brand entity name (e.g. "TotalEnergies Tunisia") |
| hubs | number | Number of station hubs operated by this partner |
| status | string | `"Active"` or `"Inactive"` |

**Relationships**: A Partner has zero or more Stations. Referenced by Station.partner.

### Station
| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique identifier (mock: `stn-*` prefix; production: `stn-` + nanouuid) |
| name | string | Station display name (e.g. "Les Berges du Lac 2 Hub") |
| latitude | number | Latitude coordinate (WGS84) |
| longitude | number | Longitude coordinate (WGS84) |
| partner | object | Reference to Partner `{ id, name }` |
| chargers | array | List of Charger objects |
| location | string | (Admin only) Zonal placement city name |
| navigate_url | string | (Optional) Deep link URL for navigation apps |

**Validation**: `latitude` ±90, `longitude` ±180. At least one charger required.

### Charger
| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique identifier |
| plug_type | string | `"CCS2"`, `"Type2"`, or `"CHAdeMO"` |
| power_output | number | Power rating in kW |
| status | string | `"Available"` or `"Occupied"` |

## State Transitions

- **Charger status**: `Available` ↔ `Occupied` (toggled in mock data)
- **Partner status**: `Active` → `Inactive` (static in mock data)

## Mock Data

```javascript
// Partners and stations data lives in apps/admin-dashboard/src/data/mockData.js
// Map portal mock data is inline within each component per the reference code
```
