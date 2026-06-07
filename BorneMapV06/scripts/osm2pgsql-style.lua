-- OSM2PGSQL Style Configuration for Tunisia
-- Defines how OSM tags should be interpreted and stored in PostgreSQL

local ways = {}

-- Default style: store ways as linestrings in gis.osm_ways
ways.way = {
    type = "line",
    merge = true,
}

-- ============================================================================
-- Tunisia-specific filters
-- ============================================================================

-- Filter for Tunisia's bounding box (approximate coordinates)
-- Tunisia: 33.7 to 37.4 (lat), 7.5 to 11.5 (lon)
local tUniq = require("osm2pgsql.unique_id")

-- ============================================================================

return ways
