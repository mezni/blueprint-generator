# Quickstart: Backend Integration

## Prerequisites

- Rust toolchain (stable) — install via [rustup](https://rustup.rs/)
- Node.js v24.16.0+ and npm v11.13.0+
- Expo Go app on iOS/Android device (for mobile testing)

## Setup

### 1. Start the Backend Service

```bash
cd backend
cargo run -p api-service
```

The server starts on `http://0.0.0.0:8080`. Verify:

```bash
curl http://localhost:8080/api/v1/stations/nearby
```

Expected: JSON array with 2 stations.

### 2. Start the Mobile App

```bash
cd apps/mobile-driver
npm install
npm run start:tunnel
```

Scan the QR code with Expo Go on your device.

### 3. Verify Integration

1. Backend running → app loads → map shows green and red markers near Tunis
2. Tap a marker → bottom drawer shows station name, provider, status, chargers
3. Stop backend → app shows error screen with "Retry Connection" button
4. Tap retry → restart backend → app reloads stations

## CI Verification

```bash
cd backend && cargo check --workspace && cargo test --workspace
cd apps/mobile-driver && npx expo export --platform web
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Backend won't compile | Run `cargo check` to see errors; verify Rust is up to date |
| App can't connect to backend | Set `EXPO_PUBLIC_API_URL` to your machine's LAN IP (e.g., `http://192.168.1.42:8080/api/v1`) |
| Map tiles not loading | Device may be offline — this is expected; markers still render |
| Cargo not found | Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` |
