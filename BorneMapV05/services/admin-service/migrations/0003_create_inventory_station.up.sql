CREATE TABLE IF NOT EXISTS inventory.station (
    id          TEXT        NOT NULL PRIMARY KEY,
    partner_id  TEXT        NOT NULL REFERENCES inventory.partner(id),
    name        TEXT        NOT NULL,
    description TEXT        NULL,
    latitude    DOUBLE PRECISION NOT NULL CHECK (latitude >= -90 AND latitude <= 90),
    longitude   DOUBLE PRECISION NOT NULL CHECK (longitude >= -180 AND longitude <= 180),
    geom        GEOGRAPHY(Point, 4326) NULL,
    status      TEXT        NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'inactive', 'maintenance')),
    is_live     BOOLEAN     NOT NULL DEFAULT FALSE,
    is_public   BOOLEAN     NOT NULL DEFAULT FALSE,
    city        TEXT        NULL,
    country     TEXT        NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by  TEXT        NOT NULL DEFAULT '',
    updated_by  TEXT        NOT NULL DEFAULT '',
    deleted_at  TIMESTAMPTZ NULL
);

CREATE INDEX IF NOT EXISTS idx_station_geom          ON inventory.station USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_station_partner_id    ON inventory.station (partner_id);
CREATE INDEX IF NOT EXISTS idx_station_status        ON inventory.station (status);
CREATE INDEX IF NOT EXISTS idx_station_live_public   ON inventory.station (is_live, is_public);
CREATE INDEX IF NOT EXISTS idx_station_city          ON inventory.station (city);
