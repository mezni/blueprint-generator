CREATE TABLE IF NOT EXISTS inventory.station_availability (
    id          TEXT        NOT NULL PRIMARY KEY,
    station_id  TEXT        NOT NULL REFERENCES inventory.station(id),
    status      TEXT        NOT NULL CHECK (status IN ('available', 'limited', 'unavailable')),
    source      TEXT        NOT NULL CHECK (source IN ('manual_partner', 'system_sync', 'admin')),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_availability_station_id ON inventory.station_availability (station_id);
