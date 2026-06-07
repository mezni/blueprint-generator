# Research: Admin Service MVP

**Branch**: `005-admin-service-mvp` | **Date**: 2026-06-02

## Research Items

### R-001: sqlx PgPool Setup and Migration Strategy

**Decision**: Use `sqlx::PgPool` with `sqlx-cli` for migrations (already used for Sprint 4 SQL files).

**Rationale**: sqlx is the idiomatic Rust PostgreSQL driver with compile-time query checking. The project already has `migrations/` directory with sqlx-compatible file naming (`NNNN_name.up.sql`/`.down.sql`). PgPool provides async connection pooling. sqlx-cli can run migrations programmatically or externally.

**Alternatives considered**:
- *diesel*: ORM overhead, not async-native, schema-first approach conflicts with raw SQL migrations already in place.
- *sea-orm*: Additional abstraction layer unnecessary for this service size; raw SQL is already written.
- *tokio-postgres directly*: No compile-time query checking, manual connection pool management.

**Implementation notes**:
- `common-db::init_pool(database_url: &str) -> PgPool` reads `PLATFORM_DB_*` env vars.
- `common-db::run_migrations(pool: &PgPool)` uses `sqlx::migrate!()` macro pointed at `services/admin-service/migrations/`.
- Migrations run before service startup (per Constitution §1.6), but the service provides a helper.
- Use `sqlx::query_as()` with `FromRow` derives for type-safe queries. Enable `offline` mode for CI builds without DB.

### R-002: ULID Generation in Rust

**Decision**: Use the `ulid` crate with `EntityPrefix` from `common-types` to generate `{PREFIX}-{ULID}` strings.

**Rationale**: The `ulid` crate provides standard ULID generation (48-bit timestamp + 80-bit randomness). Combined with `EntityPrefix::as_str()` from `common-types`, this produces the required `STN-01HXYZ...`, `PRT-01HXYZ...`, `CHG-01HXYZ...` format. No custom crypto needed — `ulid` uses `rand` internally.

**Alternatives considered**:
- *uuid v7*: Similar time-sortable property but doesn't match the ULID+prefix spec from Constitution §1.3.
- *Custom PL/pgSQL generation at DB level*: Exists in seed data but the Constitution requires application-side ID generation. DB-side generation couples identity to database roundtrips.
- *FNV-1a hash (current stub)*: Not unique, not time-sortable, violates ULID strategy. Will be removed.

**Implementation notes**:
- Add `ulid = "1"` to `common-types/Cargo.toml`.
- `common-types::generate_id(prefix: EntityPrefix) -> String` generates `ULID::new()` and formats as `"{prefix}-{ulid}"`.
- No `chrono` dependency needed — `ulid` crate handles timestamp extraction.

### R-003: Idempotency Key Table and Storage

**Decision**: Create `inventory.idempotency_key` table with `key TEXT UNIQUE`, `station_id TEXT FK`, `created_at TIMESTAMPTZ`. Lookup within the same transaction as station creation. TTL cleanup via `created_at < now() - interval '24 hours'`.

**Rationale**: Storing keys in the same database as stations guarantees transactional consistency — if the transaction commits, the key exists; if it rolls back, neither the station nor the key persists. A 24-hour TTL is sufficient for typical network retry windows and prevents unbounded growth.

**Alternatives considered**:
- *Redis/moka cache*: Fast but lost on restart; no transactional consistency with station creation; adds infrastructure dependency.
- *Unique constraint on station composite fields*: Doesn't protect against exact-duplicate POSTs with same payload, only against logical duplicates.

**Implementation notes**:
- Migration `0018_create_inventory_idempotency_key` adds the table.
- On `POST /partner/stations`: `BEGIN → SELECT FROM idempotency_key WHERE key = $1 → if found, return existing station → else INSERT station + INSERT idempotency_key → COMMIT`.
- Periodic cleanup: `DELETE FROM inventory.idempotency_key WHERE created_at < now() - interval '24 hours'`. Can be a background task or admin command — not blocking for MVP.

### R-004: Optimistic Concurrency Control

**Decision**: Use `If-Match` ETag header containing the `updated_at` timestamp. On PATCH, compare the provided value against the stored `updated_at`. Reject with `CONCURRENT_MODIFICATION` (409) if stale.

**Rationale**: ETags are the HTTP-standard mechanism for optimistic concurrency. `updated_at` is already maintained on all mutable entities and changes on every write. This avoids adding a separate version column. The `If-Match` header is idiomatic REST — clients GET a resource, receive an ETag, and send it back with PATCH.

**Alternatives considered**:
- *Last-write-wins*: Silent data loss in concurrent scenarios; violates audit intent.
- *Pessimistic locking (SELECT FOR UPDATE)*: Overkill for MVP concurrency levels; holds row locks across the request lifecycle.
- *Separate version column*: Redundant since `updated_at` already changes monotonically on every write.

**Implementation notes**:
- GET responses include `ETag: "<updated_at ISO8601>"` header.
- PATCH requests must include `If-Match: "<updated_at ISO8601>"` header.
- Repository layer: `UPDATE ... SET ... WHERE id = $1 AND updated_at = $2 RETURNING *`. If 0 rows returned, the `updated_at` didn't match → `CONCURRENT_MODIFICATION`.
- Admin and partner PATCH endpoints both enforce this (FR-028).
- Add `ConcurrentModification` to `common-errors::ErrorCode`.

### R-005: Partner ID Derivation and Provisioning

**Decision**: Replace the stub `provision_user()` with a real DB lookup. On each authenticated request, the `auth_middleware` validates the JWT. A new `resolve_current_user()` function looks up `users.user_account` by `keycloak_user_id` and joins `users.partner_membership` to get `partner_id` and membership role.

**Rationale**: The Constitution mandates `partner_id` derived from `users.partner_membership`, never from the client. The current stub in `common-auth::provisioning` uses FNV-1a hashing and always returns `partner_id: None`. For Sprint 5, the middleware must populate `partner_id` on every request from the database.

**Alternatives considered**:
- *Embed partner_id in JWT claims*: Violates Constitution — JWT is identity, not authorization state. Partner membership can change without re-issuing JWT.
- *Cache membership in-process*: Acceptable optimization but must be invalidated on membership change. Not needed for MVP scale.
- *Lookup on every request*: Simplest, correct, acceptable at MVP scale (<100 events/sec).

**Implementation notes**:
- `common-auth::guards::auth_middleware` calls a new `resolve_membership(pool, keycloak_user_id)` function.
- This function queries `SELECT ua.id, ua.email, pm.partner_id, pm.role FROM users.user_account ua LEFT JOIN users.partner_membership pm ON pm.user_id = ua.id WHERE ua.keycloak_user_id = $1`.
- If user not found, auto-provision: `INSERT INTO users.user_account (id, keycloak_user_id, email, status) VALUES (...)` (Sprint 3 requirement).
- `CurrentUser.partner_id` is populated from the query result. If `None`, the user has no partner membership.
- The `PgPool` must be accessible from the middleware. Use `axum::extract::State` or `Extension<PgPool>`.

### R-006: GIS Outbox Insertion Pattern

**Decision**: Insert `gis.sync_queue` row within the same database transaction as the station mutation. Use `sqlx::query("INSERT INTO gis.sync_queue (id, entity_type, entity_id, operation, status, created_at) VALUES ($1, $2, $3, $4, $5, $6)")`.

**Rationale**: The Constitution (§V) requires the outbox pattern — mutations emit events via outbox row → queue → worker. Synchronous insertion in the same transaction guarantees atomicity: either both the station change and the outbox event commit, or neither does. This is the standard transactional outbox pattern.

**Alternatives considered**:
- *Asynchronous event emission after commit*: Risks lost events if the service crashes between commit and event emission.
- *RabbitMQ direct publish*: Not transactional with DB writes; requires dual-write coordination.

**Implementation notes**:
- `repository::outbox_repo::insert_outbox_entry(tx, entity_type, entity_id, operation)` — takes a `sqlx::Transaction` or `sqlx::PgConnection` reference.
- Called from `station_repo` create/update/soft_delete methods, which wrap all operations in a transaction.
- ID for outbox row: `generate_id(EntityPrefix::Evt)`.
- Status: `pending`. The gis-worker (Sprint 6) will poll this table.

### R-007: Error Handling and Response Envelope Strategy

**Decision**: Create a `ServiceError` enum in `admin-service::error` that wraps `common-errors::ApiError` and `common-auth::AuthError`, implements `IntoResponse`, and always returns the standard error envelope. Add `IntoResponse` impl to `common-errors::ApiError` as well. Add `ConcurrentModification` to `ErrorCode`.

**Rationale**: The Constitution (§IV) mandates the standard error envelope `{ "success": false, "error": { "code": "", "message": "" } }`. Currently, only `AuthError` implements `IntoResponse`. The service needs a unified error type that handles auth errors, validation errors, not-found errors, and the new concurrent modification error, all returning the standard envelope.

**Alternatives considered**:
- *Handler-level manual envelope construction*: Verbose, error-prone, inconsistent.
- *Separate error types per route module*: Fragmented, duplicate envelope logic.

**Implementation notes**:
- `ServiceError` variants: `Auth(AuthError)`, `Api(ApiError)`, `Db(sqlx::Error)`, `Internal(String)`.
- `impl From<AuthError> for ServiceError`, `impl From<ApiError> for ServiceError`, `impl From<sqlx::Error> for ServiceError`.
- `impl IntoResponse for ServiceError` → maps to appropriate HTTP status + error envelope JSON.
- `sqlx::Error::RowNotFound` → `NOT_FOUND` (404).
- `sqlx::Error::Database` with constraint violation → `ALREADY_EXISTS` (409) or `ACTIVE_STATIONS_EXIST` (409).
- Success responses: `SuccessEnvelope<T>` with `PaginationMeta` for lists, new `ItemEnvelope<T>` for single items (no pagination meta).

### R-008: Module Organization and Route Structure

**Decision**: Split handlers into `routes/partner.rs` and `routes/admin.rs`. Shared models in `models/`. Repository layer in `repository/` with partner-scoped queries. Custom extractors for pagination params, `Idempotency-Key` header, and `If-Match` header.

**Rationale**: Two route files match the two API domains (partner vs admin). Repository layer enforces partner isolation at the data access level (Constitution §III). Custom extractors keep handler signatures clean and declarative.

**Alternatives considered**:
- *Single routes.rs*: Would be 500+ lines; poor separation.
- *Repository pattern with traits*: Over-engineering for single-service use; direct functions are clearer.
- *Handler-embedded queries*: Violates Constitution §III (isolation must be at repository/data-access layer).

**Implementation notes**:
- `routes/partner.rs`: 9 handlers, each extracts `CurrentUser` and passes `partner_id` to repository.
- `routes/admin.rs`: 7 handlers, no partner scoping (global).
- `repository/partner_repo.rs`: `list_partners_admin()`, `get_partner_admin()`, `create_partner()`, `update_partner()`, `soft_delete_partner()`.
- `repository/station_repo.rs`: `list_stations_partner(partner_id)`, `list_stations_admin()`, `create_station(partner_id, ...)`, `update_station(id, ...)`, `soft_delete_station(id)`.
- Partner-scoped queries: `WHERE partner_id = $1` is always present in partner repository methods.
