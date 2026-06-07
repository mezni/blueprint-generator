-- Create inventory schema for business data
-- This migration creates tables for Partner, Station, and Charger entities

-- ============================================================================
-- Partner Table
-- ============================================================================

CREATE SCHEMA IF NOT EXISTS inventory;

CREATE TABLE inventory.partner (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone VARCHAR(20),
    country VARCHAR(2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive', 'suspended')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for partner table
CREATE INDEX idx_partner_name ON inventory.partner(name);
CREATE INDEX idx_partner_status ON inventory.partner(status);
CREATE INDEX idx_partner_deleted_at ON inventory.partner(deleted_at);

COMMENT ON TABLE inventory.partner IS 'Business entity that owns charging stations';
COMMENT ON COLUMN inventory.partner.id IS 'NanoID with PRT-* prefix (e.g., PRT-ABC123XYZ1234)';
COMMENT ON COLUMN inventory.partner.deleted_at IS 'Soft delete marker - non-null when partner is inactive';

-- ============================================================================
-- Station Table
-- ============================================================================

CREATE TABLE inventory.station (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    partner_id VARCHAR(16) NOT NULL REFERENCES inventory.partner(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    address TEXT,
    latitude DECIMAL(10, 8) NOT NULL CHECK (latitude >= -90.0 AND latitude <= 90.0),
    longitude DECIMAL(11, 8) NOT NULL CHECK (longitude >= -180.0 AND longitude <= 180.0),
    osm_node_id BIGINT,
    availability_status VARCHAR(20) NOT NULL DEFAULT 'unknown' CHECK (availability_status IN ('available', 'unavailable', 'unknown')),
    capacity INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for station table
CREATE INDEX idx_station_partner_id ON inventory.station(partner_id);
CREATE INDEX idx_station_deleted_at ON inventory.station(deleted_at);
CREATE INDEX idx_station_availability ON inventory.station(availability_status);

-- Spatial index for station locations (placeholder - will be updated in gis schema migration)
-- Note: GIST index will be created in migration 006 for the gis.station_locations table

COMMENT ON TABLE inventory.station IS 'Charging station location managed by a Partner';
COMMENT ON COLUMN inventory.station.id IS 'NanoID with STN-* prefix (e.g., STN-ABC123XYZ1234)';
COMMENT ON COLUMN inventory.station.deleted_at IS 'Soft delete marker - non-null when station is inactive (excluded from public discovery)';
COMMENT ON COLUMN inventory.station.availability_status IS 'Current operational status (managed by partner)';

-- ============================================================================
-- Charger Table
-- ============================================================================

CREATE TABLE inventory.charger (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    station_id VARCHAR(16) NOT NULL REFERENCES inventory.station(id) ON DELETE CASCADE,
    connector_type VARCHAR(50) NOT NULL CHECK (connector_type IN ('chademo', 'type2', 'tesla_us', 'gb_t')),
    power_kw DECIMAL(5, 2) NOT NULL CHECK (power_kw > 0),
    status VARCHAR(20) NOT NULL DEFAULT 'available' CHECK (status IN ('available', 'in_use', 'maintenance', 'offline')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for charger table
CREATE INDEX idx_charger_station_id ON inventory.charger(station_id);
CREATE INDEX idx_charger_status ON inventory.charger(status);

COMMENT ON TABLE inventory.charger IS 'Individual charging port at a station';
COMMENT ON COLUMN inventory.charger.id IS 'NanoID with CHG-* prefix (e.g., CHG-ABC123XYZ1234)';

-- ============================================================================
-- Row-Level Security for Partner Data
-- ============================================================================

-- Note: Partner scope enforcement at API layer (see tasks.md)
-- This is a soft enforcement - partner users can still query all data
-- Real enforcement happens in application code with JWT partner_id

COMMENT ON TABLE inventory.partner IS 'Business entity that owns charging stations';
COMMENT ON TABLE inventory.station IS 'Charging station managed by a Partner';
COMMENT ON TABLE inventory.charger IS 'Charging port at a Station';
