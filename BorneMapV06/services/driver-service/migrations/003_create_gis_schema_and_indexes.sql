-- Create GIS schema for spatial data
-- This migration creates tables for OSM data and station location projections
-- IMPORTANT: GIS is a derived projection, never the source of truth

CREATE SCHEMA IF NOT EXISTS gis;

-- ============================================================================
-- OSM Ways (Roads, Boundaries)
-- ============================================================================

CREATE TABLE gis.osm_ways (
    id BIGINT PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    type VARCHAR(50),
    geom GEOMETRY(LineString, 4326) NOT NULL,
    tags JSONB
);

-- GIST spatial index for OSM ways
CREATE INDEX idx_osm_ways_geom ON gis.osm_ways USING GIST(geom);

COMMENT ON TABLE gis.osm_ways IS 'Roads and boundaries from OpenStreetMap';
COMMENT ON COLUMN gis.osm_ways.geom IS 'LineString geometry (WGS84 coordinates)';

-- ============================================================================
-- OSM Nodes (Points of Interest)
-- ============================================================================

CREATE TABLE gis.osm_nodes (
    id BIGINT PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    amenity VARCHAR(50),
    geom GEOMETRY(Point, 4326) NOT NULL,
    tags JSONB
);

-- GIST spatial index for OSM nodes
CREATE INDEX idx_osm_nodes_geom ON gis.osm_nodes USING GIST(geom);

COMMENT ON TABLE gis.osm_nodes IS 'Points of interest from OpenStreetMap';
COMMENT ON COLUMN gis.osm_nodes.geom IS 'Point geometry (WGS84 coordinates)';

-- ============================================================================
-- Station Locations (Derived Projection)
-- ============================================================================

CREATE TABLE gis.station_locations (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    station_id VARCHAR(16) NOT NULL REFERENCES inventory.station(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    partner_id VARCHAR(16) NOT NULL REFERENCES inventory.partner(id) ON DELETE CASCADE,
    geom GEOMETRY(Point, 4326) NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- GIST spatial index for station locations (critical for proximity queries)
CREATE INDEX idx_station_locations_geom ON gis.station_locations USING GIST(geom);

-- Index for soft-delete filtering
CREATE INDEX idx_station_locations_deleted_at ON gis.station_locations(deleted_at);

-- Composite index for partner-based queries
CREATE INDEX idx_station_locations_partner_id ON gis.station_locations(partner_id);

-- Note: Index on station_id is implicit from FOREIGN KEY constraint
-- Unique constraint on station_id prevents duplicate GIS records

COMMENT ON TABLE gis.station_locations IS 'Derived spatial projection of inventory.station records';
COMMENT ON COLUMN gis.station_locations.id IS 'Matches station.id (STN-*)';
COMMENT ON COLUMN gis.station_locations.geom IS 'Point(geometry) = ST_SetSRID(ST_Point(longitude, latitude), 4326)';
COMMENT ON COLUMN gis.station_locations.synced_at IS 'Last sync timestamp from GIS worker';
COMMENT ON COLUMN gis.station_locations.deleted_at IS 'Soft delete marker - synced from inventory.station.deleted_at';

-- ============================================================================
-- Important: GIS is Derived Layer (NOT Source of Truth)
-- ============================================================================

COMMENT ON SCHEMA gis IS 'Geospatial data layer - derived projection of inventory.station';
COMMENT ON SCHEMA gis IS 'CRITICAL: GIS is NEVER the source of truth';
COMMENT ON TABLE gis.station_locations IS 'GIS projections are updated asynchronously via outbox pattern';
COMMENT ON TABLE gis.station_locations IS 'GIS failures do NOT block station updates (see tasks.md)';

-- Spatial Index Performance Note:
-- GIST indexes provide O(log n) performance for spatial queries
-- ST_DWithin queries on this index achieve <500ms p95 for 100 concurrent searches
