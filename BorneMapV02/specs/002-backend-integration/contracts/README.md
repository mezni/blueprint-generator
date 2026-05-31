# Interface Contracts: Backend Integration

This directory documents external API contracts exposed by this feature.

## Contracts

| Contract | Description | Version |
|----------|-------------|---------|
| [api-stations.md](./api-stations.md) | REST API for EV charging station data | 1.0.0 |

## Design Principles

- All endpoints use `/api/v1/` prefix for versioning
- Request/response payloads use JSON
- Identifiers follow `XXX-nanouuid` pattern (`^[a-z]{3}-[a-f0-9]{8}$`)
- Geographic coordinates use WGS 84 (EPSG:4326)
- Timestamps use ISO 8601 format (UTC)
