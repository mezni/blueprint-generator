# UI Component Contracts

Since this feature operates entirely on static client-side mock data with no backend, the contracts define **component prop interfaces** and **mock data shapes** rather than API endpoints.

## Admin Dashboard Components

### DashboardOverview
- **State**: `activeTab`, `entitiesOpen`
- **Tabs**: `overview`, `partners`, `stations`, `users`, `analytics`, `settings`, `logs`
- **Nested nav**: ENTITIES → PARTNERS, STATIONS (collapsible)

### PartnersTable
- **Props**: none (reads from inline `mockPartners` array)
- **Columns**: ID, BRAND ENTITY NAME, HUBS, STATUS
- **Features**: inline text search filtering rows in real time
- **Data**: `[{ id, name, hubs, status }]`

### StationsTable
- **Props**: none (reads from inline `mockStations` array)
- **Columns**: HUB ID, NAME DESIGNATION, ZONAL PLACEMENT, STATUS
- **Features**: inline text search filtering rows in real time
- **Data**: `[{ id, name, location, status }]`

### Sidebar
- **Props**: `activeTab`, `onTabChange`, `entitiesOpen`, `onEntitiesToggle`
- **Links**: OVERVIEW, ENTITIES (collapsible → PARTNERS, STATIONS), USERS, ANALYTICS, SETTINGS, LOGS

### TopBar
- **Props**: none (static — title and MOCK ENGINE ACTIVE badge)

## Web Driver Components

### MapPortal
- **State**: `selectedStation`, `zoomIntent`
- **Children**: MapContainer, SearchOverlay, ZoomButtons, DetailCard
- **Mock Stations**: inline `MOCK_STATIONS` array with `{ id, name, latitude, longitude, partner, chargers }`

## Mobile Driver Components

### MobileMapScreen
- **State**: `selectedStation`
- **Refs**: `mapRef` for camera control
- **Children**: MapView, TopOverlay, ZoomControls, BottomSheet
- **Mock Stations**: inline `STATIC_MOCK_STATIONS` array
