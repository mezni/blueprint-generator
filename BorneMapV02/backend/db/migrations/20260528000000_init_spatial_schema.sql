CREATE EXTENSION IF NOT EXISTS "postgis";

CREATE TYPE partner_classification AS ENUM ('Private', 'Business');

CREATE TABLE partners (
    id          VARCHAR(12)     PRIMARY KEY CHECK (id ~ '^prt-[a-f0-9]{8}$'),
    name        VARCHAR(255)    NOT NULL,
    type        partner_classification    NOT NULL,
    contact_email VARCHAR(255)  NOT NULL,
    is_live     BOOLEAN         NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE TABLE stations (
    id          VARCHAR(12)     PRIMARY KEY CHECK (id ~ '^stn-[a-f0-9]{8}$'),
    partner_id  VARCHAR(12)     NOT NULL REFERENCES partners(id) ON DELETE RESTRICT,
    name        VARCHAR(255)    NOT NULL,
    geom        GEOGRAPHY(Point, 4326) NOT NULL,
    status      VARCHAR(50)     NOT NULL DEFAULT 'Available' CHECK (status IN ('Available', 'Occupied', 'Offline', 'Maintenance')),
    is_live     BOOLEAN         NOT NULL DEFAULT false,
    updated_at  TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_stations_geom       ON stations USING GIST (geom);
CREATE INDEX idx_stations_partner_id ON stations (partner_id);
CREATE INDEX idx_stations_is_live    ON stations (is_live);

CREATE TABLE chargers (
    id          VARCHAR(12)     PRIMARY KEY CHECK (id ~ '^chg-[a-f0-9]{8}$'),
    station_id  VARCHAR(12)     NOT NULL REFERENCES stations(id) ON DELETE CASCADE,
    plug_type   VARCHAR(50)     NOT NULL,
    power_output INT            NOT NULL CHECK (power_output >= 1),
    status      VARCHAR(50)     NOT NULL DEFAULT 'Available' CHECK (status IN ('Available', 'Occupied', 'Offline', 'Maintenance')),
    is_live     BOOLEAN         NOT NULL DEFAULT false,
    updated_at  TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chargers_station_id ON chargers (station_id);
