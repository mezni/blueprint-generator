# Contracts: Mobile Driver App Scaffold

## External Interfaces

This phase defines **no external interfaces**. The mobile driver app is a
self-contained diagnostic scaffold with:
- Zero network requests
- Zero backend API dependencies
- Zero integration points with external systems

## Internal Component Interface

### App.js

- **Export**: `default export function App()`
- **Props**: None (root component)
- **Children**: `<SafeAreaView>` wrapping `<StatusBar>` and `<MapScreen>`

### MapScreen.js

- **Export**: `default export function MapScreen()`
- **Props**: None (self-contained diagnostic screen)
- **Dependencies**: `react-native-maps` (MapView, Marker, PROVIDER_DEFAULT)

### package.json Scripts

| Script | Command | Purpose |
|--------|---------|---------|
| `start` | `expo start` | Local development server |
| `start:lan` | `expo start --host lan --clear` | LAN-connected device testing |
| `start:tunnel` | `expo start --tunnel --clear` | Tunnel proxy for VirtualBox USB testing |
