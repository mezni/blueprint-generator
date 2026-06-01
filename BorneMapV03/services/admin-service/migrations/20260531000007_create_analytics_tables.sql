-- Analytics schema table: raw_event (daily partitioned, append-only)

CREATE TABLE analytics.raw_event (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    event_type VARCHAR(64) NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

CREATE TABLE analytics.raw_event_default PARTITION OF analytics.raw_event
    DEFAULT;

CREATE TABLE analytics.raw_event_20260531 PARTITION OF analytics.raw_event
    FOR VALUES FROM ('2026-05-31') TO ('2026-06-01');

CREATE INDEX idx_raw_event_type ON analytics.raw_event (event_type);
CREATE INDEX idx_raw_event_created ON analytics.raw_event (created_at);
