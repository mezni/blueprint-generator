# Contract: GIS Outbox Queue

## Producer → Consumer Interface

The `gis.sync_queue` table serves as the contract between the admin-service (producer) and gis-worker (consumer).

### Row Schema

```sql
CREATE TABLE gis.sync_queue (
    id           TEXT        NOT NULL PRIMARY KEY,        -- EVT-<ULID>
    entity_type  TEXT        NOT NULL CHECK (entity_type IN ('station', 'charger')),
    entity_id    TEXT        NOT NULL,                     -- e.g., STN-<ULID>
    operation    TEXT        NOT NULL CHECK (operation IN ('insert', 'update', 'delete')),
    payload      JSONB       NULL,
    status       TEXT        NOT NULL DEFAULT 'pending'
                CHECK (status IN ('pending', 'processing', 'done', 'failed', 'dead_letter')),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ NULL
);
```

### Producer Contract (admin-service)

- **When**: On every station create, update, and soft-delete through any API path
- **Rows inserted**: Exactly one row per station mutation
  - `entity_type = 'station'`
  - `entity_id = <station_id>` (the STN-ULID of the mutated station)
  - `operation = 'insert' | 'update' | 'delete'` (matching the mutation type)
  - `payload` = JSONB with lat/lng snapshot (optional, for debugging)
  - `status = 'pending'`
- **Transactional**: Insert happens within the same DB transaction as the station mutation
- **Guarantee**: At-least-once delivery (consumer handles idempotency)

### Consumer Contract (gis-worker)

- **Polling**: SELECT pending rows ordered by `created_at`, limited by batch size
- **Processing**:
  - Atomically claim: `UPDATE ... SET status = 'processing' WHERE id = <id> AND status = 'pending'`
  - Compute geometry: `UPDATE inventory.station SET geom = ST_SetSRID(ST_MakePoint(lng, lat), 4326) WHERE id = entity_id`
  - On success: `UPDATE ... SET status = 'done', processed_at = NOW()`
  - On failure (retryable): `UPDATE ... SET status = 'failed'`
  - On failure (exhausted): `UPDATE ... SET status = 'dead_letter'`
- **Idempotency**: Each row can be safely replayed. Geometry computation is deterministic and bit-identical.
- **Retry**: Exponential backoff with jitter, max retries = `GIS_WORKER_MAX_RETRIES` (default 3)
- **Stale recovery**: On startup, reset `processing` rows older than `GIS_WORKER_STALE_PROCESSING_TIMEOUT_MS` to `pending`

### Error Codes (stored in logs, not in DB)

| Error | Cause | Handling |
|-------|-------|----------|
| `INVALID_COORDINATES` | lat/lng NULL or out of range | Fail immediately → dead_letter |
| `STATION_NOT_FOUND` | entity_id does not match any station | Fail immediately → dead_letter |
| `DB_CONNECTION_ERROR` | Transient DB failure | Retry with backoff |
| `UNKNOWN_ERROR` | Unexpected error | Retry with backoff, then dead_letter |
