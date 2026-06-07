# Bornemap Driver Mobile App

## Project Overview

Driver Mobile App implementation for the Bornemap project, built with Expo and React Native. This app enables drivers to discover charging stations, view station details, manage favorites, and submit reviews.

## Tech Stack

- **Framework**: React Native 0.76.5 with Expo 52.0.0
- **Language**: TypeScript
- **Navigation**: React Navigation with Expo Router
- **State Management**: TanStack Query (React Query)
- **Styling**: Tailwind CSS with RTL support
- **Map**: React Native Maps
- **Storage**: AsyncStorage for local data
- **Design**: Custom design tokens and system UI components

## Project Structure

```
apps/driver-mobile/
├── src/
│   ├── components/        # Reusable UI components
│   │   ├── ErrorBoundary.tsx
│   │   ├── AuthGate.tsx
│   │   └── ui/
│   │       └── StationCard.tsx
│   ├── hooks/             # Custom React hooks
│   │   ├── useAuth.ts
│   │   ├── useTheme.ts
│   │   ├── useStations.ts
│   │   ├── useFavorites.ts
│   │   └── useNetworkStatus.ts
│   ├── lib/               # Utility libraries
│   │   ├── api.ts
│   │   └── api-endpoints.ts
│   ├── pages/             # Screen components
│   │   ├── DashboardPage.tsx
│   │   ├── StationDetailPage.tsx
│   │   ├── FavoritesPage.tsx
│   │   └── ReviewForm.tsx
│   ├── services/          # Business logic services
│   │   ├── station-service.ts
│   │   ├── auth-service.ts
│   │   ├── review-service.ts
│   │   ├── offline-manager.ts
│   │   ├── notification-service.ts
│   │   ├── device-info-service.ts
│   │   ├── logger.ts
│   │   └── mock-service.ts
│   ├── styles/            # Styling utilities
│   │   └── index.ts
│   ├── theme/             # Design tokens
│   │   ├── tokens.ts
│   │   └── config.ts
│   ├── types/             # TypeScript types
│   │   └── index.ts
│   ├── utils/             # Helper functions
│   │   └── rtl-utils.ts
│   ├── app/               # Expo Router pages
│   │   ├── _app.tsx
│   │   ├── _layout.tsx
│   │   ├── dashboard.tsx
│   │   ├── station-detail.tsx
│   │   ├── favorites.tsx
│   │   └── review-form.tsx
│   └── index.ts
├── assets/                # Static assets
├── package.json
├── tsconfig.json
├── tailwind.config.ts
├── app.json
└── .env.example
```

## Installation

1. Navigate to the project directory:
```bash
cd apps/driver-mobile
```

2. Install dependencies:
```bash
npm install
```

3. Configure environment variables:
```bash
cp .env.example .env
```

4. Start development server:
```bash
npm run dev
```

## Features Implemented

### Phase 1: Setup (Completed)
- ✅ Create Expo project structure
- ✅ Initialize TypeScript configuration
- ✅ Install core dependencies
- ✅ Initialize Tailwind CSS
- ✅ Create project directory structure
- ✅ Configure environment variables
- ✅ Create base styling setup

### Phase 2: Foundational (In Progress)
- ⏳ Create map discovery service
- ⏳ Implement map interactions
- ⏳ Create offline data manager
- ⏳ Implement favorite system
- ⏳ Create favorites page UI
- ⏳ Implement review system
- ⏳ Create review form UI
- ⏳ Integrate authentication flow

### Phase 3: User Story 1 - Map Discovery (Pending)
- ⏳ Map rendering with markers
- ⏳ Station markers with info
- ⏳ View station details on tap
- ⏳ Filter stations by status
- ⏳ Search stations
- ⏳ Nearby stations functionality

### Phase 4: User Story 2 - Station Details (Pending)
- ⏳ Display station information
- ⏳ Show charger availability
- ⏳ Show station reviews
- ⏳ Calculate distances and directions
- ⏳ Share station information
- ⏳ Navigation to station

### Phase 5: User Story 3 - Favorites (Pending)
- ⏳ Add stations to favorites
- ⏳ View favorite stations
- ⏳ Remove stations from favorites
- ⏳ Persist favorites locally
- ⏳ Sync favorites with server

### Phase 6: User Story 4 - Reviews (Pending)
- ⏳ Submit reviews
- ⏳ View station reviews
- ⏳ Rate stations
- ⏳ Manage review visibility

### Phase 7: User Story 5 - Login Flow (Pending)
- ⏳ Login screen
- ⏳ Auth state management
- ⏳ Token refresh
- ⏳ Logout functionality
- ⏳ Error handling

### Phase 8: User Story 6 - Offline Mode (Pending)
- ⏳ Offline data caching
- ⏳ Offline status indicators
- ⏳ Data synchronization
- ⏳ Offline UI states

### Phase 9: User Story 7 - Cross-Cutting (Pending)
- ⏳ Global error handling
- ⏳ Loading states
- ⏳ Offline indicators
- ⏳ Empty states
- ⏳ Loading skeletons

### Phase 10: Polish & Cross-Cutting (Pending)
- ⏳ Performance optimization
- ⏳ Accessibility
- ⏳ Analytics integration
- ⏳ Push notifications
- ⏳ Testing

## Key Dependencies

- **Expo**: Core framework and tooling
- **React Native**: Mobile app framework
- **TanStack Query**: Server state management
- **React Navigation**: App navigation
- **Tailwind CSS**: Utility-first styling
- **AsyncStorage**: Local data storage
- **React Native Maps**: Map functionality

## API Integration

The app connects to the following API endpoints:
- `/api/v1/driver/stations` - Get stations
- `/api/v1/driver/stations/:id` - Get station details
- `/api/v1/driver/stations/nearby` - Get nearby stations
- `/api/v1/driver/stations/:id/chargers` - Get station chargers
- `/api/v1/driver/favorites` - Manage favorites
- `/api/v1/driver/stations/:id/reviews` - Get station reviews
- `/api/v1/driver/stations/:id/reviews` - Submit reviews
- `/api/v1/driver/auth/login` - User login
- `/api/v1/driver/auth/logout` - User logout

## Environment Variables

- `EXPO_PUBLIC_API_BASE_URL` - Driver API base URL
- `EXPO_PUBLIC_AUTH_BASE_URL` - Auth service base URL
- `EXPO_PUBLIC_REALM` - Keycloak realm
- `EXPO_PUBLIC_CLIENT_ID` - Client ID for authentication
- `EXPO_PUBLIC_SUPPORTED_LANGUAGES` - Supported languages (comma-separated)
- `EXPO_PUBLIC_MAP_LAT` - Default map latitude
- `EXPO_PUBLIC_MAP_LNG` - Default map longitude
- `EXPO_PUBLIC_MAP_DEFAULT_RADIUS_KM` - Default search radius
- `EXPO_PUBLIC_MAP_MAX_RADIUS_KM` - Maximum search radius
- `EXPO_PUBLIC_OFFLINE_CACHE_SIZE_MB` - Offline cache size

## Development

### Run in Development Mode
```bash
npm run dev
```

### Run iOS Simulator
```bash
npm run ios
```

### Run Android Emulator
```bash
npm run android
```

### Build for Production
```bash
npm run build
```

## Testing

```bash
npm run test
```

## Code Style

The project follows these conventions:
- Use TypeScript for all code
- Follow React functional components
- Use custom hooks for reusable logic
- Service classes for business logic
- Utility functions for helpers
- Tailwind CSS for styling

## Architecture Principles

The app follows the constitution principles:
- **Data-First**: Design focused on data entities and relationships
- **Service Separation**: Clear separation between UI, business logic, and data access
- **Authorization**: Centralized auth with token management
- **Contract-Driven APIs**: API contracts defined and validated
- **Event-Driven State**: Real-time updates through events
- **Soft Delete**: No permanent data deletion
- **Testing**: Comprehensive test coverage planned

## Performance Targets

- Support 10,000 concurrent users
- Handle 50 events/sec baseline
- Map rendering <300ms
- Data synchronization <500ms
- Offline response <1000ms

## License

Internal project for Bornemap.
