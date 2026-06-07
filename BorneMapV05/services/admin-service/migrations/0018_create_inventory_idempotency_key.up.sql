CREATE TABLE IF NOT EXISTS inventory.idempotency_key (
    id TEXT PRIMARY KEY,
    key TEXT UNIQUE NOT NULL,
    station_id TEXT NOT NULL REFERENCES inventory.station(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_idempotency_key_created_at
    ON inventory.idempotency_key(created_at);

CREATE INDEX IF NOT EXISTS idx_idempotency_key_key
    ON inventory.idempotency_key(key);
