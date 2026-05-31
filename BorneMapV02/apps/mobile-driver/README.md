# Mobile Driver

Cross-platform React Native app for the BorneMap charging station locator. Runs on **web** (desktop) and **mobile** (iOS/Android) from a shared codebase.

## Components

| Component | File | Description |
|---|---|---|
| `MapPortal` | `src/components/MapPortal.js` | Desktop layout orchestrator — map, search, filters, zoom, FAB, and detail panel in a single view |
| `MapScreen` | `src/screens/MapScreen.js` | Mobile layout orchestrator — same components in a scrollable screen with bottom sheet |
| `MapView.web` | `src/components/MapView.web.js` | Leaflet-based map for web (OpenStreetMap tiles) |
| `MapView.native` | `src/components/MapView.native.js` | `react-native-maps` MapView for iOS/Android |
| `SearchBar` | `src/components/SearchBar.js` | Search input with results loading/error/empty states |
| `FilterControls` | `src/components/FilterControls.js` | Collapsible filter panel — connector type (Type 2, CCS, CHAdeMO, Tesla) and status (Available, Busy, Offline) chips |
| `ZoomControls` | `src/components/ZoomControls.js` | Zoom in/out and locate-me buttons — positioned bottom-right on both platforms |
| `FAB` | `src/components/FAB.js` | Floating action button — navigate / locate-me action |
| `StationDetailPanel` | `src/components/StationDetailPanel.js` | Desktop detail panel (bottom sheet) — station name, address, available chargers, status, connector types, navigate button |
| `StationDetailSheet` | `src/components/StationDetailSheet.js` | Mobile expandable bottom sheet — peek/expanded modes with pan gesture |
| `NavBar` | `src/components/NavBar.js` | Top navigation — map center, search, favorites, profile |
| `ErrorBoundary` | `src/components/ErrorBoundary.js` | Catches render errors and shows fallback UI |

## Hooks

| Hook | File | Description |
|---|---|---|
| `useSearch` | `src/hooks/useSearch.js` | Debounced station search with loading/error handling |
| `useFilters` | `src/hooks/useFilters.js` | Manages connector type and status filter state |
| `useStationDetail` | `src/hooks/useStationDetail.js` | Fetches single station details with retry; controls sheet peek/expanded mode |
| `useAppContext` | `src/context/AppContext.js` | Global state — selected station, theme, filters, session |
| `useAnalytics` | `src/hooks/useAnalytics.js` | Emits clickstream events (`search_submit`, `filter_change`, `zoom_in`, `zoom_out`, `marker_tap`, `locate_me`) matching `contracts/api.yaml` schema |

## Cross-Platform Testing

### Prerequisites
```bash
npm install
npx expo install  # if using Expo
```

### Web (Desktop)
```bash
npx expo start --web
```
Open http://localhost:8081 in a browser.

### Mobile (iOS/Android)
```bash
npx expo start
```
Scan QR code with Expo Go, or press `i` for iOS simulator / `a` for Android emulator.

### Test Checklist

- [ ] **Map loads** with OpenStreetMap tiles centered on Tunisia
- [ ] **Station markers** appear as colored dots (green=available, red=busy, orange=unavailable)
- [ ] **Marker tap** opens station detail (desktop panel / mobile sheet)
- [ ] **Search** filters stations in real time
- [ ] **Filter chips** toggle connector type and status filters
- [ ] **Zoom controls** (+/−) change map zoom level
- [ ] **Locate-me** button requests geolocation permission
- [ ] **Navigate** button opens station in maps app
- [ ] **Close** button dismisses detail panel
- [ ] **Escape key** (desktop) closes detail panel
- [ ] **Tab navigation** cycles through all interactive controls
- [ ] **aria-labels** present on all interactive elements
- [ ] **44×44pt touch targets** on all mobile interactive elements
- [ ] **Detail panel** collapses to minimal bar below 500px viewport height (desktop)
- [ ] **Analytics events** fire: `search_submit`, `filter_change`, `zoom_in`, `zoom_out`, `marker_tap`, `locate_me`
