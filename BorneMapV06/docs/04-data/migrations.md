# Migrations

Location: `db/migrations/`

## Migration Files

| File | Description |
|------|-------------|
| `0001_extensions.sql` | PostgreSQL extensions (PostGIS, UUID, etc.) |
| `0002_inventory_schema.sql` | Create inventory schema |
| `0003_users_schema.sql` | Create users schema |
| `0004_gis_schema.sql` | Create gis schema |
| `0005_analytics_schema.sql` | Create analytics database objects |
| `0006_inventory_tables.sql` | Station, charger, availability tables |
| `0007_users_tables.sql` | Profile, favorite, review tables |
| `0008_gis_tables.sql` | GIS spatial tables |
| `0009_analytics_tables.sql` | Analytics event tables |
| `0010_indexes.sql` | Performance indexes |
| `0011_outbox_table.sql` | Outbox table for GIS sync |

## Seed Data

| File | Description |
|------|-------------|
| `seeds/dev_partners.sql` | Development partner organizations |
| `seeds/dev_stations.sql` | Development stations |
| `seeds/dev_chargers.sql` | Development chargers |
