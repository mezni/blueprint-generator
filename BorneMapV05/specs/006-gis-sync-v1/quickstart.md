# Quickstart: GIS Sync System v1

## Running the Worker

The gis-worker is part of the Docker Compose stack and starts automatically:

```bash
docker compose up gis-worker
```

For local development with hot-reload:

```bash
cargo run -p gis-worker
```

## Environment Variables

Copy the example env file and customize:

```bash
cp infra/env/gis-worker.env.example infra/env/gis-worker.env
```

Key variables (all have sensible defaults):

| Variable | Default | Description |
|----------|---------|-------------|
| `GIS_WORKER_POLL_INTERVAL_MS` | 5000 | Poll loop interval |
| `GIS_WORKER_BATCH_SIZE` | 50 | Max rows per batch |
| `GIS_WORKER_MAX_RETRIES` | 3 | Retry attempts before dead_letter |
| `GIS_WORKER_RETRY_BASE_DELAY_MS` | 1000 | Backoff base delay |
| `FF_ENABLE_GIS_SYNC` | true | Set to `false` to disable |

Database config is shared via `PLATFORM_DB_*` env vars (same as admin-service).

## Verifying the Worker

1. **Health check**: `curl http://localhost:8084/health` → `{"status":"ok"}`

2. **Check poll loop**: Watch worker logs for batch processing:
   ```
   INFO gis_worker::worker: Processing batch batch_size=3
   INFO gis_worker::worker: Row processed row_id=EVT-xxx operation=insert status=done
   ```

3. **Verify geometry sync**: After a station is created, query its geom:
   ```sql
   SELECT id, ST_AsGeoJSON(geom) FROM inventory.station WHERE geom IS NOT NULL LIMIT 5;
   ```

4. **Idempotency test**: Manually reset a done row and verify reprocessing:
   ```sql
   UPDATE gis.sync_queue SET status = 'pending' WHERE status = 'done' LIMIT 1;
   ```
   Wait for the next poll cycle, then verify station geometry is unchanged.

5. **OSM import** (one-time):
   ```bash
   # Run the OSM import CLI (after Tunisia PBF is downloaded)
   cargo run -p gis-worker --bin osm-import -- --pbf /path/to/tunisia-latest.osm.pbf
   ```

## Integration Tests

```bash
cargo test -p gis-worker
```

Tests cover:
- Outbox row state machine transitions
- Geometry computation correctness
- Invalid coordinate handling
- Idempotent replay
- Stale processing row recovery
- Retry/backoff logic
