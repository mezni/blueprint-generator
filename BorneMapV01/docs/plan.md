# BorneMap — Implementation Plan

**Version:** 1.0.0
**Model:** Phase-Isolated Validation Routing

## Execution Flow

| Phase | Objective |
|-------|-----------|
| Phase 0 | Foundation & Pipeline — local dev loop, CI, placeholder mobile screen |
| Phase 1 | Admin Spatial Core — station CRUD, map UI, mock auth, spatial queries |
| Phase 2 | Mobile Feature Expansion — full map discovery, snap sheets, local favorites |
| Phase 3 | User Interaction Layer — accounts, backend favorites, reviews |
| Phase 4 | Product Hardening — caching, contract tests, payload optimization |
| Phase 5 | Architecture Decision Point — Python retain or Rust migration |

---

## Phase 0 — Foundation & Infrastructure Loop

**Objective:** Establish the rapid validation ecosystem, local development loops, and
strict automated testing guardrails.

### Deliverables

- **Backend App Core** — FastAPI boilerplate, Pydantic schemas for request validation,
  dynamic internal global routing structures.
- **Database Backbone** — Local PostgreSQL + PostGIS container cluster, spatial backend
  extensions, initial Alembic migration history base.
- **Web Admin Portal** — React + Vite + Tailwind CSS + shadcn/ui scaffold with CartoDB
  Positron map tile integration.
- **Mobile Client** — Expo Go managed framework initialised with a simple responsive
  placeholder screen.
- **CI Pipelines** — GitHub Actions enforcing Ruff/Black lint, TypeScript `tsc` check,
  build verification, and mobile unit tests (Jest) on every PR.

### Success Criteria

> Local developer loop is fully running, mobile app serves the placeholder safely,
> all PR pipelines green.

---

## Phase 1 — Admin Spatial Core Validation

**Objective:** Validate administrative data input, spatial-first management UI/UX,
map interaction pipelines, and nearby geospatial query lookups.

### Deliverables

- **Station Repository CRUD** — ORM models for physical EV layout records connected
  through the service boundary layer using Alembic migrations.
- **Admin Map Dashboard** — Floating panel layout over full-bleed CartoDB Positron map.
  Custom green `DivIcon` markers. No default Leaflet blue pins.
- **Spatial Queries Engine** — FastAPI routes executing `ST_DWithin` over PostGIS.
  Coordinates rounded to 4 decimal places. GiST index mandatory.
- **Mock Auth** — Route dependencies checking token format to switch context between
  public drivers and system admins.

### Success Criteria

> Admin coordinates saved on the portal appear instantly on the map within localized
> bounding boxes. UX fully vetted for visual polish.

### Data Model (Phase 0 — User Management + Data Tables)

```
users
├── id SERIAL PK
├── username VARCHAR(100) UNIQUE NOT NULL
├── email VARCHAR(255) UNIQUE NOT NULL
├── role VARCHAR(50) NOT NULL DEFAULT 'viewer'
├── password_hash VARCHAR(255)
├── is_active BOOLEAN NOT NULL DEFAULT true
├── created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
└── updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

partners
├── id SERIAL PK
├── name VARCHAR(255) NOT NULL
├── contact_email VARCHAR(255)
├── phone VARCHAR(50)
├── is_active BOOLEAN NOT NULL DEFAULT true
├── created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
└── updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

stations
├── id SERIAL PK
├── partner_id INTEGER FK → partners (nullable)
├── name VARCHAR(255) NOT NULL
├── operator VARCHAR(255)
├── address VARCHAR(500)
├── location GEOGRAPHY(Point, 4326)  ← GiST index
├── plug_types TEXT[]
├── speed_kw FLOAT
├── is_active BOOLEAN NOT NULL DEFAULT true
├── created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
└── updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

chargers
├── id SERIAL PK
├── station_id INTEGER FK → stations (nullable)
├── connector VARCHAR(50) NOT NULL (CCS, Type2, CHAdeMO)
├── power_kw NUMERIC
├── status VARCHAR(50) NOT NULL DEFAULT 'available'
├── is_active BOOLEAN NOT NULL DEFAULT true
├── created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
└── updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
```

### Data Model (Phase 1)

```
stations
├── id SERIAL PK
├── name VARCHAR(255)
├── operator VARCHAR(255)
├── address VARCHAR(500)
├── location GEOGRAPHY(Point, 4326)  ← GiST index
├── plug_types TEXT[]
├── speed_kw FLOAT
├── is_active BOOLEAN
└── timestamps

chargers
├── id SERIAL PK
├── station_id INTEGER FK → stations
├── connector TEXT (CCS, Type2, CHAdeMO)
├── power_kw NUMERIC
└── timestamps
```

---

## Phase 2 — Mobile Feature Expansion & Client Validation

**Objective:** Strip out the mobile placeholder and expand the driver experience with
real-world geospatial exploration tools.

> **Gate:** Phase 2 starts only after the Phase 1 backend API is stable (stations CRUD
> + spatial queries + mock auth). Phase 1 admin UI may still be in progress.

### Deliverables

- **Full-Bleed Map** — `react-native-maps` integrated inside Expo Go.
  `StyleSheet.absoluteFillObject` — never fixed pixel height, never inside `ScrollView`.
- **Snap Sheet** — `@gorhom/bottom-sheet` with snap points `[12%, 45%, 90%]`. Haptic
  feedback on every snap transition via `expo-haptics`.
- **Station Markers** — Custom green circular markers. Never default red pins. Tap →
  snap sheet to 45%, select station, haptic.
- **Client Filtering** — Input controls to narrow discovery markers by speed type and
  plug format (CCS2, CHAdeMO, Type2, GBT).
- **Local Favorites** — `AsyncStorage` with deduplication by station ID.
  `toggleFavorite` returns new `isFavorite` state.

### Success Criteria

> Active drivers successfully operate discovery functions on real testing devices
> using the customized UI/UX.

---

## Phase 3 — User Interaction Layer

**Objective:** Transition local client storage models into a unified backend user
account model to track user retention.

### Deliverables

- **Account Core Storage** — Concrete user tables replacing mock data. `password_hash`
  field. `UNIQUE` constraints on username and email.
- **Backend Favorites Engine** — `favorites` table with FK to users and stations.
  `UNIQUE(user_id, station_id)`. Sync across separate logins.
- **Review & Feedback Core** — `reviews` table. `rating SMALLINT CHECK (1–5)`.
  `UNIQUE(user_id, station_id)` — one review per user per station.

### Data Model (Phase 3)

```
users
├── id SERIAL PK
├── username VARCHAR(100) UNIQUE
├── email VARCHAR(255) UNIQUE
├── password_hash VARCHAR(255)
└── created_at

favorites
├── id SERIAL PK
├── user_id INTEGER FK → users ON DELETE CASCADE
├── station_id INTEGER FK → stations ON DELETE CASCADE
├── UNIQUE(user_id, station_id)
└── created_at

reviews
├── id SERIAL PK
├── user_id INTEGER FK → users ON DELETE CASCADE
├── station_id INTEGER FK → stations ON DELETE CASCADE
├── rating SMALLINT CHECK (1–5)
├── comment TEXT
├── UNIQUE(user_id, station_id)
└── created_at
```

### Success Criteria

> User preferences, reviews, and favorites persist smoothly across separate device
> sessions.

---

## Phase 4 — Product Hardening

**Objective:** Stabilize and bulletproof the validated feature set for production-grade
operational status.

### Deliverables

- **Spatial Caching Layer** — Two-stage approach:
  1. **Phase 4a:** In-process Python memory cache for frequently read, quantized viewport
     queries (simple dict/TTL).
  2. **Phase 4b+:** Migrate to Redis when cross-process sharing is needed or in-memory
     eviction pressure exceeds tuning limits.
- **Contract Verification Testing** — End-to-end endpoint tests validating database
  transaction behaviors and strict schema compliance.
- **Payload Optimization** — Strip responses to spatial detail payloads, minimizing
  transfer latency to mobile clients.

### Success Criteria

> API error rates fall below 0.1% under stress validation testing.

---

## Phase 5 — Architecture Decision Point

**Objective:** Evaluate real-world platform utilization data to finalize backend
technology selections.

### Decision Paths

| Path | Trigger & Action |
|------|------------------|
| **Path A — Retain Python** | Performance benchmarks satisfy the 200 ms spatial lookup target under standard user traffic. Maintain FastAPI codebase. |
| **Path B — Rust Migration** | Concurrency limits, high-density PostGIS parsing latency, or thread constraints cannot be resolved via memory tuning. Re-implement handlers in Rust while preserving DB schemas, frontend code, and OpenAPI specs. |

---

## Real-User Validation Framework

Every phase runs this validation loop before proceeding to the next.

### Cohort Requirements

| MVP | Min Drivers | Min Admins |
|-----|-------------|------------|
| Phase 0 | — | — |
| Phase 1 | — | 2 |
| Phase 2 | 5 | — |
| Phase 3 | 5 | 1 |
| Phase 4 | 3 | 1 |
| Phase 5 | — | — |

### Session Structure (60 min)

1. **5 min** — Onboarding, consent, device check.
2. **40 min** — Scripted tasks (phase-specific list in validation script).
3. **10 min** — Semi-structured interview.
4. **5 min** — SUS (System Usability Scale) questionnaire.

### Metrics Collected

| Type | Examples |
|------|----------|
| **Quantitative** | Task success rate, time-on-task, error count, crash count |
| **Qualitative** | Pain points, surprise moments, confusion log |
| **System** | Server P95 latency, error rate, cache hit rate (Phase 4+) |

### Proceed Gate

Each phase produces `validation.md` (report) and `decision.md` (proceed/adjust/kill)
before the next phase begins. All success criteria for the phase must be met or a
remediation path documented.

---

## Immediate Execution Backlog — Weeks 1 & 2

### Week 1 — Foundation & Pipeline Integration

- Deploy local FastAPI workspace with PostGIS containers via Docker Compose.
- Configure strict Ruff, Black, TypeScript (tsc), and Jest verification workflows
  inside active GitHub Actions.
- Spin up React/Vite admin layout and Expo Go mobile repository (pnpm-managed)
  with placeholder validation message screen.

### Week 2 — Spatial Layout & Admin API

- Draft Alembic script for `GEOGRAPHY(Point, 4326)` database configuration with GiST
  index.
- Set up FastAPI routers with required dependency direction layers
  (API → Service → Repository).
- Build core backend Station CRUD endpoints and model structures.
- Implement CartoDB Positron tile layer in admin portal. Configure custom green
  `DivIcon` markers.
- Build backend User CRUD endpoints (`users/` feature domain) with is_active status
  toggle, mock auth integration.
- Create Alembic migration for `users` table (id, username, email, role, password_hash,
  is_active, timestamps).
- Implement web admin Users page with data table, add/edit dialog, activate/deactivate
  toggle, and React Query hooks.
- Update Dashboard overview to show summary stats with inline Settings link.
- Complete backend `stations/` feature domain with full models (name, operator, address,
  location GEOGRAPHY(Point,4326), plug_types, speed_kw, partner_id FK, is_active).
- Create backend `partners/` feature domain (id, name, contact_email, phone, is_active).
- Create backend `chargers/` feature domain (id, station_id FK, connector, power_kw,
  status, is_active).
- Create Alembic migrations for partners, stations, and chargers tables.
- Implement web admin Data sub-pages (Partners, Stations, Chargers) with data tables,
  add/edit dialogs, active/inactive toggle, and React Query hooks.
- Update sidebar Data accordion with clickable sub-item navigation.

---

## Global Non-Negotiables (All Phases)

These rules apply to every file, every PR, every phase.

1. Architecture flows API → Service → Repository → DB. No layer-skipping.
2. Spatial column type is always `GEOGRAPHY(Point, 4326)`. Never `GEOMETRY`.
3. Coordinates are always `[longitude, latitude]` in all API payloads.
4. `ST_DWithin` is the only spatial `WHERE` filter. `ST_Distance` is `ORDER BY` only.
5. Map is always full-screen. Never in a `ScrollView`. Never fixed pixel height.
6. Default Leaflet blue pin and RN Maps red pin are banned. Green custom markers only.
7. `expo eject` is banned. App stays in Expo Go managed runtime throughout validation.
8. No hardcoded hex colors in component files. Always reference design tokens.
9. All schema changes go through Alembic. Never manual DB edits.
10. Every PR must pass CI before merge. No exceptions.
11. Source code grouped by feature domain, not technical type. Flat `routers/`,
    `services/`, or `repositories/` directories are forbidden — use `stations/router.py`,
    `stations/service.py` instead.

---

> BorneMap optimizes for validated learning, rapid client feedback, and execution
> speed over implementation sophistication.
