# Implementation Plan: Driver Mobile App

**Branch**: `012-driver-mobile-app` | **Date**: 2026-06-04 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/012-driver-mobile-app/spec.md`

## Summary

The Driver Mobile App is a React Native Expo application that provides map-based charging station discovery, detailed station information, favorites management, reviews, and secure authentication. The app reuses existing shared packages (`shared-types`, `api-client`, `auth-client`) and implements all business logic through the existing `/api/v1/driver/*` endpoints from the driver-service. The system targets 10,000 concurrent users with 50 events/second baseline traffic and includes comprehensive observability with structured logging, performance metrics, and error telemetry.

## Technical Context

**Language/Version**: TypeScript 6.0+, React 19.2.7 (via Expo), React Native 0.76.3

**Primary Dependencies**: Expo SDK 50, React Native Reanimated, @tanstack/react-query, keycloak-js, react-native-maps, react-native-gesture-handler

**Storage**: AsyncStorage (for favorites/local state), secure storage for sensitive data

**Testing**: Jest (unit), React Native Testing Library (component tests), Detox (E2E with Android/iOS)

**Target Platform**: iOS 14+ and Android 9+ via Expo Go and production builds

**Project Type**: Mobile application (Expo/React Native)

**Performance Goals**: 10 seconds to complete discovery view from app launch, 300ms map interaction latency on 4G, 50 events/second baseline traffic

**Constraints**: <100ms RTL layout conversion, offline-safe UI with state preservation, AES-256 encryption for local storage

**Scale/Scope**: 1 mobile app, 5 core user journeys (map discovery, station details, favorites, reviews, login), 7 user stories with P1/P2/P3 prioritization

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Data-First Source of Truth ✅ PASS
- Business data comes from `platform_db` via `/api/v1/driver/*` endpoints
- Mobile app displays and interacts with data from driver-service
- Analytics events derived from user actions, never influence business logic

### II. Strict Domain & Service Separation ✅ PASS
- Mobile app consumes existing `driver-service` APIs
- No new services or domain boundaries created
- Uses existing `shared-types`, `api-client`, `auth-client` packages

### III. Ownership-Enforced Authorization ✅ PASS
- Partners and partners' stations enforced at backend
- Driver role scoped to their registered driver account
- No client-side authorization beyond UI gating

### IV. Contract-Driven REST APIs ✅ PASS
- All interactions use existing `/api/v1/driver/*` endpoints
- Standard success/error envelopes from `api-contracts`
- Pagination enforced on list endpoints

### V. Event-Driven & Derived State ✅ PASS
- Map-based discovery from driver-service (derived from `platform_db`)
- Clickstream events sent to `clickstream-service` via existing `api-client`
- No direct GIS or analytics integration needed in mobile app

### VI. Soft Delete & Auditability ✅ PASS
- Favorites automatically removed for soft-deleted stations (FR-014)
- Station visibility respects soft-delete and status rules
- No hard deletes performed on mobile

### VII. Verification Discipline ✅ PASS
- Unit tests for API clients and hooks
- Integration tests for authentication flows
- E2E tests for critical user journeys (discovery, login, favorites, reviews)
- Performance metrics collected via observability layer

## Project Structure

### Documentation (this feature)

```text
specs/012-driver-mobile-app/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── mobile-app-contracts.md
├── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
└── checklists/          # Quality checklists
    └── requirements.md
```

### Source Code (repository root)

```text
apps/driver-mobile/
├── App.tsx                           # App entry point with routing
├── index.ts                          # Entry for Expo
├── package.json                      # Dependencies and scripts
├── tsconfig.json                     # TypeScript configuration
├── tailwind.config.ts                # Tailwind configuration
├── app.json                          # Expo configuration
├── src/
│   ├── main.tsx                      # React entry point with providers
│   ├── theme/
│   │   └── tokens.ts                 # Theme tokens from design-tokens
│   ├── components/
│   │   ├── ui/                       # Reusable UI components
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── input.tsx
│   │   │   ├── modal.tsx
│   │   │   └── map-container.tsx
│   │   ├── AuthGate.tsx              # Authentication gate component
│   │   ├── ErrorBoundary.tsx         # React error boundary
│   │   ├── FavoriteButton.tsx        # Favorite toggle component
│   │   └── ReviewButton.tsx          # Review action component
│   ├── pages/                        # Page components
│   │   ├── DashboardPage.tsx         # Map discovery view
│   │   ├── StationDetailPage.tsx     # Station details view
│   │   ├── FavoritesPage.tsx         # Favorites management
│   │   └── ProfilePage.tsx           # User profile and logout
│   ├── hooks/                        # Custom React hooks
│   │   ├── useAuth.tsx               # Authentication state
│   │   ├── useAdminOverview.ts       # (N/A - not part of mobile app)
│   │   ├── useFavorites.ts           # Favorites management
│   │   ├── useStationDetail.ts       # Station details with caching
│   │   └── useClickstream.ts         # Event emission
│   ├── lib/                          # Utility functions and constants
│   │   ├── api.ts                    # API client configuration
│   │   ├── clickstream.ts            # Event emission utility
│   │   ├── types.ts                  # TypeScript types
│   │   └── utils.ts                  # Utility functions
│   ├── services/                     # Service layer (optional)
│   │   └── notification.ts           # Push notification service
│   ├── navigation/                   # Navigation setup
│   │   └── AppNavigator.tsx          # React Navigation setup
│   ├── utils/                        # Helper functions
│   │   ├── format.ts                 # Formatting utilities
│   │   └── validation.ts             # Input validation
│   ├── styles/                       # Global styles
│   │   └── index.ts                  # Style constants
│   └── types/                        # TypeScript type definitions
│       └── index.ts
├── tests/                            # Test files
│   ├── unit/
│   │   ├── hooks/
│   │   └── utils/
│   ├── integration/
│   │   └── auth/
│   └── e2e/                          # Detox tests
│       ├── discovery.spec.ts
│       ├── login.spec.ts
│       ├── favorites.spec.ts
│       └── reviews.spec.ts
├── __tests__/                        # Component tests
│   └── components/
├── android/                          # Android-specific configuration
│   └── app/build.gradle
└── ios/                              # iOS-specific configuration
    └── Podfile
```

**Structure Decision**: Selected Option 3 (Mobile + API) with Expo-based structure. The driver-mobile app is a standalone Expo project consuming the existing `/api/v1/driver/*` endpoints from the driver-service. No new backend services or API contracts are created - the mobile app reuses all existing shared packages and infrastructure.

## Complexity Tracking

> **No violations requiring justification** - All constitution principles are satisfied without tradeoffs.
