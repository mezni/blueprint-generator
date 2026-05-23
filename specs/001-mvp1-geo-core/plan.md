# Implementation Plan: MVP 1 — Geo Core (Read-Only Map)

**Branch**: `001-mvp1-geo-core` | **Date**: 2026-05-23 | **Spec**: [`spec.md`](./spec.md)

**Input**: Feature specification from `specs/001-mvp1-geo-core/spec.md`

> Companion docs (constitution and global context): [`.specify/memory/constitution.md`](../../.specify/memory/constitution.md), [`docs/architecture.md`](../../docs/architecture.md), [`docs/plan.md`](../../docs/plan.md).

## Summary

Deliver a read-only, viewport-driven map of EV charging stations across Tunisia on web and mobile, backed by a PostGIS-powered Actix Web service. Provide an admin portal with full station CRUD gated by a mock JWT issued from an env-driven allowlist. No driver accounts, no reviews, no async workers, no Redis, no Keycloak in this slice.

**Technical approach** (derived from spec, constitution, and Phase 0 research):

- Backend: single Rust binary `bornemap-backend` built from a Cargo workspace. Actix Web v4 handlers expose `GET /api/v1/stations`, `GET /api/v1/stations/{id}`, admin CRUD, and `POST /api/v1/auth/mock-login`. SQLx (compile-time verified) talks to PostgreSQL 16 + PostGIS 3.4. OpenAPI emitted by `utoipa`.
- Frontend: React 18 + Vite admin portal using `react-leaflet` + `react-leaflet-cluster`, and a React Native + Expo (SDK 50, managed) mobile app using `react-native-maps`. Both consume a generated `@bornemap/api-client`. Shared `@bornemap/geo-models` package provides `CoordinateModel`, `MapViewportModel`, and `quantizeBounds()`.
- Data: `GEOGRAPHY(Point, 4326)` with GiST index; `ST_DWithin` viewport queries; 4-decimal bbox quantization at the edge. Soft delete via `deleted_at`. Connector enum constrained by `CHECK`. Opening hours as OSM `opening_hours` strings validated server-side.
- Testing: TDD with Testcontainers PostGIS for integration, contract tests against generated OpenAPI types, Playwright (web) and Detox (mobile) for the two highest-value user flows.
- Local loop: `infrastructure/docker-compose.local.yml` runs PostGIS only; backend and frontends run on host. Seeded with ≥ 500 synthetic stations.

## Technical Context

**Language/Version**:

- Backend: Rust stable (MSRV pinned via `rust-toolchain.toml`, target 1.78+).
- Frontend Web: TypeScript 5.4 with `strict: true`.
- Frontend Mobile: TypeScript 5.4 + React Native via Expo SDK 50.

**Primary Dependencies**:

- Backend: `actix-web` 4, `sqlx` 0.7 (postgres, runtime-tokio-rustls, macros, uuid, chrono, json), `utoipa` 4 + `utoipa-swagger-ui`, `serde` / `serde_json`, `thiserror`, `tracing` + `tracing-subscriber` (json), `prometheus`, `jsonwebtoken` 9 (HS256 for mock), `uuid`, `chrono`, `tokio` 1, `tower`-free (Actix native). Opening-hours validation: `opening_hours` crate.
- Frontend Web: `react` 18, `react-dom` 18, `vite` 5, `@tanstack/react-query` 5, `leaflet` 1.9, `react-leaflet` 4, `react-leaflet-cluster` 2, `tailwindcss` 3, `shadcn/ui`, `react-hook-form` 7, `zod` 3, `opening_hours` (JS).
- Frontend Mobile: `expo` SDK 50, `react-native` 0.73, `react-native-maps` 1.10, `@tanstack/react-query` 5, `@gorhom/bottom-sheet` 4.
- Shared: `openapi-typescript-codegen` for client generation.
- Test: `cargo test` + `testcontainers` 0.15 (PostGIS image), `vitest` (web), `jest` + `@testing-library/react-native` (mobile), `playwright` 1.43 (web E2E), `detox` 20 (mobile E2E, deferred to staging job).

**Storage**:

- PostgreSQL 16 + PostGIS 3.4 — single database `bornemap_dev` (local) / `bornemap_prod` (deployed).
- No Redis, no MinIO, no object storage in MVP 1.

**Testing**:

- Unit: `cargo test --workspace` for Rust; `vitest` for web; `jest` for mobile.
- Integration: Rust tests boot PostGIS via Testcontainers.
- Contract: tests assert handler responses against the generated OpenAPI types.
- E2E: Playwright runs against staging deploy for US 1 (driver discovers nearest charger) and US 2 (admin creates a station).

**Target Platform**:

- Backend: Linux x86_64 server (Docker container, distroless base).
- Web: last 2 stable versions of Chrome, Firefox, Safari, Edge.
- Mobile: iOS 15+ (perf reference iPhone 11), Android 10+ / API 29+ (perf reference Samsung Galaxy A33 5G). Below the floor, the app shows a non-dismissible "Update your OS" screen.

**Project Type**: Web service + admin web app + mobile app. Three deployable units sharing two TypeScript packages.

**Performance Goals**:

- P95 server latency for `GET /api/v1/stations` ≤ **200 ms** with 500 stations seeded under 50 concurrent viewport queries/s.
- Map sustains ≥ **60 FPS** during sustained pan/zoom on both reference devices.
- Cold-start time to first marker render ≤ **2.5 s** (web), ≤ **3.5 s** (mobile) on 4G.

**Constraints**:

- `[lng, lat]` array form everywhere; `{lat,lng}` forbidden.
- 4-decimal viewport quantization at the edge.
- `ST_DWithin` is the only indexed spatial predicate.
- No `unwrap`/`expect`/`panic!` in non-test Rust.
- No hand-rolled HTTP types on the frontend.
- No driver accounts, reviews, async workers, Redis, MinIO, or Keycloak (these arrive in later MVPs).
- `MOCK_ADMIN_USERNAMES` env var required for the backend to start; never logged.

**Scale/Scope**:

- ≥ 500 synthetic stations seeded for MVP 1; design must accommodate 50 k+ for MVP 4 load tests without schema change.
- ≤ 5 000 markers returned per viewport (hard limit).
- 6 backend endpoints (1 auth, 2 public reads, 3 admin writes).
- 1 web app (admin portal) and 1 mobile app (driver-facing read-only).
- 1 shared TS package for API client (generated) + 1 shared TS package for geo models.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

Gates derived from `.specify/memory/constitution.md`:

| # | Principle / Rule | Status | Notes |
|---|---|---|---|
| I | Spatial-First, PostGIS-Always | **PASS** | PostgreSQL 16 + PostGIS 3.4; `GEOGRAPHY(Point, 4326)`; GiST index on `stations.location`; `ST_DWithin` primary predicate; `[lng, lat]` on wire and in code; 4-decimal quantization codified in `quantizeBounds()` and FR-007. |
| II | Backend Authority, Frontend Projection | **PASS** | All filtering, validation, and authorization in backend (FR-006, FR-007, FR-017, FR-018, FR-019). Frontend `useViewportStations()` only orchestrates fetch + render. No client-side filter-after-fetch. |
| III | API-First Contract | **PASS** | `utoipa` annotations on every handler. `@bornemap/api-client` is generated; hand-written HTTP types forbidden (FR-014). CI runs OpenAPI breaking-diff check. |
| IV | Modular Monolith Until Justified | **PASS** | Single binary `bornemap-backend`. Domain modules `station`, `identity` (mock auth only) compiled as separate Rust modules behind public service traits passing DTOs. No microservice split. |
| V | Test-First with PostGIS in CI | **PASS** | Every endpoint ships with a contract test and an integration test against Testcontainers PostGIS. TDD order enforced in `tasks.md` (next step). |
| VI | SQLx Compile-Time Verified | **PASS** | All SQL via `sqlx::query!` / `query_as!`. `.sqlx/` committed. CI runs `SQLX_OFFLINE=true cargo build`. |
| VII | Type-Driven Safety (Rust) | **PASS** | Newtype IDs (`StationId`, `CompanyId`, `ChargerId`). `Result<T, DomainError>` everywhere. `unwrap`/`expect`/`panic!` banned in non-test code via `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]`. Errors emit RFC-7807. |
| VIII | Map Interaction Runtime Rules R1–R7 | **PASS** | R1 (300 ms debounce) in `useViewportStations`; R2 (4-dp quantization) in `quantizeBounds`; R3 (off-viewport clip) before render; R4 (`tracksViewChanges={false}`) on RN markers; R5 (lazy hydration, no fetch on filter — N/A here, no filter pills until MVP 2); R6 (`['stations', q]` cache keys); R7 (cluster 15/40 px) via `react-leaflet-cluster` config. |
| IX | Status Projection Rule | **PASS** | Pin color computed by `pinColor(marker)` from backend-supplied `is_active` and `under_maintenance` only. No telemetry source. |
| X | Idempotent Profile Initialization | **N/A in MVP 1** | No profile/user creation in this MVP. Rule activates from MVP 2 onward. |
| XI | Iterative Real-User Validation | **PASS** | Validation cohort defined in `docs/plan.md` §3 and refined in the spec's Clarifications (devices + OS floors). `validation.md` and `decision.md` produced at the MVP boundary. |
| XII | Non-Goals (NG-1..NG-7) | **PASS** | FR-016 explicitly forbids bundled routing; no OCPP/payments/grid/telemetry; not splitting services. |
| Tech Stack | Pinned versions (Actix 4, SQLx 0.7, PG 16, PostGIS 3.4, utoipa 4, React 18, RN via Expo 50) | **PASS** | All locked in Technical Context. |
| Perf | P95 ≤ 200 ms, 60 FPS, cold-start ≤ 2.5 s (web) / 3.5 s (mobile) | **PASS** (design-level) | k6 load profile committed in Phase 0; perf gate measured at validation. |
| Security | Three subnets + invitation-only admins | **PARTIAL → PASS** | Three subnets formalized in Phase 5+. MVP 1 deploys to staging only (single network) with mock auth and the `MOCK_ADMIN_USERNAMES` allowlist gating admin tokens. Documented exception below; no constitutional violation because the security zone topology applies to production. |

**Result**: All gates **PASS**. No Complexity Tracking entries required.

## Project Structure

### Documentation (this feature)

```text
specs/001-mvp1-geo-core/
├── spec.md                 # Feature specification (clarified)
├── plan.md                 # This file
├── research.md             # Phase 0 output
├── data-model.md           # Phase 1 output
├── contracts/              # Phase 1 output
│   ├── README.md
│   └── openapi.yaml        # Hand-authored contract; backend regenerates and CI diffs
├── quickstart.md           # Phase 1 output
└── checklists/
    └── requirements.md     # From /speckit.specify validation
```

### Source Code (repository root)

```text
bornemap/
├── backend/                         # Cargo workspace
│   ├── Cargo.toml                   # [workspace] members + shared deps
│   ├── Cargo.lock
│   ├── rust-toolchain.toml          # 1.78+
│   ├── .sqlx/                       # Offline query metadata (committed)
│   ├── libs/
│   │   ├── common-utils/            # Errors, newtypes, RFC-7807 mapping
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── ids.rs           # StationId, CompanyId, ChargerId, UserId
│   │   │       ├── error.rs         # DomainError + Actix ResponseError impl
│   │   │       └── time.rs
│   │   └── openapi-spec/            # Bin that prints openapi.json from utoipa
│   │       ├── Cargo.toml
│   │       └── src/main.rs
│   └── services/
│       └── station-service/         # MVP 1 hosts everything inside this crate
│           ├── Cargo.toml
│           ├── migrations/
│           │   ├── 20260523_0001_init_companies.sql
│           │   ├── 20260523_0002_init_stations.sql
│           │   ├── 20260523_0003_init_chargers.sql
│           │   └── 20260523_0004_seed_synthetic.sql
│           ├── src/
│           │   ├── main.rs          # binds, /health, /metrics, mounts modules
│           │   ├── config.rs        # MOCK_ADMIN_USERNAMES, BIND_ADDR, etc.
│           │   ├── auth/
│           │   │   ├── mod.rs
│           │   │   ├── handlers.rs  # POST /api/v1/auth/mock-login
│           │   │   ├── service.rs
│           │   │   ├── claims.rs    # Canonical JWT claim shape
│           │   │   └── middleware.rs # Bearer extraction, role check
│           │   ├── station/
│           │   │   ├── mod.rs
│           │   │   ├── handlers.rs  # public + admin routes
│           │   │   ├── service.rs
│           │   │   ├── repository.rs
│           │   │   ├── models.rs    # DTOs (utoipa::ToSchema)
│           │   │   └── filters.rs   # bbox parsing, quantization
│           │   ├── companies/       # Lightweight read-only support module
│           │   │   ├── mod.rs
│           │   │   └── repository.rs
│           │   ├── observability/
│           │   │   ├── mod.rs
│           │   │   ├── logging.rs
│           │   │   └── metrics.rs
│           │   └── openapi.rs       # utoipa::OpenApi root
│           └── tests/
│               ├── common/
│               │   └── mod.rs       # Testcontainers harness
│               ├── auth_mock_login.rs
│               ├── stations_public_read.rs
│               ├── stations_admin_crud.rs
│               └── health.rs
├── frontend/                        # pnpm workspaces
│   ├── pnpm-workspace.yaml
│   ├── packages/
│   │   ├── api-client/              # Generated (CI regenerates)
│   │   │   ├── package.json
│   │   │   ├── openapi.json         # Snapshot
│   │   │   └── src/                 # generated; checked in for traceability
│   │   └── geo-models/
│   │       ├── package.json
│   │       └── src/
│   │           ├── index.ts
│   │           ├── coordinate.ts    # CoordinateModel = readonly [lng, lat]
│   │           ├── viewport.ts      # MapViewportModel, quantizeBounds, insideViewport
│   │           └── marker.ts        # StationMarkerModel, pinColor
│   ├── admin-portal/                # React + Vite
│   │   ├── package.json
│   │   ├── vite.config.ts
│   │   ├── tailwind.config.ts
│   │   ├── tsconfig.json
│   │   ├── index.html
│   │   ├── playwright.config.ts
│   │   ├── src/
│   │   │   ├── main.tsx
│   │   │   ├── app/
│   │   │   │   ├── App.tsx
│   │   │   │   ├── router.tsx
│   │   │   │   └── queryClient.ts
│   │   │   ├── features/
│   │   │   │   ├── map/
│   │   │   │   │   ├── MapView.tsx
│   │   │   │   │   ├── useViewportStations.ts
│   │   │   │   │   ├── StationDetailPanel.tsx
│   │   │   │   │   └── NavigateButton.tsx       # FR-016 URL builder
│   │   │   │   ├── auth/
│   │   │   │   │   ├── LoginPage.tsx
│   │   │   │   │   └── authStore.ts
│   │   │   │   └── stations/                    # Admin CRUD
│   │   │   │       ├── StationsTablePage.tsx
│   │   │   │       ├── StationEditPage.tsx
│   │   │   │       └── stationFormSchema.ts     # zod schema mirroring FR-017/18
│   │   │   ├── styles/
│   │   │   │   ├── map-theme.constants.ts        # BorneMapMapStyles
│   │   │   │   └── globals.css
│   │   │   └── lib/
│   │   │       └── apiClient.ts                 # Re-exports @bornemap/api-client config
│   │   └── e2e/
│   │       ├── driver-discovers-charger.spec.ts
│   │       └── admin-creates-station.spec.ts
│   └── mobile-app/                  # React Native + Expo
│       ├── package.json
│       ├── app.config.ts
│       ├── tsconfig.json
│       ├── src/
│       │   ├── App.tsx
│       │   ├── screens/
│       │   │   ├── MapScreen.tsx
│       │   │   ├── StationDetailSheet.tsx
│       │   │   └── OsUpdateRequiredScreen.tsx   # FR-020
│       │   ├── components/
│       │   │   └── ClusteredMarkers.tsx
│       │   ├── hooks/
│       │   │   ├── useViewportStations.ts
│       │   │   └── useOsFloorCheck.ts
│       │   ├── styles/
│       │   │   └── map-theme.constants.ts
│       │   └── lib/
│       │       └── deepLink.ts                  # buildNavigateUrl(coord)
│       └── e2e/                                 # Detox (staging only)
│           └── driver-discovers-charger.e2e.ts
├── infrastructure/
│   ├── docker-compose.local.yml     # PostGIS only
│   ├── env/
│   │   ├── backend.env.example
│   │   └── frontend.env.example
│   └── seed/
│       └── stations.csv             # Source for synthetic seed migration
├── docs/                            # Existing global docs (constitution mirror, plan, architecture)
├── specs/                           # Existing specs/
└── .github/workflows/
    ├── ci.yml                       # fmt, clippy, sqlx offline, cargo test, pnpm lint/test
    └── openapi-diff.yml             # Breaking-diff gate
```

**Structure Decision**: Web application layout (Option 2 from the template) extended with a third deployable unit (mobile app) and two shared packages. The backend is a single Cargo workspace containing one binary crate (`station-service`) and two library crates (`common-utils`, `openapi-spec`). All MVP 1 domains live inside `station-service` to avoid premature splits; the `services/` directory is named for future extraction (MVP 6) without forcing it now.

## Complexity Tracking

> Fill ONLY if Constitution Check has violations that must be justified.

### CT-001: Degraded-Operations Rule Deferred

**Constitution Reference**: §Performance Constraints — "if average spatial query latency > 1500 ms over any rolling 15-minute window, the frontend MUST display a cached static map snapshot and disable live fetches until latency recovers."

**Justification**: MVP 1 deploys to staging only (single-network, controlled cohort). There is no production traffic that could trigger sustained 1500 ms latency. The degraded-operations UI requires: (1) a latency-monitoring middleware emitting rolling-window metrics, (2) a frontend polling or WebSocket channel to receive degraded-state signals, and (3) a static map snapshot cache — all of which add meaningful complexity with zero staging benefit. This rule activates from MVP 4 (Redis cache layer + production deploy) onward.

**Risk**: Low. Staging environment has ≤ 500 stations and ≤ 10 concurrent users; P95 target is 200 ms, making 1500 ms threshold unreachable under normal conditions.

No other violations. Section otherwise intentionally empty.

---

## Phase 2 Plan (Reference Only — Generated by `/speckit.tasks`)

The next command `/speckit.tasks` will produce `tasks.md` containing ordered, numbered tasks (T001, T002, ...). Anticipated task shape (do not act on this — the implementer LLM will run `/speckit.tasks`):

1. Scaffold Cargo workspace + frontend pnpm workspace.
2. Author migrations 0001..0004 (TDD: migration tests via Testcontainers).
3. Author `common-utils` crate (newtypes, errors, RFC-7807 mapping).
4. Author `auth/` module (mock JWT, allowlist) — contract tests first.
5. Author `station/` repository (`ST_DWithin` query) — integration tests first.
6. Author `station/` service + handlers (public + admin) — contract tests first.
7. Author `openapi-spec` binary; commit `frontend/packages/api-client/openapi.json` snapshot.
8. Generate `@bornemap/api-client` and `@bornemap/geo-models` package skeletons.
9. Author shared `quantizeBounds`, `insideViewport`, `pinColor` with unit tests.
10. Author admin portal: login, stations table, edit form, map view, navigate button.
11. Author mobile app: OS-floor gate, map screen, detail sheet, navigate handler.
12. Author CI workflows + OpenAPI breaking-diff job.
13. Seed migration with 500 synthetic stations covering Tunisia.
14. Write Playwright E2E for US 1 and US 2; Detox skeleton for staging.
15. Write `quickstart.md` smoke procedure (already authored by `/speckit.plan`).
