-- Migration 0003: Create gis schema
-- Geospatial projection layer — NOT the source of truth.
-- Stations source of truth is inventory.station.

CREATE SCHEMA IF NOT EXISTS gis;

-- Raw OSM node data (populated by osm2pgsql in Sprint 1)
CREATE TABLE IF NOT EXISTS gis.osm_nodes (
    osm_id          BIGINT PRIMARY KEY,
    tags            JSONB,
    geom            GEOMETRY(Point, 4326) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_osm_nodes_geom ON gis.osm_nodes USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_osm_nodes_tags ON gis.osm_nodes USING GIN(tags);

-- Raw OSM way data (populated by osm2pgsql in Sprint 1)
CREATE TABLE IF NOT EXISTS gis.osm_ways (
    osm_id          BIGINT PRIMARY KEY,
    tags            JSONB,
    geom            GEOMETRY(LineString, 4326) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_osm_ways_geom ON gis.osm_ways USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_osm_ways_tags ON gis.osm_ways USING GIN(tags);

-- Derived roads from OSM ways (populated in Sprint 1)
CREATE TABLE IF NOT EXISTS gis.roads (
    id              BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    osm_id          BIGINT REFERENCES gis.osm_ways(osm_id),
    name            TEXT,
    road_type       TEXT,
    geom            GEOMETRY(LineString, 4326) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_roads_osm_id ON gis.roads(osm_id);
CREATE INDEX IF NOT EXISTS idx_roads_geom ON gis.roads USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_roads_road_type ON gis.roads(road_type);

-- Administrative boundaries from OSM relations (populated in Sprint 1)
CREATE TABLE IF NOT EXISTS gis.boundaries (
    id              BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    osm_id          BIGINT,
    name            TEXT,
    admin_level     INTEGER,
    geom            GEOMETRY(MultiPolygon, 4326) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_boundaries_osm_id ON gis.boundaries(osm_id);
CREATE INDEX IF NOT EXISTS idx_boundaries_geom ON gis.boundaries USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_boundaries_admin_level ON gis.boundaries(admin_level);

-- Points of interest from OSM (populated in Sprint 1)
CREATE TABLE IF NOT EXISTS gis.amenity_points (
    id              BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    osm_id          BIGINT,
    amenity_type    TEXT,
    name            TEXT,
    tags            JSONB,
    geom            GEOMETRY(Point, 4326) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_amenity_points_osm_id ON gis.amenity_points(osm_id);
CREATE INDEX IF NOT EXISTS idx_amenity_points_geom ON gis.amenity_points USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_amenity_points_type ON gis.amenity_points(amenity_type);

-- Station GIS layer (populated by GIS Sync Worker in Sprint 2+)
-- This is the DERIVED spatial data, NOT the source of truth
CREATE TABLE IF NOT EXISTS gis.station_locations (
    station_id      TEXT PRIMARY KEY REFERENCES inventory.station(id),
    geom            GEOMETRY(Point, 4326) NOT NULL,
    snapped_road_id BIGINT REFERENCES gis.roads(id),
    region_id       BIGINT REFERENCES gis.boundaries(id),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT station_locations_geom_valid
        CHECK (ST_IsValid(geom) AND GeometryType(geom) = 'POINT')
);

CREATE INDEX IF NOT EXISTS idx_station_locations_geom
    ON gis.station_locations USING GIST(geom);
