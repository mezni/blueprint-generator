-- Ensure spatial indexes exist with optimal configuration
-- These are created in earlier migrations; this migration
-- validates and optionally adds ANALYZE for query planning

-- Verify GIST indexes exist (no-op if already present)
CREATE INDEX IF NOT EXISTS idx_station_location ON inventory.station USING GIST (location);
CREATE INDEX IF NOT EXISTS idx_enrichment_coords ON gis.station_enrichment USING GIST (coordinates_4326);
CREATE INDEX IF NOT EXISTS idx_boundary_geom ON gis.geo_boundary USING GIST (boundary);

-- ANALYZE spatial columns for query planner optimization
ANALYZE inventory.station;
ANALYZE gis.station_enrichment;
ANALYZE gis.geo_boundary;
