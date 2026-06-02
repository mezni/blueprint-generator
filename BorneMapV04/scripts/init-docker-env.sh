#!/usr/bin/env bash
# init-docker-env.sh — Copies the docker profile env files into place
# Usage: bash scripts/init-docker-env.sh
set -euo pipefail

PROFILE="${1:-docker}"
ENV_DIR="infra/env/${PROFILE}"

if [ ! -d "${ENV_DIR}" ]; then
  echo "Error: profile directory '${ENV_DIR}' not found"
  echo "Available profiles:"
  ls -1 infra/env/
  exit 1
fi

echo "Initializing environment profile: ${PROFILE}"
cp -n "${ENV_DIR}"/*.env infra/env/ 2>/dev/null || true
echo "Done. Environment files copied from ${ENV_DIR}/ to infra/env/"
