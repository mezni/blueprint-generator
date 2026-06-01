#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

DATABASE_URL="${DATABASE_URL:-postgres://borne:devpassword@localhost:5432/borne_map}"
MIGRATION_DIR="$PROJECT_ROOT/services/admin-service/migrations"

echo "Running sqlx migrations..."
echo "  DATABASE_URL: $DATABASE_URL"
echo "  MIGRATION_DIR: $MIGRATION_DIR"

cd "$PROJECT_ROOT/services/admin-service"
DATABASE_URL="$DATABASE_URL" sqlx migrate run --source "$MIGRATION_DIR"

echo "Migrations applied successfully."
