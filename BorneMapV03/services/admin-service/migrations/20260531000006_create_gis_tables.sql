-- GIS schema tables: station_enrichment, geo_boundary

CREATE TABLE gis.station_enrichment (
    station_id VARCHAR(26) PRIMARY KEY,
    address_components JSONB,
    coordinates_4326 GEOGRAPHY(Point, 4326),
    elevation_m DECIMAL(8,2),
    timezone VARCHAR(64),
    nearby_pois JSONB,
    computed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_enrichment_coords ON gis.station_enrichment USING GIST (coordinates_4326);

CREATE TABLE gis.geo_boundary (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    boundary GEOGRAPHY(Polygon, 4326) NOT NULL,
    boundary_type VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_boundary_type CHECK (boundary_type IN ('city', 'region', 'country', 'service_area'))
);

CREATE INDEX idx_boundary_geom ON gis.geo_boundary USING GIST (boundary);
