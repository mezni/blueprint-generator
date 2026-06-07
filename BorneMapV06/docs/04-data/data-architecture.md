# Data Architecture

## Database Model

Single PostgreSQL instance with logical schema separation:

| Database | Schema | Purpose |
|----------|--------|---------|
| `platform_db` | `inventory` | Stations, chargers, availability |
| `platform_db` | `users` | Profiles, favorites, reviews |
| `platform_db` | `gis` | Derived spatial data (asynchronously synced) |
| `analytics_db` | `public` | Clickstream events, aggregated analytics |
| `keycloak_db` | `public` | Identity data (Keycloak managed only) |

## Source of Truth

| Entity | Location |
|--------|----------|
| Stations | `inventory.station` |
| Chargers | `inventory.charger` |
| Availability | `inventory.station_availability` |
| Reviews | `users.station_review` |
| Favorites | `users.favorite_station` |
| GIS data | Derived — NOT a source of truth |
| Analytics | `analytics_db` — separate database |
| Identity | `keycloak_db` — Keycloak managed only |
