# Quick Start Guide

## Prerequisites

- Node.js 18+ 
- npm or yarn
- Expo CLI
- iOS Simulator (macOS) or Android Emulator (Android Studio)

## Installation

1. **Navigate to the driver mobile app directory:**
```bash
cd apps/driver-mobile
```

2. **Install dependencies:**
```bash
npm install
```

3. **Configure environment variables:**
```bash
cp .env.example .env
```

Edit `.env` and configure your API endpoints:
```
EXPO_PUBLIC_API_BASE_URL=https://api.example.tn
EXPO_PUBLIC_AUTH_BASE_URL=https://auth.example.tn
EXPO_PUBLIC_REALM=bornemap
EXPO_PUBLIC_CLIENT_ID=bornemap-driver-mobile
```

## Development

1. **Start the development server:**
```bash
npm run dev
```

2. **Open in Expo Go app:**
   - Scan the QR code with your phone
   - Or use Expo Go app to view on device

## Running on Emulator/Simulator

### iOS Simulator
```bash
npm run ios
```

### Android Emulator
```bash
npm run android
```

## Project Structure

```
src/
├── app/              # Expo Router pages
├── components/       # React components
├── hooks/            # Custom hooks
├── services/         # Business logic
├── lib/              # Utilities
├── theme/            # Design tokens
├── types/            # TypeScript types
└── utils/            # Helper functions
```

## Available Scripts

```bash
# Start development server
npm run dev

# Run iOS
npm run ios

# Run Android
npm run android

# Build for production
npm run build
```

## Key Features

### Currently Implemented
- ✅ Expo/React Native project structure
- ✅ TypeScript configuration
- ✅ Tailwind CSS with RTL support
- ✅ Custom hooks (useAuth, useTheme, useStations, etc.)
- ✅ Service layer (StationService, AuthService, etc.)
- ✅ Error handling (ErrorBoundary)
- ✅ Authentication gate
- ✅ Station cards UI
- ✅ Favorites management
- ✅ Review system
- ✅ Offline data manager

### Roadmap
- ⏳ Map integration (react-native-maps)
- ⏳ Authentication flow
- ⏳ Push notifications
- ⏳ Analytics
- ⏳ Performance optimization

## Design Tokens

Colors:
- Primary: `#2563EB` (Blue)
- Secondary: `#6B7280` (Gray)
- Success: `#10B981` (Green)
- Error: `#EF4444` (Red)
- Surface: `#FFFFFF`
- Text: `#111827`

## API Integration

The app connects to these endpoints:
- `/api/v1/driver/stations` - Get all stations
- `/api/v1/driver/stations/:id` - Get station details
- `/api/v1/driver/stations/nearby` - Get nearby stations
- `/api/v1/driver/favorites` - Manage favorites
- `/api/v1/driver/stations/:id/reviews` - Get/review stations

## Environment Variables

- `EXPO_PUBLIC_API_BASE_URL` - API base URL
- `EXPO_PUBLIC_AUTH_BASE_URL` - Auth service URL
- `EXPO_PUBLIC_REALM` - Keycloak realm
- `EXPO_PUBLIC_CLIENT_ID` - Client ID
- `EXPO_PUBLIC_SUPPORTED_LANGUAGES` - Languages (e.g., "ar,fr")

## Debugging

### Enable Debug Mode
```bash
npm run dev -- --dev
```

### Clear Cache
```bash
npx expo start -c
```

### Check Logs
```bash
npx expo logs
```

## Common Issues

### "Module not found"
```bash
npm install
npx expo start -c
```

### iOS Build Failed
```bash
cd ios
pod install
cd ..
npm run ios
```

### Android Build Failed
```bash
cd android
./gradlew clean
cd ..
npm run android
```

## Next Steps

1. Read the full documentation: [README.md](./README.md)
2. Check implementation progress: [IMPLEMENTATION_PROGRESS.md](./IMPLEMENTATION_PROGRESS.md)
3. Review the specification: [specs/012-driver-mobile-app/spec.md](../specs/012-driver-mobile-app/spec.md)
4. Start implementing user stories in order

## Support

For issues or questions:
1. Check the main project documentation
2. Review the error messages in console
3. Verify environment variables are configured
4. Check the implementation progress document
