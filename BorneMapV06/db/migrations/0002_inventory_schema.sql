-- Migration 0002: Create inventory schema
-- Core business tables for partners, stations, and chargers.

CREATE SCHEMA IF NOT EXISTS inventory;

-- Partner (business entities that own charging stations)
CREATE TABLE IF NOT EXISTS inventory.partner (
    id              TEXT PRIMARY KEY,              -- PRT-xxxxxxxxxxxxxxxx
    name            TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ,

    CONSTRAINT partner_name_not_empty CHECK (name <> '')
);

CREATE INDEX IF NOT EXISTS idx_partner_deleted_at
    ON inventory.partner(deleted_at)
    WHERE deleted_at IS NULL;

-- Station (EV charging station locations)
CREATE TABLE IF NOT EXISTS inventory.station (
    id              TEXT PRIMARY KEY,              -- STN-xxxxxxxxxxxxxxxx
    partner_id      TEXT NOT NULL REFERENCES inventory.partner(id),
    name            TEXT NOT NULL,
    address         TEXT,
    latitude        NUMERIC(10,7) NOT NULL,
    longitude       NUMERIC(10,7) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ,

    CONSTRAINT station_name_not_empty CHECK (name <> ''),
    CONSTRAINT station_lat_valid CHECK (latitude >= -90 AND latitude <= 90),
    CONSTRAINT station_lng_valid CHECK (longitude >= -180 AND longitude <= 180)
);

CREATE INDEX IF NOT EXISTS idx_station_partner_id
    ON inventory.station(partner_id);

CREATE INDEX IF NOT EXISTS idx_station_location
    ON inventory.station USING GIST (
        ST_SetSRID(ST_MakePoint(longitude, latitude), 4326)::geography
    );

CREATE INDEX IF NOT EXISTS idx_station_deleted_at
    ON inventory.station(deleted_at)
    WHERE deleted_at IS NULL;

-- Charger (individual charging ports at stations)
CREATE TABLE IF NOT EXISTS inventory.charger (
    id              TEXT PRIMARY KEY,              -- CHG-xxxxxxxxxxxxxxxx
    station_id      TEXT NOT NULL REFERENCES inventory.station(id),
    connector_type  TEXT NOT NULL,
    power_kw        NUMERIC(6,2),
    status          TEXT NOT NULL DEFAULT 'available',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ,

    CONSTRAINT charger_status_valid CHECK (
        status IN ('available', 'in_use', 'maintenance', 'offline')
    ),
    CONSTRAINT charger_power_positive CHECK (power_kw > 0 OR power_kw IS NULL)
);

CREATE INDEX IF NOT EXISTS idx_charger_station_id
    ON inventory.charger(station_id);

CREATE INDEX IF NOT EXISTS idx_charger_status
    ON inventory.charger(status)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_charger_deleted_at
    ON inventory.charger(deleted_at)
    WHERE deleted_at IS NULL;
