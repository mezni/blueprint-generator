CREATE TABLE station_domain.stations (
    id                  UUID                      PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id          UUID                      NOT NULL REFERENCES station_domain.companies(id) ON DELETE RESTRICT,
    name                TEXT                      NOT NULL CHECK (char_length(name) BETWEEN 1 AND 200),
    address             TEXT                      NOT NULL CHECK (char_length(address) BETWEEN 1 AND 500),
    location            GEOGRAPHY(Point, 4326)    NOT NULL,
    is_active           BOOLEAN                   NOT NULL DEFAULT TRUE,
    under_maintenance   BOOLEAN                   NOT NULL DEFAULT FALSE,
    opening_hours_osm   TEXT                      NULL CHECK (char_length(opening_hours_osm) <= 500),
    is_test             BOOLEAN                   NOT NULL DEFAULT FALSE,
    created_at          TIMESTAMPTZ               NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ               NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ               NULL
);

CREATE INDEX stations_location_gix ON station_domain.stations USING GIST (location);
CREATE INDEX stations_company_idx ON station_domain.stations (company_id);
CREATE INDEX stations_active_partial_idx ON station_domain.stations (id) WHERE deleted_at IS NULL AND is_active = TRUE;
