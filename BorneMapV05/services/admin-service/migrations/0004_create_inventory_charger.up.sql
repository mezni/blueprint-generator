CREATE TABLE IF NOT EXISTS inventory.charger (
    id          TEXT        NOT NULL PRIMARY KEY,
    station_id  TEXT        NOT NULL REFERENCES inventory.station(id),
    type        TEXT        NOT NULL CHECK (type IN ('CCS', 'Type2', 'CHAdeMO')),
    power_kw    DOUBLE PRECISION NULL,
    status      TEXT        NOT NULL DEFAULT 'available' CHECK (status IN ('available', 'offline', 'fault')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by  TEXT        NOT NULL DEFAULT '',
    updated_by  TEXT        NOT NULL DEFAULT '',
    deleted_at  TIMESTAMPTZ NULL
);

CREATE INDEX IF NOT EXISTS idx_charger_station_id ON inventory.charger (station_id);
CREATE INDEX IF NOT EXISTS idx_charger_status     ON inventory.charger (status);
