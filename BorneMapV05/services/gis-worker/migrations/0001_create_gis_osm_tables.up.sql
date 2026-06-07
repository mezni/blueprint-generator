CREATE SCHEMA IF NOT EXISTS gis;

CREATE TABLE IF NOT EXISTS gis.osm_roads (
    osm_id   BIGINT PRIMARY KEY,
    name     TEXT,
    highway  TEXT,
    geom     GEOMETRY(MultiLineString, 4326),
    tags     JSONB
);

CREATE INDEX IF NOT EXISTS idx_osm_roads_geom ON gis.osm_roads USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_osm_roads_highway ON gis.osm_roads (highway);

CREATE TABLE IF NOT EXISTS gis.osm_admin_boundaries (
    osm_id      BIGINT PRIMARY KEY,
    name        TEXT,
    admin_level INTEGER,
    geom        GEOMETRY(MultiPolygon, 4326),
    tags        JSONB
);

CREATE INDEX IF NOT EXISTS idx_osm_admin_geom ON gis.osm_admin_boundaries USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_osm_admin_level ON gis.osm_admin_boundaries (admin_level);

CREATE TABLE IF NOT EXISTS gis.osm_pois (
    osm_id  BIGINT PRIMARY KEY,
    name    TEXT,
    amenity TEXT,
    geom    GEOMETRY(Point, 4326),
    tags    JSONB
);

CREATE INDEX IF NOT EXISTS idx_osm_pois_geom ON gis.osm_pois USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_osm_pois_amenity ON gis.osm_pois (amenity);
