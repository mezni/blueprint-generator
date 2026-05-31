#!/usr/bin/env bash
set -euo pipefail

# OSM Tunisia Import Script
# Downloads Tunisia OSM extract and imports charging stations via osm2pgsql
#
# Prerequisites:
#   - osm2pgsql installed (apt install osm2pgsql)
#   - PostGIS container running
#   - DATABASE_URL env var set, or default used
#
# Usage:
#   DATABASE_URL="postgres://borne:borne@localhost:5432/borne_map" ./import_tunisia.sh

DATABASE_URL="${DATABASE_URL:-postgres://borne:borne@localhost:5432/borne_map}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DATA_DIR="${SCRIPT_DIR}/data"
PBF_FILE="${DATA_DIR}/tunisia-latest.osm.pbf"
PBF_URL="https://download.geofabrik.de/africa/tunisia-latest.osm.pbf"
LUA_FILTER="${SCRIPT_DIR}/ev_filter.lua"

mkdir -p "${DATA_DIR}"

if [ ! -f "${PBF_FILE}" ]; then
    echo "Downloading Tunisia OSM extract..."
    wget -O "${PBF_FILE}" "${PBF_URL}"
else
    echo "OSM data file already exists at ${PBF_FILE}"
fi

echo "Importing charging stations into PostGIS..."
osm2pgsql \
    --database "${DATABASE_URL}" \
    --create \
    --slim \
    --drop \
    --hstore \
    --tag-transform-script "${LUA_FILTER}" \
    --input-reader "pbf" \
    "${PBF_FILE}"

echo "Import complete."
echo "Run the following to verify:"
echo "  psql \"${DATABASE_URL}\" -c 'SELECT COUNT(*) FROM stations;'"
echo "  psql \"${DATABASE_URL}\" -c 'SELECT COUNT(*) FROM chargers;'"
