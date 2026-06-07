CREATE TABLE IF NOT EXISTS users.favorite_station (
    user_id    TEXT        NOT NULL REFERENCES users.user_account(id) ON DELETE CASCADE,
    station_id TEXT        NOT NULL REFERENCES inventory.station(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, station_id)
);
