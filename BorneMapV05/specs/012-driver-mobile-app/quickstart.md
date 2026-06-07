# Quickstart: Driver Mobile App

**Feature**: Driver Mobile App (Sprint 12)
**Date**: 2026-06-04
**Purpose**: Get developers up and running quickly with the driver mobile app

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Setup](#setup)
3. [Environment Configuration](#environment-configuration)
4. [Running the App](#running-the-app)
5. [Basic Usage](#basic-usage)
6. [Testing](#testing)
7. [Development Workflow](#development-workflow)
8. [Common Issues](#common-issues)

---

## Prerequisites

### Hardware Requirements

- **Development Machine**: macOS, Windows, or Linux
- **Node.js**: 18.x or higher
- **npm**: 9.x or higher (or use pnpm/yarn)
- **Expo**: CLI tool installed globally

### Software Requirements

- **Expo CLI**: `npm install -g expo-cli`
- **Xcode**: macOS only (for iOS development)
- **Android Studio**: Android development (Windows/Linux)
- **Git**: Version control

### Required Accounts

- **GitHub**: For repository access
- **Expo Account**: For EAS Build services (optional)
- **Keycloak**: For authentication testing

---

## Setup

### 1. Clone the Repository

```bash
git clone <repository-url>
cd BorneMap
```

### 2. Install Dependencies

```bash
# Install root dependencies
npm install

# Install driver-mobile dependencies
cd apps/driver-mobile
npm install
```

### 3. Set Up Environment Variables

Create a `.env` file in the `apps/driver-mobile` directory:

```bash
cp .env.example .env
```

### 4. Configure Environment Variables

Edit `.env` with your configuration:

```bash
# Driver Service API
EXPO_PUBLIC_API_BASE_URL=https://api.example.tn
EXPO_PUBLIC_AUTH_BASE_URL=https://auth.example.tn

# Keycloak Configuration
EXPO_PUBLIC_REALM=bornemap
EXPO_PUBLIC_CLIENT_ID=bornemap-driver-mobile
EXPO_PUBLIC_SUPPORTED_LANGUAGES=ar,fr

# Map Configuration
EXPO_PUBLIC_MAP_LAT=36.8065
EXPO_PUBLIC_MAP_LNG=10.1815
EXPO_PUBLIC_MAP_DEFAULT_RADIUS_KM=10
EXPO_PUBLIC_MAP_MAX_RADIUS_KM=50

# Mapbox Token (optional, if using Mapbox instead of native maps)
EXPO_PUBLIC_MAPBOX_TOKEN=your_mapbox_token_here

# Offline Configuration
EXPO_PUBLIC_OFFLINE_CACHE_SIZE_MB=100

# Feature Flags
EXPO_PUBLIC_FF_ENABLE_REVIEWS=true
EXPO_PUBLIC_FF_ENABLE_GIS_SYNC=false
EXPO_PUBLIC_FF_ENABLE_ANALYTICS=true
```

### 5. Start Development Server

```bash
# Start Expo development server
npm run dev
```

The app will open on your device or simulator.

---

## Running the App

### Development Build (Recommended)

#### iOS

```bash
# In ios/ directory
cd ios
pod install
cd ..
npm run ios
```

#### Android

```bash
npm run android
```

#### Expo Go (Quick Test)

```bash
npm run start
```

1. Scan QR code with Expo Go app on your phone
2. Or press `i` for iOS simulator, `a` for Android emulator

### Production Build

#### EAS Build (Recommended for Production)

```bash
# Install EAS CLI
npm install -g eas-cli

# Login to Expo
eas login

# Configure project
eas build:configure

# Build for iOS
eas build --platform ios

# Build for Android
eas build --platform android
```

#### Manual Build

```bash
# iOS (Xcode)
open ios/BornemapDriverMobile.xcworkspace

# Android (Android Studio)
open android/BornemapDriverMobile/android
```

---

## Environment Configuration

### Environment Variables Reference

| Variable | Default | Required | Description |
|----------|---------|----------|-------------|
| `EXPO_PUBLIC_API_BASE_URL` | - | Yes | Driver service API base URL |
| `EXPO_PUBLIC_AUTH_BASE_URL` | - | Yes | Keycloak auth service URL |
| `EXPO_PUBLIC_REALM` | - | Yes | Keycloak realm name |
| `EXPO_PUBLIC_CLIENT_ID` | - | Yes | Keycloak client ID |
| `EXPO_PUBLIC_SUPPORTED_LANGUAGES` | - | Yes | Supported languages (comma-separated) |
| `EXPO_PUBLIC_MAP_LAT` | 36.8065 | No | Default map center latitude |
| `EXPO_PUBLIC_MAP_LNG` | 10.1815 | No | Default map center longitude |
| `EXPO_PUBLIC_MAP_DEFAULT_RADIUS_KM` | 10 | No | Default search radius |
| `EXPO_PUBLIC_MAP_MAX_RADIUS_KM` | 50 | No | Maximum search radius |
| `EXPO_PUBLIC_MAPBOX_TOKEN` | - | No | Mapbox token (optional) |
| `EXPO_PUBLIC_OFFLINE_CACHE_SIZE_MB` | 100 | No | Offline cache size in MB |
| `EXPO_PUBLIC_FF_ENABLE_REVIEWS` | true | No | Enable reviews feature |
| `EXPO_PUBLIC_FF_ENABLE_GIS_SYNC` | false | No | Enable GIS sync feature |
| `EXPO_PUBLIC_FF_ENABLE_ANALYTICS` | true | No | Enable analytics tracking |

### Loading Environment Variables

```typescript
// src/lib/config.ts
const config = {
  apiBaseUrl: process.env.EXPO_PUBLIC_API_BASE_URL || 'https://api.example.tn',
  authBaseUrl: process.env.EXPO_PUBLIC_AUTH_BASE_URL || 'https://auth.example.tn',
  realm: process.env.EXPO_PUBLIC_REALM || 'bornemap',
  clientId: process.env.EXPO_PUBLIC_CLIENT_ID || 'bornemap-driver-mobile',
  supportedLanguages: (process.env.EXPO_PUBLIC_SUPPORTED_LANGUAGES || 'fr').split(','),
  map: {
    defaultLat: parseFloat(process.env.EXPO_PUBLIC_MAP_LAT || '36.8065'),
    defaultLng: parseFloat(process.env.EXPO_PUBLIC_MAP_LNG || '10.1815'),
    defaultRadiusKm: parseInt(process.env.EXPO_PUBLIC_MAP_DEFAULT_RADIUS_KM || '10'),
    maxRadiusKm: parseInt(process.env.EXPO_PUBLIC_MAP_MAX_RADIUS_KM || '50'),
  },
  featureFlags: {
    enableReviews: process.env.EXPO_PUBLIC_FF_ENABLE_REVIEWS === 'true',
    enableGisSync: process.env.EXPO_PUBLIC_FF_ENABLE_GIS_SYNC === 'true',
    enableAnalytics: process.env.EXPO_PUBLIC_FF_ENABLE_ANALYTICS === 'true',
  },
};

export default config;
```

---

## Basic Usage

### 1. Map Discovery

```typescript
import { useQuery } from '@tanstack/react-query';
import { apiClient } from '@/lib/api';

function DashboardPage() {
  const { data, isLoading, error } = useQuery({
    queryKey: ['stations'],
    queryFn: () => apiClient.get('/driver/stations', {
      lat: 36.8065,
      lng: 10.1815,
      radius_km: 10,
    }),
  });

  if (isLoading) return <MapSkeleton />;

  if (error) return <MapError />;

  return (
    <MapView>
      {data?.data.stations.map(station => (
        <MapView.Marker
          key={station.id}
          coordinate={{ latitude: station.latitude, longitude: station.longitude }}
          onPress={() => navigateToStation(station.id)}
        />
      ))}
    </MapView>
  );
}
```

### 2. Station Details

```typescript
import { useQuery } from '@tanstack/react-query';
import { useNavigation } from '@react-navigation/native';

function StationDetailPage({ route }) {
  const { stationId } = route.params;
  const { data } = useQuery({
    queryKey: ['station', stationId],
    queryFn: () => apiClient.get(`/driver/stations/${stationId}`),
  });

  if (!data) return <Loading />;

  return (
    <View>
      <StationHeader station={data.data.station} />
      <ChargerList chargers={data.data.station.chargers} />
      <ReviewList stationId={stationId} />
    </View>
  );
}
```

### 3. Favorites Management

```typescript
import { useQuery, useMutation } from '@tanstack/react-query';
import { apiClient } from '@/lib/api';

function useFavorites() {
  const { data, isLoading } = useQuery({
    queryKey: ['favorites'],
    queryFn: () => apiClient.get('/driver/favorites'),
  });

  const addFavorite = useMutation({
    mutationFn: (stationId: string) =>
      apiClient.post(`/driver/favorites/${stationId}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['favorites'] });
    },
  });

  const removeFavorite = useMutation({
    mutationFn: (stationId: string) =>
      apiClient.delete(`/driver/favorites/${stationId}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['favorites'] });
    },
  });

  return { favorites: data?.data.favorites || [], isLoading, addFavorite, removeFavorite };
}
```

### 4. Review Submission

```typescript
import { useMutation } from '@tanstack/react-query';
import { apiClient } from '@/lib/api';

function ReviewForm({ stationId }) {
  const [rating, setRating] = useState(5);
  const [comment, setComment] = useState('');

  const submitReview = useMutation({
    mutationFn: () =>
      apiClient.post('/driver/reviews', {
        station_id: stationId,
        rating,
        comment,
      }),
  });

  const handleSubmit = () => {
    submitReview.mutate();
  };

  return (
    <View>
      <RatingSelector rating={rating} onRatingChange={setRating} />
      <TextInput
        value={comment}
        onChangeText={setComment}
        placeholder="Write your review..."
      />
      <Button onPress={handleSubmit} disabled={submitReview.isPending}>
        Submit Review
      </Button>
    </View>
  );
}
```

### 5. Authentication

```typescript
import { useAuth } from '@/hooks/useAuth';
import { Button } from '@/components/ui/button';

function AuthGate({ children }) {
  const { isInitialized, isAuthenticated, login, logout } = useAuth();

  if (!isInitialized) {
    return <Loading />;
  }

  if (!isAuthenticated) {
    return (
      <View>
        <Button onPress={login}>Sign in with Keycloak</Button>
      </View>
    );
  }

  return <>{children}</>;
}
```

---

## Testing

### Unit Tests

```bash
# Run all unit tests
npm test

# Run tests in watch mode
npm run test:watch

# Run tests with coverage
npm run test:coverage
```

### Component Tests

```bash
# Run component tests
npm run test:components
```

### Integration Tests

```bash
# Run integration tests
npm run test:integration
```

### E2E Tests (Detox)

```bash
# Start iOS simulator
npm run e2e:ios

# Start Android emulator
npm run e2e:android
```

### Test Configuration

```typescript
// jest.config.js
module.exports = {
  preset: 'react-native',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
  testMatch: ['**/__tests__/**/*.test.tsx'],
  transformIgnorePatterns: [
    'node_modules/(?!(react-native|@react-native|expo)/)',
  ],
  collectCoverageFrom: [
    'src/**/*.{ts,tsx}',
    '!src/**/*.d.ts',
    '!src/**/*.test.{ts,tsx}',
  ],
  coverageThreshold: {
    global: {
      branches: 80,
      functions: 80,
      lines: 80,
      statements: 80,
    },
  },
};
```

---

## Development Workflow

### Adding a New Screen

1. **Create component** in `src/pages/`:
   ```bash
   touch src/pages/NewScreen.tsx
   ```

2. **Add to navigation** in `src/navigation/AppNavigator.tsx`:
   ```typescript
   <Stack.Screen name="NewScreen" component={NewScreen} />
   ```

3. **Add route handler**:
   ```typescript
   function NewScreen() {
     return <View>...</View>;
   }
   ```

4. **Add tests**:
   ```bash
   touch src/pages/__tests__/NewScreen.test.tsx
   ```

### Adding a New Feature

1. **Create hook** in `src/hooks/`:
   ```bash
   touch src/hooks/useNewFeature.ts
   ```

2. **Implement feature logic**:
   ```typescript
   export function useNewFeature() {
     // Implementation
   }
   ```

3. **Create API client method** in `src/lib/api.ts`:
   ```typescript
   async function newFeature() {
     return apiClient.get('/driver/new-feature');
   }
   ```

4. **Update types** in `src/lib/types.ts`:
   ```typescript
   export interface NewFeatureResponse {
     // Type definitions
   }
   ```

5. **Add tests**:
   ```typescript
   test('useNewFeature works correctly', () => {
     // Test implementation
   });
   ```

### Code Style

- **Prettier**: Run `npm run format` to format code
- **ESLint**: Run `npm run lint` to check code quality
- **TypeScript**: Run `npm run typecheck` to check types

### Build Checks

```bash
# Type check
npm run typecheck

# Lint
npm run lint

# Build for production
npm run build
```

---

## Common Issues

### Issue: "Module not found" error

**Solution**:
```bash
rm -rf node_modules package-lock.json
npm install
```

### Issue: Expo CLI not found

**Solution**:
```bash
npm install -g expo-cli
```

### Issue: iOS build fails with CocoaPods error

**Solution**:
```bash
cd ios
pod install
cd ..
npm run ios
```

### Issue: Android build fails with Gradle error

**Solution**:
```bash
cd android
./gradlew clean
cd ..
npm run android
```

### Issue: Map doesn't load

**Solution**:
- Ensure API base URL is correct in `.env`
- Check that driver-service is running
- Verify map API credentials

### Issue: Authentication fails

**Solution**:
- Check Keycloak configuration in `.env`
- Ensure client ID matches Keycloak configuration
- Verify JWT token is being stored correctly

### Issue: Offline mode not working

**Solution**:
- Check `EXPO_PUBLIC_OFFLINE_CACHE_SIZE_MB` in `.env`
- Verify AsyncStorage is working
- Check that offline queue is being populated

---

## Next Steps

1. **Review the data model**: See `data-model.md`
2. **Read the contracts**: See `contracts/mobile-app-contracts.md`
3. **Study the implementation plan**: See `plan.md`
4. **Check out the tests**: See `tests/`
5. **Start coding**: Follow the user stories in `spec.md`

---

## Additional Resources

- **Expo Documentation**: https://docs.expo.dev
- **React Native Documentation**: https://reactnative.dev
- **React Navigation**: https://reactnavigation.org
- **TanStack Query**: https://tanstack.com/query/latest
- **Project Constitution**: `.specify/memory/constitution.md`
- **API Documentation**: `docs/API.md`

---

## Getting Help

- **GitHub Issues**: Report bugs and feature requests
- **Project Chat**: Ask questions in the project communication channel
- **Documentation**: Check `docs/` for additional information
