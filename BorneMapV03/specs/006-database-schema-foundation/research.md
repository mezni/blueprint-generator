# Research: Database & Schema Foundation

## 1. Migration Tool: sqlx

**Decision**: sqlx 0.8 compile-time checked migrations with `sqlx::migrate!()` called at Admin Service startup.

**Rationale**:
- Rust-native, no external CLI dependency at runtime
- Compile-time SQL verification via `sqlx::prepare` or `cargo sqlx prepare`
- Transactional migrations (each migration wrapped in a DB transaction)
- Migration scripts are plain SQL tracked in `services/admin-service/migrations/`
- `sqlx::migrate!()` runs all pending migrations in order on startup
- No ORM overhead — direct SQL control

**Alternatives considered**:
- Diesel CLI: Requires `diesel` CLI tool; migration system is less ergonomic for pure SQL
- `refinery`: Lighter but less mature than sqlx in the Rust ecosystem
- Custom shell scripts: No compile-time verification, error-prone

**Integration**:
- Add `sqlx` dependency with `runtime-tokio`, `postgres`, `migrate` features to `admin-service`
- Add `sqlx` with `runtime-tokio`, `postgres` features to `common-db`
- `common-db` exports `PgPool` and a `run_migrations()` function
- Admin Service calls `common_db::run_migrations(pool).await` before binding the HTTP listener
- Migration files: `YYYYMMDDHHMMSS_description.sql` format

---

## 2. SQLx Feature Flags

| Feature | Purpose |
|---------|---------|
| `runtime-tokio` | Async runtime support |
| `postgres` | PostgreSQL driver |
| `migrate` | Embedded migration support (admin-service only) |
| `chrono` | Chrono type support |
| `uuid` | UUID type support |

---

## 3. PostgreSQL Schema Ownership Pattern

**Decision**: Each service receives a dedicated PostgreSQL role with schema-scoped privileges.

**Rationale**:
- Connection credentials (role/password) are injected via environment variables per service
- Roles are pre-created in the initial migration and granted only the required schema privileges
- No `GRANT` to `public` — each role explicitly authorized

**Role Structure**:

| Role | Owned Schema | Can Read | Can Write |
|------|-------------|----------|-----------|
| `admin_service` | `inventory` | all schemas | `inventory`, `users` |
| `driver_service` | `users` | all schemas | `users` |
| `gis_worker` | `gis` | `inventory` | `gis` |
| `clickstream_service` | `analytics` | none | `analytics` |
| `analytics_worker` | (shares analytics) | `analytics` | `analytics` |

**Database-level enforcement**:
- `REVOKE ALL ON SCHEMA inventory FROM public;`
- `GRANT USAGE ON SCHEMA inventory TO admin_service;`
- `GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA inventory TO admin_service;`
- Read access to `inventory` for `gis_worker`: `GRANT SELECT ON ALL TABLES IN SCHEMA inventory TO gis_worker;`

**Alternatives considered**:
- Row-level security (RLS): Overkill for schema-level isolation; RLS reserved for multi-tenant data within a table
- Application-layer enforcement: Not reliable — defense in depth requires DB-level enforcement

---

## 4. PostGIS GEOGRAPHY(Point) with SRID 4326

**Decision**: All spatial columns use `GEOGRAPHY(Point, 4326)` with a GIST index.

**Rationale**:
- `GEOGRAPHY` type automatically handles earth curvature calculations
- SRID 4326 (WGS 84) is the standard GPS coordinate system
- GIST index on `GEOGRAPHY(Point)` enables fast `<->` (distance) and `ST_DWithin` queries
- PostGIS `GEOGRAPHY` uses meters for distance calculations natively

**Index pattern**:
```sql
CREATE INDEX idx_station_location ON inventory.station USING GIST (location);
```

**Query pattern**:
```sql
SELECT id, name, location
FROM inventory.station
WHERE ST_DWithin(location, ST_MakePoint(:lng, :lat)::geography, :radius_meters);
```

**Note**: `station.location` in `inventory` is the single source of truth. GIS-derived artifacts (enriched geo data) live in `gis` schema.

---

## 5. Deterministic ID Generation

**Decision**: Use the existing `nanoid` crate (already in `contracts` Cargo.toml) with custom alphabets and prefix enforcement.

**Rationale**:
- `nanoid` already a dependency in the `contracts` crate
- Prefixes (`USR_`, `PRT_`, `STN_`, `CHG_`, `REV_`) ensure human-readable entity identification
- Application-layer generation keeps IDs portable (no DB sequence dependency)

**ID Format**:

| Entity | Prefix | Full Format | Example |
|--------|--------|-------------|---------|
| User | `USR-` | `USR-` + 21 chars | `USR-a1b2c3d4e5f6g7h8i9j0k` |
| Partner | `PRT-` | `PRT-` + 21 chars | `PRT-x1y2z3w4v5u6t7s8r9q0p` |
| Station | `STN-` | `STN-` + 21 chars | `STN-m9n8o7p6q5r4s3t2u1v0w` |
| Charger | `CHG-` | `CHG-` + 21 chars | `CHG-l0k1j2h3g4f5d6s7a8p9o` |
| Review | `REV-` | `REV-` + 21 chars | `REV-zx9cv8bn7m6l5k4j3h2g1` |

**Implementation**: A shared ID module in `common-types` or the `contracts` crate:
```rust
pub fn generate_id(prefix: &str) -> String {
    format!("{}-{}", prefix, nanoid::nanoid!(21))
}
```

---

## 6. Keycloak Identity Mapping

**Decision**: On first authentication, the Auth middleware (existing in `common-auth`) checks for a matching `user_account` record; if absent, it provisions one via a DB insert.

**Rationale**:
- The 1:1 mapping of Keycloak `sub` → `user_account.keycloak_id` means no duplicate accounts
- The mapping happens at the service layer (Admin/Driver Service) after JWT validation
- The Auth middleware already validates the JWT; extending it with a DB lookup is the cleanest integration point

**Flow**:
1. User authenticates via Keycloak → receives access token
2. Request arrives at service with `Authorization: Bearer <token>`
3. Auth middleware validates JWT, extracts `sub` claim
4. Middleware queries `users.user_account` for `keycloak_id = sub`
5. If no record found: INSERT into `users.user_account` with `keycloak_id = sub`
6. If record found: continue (update `last_login_at` timestamp)
7. Attach `UserContext` (with `user_account.id`, roles) to request extensions

**Role synchronization**: Roles are NOT stored in the application database. The middleware extracts `realm_access.roles` from the JWT on every request, ensuring Keycloak remains the single source of truth.

---

## 7. Daily Table Partitioning for Analytics

**Decision**: `analytics.raw_event` uses PostgreSQL declarative partitioning by day on a `created_at` timestamp column.

**Rationale**:
- ~100K events/day → daily partitions yield ~3M rows/month, ~36M/year per partition
- Daily partitioning enables efficient partition pruning for time-range queries
- Old partitions can be DETACHed (archived) without affecting writes to current partition
- PostgreSQL declarative partitioning is built-in and requires no extension

**Schema**:
```sql
CREATE TABLE analytics.raw_event (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    event_type VARCHAR(64) NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

CREATE TABLE analytics.raw_event_20260531 PARTITION OF analytics.raw_event
    FOR VALUES FROM ('2026-05-31') TO ('2026-06-01');
```

**Partition management**: A cron-like scheduler (or a lightweight tokio task in Clickstream Service) creates the next day's partition at midnight.

---

## 8. Existing Infrastructure Assessment

- Docker Compose already has `postgis/postgis:16-3.4-alpine` with healthcheck
- `DATABASE_URL` env var already wired for admin-service, driver-service, gis-sync-worker
- `crates/common-db/` exists as a skeleton — will be populated with pool + migration logic
- `contracts` crate already has nanoid, chrono, uuid dependencies — can host ID generation
- No init scripts exist for the database (suggested: `infra/compose/db/init/01-postgis-init.sql`)

**Init script needed**:
```sql
-- infra/compose/db/init/01-postgis-init.sql
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```
