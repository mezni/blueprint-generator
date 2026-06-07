# Component Contracts: Driver Web App

**Branch**: `009-driver-web-app` | **Date**: 2026-06-03

## Overview

Props interfaces for all driver-web components. Components are organized by the architecture defined in `plan.md`.

---

## Layout

```typescript
// Top-level app layout
interface AppLayoutProps {
  children: React.ReactNode;
}

// Header bar
interface HeaderProps {
  onSearchToggle: () => void;
  onFavoritesToggle: () => void;
  favoritesFilterActive: boolean;
}

// Full-screen map container with side panel
interface MapLayoutProps {
  children: React.ReactNode;
  sidePanelOpen: boolean;
  sidePanelWidth: number; // 260–400px
}
```

---

## MapView

```typescript
// Main map view — container for all map-related components
interface MapViewProps {
  onStationSelect: (stationId: string) => void;
  selectedStationId: string | null;
}

// Map state overlay (skeleton/spinner/empty-state)
type MapState = "idle" | "active" | "station-selected";

interface MapStateOverlayProps {
  state: MapState;
  hasStations: boolean;
}
```

---

## StationMarkers

```typescript
interface StationMarkersProps {
  map: L.Map;                    // Leaflet map instance (from onMount)
  stations: StationListItem[];
  selectedStationId: string | null;
  onMarkerClick: (stationId: string) => void;
  clusterOptions?: L.MarkerClusterGroupOptions;
}
```

---

## StationDetailPanel

```typescript
interface StationDetailPanelProps {
  stationId: string;
  onClose: () => void;
}
```

---

## StationInfo

```typescript
interface StationInfoProps {
  name: string;
  description: string | null;
  address: string | null;        // Derived from city + country
  distanceKm: number | null;
}
```

---

## ChargerList

```typescript
interface ChargerListProps {
  chargers: Charger[];
  chargerTypes: ChargerTypeInfo[];
}

interface ChargerItemProps {
  connectorType: ConnectorType;
  powerKw: number | null;
  status: ChargerStatus;
}
```

---

## SearchOverlay

```typescript
interface SearchOverlayProps {
  isOpen: boolean;
  onClose: () => void;
  onStationSelect: (stationId: string) => void;
}
```

---

## SearchResults

```typescript
interface SearchResultsProps {
  results: StationListItem[];
  isLoading: boolean;
  onStationSelect: (stationId: string) => void;
  emptyMessage?: string;         // Default: "No stations found"
}
```

---

## FavoriteButton

```typescript
interface FavoriteButtonProps {
  stationId: string;
  isFavorited: boolean;
  onToggle: (stationId: string) => void;
  disabled?: boolean;
  size?: "sm" | "md" | "lg";    // Default: "md"
}
```

---

## ReviewSection

```typescript
interface ReviewSectionProps {
  stationId: string;
  reviewSummary: ReviewSummary | null;
  userReview: Review | null;     // Current user's review, if exists
}

interface ReviewListProps {
  userReview: Review | null;     // Only user's own review is available
}

interface ReviewFormProps {
  stationId: string;
  existingReview: Review | null; // Null = create, non-null = edit
  onSuccess: () => void;
  onCancel?: () => void;
}
```

---

## AuthModal

```typescript
interface AuthModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  gatedAction: () => Promise<void>; // Action to execute after login
}
```

---

## MapContainer (Sprint 8 — Updated)

```typescript
// Updated to support clustering integration
interface MapContainerProps {
  className?: string;
  center?: [number, number];
  zoom?: number;
  onMount?: (map: L.Map) => void;
  onViewportChange?: (bounds: L.LatLngBounds, zoom: number) => void;
}
```
