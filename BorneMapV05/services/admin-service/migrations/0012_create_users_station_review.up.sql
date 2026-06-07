CREATE TABLE IF NOT EXISTS users.station_review (
    id         TEXT        NOT NULL PRIMARY KEY,
    user_id    TEXT        NOT NULL REFERENCES users.user_account(id) ON DELETE CASCADE,
    station_id TEXT        NOT NULL REFERENCES inventory.station(id),
    rating     INTEGER     NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment    TEXT        NULL,
    status     TEXT        NOT NULL DEFAULT 'published' CHECK (status IN ('published', 'hidden', 'flagged', 'deleted')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_station_review_user_station UNIQUE (user_id, station_id)
);

CREATE INDEX IF NOT EXISTS idx_review_station_id ON users.station_review (station_id);
CREATE INDEX IF NOT EXISTS idx_review_user_id   ON users.station_review (user_id);
