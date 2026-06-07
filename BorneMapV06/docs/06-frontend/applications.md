# Frontend Applications

## Driver Web App

- **Stack:** React + Vite + Tailwind CSS + shadcn/ui
- **Maps:** Leaflet
- **Theme:** Bright Theme (light, outdoor-optimized, `ev-*` namespace)
- **Audience:** Public and registered drivers
- **Platform:** Web browser

### Pages
- HomePage (map + station list)
- StationDetailPage
- SearchResultsPage
- FavoritesPage
- ProfilePage
- LoginPage

**Configuration:** `apps/driver-web/tailwind.config.js`

## Driver Mobile App

- **Stack:** React Native Expo
- **Theme:** Bright Theme (light, outdoor-optimized, `ev-*` namespace)
- **Audience:** Public and registered drivers
- **Platform:** iOS + Android

### Screens
- MapScreen
- StationListScreen
- StationDetailScreen
- SearchScreen
- FavoritesScreen
- ProfileScreen
- LoginScreen

**Configuration:** `apps/driver-mobile/tailwind.config.js`

## Partner Dashboard

- **Stack:** React + Vite + Tailwind CSS + shadcn/ui
- **Theme:** Admin Theme (light, operational, `admin-*` namespace)
- **Audience:** Partner users managing stations and charging infrastructure
- **Platform:** Web browser

### Pages
- OverviewPage (real-time metrics, revenue, usage)
- StationsPage (station list, status monitoring)
- StationEditPage (charger management)
- ChargersPage (detailed charger specs)
- AvailabilityPage (booking calendar)
- ReportsPage (analytics, exports)

**Configuration:** `apps/partner-dashboard/tailwind.config.js`  
**Design Guide:** `docs/06-frontend/admin-theme.md`

## Admin Dashboard

- **Stack:** React + Vite + Tailwind CSS + shadcn/ui
- **Theme:** Admin Theme (light, operational, `admin-*` namespace)
- **Audience:** Admin users managing the platform
- **Platform:** Web browser

### Pages
- OverviewPage (system metrics, revenue, active sessions)
- UsersPage (driver list, verification status)
- PartnersPage (partner management, tiers)
- StationsPage (full network inventory, approval workflow)
- ChargersPage (device management, firmware)
- ReviewsPage (user reviews, moderation)
- ReportsPage (analytics, exports, compliance)

**Configuration:** `apps/admin-dashboard/tailwind.config.js`  
**Design Guide:** `docs/06-frontend/admin-theme.md`
