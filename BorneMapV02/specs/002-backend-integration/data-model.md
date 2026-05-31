# Data Model: Backend Integration

## Overview

Data entities for the backend API service and mobile app integration. Data originates from in-memory mock data in the Rust backend and is consumed as JSON by the React Native frontend.

## Entities

### Station

A physical EV charging location with geographic coordinates.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string (stn-xxxxxxxx) | yes | Unique station identifier |
| name | string | yes | Display name of the station |
| provider_id | string (prv-xxxxxxxx) | yes | Foreign key to Provider entity |
| provider_name | string | yes | Display name of the operating provider |
| latitude | f64 (WGS 84) | yes | Geographic latitude |
| longitude | f64 (WGS 84) | yes | Geographic longitude |
| status | enum: Available/Occupied | yes | Current operational status |
| chargers | Charger[] | yes | List of charging units at this station |
| updated_at | datetime (ISO 8601) | yes | Last status update timestamp |

**Constraints:**
- ID MUST match pattern `^stn-[a-f0-9]{8}$`
- Coordinates MUST use WGS 84 (EPSG:4326)
- A station MUST have at least one charger
- Status values are limited to "Available" or "Occupied"

### Charger

An individual charging unit at a station.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string (chg-xxxxxxxx) | yes | Unique charger identifier |
| plug_type | string | yes | Connector standard (e.g., CCS2, CHAdeMO, Type 2) |
| power_output | u32 (kW) | yes | Maximum power output in kilowatts |
| status | enum: Available/Occupied | yes | Current operational status |

**Constraints:**
- ID MUST match pattern `^chg-[a-f0-9]{8}$`
- Power output in whole kilowatts (integer)
- Status values are limited to "Available" or "Occupied"

### Provider

The organization operating a charging station.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string (prv-xxxxxxxx) | yes | Unique provider identifier |
| name | string | yes | Display name of the provider |

**Constraints:**
- ID MUST match pattern `^prv-[a-f0-9]{8}$`
- Provider entity is embedded (referenced by provider_id/provider_name in Station) rather than stored separately in v1

## Relationships

```
Provider (1) ──< (N) Station (1) ──< (N) Charger
```

- A Provider operates many Stations
- A Station belongs to exactly one Provider
- A Station has many Chargers
- A Charger belongs to exactly one Station

## Identifiers

All transactional entity IDs follow the `XXX-nanouuid` pattern:

| Prefix | Entity | Example |
|--------|--------|---------|
| `stn-` | Station | `stn-e3b0c442` |
| `chg-` | Charger | `chg-7b2a19f4` |
| `prv-` | Provider | `prv-k9x2m47a` |

Pattern: `^[a-z]{3}-[a-f0-9]{8}$`

## State Transitions

```
Station Status: Available ↔ Occupied
Charger Status: Available ↔ Occupied
```

In v1, status is static (set in mock data). Future versions will support dynamic transitions as chargers are used/released.

## Data Flow

```
[Rust Backend]                       [React Native Frontend]
                                     
AppState (RwLock)                        MapScreen
  │                                        │
  ├── generate_mock_data()                 │
  │     │                                  │
  │     ▼                                  │
  │   Vec<Station>                         │
  │     │                                  │
  │     ▼                                  │
  │   GET /api/v1/stations/nearby          │
  │     │                                  │
  │     └──── JSON ──────────────────────► │
  │                                        ├── setStations(data)
  │                                        ├── map() → Markers
  │                                        └── onPress → StationCard
```
