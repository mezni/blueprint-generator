# Data Model: Cross-Platform UI Synchronization

## Client-Side State

### NavigationState

Shared navigation state consumed by both desktop NavBar and mobile BottomTabBar.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `activeTab` | Enum | Yes | One of: `map`, `explore`, `saved`, `profile` |
| `previousTab` | Enum | No | Previously active tab (for animated transitions) |

**Validation rules:**
- `activeTab` MUST be one of the four enum values
- Default value on first render: `map`

### SearchState

State of the search/filter bar on the map view.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query` | String | No | Current search text (empty string when idle) |
| `filters` | FilterState | No | Active filter set (nullable — null means no filters applied) |
| `results` | StationSummary[] | No | Search results from `GET /api/v1/search` (empty array when no search performed) |
| `isSearching` | Boolean | Yes | True while a search request is in-flight |
| `error` | String | No | Error message if the last search failed (null when no error) |

### FilterState

Mutable filter state synced across platforms via `GET/PUT /api/v1/filters`.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connector_types` | ConnectorType[] | No | Filter by connector type (empty array = all types) |
| `status` | StationStatus[] | No | Filter by station status (empty array = all statuses) |
| `min_available` | Integer | No | Minimum available chargers (null = no minimum) |

**Validation rules:**
- `min_available` MUST be >= 0 when present
- `connector_types` items MUST be one of: `type_2`, `type_2_combo`, `chademo`, `ccs`, `tesla`
- `status` items MUST be one of: `available`, `busy`, `offline`, `unknown`

### StationDetailState

State of the station detail panel/sheet.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `station` | StationDetail | No | Currently selected station (null when panel/sheet is closed) |
| `sheetMode` | Enum | Yes | One of: `closed`, `peek` (mobile only), `expanded` |
| `isLoading` | Boolean | Yes | True while station detail is being fetched |
| `error` | String | No | Error message if detail fetch failed |

**Validation rules:**
- `sheetMode` transitions: `closed → peek (mobile) / expanded (desktop) → expanded → peek → closed`
- Desktop always transitions directly `closed → expanded` (no peek state)

### ViewportState

Map viewport state shared by clickstream events.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `center` | GeoPoint | Yes | `{lat: number, lng: number}` |
| `zoom` | Number | Yes | Zoom level (1-18) |
| `bounds` | Bounds | Yes | Visible map bounds `{north, south, east, west}` |

### GeoPoint

| Field | Type | Required |
|-------|------|----------|
| `lat` | Number | Yes |
| `lng` | Number | Yes |

### Bounds

| Field | Type | Required |
|-------|------|----------|
| `north` | Number | Yes |
| `south` | Number | Yes |
| `east` | Number | Yes |
| `west` | Number | Yes |

## API Response Shapes (from contracts/api.yaml)

### StationSummary

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `station_id` | String (uuid) | Yes | Station identifier (`stn-` nanouuid) |
| `station_name` | String | Yes | Display name of the charging station |
| `address` | String | Yes | Street address |
| `available_chargers` | Integer | Yes | Number of currently available chargers |
| `total_chargers` | Integer | Yes | Total number of chargers at the station |
| `status` | StationStatus | Yes | Current operational status |

### StationDetail

Extends StationSummary with:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connector_types` | ConnectorType[] | Yes | Types of connectors available (at least one) |
| `navigate_url` | String (uri) | Yes | Deep link to Google Maps / Waze directions |

### ClickstreamEvent

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_name` | Enum | Yes | One of: `marker_tap`, `search_submit`, `filter_change`, `zoom_in`, `zoom_out`, `locate_me` |
| `platform` | Enum | Yes | `desktop_web` or `mobile_app` |
| `session_id` | String (uuid) | Yes | Client-generated session identifier |
| `timestamp` | String (datetime) | Yes | ISO-8601 UTC |
| `properties` | Object | No | Variable payload (see spec) |

## Platform Components

| Component | Platform | Description |
|-----------|----------|-------------|
| `NavBar` | Desktop web | Horizontal bar with 4 nav items, underline active indicator |
| `BottomTabBar` | Mobile | Bottom tab navigator with 4 items, filled icon active indicator |
| `MapPortal` | Desktop web | Full-height map viewport with overlaid search/filter panel |
| `MapScreen` | Mobile | Full-height MapView with compact header |
| `SearchBar` | Both | Text input with debounced search submission |
| `FilterControls` | Both | Filter chips/dropdowns for connector type, status, min availability |
| `StationDetailPanel` | Desktop web | Fixed-height bottom panel, dismiss via X or outside click |
| `StationDetailSheet` | Mobile | Draggable bottom sheet with peek (120px) / expanded (70%) states |
| `ZoomControls` | Both | Zoom in/out + locate-me, inline group (desktop) or floating (mobile) |
| `FAB` | Both | Floating action button (bottom-center), platform-specific styling |

## State Transitions

### Station Detail (Mobile)

```
closed → peek (120px) → expanded (70%) → peek → closed
        ↳ (marker tap)   ↳ (drag up)    ↳ (swipe down past peek threshold)
```

### Station Detail (Desktop)

```
closed → expanded → closed
        ↳ (marker tap)   ↳ (X click or outside click)
```

### Filter Sync Flow

```
Client A sets filters
  → PUT /api/v1/filters?session_id=A {filters}
  → Server stores {filters, updated_at: now} keyed by A

Client B polls
  → GET /api/v1/filters?session_id=A
  → Server returns {filters, updated_at}
  → Client B merges filters locally (last-writer-wins by updated_at)
```
