CREATE TABLE IF NOT EXISTS analytics.event_dead_letter (
    id            TEXT        NOT NULL PRIMARY KEY,
    event_id      TEXT        NOT NULL,
    event_name    TEXT        NULL,
    error_code    TEXT        NULL,
    error_message TEXT        NULL,
    raw_payload   JSONB       NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
