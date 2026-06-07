#!/bin/bash
# Health Check Script
# Validates that all services in the Bornemap stack are healthy.
#
# Usage:
#   ./infra/compose/healthcheck.sh        # Check all services
#   ./infra/compose/healthcheck.sh postgres   # Check specific service
#
# Exit codes:
#   0 — All checked services are healthy
#   1 — One or more services are not healthy

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.yml"

check_service() {
    local service_name="$1"
    local status

    status=$(docker compose -f "$COMPOSE_FILE" ps --format json "$service_name" 2>/dev/null \
        | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('Health',''))" 2>/dev/null || echo "unknown")

    case "$status" in
        "healthy")
            echo "✅ $service_name: healthy"
            return 0
            ;;
        "starting"|"")
            echo "⏳ $service_name: starting or not available"
            return 1
            ;;
        "unhealthy")
            echo "❌ $service_name: unhealthy"
            return 1
            ;;
        *)
            echo "❓ $service_name: status unknown ($status)"
            return 1
            ;;
    esac
}

echo "=== BorneMap Health Check ==="
echo ""

SERVICES=("postgres" "driver-service" "pgadmin")

if [ $# -gt 0 ]; then
    SERVICES=("$@")
fi

ALL_HEALTHY=true

for service in "${SERVICES[@]}"; do
    if ! check_service "$service"; then
        ALL_HEALTHY=false
    fi
done

echo ""

if [ "$ALL_HEALTHY" = true ]; then
    echo "✅ All services are healthy!"
    exit 0
else
    echo "❌ Some services are not healthy"
    echo "   Run 'docker compose -f $COMPOSE_FILE logs' for details"
    exit 1
fi
