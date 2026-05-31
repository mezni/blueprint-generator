# Developer Quickstart for BorneMap

## Web Admin Portal

1. Navigate to the web project:

```bash
cd bornemap/frontend/web
```

2. Install dependencies (if not done):

```bash
pnpm install
```

3. Start the development server:

```bash
pnpm start
```

4. Access the portal in your browser at http://localhost:5173


## Mobile App (Expo Go)

1. Navigate to the mobile project:

```bash
cd bornemap/frontend/mobile
```

2. Install dependencies (if not done):

```bash
pnpm install
```

3. Run the mobile app dev server (you can use the helper script from `bornemap/frontend`):

```bash
# From the frontend root folder
./start-mobile.sh
```

or directly inside mobile:

```bash
pnpm start
```

4. Open Expo Go on your physical device and scan the QR code displayed in the terminal or browser.


## Environment Cleanup and Troubleshooting

### Kill lingering Expo processes on common ports

```bash
fuser -k 8081/tcp
fuser -k 19000/tcp
fuser -k 19001/tcp
fuser -k 19002/tcp
```

### Clear Expo cache before starting

```bash
pnpm start -- --clear
# or
expo start --clear
```

### Upgrade Expo CLI to latest recommended version

```bash
npm uninstall -g expo-cli
npm install -g expo
```

### Install missing TypeScript dependencies

```bash
pnpm add -D @types/react-native
```

## Notes

- Ensure your mobile device is connected to the same local network as your development machine.
- Switch Expo connection modes (Tunnel/LAN/Local) in the Expo developer tools if scanning QR codes fails.
- Run the helper script `start-mobile.sh` from the frontend root folder for ease of use.

---

This quickstart covers the basic setup and common solutions for development issues related to your recent changes and aligns with the foundation infrastructure specifications and architecture.
