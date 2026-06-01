# Contract: Schema Ownership

## Status

**Adopted**: 2026-05-31
**Scope**: Database schema access control across all services
**Owner**: Platform Architecture

## Principle

Each PostgreSQL schema has exactly one owning service role. Cross-schema writes are FORBIDDEN. Cross-schema reads are permitted where explicitly authorized.

## Ownership Matrix

| Schema | Owning Role | Can Write | Can Read (all) | Read-Only Exceptions |
|--------|-------------|-----------|----------------|---------------------|
| `inventory` | `admin_service` | Admin Service | Admin Service | GIS Sync Worker (read station for enrichment) |
| `users` | `driver_service` | Driver Service, Admin Service | Driver Service, Admin Service | — |
| `gis` | `gis_worker` | GIS Sync Worker | GIS Sync Worker, Admin Service, Driver Service | — |
| `analytics` | `clickstream_service` | Clickstream Service, analytics workers | Admin Service (for reporting) | — |

## Enforcement

1. **Database roles**: Each service connects with a dedicated PostgreSQL role.
2. **Credential isolation**: Connection strings with different roles are injected per service via environment variables.
3. **Privilege grants**: Each role is GRANTed only the privileges it needs.
4. **No PUBLIC grants**: All schemas REVOKE ALL FROM PUBLIC.
5. **Defense in depth**: Service-layer authorization acts as secondary enforcement.

## Role Definitions

```sql
-- Create roles
CREATE ROLE admin_service WITH LOGIN PASSWORD ':admin_password';
CREATE ROLE driver_service WITH LOGIN PASSWORD ':driver_password';
CREATE ROLE gis_worker WITH LOGIN PASSWORD ':gis_password';
CREATE ROLE clickstream_service WITH LOGIN PASSWORD ':clickstream_password';

-- Revoke public access
REVOKE ALL ON SCHEMA inventory, users, gis, analytics FROM PUBLIC;

-- Admin Service: owns inventory, writes users
GRANT ALL ON SCHEMA inventory TO admin_service;
GRANT ALL ON ALL TABLES IN SCHEMA inventory TO admin_service;
GRANT USAGE ON SCHEMA users TO admin_service;
GRANT INSERT, UPDATE, SELECT ON ALL TABLES IN SCHEMA users TO admin_service;
GRANT USAGE ON SCHEMA gis TO admin_service;
GRANT SELECT ON ALL TABLES IN SCHEMA gis TO admin_service;
GRANT USAGE ON SCHEMA analytics TO admin_service;
GRANT SELECT ON ALL TABLES IN SCHEMA analytics TO admin_service;

-- Driver Service: owns users
GRANT ALL ON SCHEMA users TO driver_service;
GRANT ALL ON ALL TABLES IN SCHEMA users TO driver_service;
GRANT USAGE ON SCHEMA inventory TO driver_service;
GRANT SELECT ON inventory.station, inventory.charger, inventory.review TO driver_service;
GRANT USAGE ON SCHEMA gis TO driver_service;
GRANT SELECT ON ALL TABLES IN SCHEMA gis TO driver_service;

-- GIS Worker: owns gis, reads inventory
GRANT ALL ON SCHEMA gis TO gis_worker;
GRANT ALL ON ALL TABLES IN SCHEMA gis TO gis_worker;
GRANT USAGE ON SCHEMA inventory TO gis_worker;
GRANT SELECT ON inventory.station TO gis_worker;

-- Clickstream Service: owns analytics
GRANT ALL ON SCHEMA analytics TO clickstream_service;
GRANT ALL ON ALL TABLES IN SCHEMA analytics TO clickstream_service;
```

## Future Defaults

Any new schema added to the database MUST:
1. Be documented in this contract.
2. Have a designated owning service role.
3. Be added to the REVOKE PUBLIC statement.
4. Receive explicit GRANT statements per the ownership matrix.
