# Quickstart: Mobile Driver App

## Prerequisites

- Node.js v24.16.0
- npm v11.13.0
- Expo Go app installed on a physical device (iOS or Android)
- Git

## Setup

```bash
# 1. Navigate to the mobile driver app
cd apps/mobile-driver

# 2. Clean install (resolve any previous cache issues)
rm -rf node_modules .expo package-lock.json

# 3. Install dependencies
npm install

# 4. Launch via tunnel (for VirtualBox / physical device testing)
npm run start:tunnel

# Alternative: LAN mode (if on same network)
npm run start:lan
```

## Verify

1. Scan the QR code displayed in the terminal with Expo Go
2. Confirm the map viewport renders centered on Tunis, Tunisia
3. Confirm the marker at the center shows "Tunis Core Baseline"
4. Confirm the debug overlay displays "BorneMap Sandbox Mode"
5. Toggle airplane mode and verify the app does not crash (silent grey tile area)

## CI Verification

```bash
# Run the same build step as CI
cd apps/mobile-driver
npm ci
npx expo export --platform web
```

## Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|-------------|-----|
| White screen on launch | Native module conflict | `rm -rf node_modules .expo && npm install` |
| Map shows grey area | Missing cached tiles (offline) | Expected behavior — no action needed |
| App crashes on Android | Missing Google Play Services | Install Google Play Services or use iOS device |
| Tunnel connection fails | VirtualBox network isolation | Restart tunnel: `npm run start:tunnel` |
