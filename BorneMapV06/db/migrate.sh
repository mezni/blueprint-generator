#!/bin/bash
# Migration Runner
# Wrapper for sqlx-cli to manage database migrations.
#
# Usage:
#   ./db/migrate.sh            # Run all pending migrations
#   ./db/migrate.sh info       # Show pending/applied migrations
#   ./db/migrate.sh reset      # Drop and recreate database
#   ./db/migrate.sh revert     # Revert last migration
#
# Environment:
#   DATABASE_URL               Required. PostgreSQL connection string.
#   SQLX_OFFLINE               Optional. Set to "true" for offline mode.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MIGRATIONS_DIR="$SCRIPT_DIR/migrations"

if [ -z "${DATABASE_URL:-}" ]; then
    # Try to load from .env file
    if [ -f "$SCRIPT_DIR/../.env" ]; then
        source "$SCRIPT_DIR/../.env"
    elif [ -f "$SCRIPT_DIR/../infra/env/.env.example" ]; then
        source "$SCRIPT_DIR/../infra/env/.env.example"
    fi
fi

if [ -z "${DATABASE_URL:-}" ]; then
    echo "ERROR: DATABASE_URL is not set."
    echo ""
    echo "Set DATABASE_URL or create a .env file:"
    echo "  cp infra/env/.env.example .env"
    echo ""
    echo "Then run: ./db/migrate.sh"
    exit 1
fi

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo "sqlx-cli not found. Install with:"
    echo "  cargo install sqlx-cli"
    echo ""
    echo "Or run migrations manually via psql:"
    echo "  psql \$DATABASE_URL < db/migrations/0001_extensions.sql"
    exit 1
fi

cd "$SCRIPT_DIR/.."

CMD="${1:-run}"

case "$CMD" in
    run)
        echo "Running pending migrations..."
        sqlx migrate run --source "$MIGRATIONS_DIR"
        echo "Migrations complete."
        ;;
    info)
        echo "Migration status:"
        sqlx migrate info --source "$MIGRATIONS_DIR"
        ;;
    reset)
        echo "WARNING: This will DROP all tables in the database."
        read -p "Are you sure? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "Resetting database..."
            sqlx database reset --source "$MIGRATIONS_DIR"
            echo "Database reset complete."
        else
            echo "Reset cancelled."
        fi
        ;;
    revert)
        echo "Reverting last migration..."
        sqlx migrate revert --source "$MIGRATIONS_DIR"
        echo "Revert complete."
        ;;
    *)
        echo "Usage: $0 [run|info|reset|revert]"
        exit 1
        ;;
esac
