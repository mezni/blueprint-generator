CREATE TABLE station_domain.chargers (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    station_id  UUID        NOT NULL REFERENCES station_domain.stations(id) ON DELETE CASCADE,
    connector   TEXT        NOT NULL CHECK (connector IN ('Type2', 'CCS', 'CHAdeMO', 'Type2_Tethered')),
    power_kw    NUMERIC(6,2) NOT NULL CHECK (power_kw > 0 AND power_kw <= 600),
    is_active   BOOLEAN     NOT NULL DEFAULT TRUE,
    is_test     BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ NULL
);

CREATE INDEX chargers_station_idx ON station_domain.chargers (station_id);
