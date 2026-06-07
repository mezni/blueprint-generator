CREATE TABLE IF NOT EXISTS gis.sync_queue (
    id           TEXT        NOT NULL PRIMARY KEY,
    entity_type  TEXT        NOT NULL CHECK (entity_type IN ('station', 'charger')),
    entity_id    TEXT        NOT NULL,
    operation    TEXT        NOT NULL CHECK (operation IN ('insert', 'update', 'delete')),
    payload      JSONB       NULL,
    status       TEXT        NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'processing', 'done', 'failed', 'dead_letter')),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ NULL
);

CREATE INDEX IF NOT EXISTS idx_sync_queue_status          ON gis.sync_queue (status);
CREATE INDEX IF NOT EXISTS idx_sync_queue_entity          ON gis.sync_queue (entity_type, entity_id);
