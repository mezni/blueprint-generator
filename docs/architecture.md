# BorneMap — Architecture Reference

> **Canonical architecture document.** Derived from and subordinate to the
> constitution at `.specify/memory/constitution.md`. In any conflict between this
> document and the constitution, **the constitution wins**.

**Last Updated**: 2026-05-24 (revised) | **Constitution Version**: 1.0.0

---

## 1. System Context

BorneMap is a geospatial EV charging discovery platform serving three actor groups:

```
┌─────────────────────────────────────────────────────────┐
│                    Public Users                          │
│   (Discover stations, view map, inspect metadata)        │
└────────────────────────┬────────────────────────────────┘
                         │ HTTPS
                         ▼
┌─────────────────────────────────────────────────────────┐
│              Internet / CDN / TLS Proxy                   │
└────────────────────────┬────────────────────────────────┘
                         │
              ┌──────────┴──────────┐
              ▼                     ▼
┌──────────────────────┐  ┌──────────────────────┐
│   Web Admin Portal   │  │  Mobile Driver App   │
│   (React + Vite)     │  │  (React Native/Expo) │
│   Public DMZ          │  │  Public DMZ          │
└──────────┬───────────┘  └──────────┬───────────┘
           │                          │
           └──────────┬──────────────┘
                      │ HTTPS / X-Mock-Token (phase 1-2)
                      ▼
┌──────────────────────────────────────┐
│         FastAPI Backend              │
│  (Python 3.12+, private network)     │
│                                      │
│  ┌─────────┐  ┌─────────┐           │
│  │ API     │→ │ Service │→ ...      │
│  │ Layer   │  │ Layer   │           │
│  └─────────┘  └─────────┘           │
│                      │              │
│                      ▼              │
│               ┌──────────┐          │
│               │Repository│          │
│               │  Layer   │          │
│               └──────────┘          │
└──────────────────┬──────────────────┘
                   │ SQL
                   ▼
┌──────────────────────────────────────┐
│    PostgreSQL + PostGIS              │
│  (Data Storage, isolated network)    │
│                                      │
│  ┌─────────────┐  ┌────────────┐    │
│  │ GEOGRAPHY   │  │  GiST      │    │
│  │ (Point,4326)│  │  Indexes   │    │
│  └─────────────┘  └────────────┘    │
└──────────────────────────────────────┘
```

---

## 2. Architecture Layers

### 2.1 API Layer

- **Framework:** FastAPI (Python 3.12+)
- **Purpose:** Request routing, input validation (Pydantic), auth middleware, OpenAPI
  schema generation.
- **Rules:**
  - Never contains business logic.
  - Calls Service Layer only.
  - OpenAPI 3.1 schema is the single source of truth for all API contracts.
  - All responses use structured JSON; errors use RFC 7807 `application/problem+json`.

### 2.2 Service Layer

- **Purpose:** Business rules, orchestration, validation, cross-cutting concerns.
- **Rules:**
  - Never contains SQL or ORM queries.
  - Calls Repository Layer only.
  - Validates permissions and spatial constraints before delegating to repositories.
  - Auth/identity logic lives here (mock JWT → argon2 → Keycloak per phase).

### 2.3 Repository Layer

- **Purpose:** Data access, spatial queries, persistence logic.
- **Rules:**
  - SQLAlchemy 2.x (async) for all database interaction.
  - All spatial queries use `ST_DWithin` for filtering; `ST_Distance` for
    sorting/ranking only.
  - Every spatial column MUST have a GiST index.
  - Repository methods accept and return domain models (not ORM instances across
    layer boundaries).

### 2.4 Database Layer

- **Engine:** PostgreSQL + PostGIS
- **Spatial Type:** `GEOGRAPHY(Point, 4326)` — never `GEOMETRY`.
- **Coordinate Order:** `[longitude, latitude]` everywhere (wire, storage, code).
- **Migrations:** Alembic sequential revisions with downgrade paths.

---

## 3. Repository Layout

### 3.1 Monorepo Structure

```
bornemap/
├── backend/
│   ├── app/
│   │   ├── main.py
│   │   ├── core/
│   │   │   ├── config.py         # pydantic-settings BaseSettings
│   │   │   ├── database.py       # SQLAlchemy async engine + session
│   │   │   └── dependencies.py   # Depends() factories (db session, auth)
│   │   ├── stations/             # feature domain — NOT type-scoped
│   │   │   ├── models.py
│   │   │   ├── schemas.py
│   │   │   ├── repository.py
│   │   │   ├── service.py
│   │   │   └── router.py
│   │   ├── users/                # (same pattern)
│   │   ├── reviews/              # (same pattern)
│   │   └── health/
│   │       └── router.py
│   ├── migrations/
│   ├── tests/
│   ├── alembic.ini
│   ├── pyproject.toml
│   └── Dockerfile
├── web/                           # React admin portal
│   ├── src/
│   │   ├── main.tsx
│   │   ├── components/
│   │   │   ├── ui/               # shadcn/ui generated components
│   │   │   └── map/              # Leaflet wrapper components
│   │   ├── features/stations/    # feature-scoped hooks, views, forms
│   │   ├── lib/api.ts            # typed fetch client
│   │   └── styles/globals.css
│   ├── vite.config.ts
│   └── tailwind.config.ts
├── mobile/                        # Expo Go React Native
│   ├── __tests__/                 # Jest smoke tests
│   ├── app/                       # Expo Router file-based routing
│   │   ├── (tabs)/
│   │   │   ├── index.tsx          # Map tab
│   │   │   └── favorites.tsx      # Favorites tab
│   │   └── _layout.tsx
│   ├── components/
│   │   ├── map/
│   │   └── sheets/
│   ├── hooks/
│   ├── lib/
│   │   ├── api.ts                 # shared API client (same pattern as web)
│   │   └── storage.ts            # AsyncStorage wrapper
│   ├── .npmrc                     # pnpm config (hoist deps for RN)
│   ├── jest.config.js             # jest-expo preset with pnpm support
│   ├── metro.config.js            # @expo/metro-config wrapper
│   ├── package.json
│   └── pnpm-lock.yaml             # pnpm lockfile
├── docker-compose.yml
└── .github/workflows/
    ├── backend.yml
    └── frontend.yml
```

### 3.2 Feature-Scoped Convention

Every backend feature domain is a package. Flat `routers/` or `services/`
directories are forbidden.

```
stations/               # CORRECT: feature-scoped
├── models.py           # SQLAlchemy ORM
├── schemas.py          # Pydantic request/response
├── repository.py       # Raw DB queries only
├── service.py          # Business logic only
└── router.py           # HTTP routes only
```

---

## 4. Technology Stack

| Tier | Technology | Version | Purpose |
|------|-----------|---------|---------|
| Backend Runtime | Python | 3.12+ | Application server |
| Backend Framework | FastAPI | latest | HTTP API |
| ORM | SQLAlchemy | 2.x (async) | Database access |
| Migrations | Alembic | latest | Schema evolution |
| Spatial Engine | PostgreSQL + PostGIS | 16+ | Spatial persistence |
| Web Frontend | React + Vite | 18 / 5 | Admin portal |
| Web Map | Leaflet + react-leaflet | latest | Map visualization |
| Mobile Runtime | Expo Go (Managed) | SDK 51+ | Driver app |
| Mobile Map | react-native-maps | 1.14+ | Native map views |
| Server State | @tanstack/react-query | v5 | HTTP state management |
| Styling | Tailwind CSS + shadcn/ui | latest | Design system |
| Package Manager | pnpm | 9+ | Mobile dependency management |
| Linting | Ruff + Black | latest | Python code quality |
| Type Check | TypeScript (`strict: true`) | 5+ | Frontend type safety |
| Mobile Testing | Jest + jest-expo + RNTL | latest | Mobile unit tests |
| CI | GitHub Actions | — | Automated pipelines |

### Auth Phases

| Phase | Strategy | Details |
|-------|----------|---------|
| 1–2 | Mock JWT via `X-Mock-Token` header | base64-encoded JSON payload: `{sub, username, roles, iat, exp}` |
| 3–4 | Internal Identity | argon2 password hashing, database-bound user store |
| 5+ | Keycloak OIDC | OAuth2 Authorization Code + PKCE |

### Phase 3+ Additions

| Service | Technology | Purpose |
|---------|-----------|---------|
| Message Queue | RabbitMQ via aio-pika | Async task processing |
| Object Storage | MinIO (S3) via boto3 | File storage, presigned URLs |
| Cache | Redis 7 via redis-py | Session/query caching |

---

## 5. Local Development

### 5.1 Docker Compose

```yaml
services:
  db:
    image: postgis/postgis:16-3.4
    environment:
      POSTGRES_USER: bornemap
      POSTGRES_PASSWORD: bornemap
      POSTGRES_DB: bornemap
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data

  backend:
    build: ./backend
    ports:
      - "8000:8000"
    environment:
      DATABASE_URL: postgresql+asyncpg://bornemap:bornemap@db:5432/bornemap
    depends_on:
      - db
    volumes:
      - ./backend:/app
    command: uvicorn app.main:app --host 0.0.0.0 --port 8000 --reload

volumes:
  pgdata:
```

### 5.2 Backend Dockerfile

```dockerfile
FROM python:3.12-slim
RUN pip install \
  fastapi uvicorn[standard] \
  sqlalchemy[asyncio] asyncpg \
  geoalchemy2 shapely \
  alembic pydantic-settings
COPY app/ /app
WORKDIR /app
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000"]
```

---

## 6. Backend Implementation Patterns

### 6.1 Config & Database

```python
# app/core/config.py
from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    database_url: str
    secret_key: str = "dev-secret-change-in-prod"
    class Config:
        env_file = ".env"

settings = Settings()
```

```python
# app/core/database.py
from sqlalchemy.ext.asyncio import (
    create_async_engine, async_sessionmaker, AsyncSession
)
from sqlalchemy.orm import DeclarativeBase
from app.core.config import settings

engine = create_async_engine(settings.database_url, echo=False)
AsyncSessionLocal = async_sessionmaker(engine, expire_on_commit=False)

class Base(DeclarativeBase):
    pass

async def get_db() -> AsyncSession:
    async with AsyncSessionLocal() as session:
        yield session
```

### 6.2 CoordinatePoint Schema

Every spatial API payload uses this schema to enforce `[longitude, latitude]`:

```python
from pydantic import BaseModel, Field, model_validator
from typing import Literal

class CoordinatePoint(BaseModel):
    coordinates: list[float] = Field(..., min_length=2, max_length=2)
    type: Literal["Point"] = "Point"

    @model_validator(mode="after")
    def validate_bounds(self):
        lon, lat = self.coordinates
        if not (-180 <= lon <= 180):
            raise ValueError("Longitude out of range")
        if not (-90 <= lat <= 90):
            raise ValueError("Latitude out of range")
        return self
```

### 6.3 Mock Auth (Phase 1–2)

```python
# X-Mock-Token header: base64({"sub":"1","username":"admin","roles":["admin"],"iat":0,"exp":9999999999})
def get_current_user(x_mock_token: Annotated[str | None, Header()] = None):
    if not x_mock_token:
        raise HTTPException(status_code=401, detail="Missing token")
    try:
        payload = json.loads(base64.b64decode(x_mock_token))
        required = {"sub", "username", "roles", "iat", "exp"}
        if not required.issubset(payload.keys()):
            raise ValueError
        return payload
    except Exception:
        raise HTTPException(status_code=401, detail="Invalid token")
```

### 6.4 Spatial Query Pattern

```python
from geoalchemy2.functions import ST_DWithin, ST_AsGeoJSON, ST_MakePoint

# ST_DWithin is the ONLY spatial filter — uses GiST index
query = (
    select(Station, ST_AsGeoJSON(Station.location).label("location_geojson"))
    .where(ST_DWithin(
        Station.location,
        f"SRID=4326;POINT({lon} {lat})",
        radius_m
    ))
    .where(Station.is_active == True)
)
```

---

## 7. Spatial Data Pipeline

### 7.1 Coordinate Model

```
Wire format:    [longitude, latitude]    (JSON array)
DB storage:     GEOGRAPHY(Point, 4326)   (PostGIS)
WKT format:     POINT(lon lat)           (no comma)
Leaflet:        [lat, lng]               (Leaflet API convention — convert at boundary)
react-native-maps:  {latitude, longitude}  (library requires lat first)
```

### 7.2 Viewport Quantization

Bounding box coordinates are quantized to **4 decimal places** (~11 m precision)
before use as cache keys or repository parameters.

### 7.3 Indexing

Every spatial column requires an explicit GiST index:

```sql
CREATE INDEX idx_stations_location ON stations USING GIST (location);
CREATE INDEX idx_chargers_location  ON chargers  USING GIST (location);
```

---

## 8. Web Admin Portal

### 8.1 Design Tokens

```typescript
// tailwind.config.ts — never hardcode hex in component files
export default {
  theme: {
    extend: {
      colors: {
        accent: {
          DEFAULT: "#22c55e",   // green-500  (electric green)
          light: "#4ade80",     // green-400
          dark: "#16a34a",      // green-600
          muted: "#dcfce7",     // green-100
        },
        surface: {
          DEFAULT: "#ffffff",
          overlay: "rgba(255,255,255,0.92)",
          card: "#f9fafb",
        },
      },
      borderRadius: {
        xl: "0.75rem",
        "2xl": "1rem",
      },
      boxShadow: {
        float: "0 4px 24px rgba(0,0,0,0.10)",
        card: "0 2px 12px rgba(0,0,0,0.07)",
      },
    },
  },
}
```

### 8.2 Floating Panel Layout

The map is always full-screen (`h-screen w-full`). All panels float on top:

```
┌─────────────────────────────────────────────────────┐
│  MAP (full screen, z-index: 0)                       │
│                                                       │
│  ┌─────────────────────┐    ┌──────────────────┐     │
│  │ Search + Filters    │    │ Station Form     │     │
│  │ top-left float      │    │ top-right float   │     │
│  │ z-10, backdrop-blur │    │ z-10              │     │
│  └─────────────────────┘    └──────────────────┘     │
│                                                       │
│       [green marker pins on map]                      │
│                                                       │
│  ┌─────────────────────────────────────────────┐     │
│  │ Station list card   bottom-left float        │     │
│  │ max-h-80, overflow-y-auto, z-10              │     │
│  └─────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────┘
```

```typescript
const floatingPanel = `
  absolute z-10 bg-white/90 backdrop-blur-md
  rounded-2xl shadow-float border border-white/60
  p-4
`
```

### 8.3 Custom Green Marker

```typescript
// Applies to web (Leaflet) — never default blue pin
const stationIcon = L.divIcon({
  className: "",
  html: `<div style="
    width:32px; height:32px; border-radius:50%;
    background:#22c55e; border:3px solid white;
    box-shadow:0 2px 8px rgba(0,0,0,0.25);
    display:flex; align-items:center; justify-content:center;
  ">...</div>`,
  iconSize: [32, 32],
  iconAnchor: [16, 16],
})
```

---

## 9. Mobile App

### 9.1 Map Screen

```typescript
// Full-bleed map. Never in a ScrollView. Never fixed pixel height.
<MapView style={StyleSheet.absoluteFillObject} />
<BottomSheet
  snapPoints={["12%", "45%", "90%"]}
  index={0}
  backgroundStyle={{ borderRadius: 24 }}
>
  <BottomSheetScrollView>
    {selectedStation ? <StationDetail /> : <StationList />}
  </BottomSheetScrollView>
</BottomSheet>
```

### 9.2 Snap Sheet Behavior

| State | Index | Height | Trigger |
|-------|-------|--------|---------|
| Peek | 0 | 12% | Default — shows station count + handle |
| Half | 1 | 45% | Tap a marker — shows station card |
| Full | 2 | 90% | Tap "View Details" — full info + reviews |
| Collapse | 0 | 12% | Tap map (not a marker) |

Use `expo-haptics` on every snap transition.

### 9.3 Custom Marker (Mobile)

```typescript
// Never default red pin
<Marker
  coordinate={{ latitude: station.lat, longitude: station.lon }}
  onPress={() => {
    setSelectedStation(station)
    bottomSheetRef.current?.snapToIndex(1)
    Haptics.impactAsync(Haptics.ImpactFeedbackStyle.Light)
  }}
>
  <View style={{
    width: 36, height: 36, borderRadius: 18,
    backgroundColor: "#22c55e",
    borderWidth: 3, borderColor: "white",
    shadowColor: "#000", shadowOffset: {width:0,height:2},
    shadowOpacity: 0.25, shadowRadius: 4,
  }} />
</Marker>
```

### 9.4 Testing

Mobile tests use **Jest** with the `jest-expo` preset and `@testing-library/react-native`. The
`transformIgnorePatterns` accounts for pnpm's `.pnpm/` virtual store layout:

```typescript
// jest.config.js — pnpm-compatible transform patterns
module.exports = {
  preset: "jest-expo",
  transformIgnorePatterns: [
    "node_modules/(?!.pnpm/|((jest-)?react-native|@react-native(-community)?|@react-native/js-polyfills|expo(nent)?|@expo(nent)?/.*|@expo-google-fonts/.*|expo-modules-core|react-navigation|@react-navigation/.*|@unimodules/.*|unimodules|sentry-expo|native-base|react-native-svg)/)",
  ],
};
```

Run tests with:

```bash
pnpm test
pnpm test -- --watch  # watch mode
```

### 9.5 Local Favorites (AsyncStorage)

```typescript
// Key: "bornemap_favorites"
// Value: JSON.stringify(StationResponse[])
// Deduplicate by id on every write
export const toggleFavorite = async (station) => {
  const favorites = await getFavorites()
  const exists = favorites.some(f => f.id === station.id)
  const updated = exists
    ? favorites.filter(f => f.id !== station.id)
    : [...favorites, station]
  await AsyncStorage.setItem("bornemap_favorites", JSON.stringify(updated))
  return !exists  // returns new isFavorite state
}
```

---

## 10. API Client (Shared Pattern)

```typescript
// lib/api.ts — same pattern for both web and mobile
const BASE_URL =
  process.env.EXPO_PUBLIC_API_URL ?? "http://localhost:8000/api/v1"

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE_URL}${path}`, {
    headers: { "Content-Type": "application/json", ...options?.headers },
    ...options,
  })
  if (!res.ok) {
    const error = await res.json().catch(() => ({}))
    throw new Error(error.detail ?? `HTTP ${res.status}`)
  }
  return res.json()
}

export const api = {
  stations: {
    list: () => request<StationResponse[]>("/stations"),
    get: (id: number) => request<StationResponse>(`/stations/${id}`),
    nearby: (params: NearbyQuery) =>
      request<StationResponse[]>(`/stations/nearby?${new URLSearchParams(params)}`),
    create: (data: StationCreate, token: string) =>
      request<StationResponse>("/stations", {
        method: "POST",
        body: JSON.stringify(data),
        headers: { "X-Mock-Token": token },
      }),
    update: (id: number, data: Partial<StationCreate>, token: string) =>
      request<StationResponse>(`/stations/${id}`, {
        method: "PATCH",
        body: JSON.stringify(data),
        headers: { "X-Mock-Token": token },
      }),
    delete: (id: number, token: string) =>
      request<void>(`/stations/${id}`, {
        method: "DELETE",
        headers: { "X-Mock-Token": token },
      }),
  },
}
```

---

## 11. Security Architecture

### 11.1 Mock Auth Flow (Phase 1–2)

```
Client                          FastAPI
  │                                │
  │── GET /stations ──────────────→│
  │   X-Mock-Token: base64({...})  │→ decode + validate claims
  │←─ 200 { stations } ───────────│
```

### 11.2 Data Protection

- All inter-zone traffic uses internal DNS; mTLS where supported.
- Passwords hashed with argon2 (phases 3–4).
- Presigned URLs for object storage access (no public buckets).
- API responses exclude internal IDs / debugging information in production.

---

## 12. Performance Constraints

| Metric | Target |
|--------|--------|
| `/api/v1/stations` viewport queries (p95) | ≤ 200 ms server-side |
| `/health/ready` (p99) | ≤ 50 ms |
| Map interaction (pan/zoom) | 60 FPS on reference device |
| Production uptime | ≥ 99.5% |
| Degraded operations threshold | > 1500 ms avg latency over 15 min |

---

## 13. CI/CD Pipeline

### 13.1 Backend Pipeline

```yaml
# .github/workflows/backend.yml
jobs:
  check:
    services:
      postgres:
        image: postgis/postgis:16-3.4
        env:
          POSTGRES_USER: bornemap
          POSTGRES_PASSWORD: bornemap
          POSTGRES_DB: bornemap
        ports: ["5432:5432"]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with: {python-version: "3.12"}
      - run: pip install ruff black pytest httpx
      - run: ruff check .
      - run: black --check .
      - run: pytest tests/ -v
        env:
          DATABASE_URL: postgresql+asyncpg://bornemap:bornemap@localhost:5432/bornemap
```

### 13.2 Frontend Pipeline

```yaml
# .github/workflows/frontend.yml
jobs:
  web:
    runs-on: ubuntu-latest
    timeout-minutes: 10
    defaults:
      run:
        working-directory: bornemap/frontend/web
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: "20"
          cache: "npm"
          cache-dependency-path: bornemap/frontend/web/package-lock.json
      - run: npm ci
      - run: npx tsc --noEmit
      - run: npm run build

  mobile:
    runs-on: ubuntu-latest
    timeout-minutes: 10
    defaults:
      run:
        working-directory: bornemap/frontend/mobile
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: "20"
          cache: "pnpm"
          cache-dependency-path: bornemap/frontend/mobile/pnpm-lock.yaml
      - run: pnpm install --frozen-lockfile
      - run: pnpm typecheck
      - run: pnpm test
```


### 13.3 Contract Checks

1. OpenAPI diff against previous `main` snapshot — breaking changes fail CI.
2. Generated client type-check against current API schema.

---

## 14. Key Architectural Decisions

### ADR-001: Python over Rust (Provisional)

- **Context:** Constitution §5 (Rust Escape Hatch).
- **Decision:** Python 3.12 + FastAPI for validation phase.
- **Rationale:** Maximum velocity, contract agility, compile-free hot reload.
- **When to revisit:** Empirical profiling shows Python bottleneck unoptimizable via
  query refinement or caching.

### ADR-002: GEOGRAPHY over GEOMETRY

- **Context:** Constitution §8.
- **Decision:** Always `GEOGRAPHY(Point, 4326)` for spatial columns.
- **Rationale:** Automatic great-circle calculations, correct distance measurements
  on spheroidal Earth, consistent indexing.
- **Migration:** Not possible without full data re-index — permanent choice.

### ADR-003: Monorepo + Feature-Scoped Layout

- **Decision:** Single repository; code grouped by feature domain, not technical type.
- **Rationale:** Fast iteration, atomic changes across tiers, simplified CI.
- **Structure:** `backend/app/stations/` (models + schemas + repo + service + router).

### ADR-004: React Query for Server State

- **Decision:** `@tanstack/react-query` handles all server state in frontends.
- **Rationale:** Eliminates hand-written fetch/loading/error logic, enables
  automatic cache invalidation, deduplication, and background refetching.
- **Rule:** Presentation components never call `fetch` or `axios` directly.

---

## 15. API Contract Standards

- **Format:** OpenAPI 3.1 generated from Pydantic models via FastAPI introspection.
- **Breaking Changes:** CI enforces diff against previous `main` snapshot.
- **Client Generation:** `openapi-typescript-codegen` from `openapi.json`.
- **Coordinate Convention:** All endpoint payloads use `[lng, lat]` arrays via
  `CoordinatePoint` schema (validated by `@model_validator`).
- **Error Format:** RFC 7807 `application/problem+json`.

```json
{
  "type": "https://api.bornemap.tn/errors/station-not-found",
  "title": "Station Not Found",
  "status": 404,
  "detail": "No station with id '1' exists.",
  "instance": "/api/v1/stations/1"
}
```
