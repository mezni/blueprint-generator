#!/usr/bin/env bash
set -euo pipefail

# health-check.sh — Post-deploy health verification for BorneMap
# Usage: ./health-check.sh <group_name>
# Groups: infrastructure, auth, backend, frontend

GROUP="${1:?Usage: $0 <group_name>}"
BASE_URL="${HEALTH_CHECK_URL:-http://localhost}"
MAX_RETRIES=12
RETRY_INTERVAL=5

check_endpoint() {
  local name="$1"
  local url="$2"
  local retries=0

  echo -n "  Checking $name at $url..."
  while [ $retries -lt $MAX_RETRIES ]; do
    status=$(curl -so /dev/null -w "%{http_code}" "$url" 2>/dev/null || true)
    if [ "$status" = "200" ] || [ "$status" = "000" ]; then
      echo " OK (HTTP $status)"
      return 0
    fi
    retries=$((retries + 1))
    sleep $RETRY_INTERVAL
  done
  echo " FAILED after $MAX_RETRIES retries (last status: $status)"
  return 1
}

echo "=== Health Check: $GROUP ==="

case "$GROUP" in
  infrastructure)
    check_endpoint "PostgreSQL" "$BASE_URL:5432"
    check_endpoint "RabbitMQ" "$BASE_URL:15672/api/health/checks/alarms"
    check_endpoint "Traefik" "$BASE_URL:80/api/http/routers"
    ;;
  auth)
    check_endpoint "Keycloak" "$BASE_URL:8080/health/ready"
    ;;
  backend)
    check_endpoint "Admin Service" "$BASE_URL:8081/health"
    check_endpoint "Driver Service" "$BASE_URL:8082/health"
    check_endpoint "Clickstream Service" "$BASE_URL:8083/health"
    check_endpoint "GIS Sync Worker" "$BASE_URL:8084/health"
    ;;
  frontend)
    check_endpoint "Driver Web" "$BASE_URL:3000/"
    check_endpoint "Admin Dashboard" "$BASE_URL:3001/"
    check_endpoint "Partner Dashboard" "$BASE_URL:3002/"
    ;;
  all)
    "$0" infrastructure
    "$0" auth
    "$0" backend
    "$0" frontend
    ;;
  *)
    echo "Unknown group: $GROUP"
    echo "Usage: $0 {infrastructure|auth|backend|frontend|all}"
    exit 1
    ;;
esac

echo "=== Health check complete: $GROUP ==="
