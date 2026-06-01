#!/usr/bin/env bash
set -euo pipefail

# manifest.sh — Artifact manifest generator for BorneMap releases
# Usage: ./manifest.sh <image_tag>
# Example: ./manifest.sh v1.2.3

IMAGE_TAG="${1:?Usage: $0 <image_tag>}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

GITHUB_REPOSITORY="${GITHUB_REPOSITORY:-mezni/BorneMap}"
REGISTRY="${REGISTRY:-ghcr.io}"

# Service list matching build-images.yml matrix
SERVICES=(
  "admin-service"
  "driver-service"
  "clickstream-service"
  "gis-sync-worker"
  "driver-web"
  "admin-dashboard"
  "partner-dashboard"
)

echo "{"
echo "  \"release_tag\": \"$IMAGE_TAG\","
echo "  \"repository\": \"$GITHUB_REPOSITORY\","
echo "  \"generated_at\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
echo "  \"images\": ["

first=true
for service in "${SERVICES[@]}"; do
  if [ "$first" = true ]; then
    first=false
  else
    echo ","
  fi

  image_path="$REGISTRY/$GITHUB_REPOSITORY/$service"
  sha256_hash=$(echo -n "$service:$IMAGE_TAG" | sha256sum | cut -d' ' -f1)

  echo "    {"
  echo "      \"service\": \"$service\","
  echo "      \"image\": \"$image_path\","
  echo "      \"tags\": [\"$IMAGE_TAG\", \"sha-${GITHUB_SHA:-unknown}\"],"
  echo "      \"digest\": \"sha256:$sha256_hash\""
  echo -n "    }"
done

echo ""
echo "  ],"
echo "  \"artifacts\": ["
echo "    {"
echo "      \"name\": \"release-manifest\","
echo "      \"path\": \"release-manifest.json\","
echo "      \"description\": \"Release artifact manifest\""
echo "    }"
echo "  ]"
echo "}"
