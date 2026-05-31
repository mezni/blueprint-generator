# Data Model: Mobile Driver App Scaffold

## Overview

This phase introduces no persistent storage or backend database. The following
are conceptual UI-layer data structures used by the mobile driver app. All data
is ephemeral (in-memory, React component state).

## Entities

### MapViewport

Represents the geographic window displayed on screen.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| latitude | number | 36.8065 | Center latitude (WGS 84) |
| longitude | number | 10.1815 | Center longitude (WGS 84) |
| latitudeDelta | number | 0.12 | Vertical span in degrees at current zoom |
| longitudeDelta | number | 0.06 | Horizontal span in degrees at current zoom |

**Constraints:**
- Coordinates MUST use WGS 84 (EPSG:4326)
- Viewport MUST initialize to the Tunis center on first render
- Full pan, zoom, and gesture interaction MUST be supported (per Q2 clarification)

### MapMarker

Represents a pinned location on the map.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| latitude | number | 36.8065 | Marker geographic latitude |
| longitude | number | 10.1815 | Marker geographic longitude |
| title | string | "Tunis Core Baseline" | Display label for the marker callout |
| description | string | "Phase 1 Offline Isolation Landmark Checkpoint" | Detail text for the marker callout |

**Constraints:**
- Exactly one marker is rendered at the Tunis center coordinate
- Marker is static (no drag interaction)

### DebugOverlay

Presents diagnostic state information overlaid on the map.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| modeText | string | "BorneMap Sandbox Mode" | Primary diagnostic label (bold) |
| subText | string | "Tunisia Map Layer Rendered Offline" | Secondary diagnostic subtitle |
| visible | boolean | true | Whether the overlay is displayed |
| interactive | boolean | false | Whether the overlay blocks map gestures |

**Constraints:**
- Overlay MUST NOT block map gestures (touch-through)
- Overlay MUST remain visible when map component fails (per Q3 clarification)
- Overlay MUST have semi-transparent white background with rounded corners and shadow

## State Transitions

```
App Launch
    │
    ▼
MapView Mounting ──► MapReady (normal)
    │                      │
    │                      ├──► User pans/zooms (viewport updates)
    │                      └──► Marker display (static)
    │
    └──► MapInitFailure ──► FallbackScreen + DebugOverlay
```

## Data Flow

```
User Device
    │
    ├── Expo Go runtime
    │   ├── App.js ──► SafeAreaView ──► StatusBar
    │   └── MapScreen.js
    │       ├── MapView (PROVIDER_DEFAULT)
    │       │   ├── initialRegion: TUNISIA_CENTER
    │       │   └── Marker (Tunis coordinate)
    │       └── DebugOverlay (absolute positioned View)
    │
    └── No network calls — entirely self-contained
```
