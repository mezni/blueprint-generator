#!/bin/bash

################################################################################
# OSM Import Script for Tunisia
# Runs osm2pgsql to import OpenStreetMap data into PostgreSQL/PostGIS
################################################################################

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Configuration
OSM_FILE="${1:-${OSM_FILE:-}}"
DB_NAME="${2:-${DB_NAME:-borne_map}}"
DB_USER="${3:-${DB_USER:-postgres}}"
DB_HOST="${4:-${DB_HOST:-localhost}}"
DB_PORT="${5:-${DB_PORT:-5432}}"
SCALE="${6:-${SCALE:-1}}"

# Default OSM file if not provided
if [[ -z "$OSM_FILE" ]]; then
    echo -e "${RED}Error: OSM file path required${NC}"
    echo "Usage: $0 <osm-file.pbf> [db-name] [db-user] [db-host] [db-port] [scale]"
    echo "Example: $0 /data/tunisia.osm.pbf"
    exit 1
fi

# Check if OSM file exists
if [[ ! -f "$OSM_FILE" ]]; then
    echo -e "${RED}Error: OSM file not found: $OSM_FILE${NC}"
    exit 1
fi

# Log function
log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

################################################################################
# Step 1: Validate parameters
################################################################################

log "Validating parameters..."

if [[ "$OSM_FILE" != *.pbf ]]; then
    warn "OSM file should be in PBF format (.pbf)"
fi

if [[ "$SCALE" -lt 1 ]]; then
    error "Scale must be >= 1"
fi

log "Parameters validated successfully"

################################################################################
# Step 2: Create database connection string
################################################################################

DB_URL="postgresql://${DB_USER}@${DB_HOST}:${DB_PORT}/${DB_NAME}?sslmode=disable"

log "Database URL: ${DB_URL:0:30}..."

################################################################################
# Step 3: Check if database exists, create if not
################################################################################

log "Checking database connectivity..."

if ! psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -c '\l' | grep -q "$DB_NAME"; then
    warn "Database '$DB_NAME' does not exist, creating..."
    createdb -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" "$DB_NAME" || {
        error "Failed to create database"
    }
    log "Database created successfully"
else
    log "Database already exists"
fi

################################################################################
# Step 4: Drop existing OSM tables (if any)
################################################################################

log "Dropping existing OSM tables..."

psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" <<EOF
DROP TABLE IF EXISTS gis.osm_ways CASCADE;
DROP TABLE IF EXISTS gis.osm_nodes CASCADE;
DROP TABLE IF EXISTS gis.osm_relations CASCADE;
EOF

log "Existing OSM tables dropped"

################################################################################
# Step 5: Create schema and enable PostGIS
################################################################################

log "Creating GIS schema and enabling PostGIS..."

psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" <<EOF
CREATE SCHEMA IF NOT EXISTS gis;
CREATE EXTENSION IF NOT EXISTS postgis;
EOF

log "GIS schema and PostGIS enabled"

################################################################################
# Step 6: Create OSM tables
################################################################################

log "Creating OSM tables..."

psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" <<EOF
CREATE TABLE gis.osm_ways (
    id BIGSERIAL PRIMARY KEY,
    osm_id BIGINT NOT NULL,
    geom GEOMETRY(LINESTRING, 4326) NOT NULL,
    tags HSTORE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE gis.osm_nodes (
    id BIGSERIAL PRIMARY KEY,
    osm_id BIGINT NOT NULL,
    geom GEOMETRY(POINT, 4326) NOT NULL,
    tags HSTORE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE gis.osm_relations (
    id BIGSERIAL PRIMARY KEY,
    osm_id BIGINT NOT NULL,
    members JSONB,
    tags HSTORE,
    geom GEOMETRY(GEOMETRY, 4326),
    created_at TIMESTAMP DEFAULT NOW()
);
EOF

log "OSM tables created"

################################################################################
# Step 7: Create GIST indexes for spatial queries
################################################################################

log "Creating GIST spatial indexes..."

psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" <<EOF
CREATE INDEX IF NOT EXISTS gis_osm_ways_geom_idx ON gis.osm_ways USING GIST (geom);
CREATE INDEX IF NOT EXISTS gis_osm_nodes_geom_idx ON gis.osm_nodes USING GIST (geom);
CREATE INDEX IF NOT EXISTS gis_osm_relations_geom_idx ON gis.osm_relations USING GIST (geom);
EOF

log "Spatial indexes created"

################################################################################
# Step 8: Run osm2pgsql
################################################################################

log "Starting osm2pgsql import..."

SCALE_ARG=""
if [[ "$SCALE" -gt 1 ]]; then
    SCALE_ARG="--scale $SCALE"
fi

osm2pgsql \
    --username "$DB_USER" \
    --host "$DB_HOST" \
    --port "$DB_PORT" \
    --database "$DB_NAME" \
    --output custom \
    --style "$SCRIPT_DIR/osm2pgsql-style.lua" \
    --create \
    --slim \
    --cache $((4 * SCALE)) \
    --hstore-all \
    --hstore-add-fields \
    --number-of-processes 4 \
    "$OSM_FILE"

EXIT_CODE=$?

if [[ $EXIT_CODE -ne 0 ]]; then
    error "osm2pgsql failed with exit code $EXIT_CODE"
fi

log "osm2pgsql import completed successfully"

################################################################################
# Step 9: Analyze tables for query optimization
################################################################################

log "Analyzing tables..."

psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" <<EOF
ANALYZE gis.osm_ways;
ANALYZE gis.osm_nodes;
ANALYZE gis.osm_relations;
EOF

log "Tables analyzed"

################################################################################
# Step 10: Verify import
################################################################################

log "Verifying import..."

TABLE_COUNT=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM gis.osm_ways;")
NODE_COUNT=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM gis.osm_nodes;")
RELATION_COUNT=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM gis.osm_relations;")

log "Imported data:"
log "  Ways: $TABLE_COUNT"
log "  Nodes: $NODE_COUNT"
log "  Relations: $RELATION_COUNT"

if [[ "$TABLE_COUNT" -lt 1000 ]]; then
    warn "Expected at least 1000 ways, got $TABLE_COUNT"
fi

log -e "\n${GREEN}========================================${NC}"
log -e "${GREEN}OSM Import Completed Successfully!${NC}"
log -e "${GREEN}========================================${NC}"

exit 0
