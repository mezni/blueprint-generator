-- Inventory schema tables: partner, station, charger, review

CREATE TABLE inventory.partner (
    id VARCHAR(26) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone VARCHAR(50),
    website VARCHAR(500),
    status VARCHAR(32) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    CONSTRAINT chk_partner_status CHECK (status IN ('active', 'suspended', 'inactive')),
    CONSTRAINT chk_partner_deleted CHECK (deleted_at IS NULL OR deleted_at > created_at)
);

CREATE INDEX idx_partner_status ON inventory.partner (status);

CREATE TABLE inventory.station (
    id VARCHAR(26) PRIMARY KEY,
    partner_id VARCHAR(26) NOT NULL REFERENCES inventory.partner(id),
    name VARCHAR(255),
    address TEXT,
    location GEOGRAPHY(Point, 4326) NOT NULL,
    status VARCHAR(32) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    CONSTRAINT chk_station_status CHECK (status IN ('active', 'maintenance', 'offline', 'removed')),
    CONSTRAINT chk_station_deleted CHECK (deleted_at IS NULL OR deleted_at > created_at)
);

CREATE INDEX idx_station_location ON inventory.station USING GIST (location);
CREATE INDEX idx_station_partner ON inventory.station (partner_id);
CREATE INDEX idx_station_status ON inventory.station (status);

CREATE TABLE inventory.charger (
    id VARCHAR(26) PRIMARY KEY,
    station_id VARCHAR(26) NOT NULL REFERENCES inventory.station(id),
    connector_type VARCHAR(50) NOT NULL,
    power_kw DECIMAL(6,1) NOT NULL CHECK (power_kw > 0),
    status VARCHAR(32) NOT NULL DEFAULT 'available',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    CONSTRAINT chk_charger_connector CHECK (connector_type IN ('CCS', 'CHAdeMO', 'Type2', 'GB/T', 'Tesla')),
    CONSTRAINT chk_charger_status CHECK (status IN ('available', 'occupied', 'fault', 'offline')),
    CONSTRAINT chk_charger_deleted CHECK (deleted_at IS NULL OR deleted_at > created_at)
);

CREATE INDEX idx_charger_station ON inventory.charger (station_id);
CREATE INDEX idx_charger_status ON inventory.charger (status);

CREATE TABLE inventory.review (
    id VARCHAR(26) PRIMARY KEY,
    station_id VARCHAR(26) NOT NULL REFERENCES inventory.station(id),
    user_id VARCHAR(26) NOT NULL,
    rating SMALLINT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    CONSTRAINT uq_review_station_user UNIQUE (station_id, user_id),
    CONSTRAINT chk_review_deleted CHECK (deleted_at IS NULL OR deleted_at > created_at)
);

CREATE INDEX idx_review_station ON inventory.review (station_id);
CREATE INDEX idx_review_user ON inventory.review (user_id);
