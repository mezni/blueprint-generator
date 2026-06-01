#!/usr/bin/env bash
set -euo pipefail

# deploy.sh — Deployment orchestration for BorneMap
# Usage: ./deploy.sh <image_tag>
# Example: ./deploy.sh v1.2.3

IMAGE_TAG="${1:?Usage: $0 <image_tag>}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
COMPOSE_DIR="$PROJECT_DIR/infra/compose"
COMPOSE_FILE="$COMPOSE_DIR/docker-compose.yml"
COMPOSE_PROD_FILE="$COMPOSE_DIR/docker-compose.prod.yml"

echo "=== BorneMap Deployment ==="
echo "Image tag: $IMAGE_TAG"
echo "Compose directory: $COMPOSE_DIR"

# Validate .env completeness
if [ ! -f "$PROJECT_DIR/.env" ]; then
  echo "ERROR: .env file not found at $PROJECT_DIR/.env"
  exit 1
fi
echo "  .env file found"

# Verify compose integrity
docker compose -f "$COMPOSE_FILE" -f "$COMPOSE_PROD_FILE" config --quiet
echo "  Compose configuration valid"

# Export image tag for compose interpolation
export IMAGE_TAG

# Rolling restart per group
restart_group() {
  local group_name="$1"
  shift
  local services=("$@")

  echo "--- Restarting group: $group_name ---"
  for service in "${services[@]}"; do
    echo "  Deploying $service..."
    docker compose -f "$COMPOSE_FILE" -f "$COMPOSE_PROD_FILE" up -d --no-deps "$service"
    echo "  $service deployed"
  done

  "$SCRIPT_DIR/health-check.sh" "$group_name"
  echo "--- Group $group_name health check passed ---"
}

# Group 1: Infrastructure
restart_group "infrastructure" "postgres" "rabbitmq" "traefik"

# Group 2: Auth
restart_group "auth" "keycloak"

# Group 3: Backend services
restart_group "backend" "admin-service" "driver-service" "clickstream-service" "gis-sync-worker"

# Group 4: Frontend apps
restart_group "frontend" "driver-web" "admin-dashboard" "partner-dashboard"

echo "=== Deployment complete ==="
