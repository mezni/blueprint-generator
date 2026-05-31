#!/bin/bash
# Script to start BorneMap mobile app with Expo Go

cd "$(dirname "$0")/mobile" || exit 1

if command -v pnpm &>/dev/null; then
  pnpm start
else
  npx expo start
fi
