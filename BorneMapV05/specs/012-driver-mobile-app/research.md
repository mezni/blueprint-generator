# Research: Driver Mobile App Implementation

**Feature**: Driver Mobile App (Sprint 12)
**Date**: 2026-06-04
**Status**: Complete

## Research Summary

Research completed for Expo/React Native mobile app implementation focusing on performance, security, testing, and architectural patterns specific to the mobile platform.

---

## Decision 1: Mobile App Architecture Pattern

**Decision**: Implement **Offline-First Architecture with Optimistic UI Updates**

**Rationale**:
- Users may experience intermittent network connectivity in coverage areas
- Offline-safe UI requirement mandates state preservation and sync
- Optimistic updates improve perceived responsiveness and reduce perceived latency
- Aligns with app's requirement to maintain functional UI during network interruptions

**Alternatives Considered**:
1. **Network-First with Loading States**: Simpler but poor UX during connectivity issues
2. **Hybrid Approach**: Too complex, requires complex state synchronization logic

**Implementation Pattern**:
- Local cache for stations, favorites, and cached reviews
- Write operations use optimistic UI updates
- Background sync when network restores (using React Native background fetch)
- Clear error states and retry mechanisms for failed offline operations
- State preservation: maintain current screen navigation and view state

---

## Decision 2: State Management Strategy

**Decision**: Use **React Query with TanStack Query for Server State + Context for Client State**

**Rationale**:
- React Query provides excellent caching, offline support, and automatic retry
- No additional state management library needed (reduces complexity)
- Works well with TypeScript and fits Expo's React Native ecosystem
- Simplifies data fetching for map discovery, station details, and favorites

**Implementation Pattern**:
```typescript
// Server state (API calls)
useQuery('stations', fetchStations)  // Map discovery
useQuery(['station', id], fetchStationDetail)  // Station details
useMutation('addFavorite', addFavorite)  // Favorites
useMutation('submitReview', submitReview)  // Reviews

// Client state (UI state)
AuthContext - authentication state
FavoritesContext - local favorites storage
```

**Alternatives Considered**:
1. **Redux**: Overkill for this use case, additional boilerplate
2. **Zustand**: Good alternative, but React Query already solves server state

---

## Decision 3: Offline-Safe Architecture

**Decision**: Implement **Multi-Layered Caching Strategy**

**Rationale**:
- Map discovery needs fast access without waiting for API calls
- Favorites need to work offline and sync later
- Reviews need to cache and update when network available
- Users expect immediate feedback even when offline

**Implementation Pattern**:
1. **React Query Cache**: In-memory cache with offline support
2. **AsyncStorage**: Persistent storage for favorites and cached station data
3. **React Native AsyncStorage**: Native async storage with encryption
4. **Cache Invalidation**: Automatic cache clearing on background updates

**Cache Layers**:
- **Session Cache**: In-memory React Query cache (fastest, expires on app restart)
- **Local Cache**: AsyncStorage (persists across app restarts)
- **Background Sync**: React Native background fetch API for queued operations

**Alternatives Considered**:
1. **Offline-First with IndexedDB**: More complex, unnecessary for mobile
2. **Simple Local Cache**: No persistence, violates offline requirement

---

## Decision 4: Performance Optimization Strategy

**Decision**: Implement **Multi-Level Performance Optimizations**

**Rationale**:
- Mobile performance is critical for user experience
- Map interactions need <300ms latency
- App launch needs <10 seconds
- Supports 10,000 concurrent users requirement

**Optimization Layers**:
1. **Code Optimization**:
   - Lazy loading for heavy components (maps, station details)
   - Code splitting with React.lazy and Suspense
   - Tree shaking and dead code elimination

2. **Data Optimization**:
   - Pagination for map discovery (default 20 stations per page)
   - Debouncing map viewport updates (300-500ms)
   - Caching with React Query TTL (10 minutes default)
   - Compression for API responses (gzip in Vite)

3. **Rendering Optimization**:
   - React.memo for static components
   - useCallback/useMemo for expensive computations
   - Virtualized lists for favorites and reviews
   - Platform-specific rendering (iOS vs Android)

4. **Network Optimization**:
   - API response caching
   - Request batching for multiple station queries
   - Offline queuing for failed requests

**Performance Targets**:
- Map interaction <300ms p95 on 4G
- App launch <10 seconds
- Station detail loading <2 seconds
- Favorites toggle <500ms

**Alternatives Considered**:
1. **Basic Caching Only**: Insufficient for performance targets
2. **Full PWA**: Not needed for React Native, heavier than necessary

---

## Decision 5: Secure Storage Implementation

**Decision**: Use **Expo Secure Store with AES-256 Encryption**

**Rationale**:
- Native encrypted storage meets security requirements
- AES-256 encryption for all sensitive data at rest
- Provides PIN/biometric lock integration
- Cross-platform support (iOS Keychain, Android Keystore)

**Implementation Pattern**:
```typescript
import * as SecureStore from 'expo-secure-store';

// Encrypted storage for sensitive data
async function saveToken(token: string) {
  await SecureStore.setItemAsync('auth_token', token);
}

async function getToken(): Promise<string | null> {
  return await SecureStore.getItemAsync('auth_token');
}

// PIN/biometric lock integration
import * as SecureStore from 'expo-secure-store';

async function setPin(pin: string) {
  await SecureStore.setItemAsync('app_pin', pin);
}

async function verifyPin(pin: string): Promise<boolean> {
  const storedPin = await SecureStore.getItemAsync('app_pin');
  return pin === storedPin;
}
```

**Alternatives Considered**:
1. **AsyncStorage Without Encryption**: Security violation
2. **Device Keychain Only**: No cross-platform standard, device-specific only
3. **Third-party Crypto Libraries**: Overkill, Secure Store sufficient

---

## Decision 6: Testing Strategy

**Decision**: Implement **Three-Layer Testing Strategy**

**Rationale**:
- Unit tests for business logic and utility functions
- Integration tests for API clients and authentication
- E2E tests for critical user journeys (discovery, login, favorites, reviews)

**Testing Stack**:
- **Unit**: Jest with React Native Testing Library
- **Integration**: React Native Testing Library with mocked APIs
- **E2E**: Detox (supports both iOS and Android)

**Test Coverage Targets**:
- Unit tests: 80% coverage for business logic
- Integration tests: All API client functions and hooks
- E2E tests: All P1 user stories (discovery, login, favorites, reviews)

**Test Execution**:
```bash
# Unit tests
npm run test

# E2E tests (requires simulators/emulators)
npm run test:e2e
```

**Alternatives Considered**:
1. **Only Unit Tests**: Missing integration and E2E coverage
2. **Only E2E Tests**: Expensive, slow, limited coverage

---

## Decision 7: Map Integration Strategy

**Decision**: Use **react-native-maps with Expo EAS Configuration**

**Rationale**:
- Industry-standard React Native map library
- Full support for custom markers and overlays
- Works with Expo and native platforms
- Integrates well with Expo's native modules system

**Implementation Pattern**:
```typescript
import MapView from 'react-native-maps';

<MapView
  initialRegion={{
    latitude: 36.8065,
    longitude: 10.1815,
    latitudeDelta: 0.0922,
    longitudeDelta: 0.0421,
  }}
  showsUserLocation={true}
  onRegionChangeComplete={handleRegionChange}
  onMarkerPress={handleMarkerPress}
>
  {stations.map(station => (
    <MapView.Marker
      key={station.id}
      coordinate={{
        latitude: station.latitude,
        longitude: station.longitude,
      }}
      onPress={() => navigateToStation(station.id)}
    />
  ))}
</MapView>
```

**Features**:
- Custom markers for stations with distance indicators
- Map markers with event emission on press
- Debounced region changes (300-500ms)
- Offline map tiles with caching
- User location with permissions handling

**Alternatives Considered**:
1. **Google Maps SDK**: Platform-specific, more complex setup
2. **Mapbox GL**: Good alternative, but requires additional tokens
3. **Leaflet (Web)**: Not suitable for React Native

---

## Decision 8: Clickstream Integration

**Decision**: Use **Existing api-client with Event Taxonomy**

**Rationale**:
- Reuses existing `clickstream-service` and `event-taxonomy` packages
- Follows project standards for event tracking
- Standard event envelope and validation
- Supports both anonymous and authenticated users

**Implementation Pattern**:
```typescript
import { emitEvent } from '@/lib/clickstream';

// Event emission utility
function emitStationEvent(eventName: string, stationId: string) {
  emitEvent('station.clicked', {
    station_id: stationId,
    event_id: crypto.randomUUID(),
  });
}

// Map marker press event
<MapView.Marker
  onPress={() => {
    emitStationEvent('station.marker_clicked', station.id);
    navigateToStation(station.id);
  }}
/>
```

**Events Emitted**:
- `station.marker_clicked`
- `station.opened`
- `favorite_station.added`
- `favorite_station.removed`
- `review.submitted`

**Alternatives Considered**:
1. **Custom Event System**: Reinventing wheel, no standardization
2. **Firebase Analytics**: Requires new SDK integration, not following project standards

---

## Decision 9: Navigation Strategy

**Decision**: Use **React Navigation v6 with Native Stack Navigator**

**Rationale**:
- Industry-standard navigation for React Native
- Good performance with hardware back button support
- Easy integration with Expo
- Type-safe routing with TypeScript

**Implementation Pattern**:
```typescript
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';

const Stack = createNativeStackNavigator();

function AppNavigator() {
  return (
    <NavigationContainer>
      <Stack.Navigator>
        <Stack.Screen name="Map" component={DashboardPage} />
        <Stack.Screen name="Station" component={StationDetailPage} />
        <Stack.Screen name="Favorites" component={FavoritesPage} />
        <Stack.Screen name="Profile" component={ProfilePage} />
      </Stack.Navigator>
    </NavigationContainer>
  );
}
```

**Features**:
- Deep linking support
- Native transitions and gestures
- TypeScript type safety
- Back button handling
- State preservation during navigation

**Alternatives Considered**:
1. **React Navigation v7**: Not yet stable for Expo
2. **Gestalt**: Only for web, not mobile

---

## Decision 10: RTL Implementation

**Decision**: Use **react-native-locale-identify with Platform-Level RTL Support**

**Rationale**:
- Detects user's locale automatically
- React Native has native RTL support
- Works with all components and text
- Requires minimal custom code

**Implementation Pattern**:
```typescript
import { I18nManager } from 'react-native';
import { getLocales } from 'react-native-locale-identify';

// Detect RTL language on app launch
const locale = getLocales()[0];
const isRTL = locale.languageCode === 'ar' || locale.languageCode === 'ar-SA';

// Enable RTL if needed
if (isRTL) {
  I18nManager.allowRTL(true);
  I18nManager.isRTL = true;
}

// Use in styles
<Text style={{ textAlign: isRTL ? 'right' : 'left' }}>
  {text}
</Text>
```

**Features**:
- Automatic RTL detection on app launch
- RTL layout flipping for all components
- Support for Arabic and French RTL
- No manual layout adjustments needed

**Alternatives Considered**:
1. **Manual RTL Handling**: Error-prone and labor-intensive
2. **Platform-Specific RTL**: Requires separate code paths for iOS/Android

---

## Research Conclusion

All technical unknowns have been resolved through research and industry best practices. The selected architecture, state management, testing strategy, and optimization techniques align with:
- Project constitution principles
- Performance requirements
- Security requirements
- Testing requirements
- User experience requirements

The implementation plan is now ready for Phase 1 design artifacts (data model, contracts, quickstart).