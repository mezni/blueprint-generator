#!/usr/bin/env bash
# smoke-test.sh — Validate all services are healthy after docker compose up
# Usage: bash scripts/smoke-test.sh [--verbose]
set -euo pipefail

VERBOSE=false
if [[ "${1:-}" == "--verbose" ]]; then
  VERBOSE=true
fi

PASS=0
FAIL=0
FAILURES=""

pass() {
  PASS=$((PASS + 1))
  echo "  ✓ $1"
}

fail() {
  FAIL=$((FAIL + 1))
  local msg="$1"
  FAILURES="${FAILURES}  ✗ ${msg}"$'\n'
  echo "  ✗ ${msg}"
}

check_http() {
  local label="$1" url="$2" expected="$3"
  local status
  status=$(curl -s -o /dev/null -w "%{http_code}" --max-time 5 "$url" 2>/dev/null || true)
  if [[ "$status" == "$expected" ]]; then
    pass "$label (HTTP $status)"
  else
    fail "$label — expected HTTP $expected, got $status"
  fi
}

check_json() {
  local label="$1" url="$2" field="$3" expected="$4"
  local value
  value=$(curl -s --max-time 5 "$url" 2>/dev/null | python3 -c "import sys,json; print(json.load(sys.stdin).get('$field',''))" 2>/dev/null || true)
  if [[ "$value" == "$expected" ]]; then
    pass "$label (.$field = $value)"
  else
    fail "$label — expected .$field = $expected, got $value"
  fi
}

echo "=== BorneMap Smoke Test ==="
echo ""

# --- Docker Compose Health ---
echo "--- Docker Compose ---"
COMPOSE_FILE="infra/compose/docker-compose.yml"
if docker compose -f "$COMPOSE_FILE" ps --status healthy 2>/dev/null | grep -q .; then
  pass "docker compose ps shows healthy containers"
else
  fail "docker compose ps — no healthy containers found"
fi

# --- Service Health Endpoints ---
echo "--- Service /health Endpoints ---"
check_http "driver-service /health" "http://localhost:3001/health" "200"
check_http "admin-service /health" "http://localhost:3002/health" "200"
check_http "clickstream-service /health" "http://localhost:3003/health" "200"
check_http "gis-worker /health" "http://localhost:3004/health" "200"
check_http "analytics-writer /health" "http://localhost:3005/health" "200"

check_json "driver-service version" "http://localhost:3001/health" "status" "ok"
check_json "admin-service version" "http://localhost:3002/health" "status" "ok"
check_json "clickstream-service version" "http://localhost:3003/health" "status" "ok"
check_json "gis-worker version" "http://localhost:3004/health" "status" "ok"
check_json "analytics-writer version" "http://localhost:3005/health" "status" "ok"

# --- Service Readiness Endpoints ---
echo "--- Service /ready Endpoints ---"
check_http "driver-service /ready" "http://localhost:3001/ready" "200"
check_http "admin-service /ready" "http://localhost:3002/ready" "200"
check_http "clickstream-service /ready" "http://localhost:3003/ready" "200"
check_http "gis-worker /ready" "http://localhost:3004/ready" "200"
check_http "analytics-writer /ready" "http://localhost:3005/ready" "200"

# --- PostgreSQL Connectivity ---
echo "--- PostgreSQL ---"
PG_CONTAINER=$(docker compose -f "$COMPOSE_FILE" ps -q postgres 2>/dev/null || true)
if [[ -n "$PG_CONTAINER" ]]; then
  if docker exec "$PG_CONTAINER" pg_isready -U borne -d bornemap >/dev/null 2>&1; then
    pass "PostgreSQL pg_isready"
  else
    fail "PostgreSQL pg_isready failed"
  fi
else
  fail "PostgreSQL container not found"
fi

# --- RabbitMQ Connectivity ---
echo "--- RabbitMQ ---"
RMQ_CONTAINER=$(docker compose -f "$COMPOSE_FILE" ps -q rabbitmq 2>/dev/null || true)
if [[ -n "$RMQ_CONTAINER" ]]; then
  if docker exec "$RMQ_CONTAINER" rabbitmq-diagnostics -q ping >/dev/null 2>&1; then
    pass "RabbitMQ ping"
  else
    fail "RabbitMQ ping failed"
  fi
else
  fail "RabbitMQ container not found"
fi

# --- Keycloak Health ---
echo "--- Keycloak ---"
check_http "Keycloak health" "http://localhost:8090/health/ready" "200"

# --- Traefik Routing ---
echo "--- Traefik ---"
check_http "Traefik dashboard" "http://localhost:8080/dashboard/" "200"
check_http "Traefik ping" "http://localhost:8080/api/http/routers" "200"

# --- Summary ---
echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
if [[ "$FAIL" -gt 0 ]]; then
  echo ""
  echo "Failures:"
  echo "$FAILURES"
  exit 1
fi
exit 0
