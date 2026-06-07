CREATE TABLE IF NOT EXISTS analytics.raw_event (
    event_id     TEXT        NOT NULL,
    event_name   TEXT        NOT NULL,
    session_id   TEXT        NOT NULL,
    user_id      TEXT        NULL,
    anonymous_id TEXT        NULL,
    actor_role   TEXT        NULL,
    occurred_at  TIMESTAMPTZ NOT NULL,
    ingested_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    path         TEXT        NULL,
    payload      JSONB       NULL,
    metadata     JSONB       NULL
) PARTITION BY RANGE (occurred_at);
