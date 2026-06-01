# Service Matrix

## Purpose

Define per-service responsibilities, owned tables, and technology stack.
Each service row defines its exclusive ownership scope.

## Version

1.0.0

## Matrix

| Service | Owned Schema | Owned Tables | Tech Stack | Entrypoint |
|---------|-------------|--------------|------------|------------|
| Keycloak | — | — | Keycloak 24+ | `/auth/` |
| Admin Service | `inventory` | `partner`, `station`, `charger`, `station_availability` | Rust + Axum | `/admin/` |
| Driver Service | `users` (co-owned) | None exclusively | Rust + Axum | `/driver/` |
| Clickstream Service | — (writes to RMQ) | — | Rust + Axum | `/events/` |
| GIS Sync Worker | `gis` | `roads`, `boundaries`, `station_geospatial_view` | Rust + worker | Internal |
| All services | `users` (Driver + Admin) | `user_account`, `user_profile`, `partner_membership`, `favorite_station`, `station_review` | — | — |

## Partner Scoping

All partner data queries MUST filter by `partner_id` at the repository level.
No exceptions at the API layer. Violation = architectural defect.
