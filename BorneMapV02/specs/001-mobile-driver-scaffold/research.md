# Research: Mobile Driver App Scaffold

## Technology Decisions

### Map Rendering Provider

- **Decision**: `PROVIDER_DEFAULT` (Apple Maps on iOS, Google Maps on Android)
- **Rationale**: react-native-maps 1.14.0 defaults to the platform-native map
  provider. No custom provider configuration needed for the diagnostic scaffold.
  Choosing a specific provider (Google Maps exclusively) would require API keys
  and additional native setup, violating the "validation before optimization"
  principle for this phase.
- **Alternatives considered**: Google Maps exclusively (requires API key setup),
  Mapbox (requires token + extra native modules incompatible with Expo Go)

### Offline Map Rendering Strategy

- **Decision**: Accept platform-native tile caching behavior; no bundled tiles
- **Rationale**: The scaffold's purpose is to validate the render tree and viewport
  positioning, not to guarantee tile rendering offline. Platform-native map kits
  (Apple Maps, Google Maps) cache recently viewed tiles at the OS level. The app
  must not crash or show errors when tiles are unavailable — a silent grey tile
  area is the expected behavior (per Q1 clarification).
- **Alternatives considered**: Bundling static PNG tiles (adds MBs to bundle,
  violates simplicity principle), custom tile server (requires backend, out of scope)

### Expo SDK Version Targeting

- **Decision**: Expo SDK 51 with React Native 0.74.1
- **Rationale**: Constitution Principle II locks these versions for the MVP.
  SDK 51 is the latest stable for the 51.x line, providing the best compatibility
  with react-native-maps 1.14.0 and the managed workflow.
- **Alternatives considered**: SDK 52 (not yet locked), bare React Native
  (contradicts the Expo Go requirement in the constitution)

### CI/CD Approach

- **Decision**: GitHub Actions with `npx expo export --platform web` for build
  verification
- **Rationale**: Web export validates the JavaScript bundle, Metro bundler
  configuration, and dependency resolution without requiring native build tooling
  (Xcode, Android SDK) on the CI runner. This keeps CI fast and infrastructure-free.
- **Alternatives considered**: EAS Build (requires Expo account), native builds
  (requires macOS/Xcode runners)

### Error Handling Strategy

- **Decision**: Try-catch around MapView with text fallback screen; debug overlay
  persists on error (per Q3 clarification)
- **Rationale**: A crash gives no diagnostic information. A fallback screen with
  the error description preserves the debug overlay's purpose — providing immediate
  visual feedback about app state.
- **Alternatives considered**: Allowing native crash (wastes debugging time),
  swallowing errors silently (hides failure state from developer)

## Expo Go Compatibility Notes

- react-native-maps 1.14.0 requires the `expo-maps` compatible build or the bare
  `react-native-maps` package — verify that `react-native-maps` functions correctly
  in Expo Go SDK 51 managed workflow
- The `PROVIDER_DEFAULT` setting works in Expo Go without additional configuration
- Debug overlay uses standard React Native Views — fully compatible with Expo Go
- No custom native modules required

## Map Coordinate Configuration

- **Center**: Latitude 36.8065, Longitude 10.1815 (Tunis, Tunisia)
- **Initial deltas**: latitudeDelta 0.12, longitudeDelta 0.06 (urban zoom level)
- **SRID**: WGS 84 (EPSG:4326) — react-native-maps default coordinate system
