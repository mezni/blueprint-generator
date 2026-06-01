# Architecture Contract

## Purpose

Define the service boundaries, communication model, and architectural
invariants of the BorneMap platform. Every future EPIC must conform to these
boundaries.

## Version

1.0.0 — Derived from constitution v1.0.0 (Specify) / v5.1 (Platform)

## Services

| Service | Role | DB Access | Public-Facing |
|---------|------|-----------|---------------|
| Keycloak | Identity provider — auth, tokens, sessions, OAuth, roles | None (federated) | Via Traefik |
| Admin Service | System of record for `inventory` schema | `inventory` (sole writer), `users` (read) | Via Traefik |
| Driver Service | Discovery + user actions | `users` (shared write), `inventory` (read) | Via Traefik |
| Clickstream Service | Event ingestion → RabbitMQ | None (writes to RMQ only) | Via Traefik |
| GIS Sync Worker | Derive spatial artifacts from `inventory.station` | `gis` (sole writer), `inventory` (read) | Internal |
| Traefik | Edge proxy — single public entrypoint | None | Yes (port 80/443) |

## Communication Rules

- **Allowed**: REST (frontend ↔ services), RabbitMQ (async events),
  DB access within owning service
- **Forbidden**: Cross-service DB access, `inventory` writes outside Admin,
  `gis` writes outside GIS Worker

## Architectural Invariant

`inventory.station` is the single source of truth for physical infrastructure.
All other systems (GIS, analytics) are derived projections. GIS NEVER writes
back to inventory.

## Enforcement

All service PRs must be reviewed for DB access scope compliance. Automated CI
must include a step verifying no service accesses a schema it does not own.
