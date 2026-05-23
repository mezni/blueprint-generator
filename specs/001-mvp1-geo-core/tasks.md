# Tasks: MVP 1 — Geo Core (Read-Only Map)

**Input**: Design documents from `/specs/001-mvp1-geo-core/`

**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/openapi.yaml, quickstart.md

**Tests**: TDD is mandated by constitution Principle V ("Test-First with PostGIS in CI") and the spec explicitly requires contract tests + integration tests + E2E. Test tasks are included throughout.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story. US3 (Mock Auth) is elevated to the Foundational phase because US2 (Admin CRUD) cannot function without it.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Per `plan.md` Project Structure:

- **Backend**: `bornemap/backend/` — Cargo workspace with `libs/common-utils/`, `libs/openapi-spec/`, `services/station-service/`
- **Frontend Web**: `bornemap/frontend/admin-portal/` — React + Vite
- **Frontend Mobile**: `bornemap/frontend/mobile-app/` — React Native + Expo
- **Shared Packages**: `bornemap/frontend/packages/api-client/`, `bornemap/frontend/packages/geo-models/`
- **Infrastructure**: `bornemap/infrastructure/`
- **CI**: `bornemap/.github/workflows/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization — Cargo workspace, pnpm workspace, Docker Compose, env files.

- [x] T001 Create top-level repo directory `bornemap/` with root `README.md` and `.gitignore` (Rust `target/`, Node `node_modules/`, `.env`, `.sqlx/`)
- [x] T002 [P] Initialize Cargo workspace in `bornemap/backend/Cargo.toml` with members `libs/common-utils`, `libs/openapi-spec`, `services/station-service`; add `rust-toolchain.toml` pinning 1.78+
- [x] T003 [P] Initialize pnpm workspace in `bornemap/frontend/pnpm-workspace.yaml` with packages `packages/*`, `admin-portal`, `mobile-app`; add root `package.json` with scripts `lint`, `typecheck`, `test`
- [x] T004 [P] Create `bornemap/infrastructure/docker-compose.local.yml` running `postgis/postgis:16-3.4` on port 5432 with credentials `bornemap/bornemap`, database `bornemap_dev`
- [x] T005 [P] Create `bornemap/infrastructure/env/backend.env.example` with `MOCK_ADMIN_USERNAMES=alice,bob`, `MOCK_JWT_SECRET=dev-secret-change-me`, `BIND_ADDR=0.0.0.0:8000`, `DATABASE_URL=postgres://bornemap:bornemap@localhost:5432/bornemap_dev`, `BORNEMAP_HIDE_TEST_ROWS=false`
- [x] T006 [P] Create `bornemap/infrastructure/env/frontend.env.example` with `VITE_API_BASE_URL=http://localhost:8000`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented — migrations, common-utils, auth, OpenAPI spec bin, shared TS packages.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Database Migrations

- [x] T007 Create migration `bornemap/backend/services/station-service/migrations/20260523_0001_init_companies.sql` per data-model.md §3.1: schema `station_domain`, table `companies` with PK, `name` CHECK, `is_test`, audit fields, `deleted_at`
- [x] T008 Create migration `bornemap/backend/services/station-service/migrations/20260523_0002_init_stations.sql` per data-model.md §3.2: table `stations` with FK to `companies`, `GEOGRAPHY(Point, 4326)` location, GiST index `stations_location_gix`, `stations_company_idx`, `stations_active_partial_idx`, `opening_hours_osm` TEXT, `is_test`, audit fields, `deleted_at`
- [x] T009 Create migration `bornemap/backend/services/station-service/migrations/20260523_0003_init_chargers.sql` per data-model.md §3.3: table `chargers` with FK to `stations` ON DELETE CASCADE, `connector` CHECK constraint for `Type2`, `CCS`, `CHAdeMO`, `Type2_Tethered`, `power_kw NUMERIC(6,2)` CHECK, `is_test`, audit fields, `deleted_at`, index `chargers_station_idx`
- [x] T010 Create migration `bornemap/backend/services/station-service/migrations/20260523_0004_seed_synthetic.sql` per data-model.md §11 + research.md R-015: insert 1 demo company + 500 stations from `infrastructure/seed/stations.csv` + 1–4 chargers each, all `is_test=TRUE`, deterministic UUID v5
- [x] T011 Create `bornemap/infrastructure/seed/stations.csv` with 500 rows covering Tunisia's bounding box, columns: `id,company_id,name,address,lng,lat,is_active,under_maintenance,opening_hours_osm`

### Backend: common-utils Crate

- [x] T012 Create `bornemap/backend/libs/common-utils/Cargo.toml` with deps: `serde`, `serde_json`, `uuid`, `chrono`, `thiserror`; NO `actix-web` dependency (domain library must be HTTP-framework-agnostic per Constitution Principle IV)
- [x] T013 [P] Create `bornemap/backend/libs/common-utils/src/ids.rs` with newtype wrappers `StationId(UUID)`, `CompanyId(UUID)`, `ChargerId(UUID)`, `UserId(UUID)` — all implement `Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, FromSql, ToSql, ToSchema`
- [x] T014 [P] Create `bornemap/backend/libs/common-utils/src/error.rs` with `DomainError` enum (Validation, NotFound, Unauthorized, Forbidden, Conflict, Internal), `impl Display` via `thiserror`, `impl From<sqlx::Error>`; the `actix_web::ResponseError` impl lives in `station-service` (see T019a) to keep `common-utils` framework-agnostic
- [x] T015 [P] Create `bornemap/backend/libs/common-utils/src/time.rs` with helper `now_utc() -> chrono::Utc::now()` and `to_rfc3339()`
- [x] T016 Create `bornemap/backend/libs/common-utils/src/lib.rs` re-exporting `ids`, `error`, `time`; crate-level lint attribute `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]`

### Backend: station-service Main + Config

- [x] T017 Create `bornemap/backend/services/station-service/Cargo.toml` with deps: `actix-web` 4, `sqlx` 0.7 (postgres, runtime-tokio-rustls, macros, uuid, chrono, json), `utoipa` 4 + `utoipa-swagger-ui`, `serde`/`serde_json`, `thiserror`, `tracing` + `tracing-subscriber` (json), `prometheus`, `jsonwebtoken` 9, `uuid`, `chrono`, `tokio` 1, `opening_hours` crate, `common-utils` path dep
- [x] T018 Create `bornemap/backend/services/station-service/src/config.rs` reading `MOCK_ADMIN_USERNAMES` (comma-separated, trimmed, fail-closed on empty), `MOCK_JWT_SECRET`, `BIND_ADDR`, `DATABASE_URL`, `BORNEMAP_HIDE_TEST_ROWS` (default `true` in prod, `false` in dev/staging — controls whether public reads filter `is_test=TRUE` rows per data-model.md §6) from env; `AppConfig::from_env() -> Result<Self, DomainError>`
- [x] T018a Create `bornemap/backend/services/station-service/src/lib.rs` that exports the OpenApi registry (`utoipa::OpenApi`), handler configuration, DTOs, and service traits — required so that `openapi-spec` bin (T034/T035) can import the handler registry without depending on the binary crate
- [x] T019 Create `bornemap/backend/services/station-service/src/main.rs` with Actix Web server bind, `/health/live` + `/health/ready` + `/metrics` endpoints, `tracing` init with JSON subscriber, `SqlxPool` from `DATABASE_URL`, mount `auth/` and `station/` modules, `utoipa::OpenApi` mount at `/swagger-ui`
- [x] T019a Create `bornemap/backend/services/station-service/src/error_adapter.rs` with `impl actix_web::ResponseError for DomainError` emitting RFC-7807 `application/problem+json` — this is the HTTP adapter that translates domain errors into HTTP responses, keeping `common-utils` framework-agnostic

### Backend: Auth Module (US3 — Elevated to Foundational)

- [x] T020 Create `bornemap/backend/services/station-service/src/auth/mod.rs` re-exporting `handlers`, `service`, `claims`, `middleware`
- [x] T021 Create `bornemap/backend/services/station-service/src/auth/claims.rs` with `TokenClaims` struct: `sub: Uuid`, `preferred_username: String`, `realm_access: RealmAccess { roles: Vec<String> }`, `iat: i64`, `exp: i64`; implement `Serialize`/`Deserialize`; constant `ADMIN_ROLE: &str = "admin"`
- [x] T022 Create `bornemap/backend/services/station-service/src/auth/service.rs` with `MockAuthService::login(username, role, allowlist, secret) -> Result<(TokenClaims, String), DomainError>`; validates allowlist membership for admin role; rejects driver role with 400; encodes HS256 JWT with 1h expiry; returns `MockLoginResponse`
- [x] T023 Create `bornemap/backend/services/station-service/src/auth/handlers.rs` with `POST /api/v1/auth/mock-login` handler; extracts `MockLoginRequest` JSON; calls `MockAuthService::login`; returns 200 + `MockLoginResponse` or 400/403 + RFC-7807; annotated with `utoipa::path`
- [x] T024 Create `bornemap/backend/services/station-service/src/auth/middleware.rs` with Actix middleware extracting Bearer token, decoding JWT, validating `exp`, attaching `TokenClaims` to request extensions; returns 401 + RFC-7807 on invalid/expired token
- [x] T025 Create `bornemap/backend/services/station-service/src/auth/models.rs` with `MockLoginRequest { username, role }`, `MockLoginResponse { access_token, token_type, expires_in }` — all annotated `utoipa::ToSchema`

### Backend: Station Module — Models + Filters (shared across US1 and US2)

- [x] T026 Create `bornemap/backend/services/station-service/src/station/mod.rs` re-exporting `handlers`, `service`, `repository`, `models`, `filters`
- [x] T027 Create `bornemap/backend/services/station-service/src/station/models.rs` with DTOs per data-model.md §12: `StationMarker`, `StationDetail`, `Charger`, `Company`, `BboxQuery`, `StationListResponse { viewport, markers, truncated }`, `AdminStationCreate`, `AdminStationPatch`, `AdminStationListResponse { items, next_cursor }`; all `#[derive(utoipa::ToSchema)]`; `coord` field is `[f64; 2]` ordered `[lng, lat]`
- [x] T028 Create `bornemap/backend/services/station-service/src/station/filters.rs` with `parse_bbox(query: &str) -> Result<BboxQuery, DomainError>` (validates west≤east, -180≤lng≤180, -90≤lat≤90) and `quantize_bbox(bbox: &mut BboxQuery)` rounding each bound to 4 decimals (round-half-away-from-zero per R-002)

### Backend: Companies Read-Only Support

- [x] T029 Create `bornemap/backend/services/station-service/src/companies/mod.rs` re-exporting `repository`
- [x] T030 Create `bornemap/backend/services/station-service/src/companies/repository.rs` with `CompanyRepository::get_by_id(pool, id) -> Result<Company, DomainError>` using `sqlx::query_as!`

### Backend: Observability

- [x] T031 [P] Create `bornemap/backend/services/station-service/src/observability/mod.rs` re-exporting `logging`, `metrics`
- [x] T032 [P] Create `bornemap/backend/services/station-service/src/observability/logging.rs` with `init_tracing()` setting `tracing_subscriber::fmt().json().with_target(false).finish()` and `TraceIdMiddleware` — an Actix middleware that generates a UUID v4 `trace_id` per request, inserts it into the `tracing::Span`, and sets `X-Trace-Id` response header (FR-013)
- [x] T033 [P] Create `bornemap/backend/services/station-service/src/observability/metrics.rs` with `REQUEST_COUNTER` and `REQUEST_DURATION_HISTOGRAM` Prometheus metrics; middleware that increments on each request

### Backend: OpenAPI Spec Binary

- [x] T034 Create `bornemap/backend/libs/openapi-spec/Cargo.toml` depending on `station-service` (as lib) and `utoipa`
- [x] T035 Create `bornemap/backend/libs/openapi-spec/src/main.rs` that builds `utoipa::OpenApi` from the station-service handler registry and prints the JSON to stdout

### Backend: Integration Test Harness

- [x] T036 Create `bornemap/backend/services/station-service/tests/common/mod.rs` with Testcontainers harness: `setup_test_db()` → spins up `postgis/postgis:16-3.4`, applies migrations via `sqlx migrate run`, inserts 50 lightweight fixture stations directly via SQL (separate from the 500-row production seed); returns `SqlxPool`; `teardown_test_db()` drops the container

### Frontend: Shared Packages

- [x] T037 Create `bornemap/frontend/packages/api-client/package.json` with `name: "@bornemap/api-client"`, scripts for `openapi-typescript-codegen`; add placeholder `openapi.json` snapshot (empty initially)
- [x] T038 Create `bornemap/frontend/packages/geo-models/package.json` with `name: "@bornemap/geo-models"`, deps: `zod`; tsconfig with `strict: true`
- [x] T039 [P] Create `bornemap/frontend/packages/geo-models/src/coordinate.ts` with `CoordinateModel = readonly [number, number]` type alias and Zod schema `coordinateSchema = z.tuple([z.number(), z.number()])`
- [x] T040 [P] Create `bornemap/frontend/packages/geo-models/src/viewport.ts` with `MapViewportModel { west, south, east, north }`, `quantizeBounds(viewport) => MapViewportModel` rounding to 4 decimals (round-half-away-from-zero), `insideViewport(coord, viewport) => boolean`
- [x] T041 [P] Create `bornemap/frontend/packages/geo-models/src/marker.ts` with `StationMarkerModel { id, name, coord: CoordinateModel, isActive, underMaintenance }`, `pinColor(marker) => string` returning `activeGreen` when `isActive && !underMaintenance`, else `inactiveRed`
- [x] T042 Create `bornemap/frontend/packages/geo-models/src/index.ts` re-exporting all types; add `vitest` unit tests for `quantizeBounds`, `insideViewport`, `pinColor` in `src/__tests__/`

### Backend: sqlx Offline Metadata

- [ ] T043 After migrations and all `sqlx::query!` / `query_as!` calls are in place, run `cargo sqlx prepare --workspace` and commit the generated `bornemap/backend/.sqlx/` directory; verify `SQLX_OFFLINE=true cargo build --workspace` succeeds

**Checkpoint**: Foundation ready — auth works, DB schema exists, common types defined, shared packages available. User story implementation can now begin.

---

## Phase 3: User Story 1 — Driver discovers nearest charger (Priority: P1) 🎯 MVP

**Goal**: A driver opens the web or mobile app, sees a map centered on Tunisia, pans/zooms to discover chargers as colored pins, taps a pin for details, and can navigate via deep link.

**Independent Test**: A driver can locate the nearest charger to a chosen address and view its details in under 30 seconds, on both web and mobile, using only the deployed staging build.

### Backend: Station Repository + Service + Handlers for US1

- [x] T044 [US1] Create `bornemap/backend/services/station-service/src/station/repository.rs` with `StationRepository::list_by_viewport(pool, bbox: &BboxQuery) -> Result<Vec<StationMarker>, DomainError>` using canonical SQL from data-model.md §7 (`ST_DWithin` + `LIMIT 5000` + `WHERE deleted_at IS NULL`). Decision: `name` is included in the viewport marker DTO (R5 exception) because pin tooltips require it for usability; the overhead is negligible (≤200 chars per marker)
- [x] T045 [US1] Add `StationRepository::get_by_id(pool, id: StationId) -> Result<StationDetail, DomainError>` to `repository.rs` using canonical SQL from data-model.md §8 (stations JOIN companies) + charger sub-query from §8
- [x] T046 [US1] Create `bornemap/backend/services/station-service/src/station/service.rs` with `StationService::list_by_viewport(bbox: &BboxQuery) -> Result<StationListResponse, DomainError>` that calls `parse_bbox` → `quantize_bbox` → `repository::list_by_viewport` → wraps in `StationListResponse { viewport, markers, truncated }`
- [x] T047 [US1] Add `StationService::get_by_id(id: StationId) -> Result<StationDetail, DomainError>` to `service.rs`
- [x] T048 [US1] Create `bornemap/backend/services/station-service/src/station/handlers.rs` with `GET /api/v1/stations?bbox=...` handler extracting `bbox` query param, calling `StationService::list_by_viewport`, returning 200 + `StationListResponse` or 400 + RFC-7807; annotated with `utoipa::path`
- [x] T049 [US1] Add `GET /api/v1/stations/{id}` handler to `handlers.rs` calling `StationService::get_by_id`, returning 200 + `StationDetail` or 404 + RFC-7807; annotated with `utoipa::path`

### Backend: Integration Tests for US1

- [x] T050 [US1] Create `bornemap/backend/services/station-service/tests/stations_public_read.rs` with integration tests using Testcontainers harness: (1) viewport query returns seeded stations, (2) empty viewport returns `markers: []` not 404, (3) bbox with >4 decimals gets quantized in response, (4) west > east returns 400, (5) full detail returns company + chargers, (6) soft-deleted station excluded from viewport, (7) `LIMIT 5000` truncation sets `truncated: true`, (8) `GET /api/v1/stations/{soft-deleted-id}` returns 404 (FR-015); also add contract tests asserting response JSON shapes match `utoipa`-generated OpenAPI schemas for `StationListResponse` and `StationDetail`

### Backend: OpenAPI Snapshot

- [x] T052 [US1] Run `cargo run -p openapi-spec --release > bornemap/frontend/packages/api-client/openapi.json` to generate the runtime OpenAPI snapshot; commit the file

### Frontend: Generate API Client

- [x] T053 [US1] Run `openapi-typescript-codegen` against `openapi.json` to generate `bornemap/frontend/packages/api-client/src/` with typed client classes; verify generated types include `StationMarker`, `StationDetail`, `ConnectorType`, `BboxQuery`, `MockLoginRequest`, `MockLoginResponse`, `Problem`

### Frontend: Web Admin Portal — Map View for US1

- [x] T054 [US1] Create `bornemap/frontend/admin-portal/package.json` with deps: `react` 18, `react-dom` 18, `vite` 5, `@tanstack/react-query` 5, `leaflet` 1.9, `react-leaflet` 4, `react-leaflet-cluster` 2, `tailwindcss` 3, `shadcn/ui`, `react-hook-form` 7, `zod` 3, `react-router-dom` 6, `@bornemap/api-client`, `@bornemap/geo-models`
- [x] T055 [P] [US1] Create `bornemap/frontend/admin-portal/vite.config.ts` with proxy `/api` → `http://localhost:8000`; `bornemap/frontend/admin-portal/tailwind.config.ts`; `bornemap/frontend/admin-portal/tsconfig.json` with `strict: true`; add `browserslist` config in `package.json` targeting last 2 stable versions of Chrome, Firefox, Safari, Edge (FR-020)
- [x] T056 [P] [US1] Create `bornemap/frontend/admin-portal/index.html` with `<div id="root">`; `bornemap/frontend/admin-portal/src/main.tsx` mounting React app with `QueryClientProvider` and `BrowserRouter`
- [x] T057 [P] [US1] Create `bornemap/frontend/admin-portal/src/app/App.tsx` with router: `/login`, `/map`, `/admin/stations`, `/admin/stations/new`, `/admin/stations/:id/edit`; `bornemap/frontend/admin-portal/src/app/router.tsx`; `bornemap/frontend/admin-portal/src/app/queryClient.ts` with staleTime 60s, `networkMode: 'offlineFirst'` to support cached viewport when offline (US1-Acceptance-4)
- [x] T058 [P] [US1] Create `bornemap/frontend/admin-portal/src/lib/apiClient.ts` re-exporting configured `@bornemap/api-client` instance with base URL from `VITE_API_BASE_URL`
- [x] T059 [P] [US1] Create `bornemap/frontend/admin-portal/src/styles/map-theme.constants.ts` with `BorneMapMapStyles` object: `pins.activeGreen = '#22c55e'`, `pins.inactiveRed = '#ef4444'`, cluster radius 40px, cluster min markers 15 (R7)
- [x] T060 [US1] Create `bornemap/frontend/admin-portal/src/features/map/useViewportStations.ts` hook: (1) 300ms debounce on viewport change (R1), (2) calls `apiClient.stationsListByViewport({bbox})`, (3) React Query cache key `['stations', quantizedBbox]` (R6), (4) staleTime 60s, (5) clips off-viewport markers before render (R3)
- [x] T061 [US1] Create `bornemap/frontend/admin-portal/src/features/map/MapView.tsx` with `react-leaflet` map centered on Tunisia `[36.8, 10.2]` zoom 7; `react-leaflet-cluster` at R7 thresholds; `Marker` components using `pinColor()` from geo-models; `useViewportStations` on `moveend`/`zoomend`; skeleton markers while data loads; offline/cached state when network unavailable (edge case: US1-Acceptance-4)
- [x] T062 [US1] Create `bornemap/frontend/admin-portal/src/features/map/StationDetailPanel.tsx` slide-in panel showing station name, address, company, charger count, opening hours (parsed by `opening_hours.js` library per FR-017), `is_active`/`under_maintenance` badges; display "Hours unknown" when `opening_hours_osm` is null
- [x] T063 [US1] Create `bornemap/frontend/admin-portal/src/features/map/NavigateButton.tsx` that opens `https://www.google.com/maps/dir/?api=1&destination={lat},{lng}` via `window.location.assign()` (FR-016, R-012)

### Frontend: Mobile App — Map View for US1

- [x] T064 [US1] Create `bornemap/frontend/mobile-app/package.json` with deps: `expo` SDK 50, `react-native` 0.73, `react-native-maps` 1.10, `@tanstack/react-query` 5, `@gorhom/bottom-sheet` 4, `@bornemap/api-client`, `@bornemap/geo-models`
- [x] T065 [P] [US1] Create `bornemap/frontend/mobile-app/app.config.ts` with Expo config; `bornemap/frontend/mobile-app/tsconfig.json` with `strict: true`
- [x] T066 [P] [US1] Create `bornemap/frontend/mobile-app/src/App.tsx` with `QueryClientProvider`, `NavigationContainer`, stack navigator with `MapScreen` and `OsUpdateRequiredScreen`
- [x] T067 [US1] Create `bornemap/frontend/mobile-app/src/hooks/useOsFloorCheck.ts` that reads `Platform.Version` and returns `isSupported: boolean` based on Android ≥ 10 / iOS ≥ 15 (FR-020)
- [x] T068 [US1] Create `bornemap/frontend/mobile-app/src/screens/OsUpdateRequiredScreen.tsx` — non-dismissible fullscreen "Update your OS to use BorneMap" screen; exits if OS below floor (FR-020)
- [x] T069 [US1] Create `bornemap/frontend/mobile-app/src/hooks/useViewportStations.ts` — same logic as web version but using `react-native-maps` `onRegionChangeComplete`; 300ms debounce; cache key `['stations', quantizedBbox]`
- [x] T070 [US1] Create `bornemap/frontend/mobile-app/src/styles/map-theme.constants.ts` mirroring web `BorneMapMapStyles`
- [x] T071 [US1] Create `bornemap/frontend/mobile-app/src/components/ClusteredMarkers.tsx` rendering `Marker` components with `tracksViewChanges={false}` (R4), `pinColor` from `pinColor()`, clustering via `react-native-maps` `Cluster` component (built-in) with `radius` prop set to trigger at R7 thresholds (>15 markers within 40px equivalent)
- [x] T072 [US1] Create `bornemap/frontend/mobile-app/src/screens/MapScreen.tsx` with `react-native-maps` `MapView` centered on Tunisia, `ClusteredMarkers`, `useViewportStations`, bottom sheet trigger on marker press; skeleton markers while data loads; offline/cached state when network unavailable (edge case: US1-Acceptance-4)
- [x] T073 [US1] Create `bornemap/frontend/mobile-app/src/screens/StationDetailSheet.tsx` with `@gorhom/bottom-sheet` showing station name, address, chargers, opening hours, "Navigate" button
- [x] T074 [US1] Create `bornemap/frontend/mobile-app/src/lib/deepLink.ts` with `buildNavigateUrl(coord: CoordinateModel) => string` returning `https://www.google.com/maps/dir/?api=1&destination={lat},{lng}`; navigate via `Linking.openURL()` (FR-016)

**Checkpoint**: User Story 1 fully functional — driver can open map, see pins, tap for detail, navigate. Testable independently on web + mobile.

---

## Phase 4: User Story 2 — Admin manages station data (Priority: P1)

**Goal**: An admin logs into the admin portal, lists stations, creates a new station, edits a station, and soft-deletes a station. Soft-deleted stations disappear from the public map.

**Independent Test**: A new admin can create a station with valid coordinates, see it appear on the map within 60 s, then soft-delete it and confirm it disappears.

### Backend: Station Repository + Service + Handlers for US2

- [x] T075 [US2] Add `StationRepository::admin_list(pool, limit, cursor, include_deleted, include_test) -> Result<(Vec<StationDetail>, Option<String>), DomainError>` to `repository.rs` — paginated with cursor-based pagination, optional `include_deleted`/`include_test` filters
- [x] T076 [US2] Add `StationRepository::admin_create(pool, input: AdminStationCreate) -> Result<StationId, DomainError>` to `repository.rs` using canonical SQL from data-model.md §9 (`ST_MakePoint(lng, lat)::geography`); also insert initial chargers if provided
- [x] T077 [US2] Add `StationRepository::admin_patch(pool, id: StationId, input: AdminStationPatch) -> Result<bool, DomainError>` to `repository.rs` — partial update building SET clause from non-null DTO fields (see design note below on constitution-compliant dynamic SQL). All validation MUST already be performed by the service layer before reaching this method

**Design Note — Constitution-compliant partial-update SQL (T077)**: Principle VI bans "dynamically concatenated SQL strings." The approach for `admin_patch`: define a single `sqlx::query!` that sets ALL updatable columns, binding `NULL` for fields the caller did not change (the service layer resolves "unchanged" by reading current values first via `get_by_id`). This avoids dynamic SQL while keeping the query compile-time verified. Alternative: use `sqlx::query_as!` with a runtime-built query string if the read-then-write overhead is unacceptable — this requires a documented Complexity Tracking entry in `plan.md` justifying the deviation.
- [x] T078 [US2] Add `StationRepository::admin_soft_delete(pool, id: StationId) -> Result<bool, DomainError>` to `repository.rs` using canonical SQL from data-model.md §10 (`SET deleted_at = NOW()`)
- [x] T079 [US2] Add `StationService::admin_list(...)`, `StationService::admin_create(...)`, `StationService::admin_patch(...)`, `StationService::admin_soft_delete(...)` to `service.rs` — each validates input, calls repository, returns result; `admin_create` and `admin_patch` validate `opening_hours_osm` with `opening_hours` crate (rejecting with 422 + RFC-7807 citing parse error); validates `connector` enum; validates `coord` range [-180,180]/[-90,90]. Per Constitution Principle II, ALL validation lives here — the repository receives only pre-validated data
- [x] T080 [US2] Add admin handlers to `handlers.rs`: `GET /api/v1/admin/stations` (paginated list with `include_deleted`/`include_test`), `POST /api/v1/admin/stations` (201 on success, 422 on validation), `PATCH /api/v1/admin/stations/{id}` (200 on success, 404/422), `DELETE /api/v1/admin/stations/{id}` (204 on success, 404); all require admin Bearer token via auth middleware; all annotated `utoipa::path`

### Backend: Integration Tests for US2

- [x] T081 [P] [US2] Create `bornemap/backend/services/station-service/tests/stations_admin_crud.rs` with integration tests: (1) admin create station returns 201 + detail, (2) create with bad connector returns 422, (3) create with bad opening_hours returns 422 with parse error, (4) admin patch toggles `under_maintenance`, (5) admin soft delete returns 204 and station disappears from public viewport, (6) admin list with `include_deleted=true` shows soft-deleted station, (7) non-admin token → 403, (8) expired token → 401, (9) driver role mock-login → 400, (10) concurrent patch to same station — last-write-wins on `updated_at` (edge case)

### Backend: Auth Integration Tests

- [ ] T082 [P] Create `bornemap/backend/services/station-service/tests/auth_mock_login.rs` with integration tests: (1) allowlisted admin login → 200 + JWT with canonical claim shape, (2) non-allowlisted → 403, (3) driver role → 400, (4) expired token → 401 on admin endpoint, (5) JWT claim shape matches `TokenClaims` struct exactly, (6) decoded JWT `sub` is a valid UUID v4, (7) `preferred_username` matches request, (8) `realm_access.roles` is `["admin"]`, (9) `exp` - `iat` = 3600, (10) `MOCK_ADMIN_USERNAMES=""` causes server startup failure (fail-closed FR-019), (11) verify `MOCK_ADMIN_USERNAMES` never appears in log output

### Frontend: Admin Portal — Auth + CRUD Pages for US2

- [x] T083 [US2] Create `bornemap/frontend/admin-portal/src/features/auth/authStore.ts` — persists JWT in `localStorage`, provides `login(username)`, `logout()`, `isAuthenticated`, `token` getter; decodes JWT to check `exp`
- [x] T084 [US2] Create `bornemap/frontend/admin-portal/src/features/auth/LoginPage.tsx` — form with username input; calls `POST /api/v1/auth/mock-login` with `{username, role: "admin"}`; stores token via `authStore`; redirects to `/admin/stations`; shows 403 error inline
- [x] T085 [US2] Create `bornemap/frontend/admin-portal/src/features/stations/stationFormSchema.ts` — Zod schema mirroring FR-017/018: `company_id` UUID, `name` 1-200 chars, `address` 1-500 chars, `coord` tuple `[lng, lat]` with range validation, `opening_hours_osm` optional string ≤500 chars, `chargers` array with `connector` enum literal and `power_kw` >0 ≤600
- [x] T086 [US2] Create `bornemap/frontend/admin-portal/src/features/stations/StationsTablePage.tsx` — paginated table fetching `GET /api/v1/admin/stations` with cursor; columns: name, address, company, is_active, under_maintenance, actions (edit/delete); client-side search filtering within the currently loaded page only (known limitation — server-side `?q=` search deferred to MVP 2 per Constitution Principle II); "Add Station" button
- [x] T087 [US2] Create `bornemap/frontend/admin-portal/src/features/stations/StationEditPage.tsx` — form using `react-hook-form` + `stationFormSchema`; on submit calls `PATCH /api/v1/admin/stations/{id}`; success toast + navigate back to table; shows validation errors from 422 RFC-7807 response
- [x] T088 [US2] Add "New Station" page at `/admin/stations/new` in router — same form as edit but calls `POST /api/v1/admin/stations`; redirects to table on 201

### Frontend: E2E Tests for US1 + US2

- [x] T089 [US2] Create `bornemap/frontend/admin-portal/playwright.config.ts` targeting `http://localhost:5173`
- [x] T090 [P] [US1] Create `bornemap/frontend/admin-portal/e2e/driver-discovers-charger.spec.ts` — Playwright test: open map → pan to Tunis → verify pins render → click pin → verify detail panel → click Navigate → verify Google Maps URL
- [x] T091 [P] [US2] Create `bornemap/frontend/admin-portal/e2e/admin-creates-station.spec.ts` — Playwright test: login as admin → navigate to stations → create station with valid data → verify station appears in table → toggle maintenance → verify pin color flips → soft delete → verify station gone from table

### Frontend: Mobile E2E Skeleton for US1

- [x] T092 [US2] Create `bornemap/frontend/mobile-app/e2e/driver-discovers-charger.e2e.ts` — Detox skeleton config (runs only in staging/nightly CI job)

**Checkpoint**: User Stories 1 AND 2 both work independently. Admin CRUD is functional and gated by auth. Public map reflects admin changes.

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: CI, documentation, final validation, performance baseline.

- [x] T093 [P] Create `bornemap/.github/workflows/ci.yml` per research.md R-014: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `SQLX_OFFLINE=true cargo build --workspace --release`, `cargo test --workspace` (Testcontainers), `pnpm -r install --frozen-lockfile`, `pnpm -r lint && pnpm -r typecheck && pnpm -r test`, OpenAPI drift gate
- [ ] T093a [P] Create `bornemap/backend/Dockerfile` — multi-stage build: `rust:1.78` builder, `gcr.io/distroless/cc` runtime, image ≤ 50 MB per constitution; tag format `v[MAJOR].[MINOR].[PATCH]-[GIT_SHA]`
- [x] T094 [P] Create `bornemap/.github/workflows/openapi-diff.yml` per R-014: manual trigger, runs `oasdiff` against `main` baseline, reports breaking changes
- [x] T095 [P] Create `bornemap/frontend/admin-portal/src/styles/globals.css` with Tailwind base imports + BorneMap brand tokens (colors, spacing)
- [x] T096 Regenerate `bornemap/frontend/packages/api-client/` from final `openapi.json` snapshot; verify `ConnectorType` constants exported; verify no hand-rolled HTTP types in `frontend/` (SC-007)
- [ ] T097 Run `cargo sqlx prepare --workspace` to re-verify `bornemap/backend/.sqlx/` offline metadata is current after all code changes; verify `SQLX_OFFLINE=true cargo build --workspace` succeeds (SC-009 / Principle VI)
- [ ] T098 Create `bornemap/backend/services/station-service/tests/health.rs` — integration test: `/health/live` returns 200, `/health/ready` returns 200 when DB is up, `/health/ready` returns 503 when DB is unreachable
- [ ] T099 Verify GiST index exists on `station_domain.stations.location` by running introspection query `\d+ station_domain.stations` in CI (SC-008 / Principle I)
- [ ] T100 Run k6 load test profile: 50 concurrent viewport queries/s against 500 seeded stations; verify P95 ≤ 200 ms (SC-001)
- [ ] T100a Profile map FPS on reference devices (Samsung Galaxy A33 5G + iPhone 11) using platform frame profiler during a 10-second sustained pan/zoom gesture; verify ≥ 60 FPS (SC-002)
- [ ] T100b Measure cold-start time to first marker render on 4G throttled connection: web (DevTools Network 4G throttle) ≤ 2.5 s, mobile (Network Link Conditioner 4G) ≤ 3.5 s (SC-003)
- [ ] T101 Verify `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` is set on the `station-service` binary crate; run `cargo clippy` and confirm zero violations (SC-009)
- [ ] T102 Walk through `specs/001-mvp1-geo-core/quickstart.md` steps 1–11 end-to-end on a clean machine; document any deviations
- [ ] T102a Create `specs/001-mvp1-geo-core/validation.md` template per constitution Principle XI: cohort definition (≥5 drivers + ≥2 admins), per-task success/failure breakdown, SUS score fields, SC-001..SC-009 measurement sections, proceed/kill gate for MVP 2

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2)
- **User Story 2 (Phase 4)**: Depends on Foundational (Phase 2) + US1 backend handlers must exist (admin changes visible on public map); US3 (auth) implementation is in Phase 2, auth tests are in Phase 4 (T082)
- **Polish (Phase 5)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Foundational — no dependencies on other stories
- **US2 (P1)**: Can start after Foundational — auth (US3) is in Foundational; admin changes should be visible on US1's public map
- **US3 (P2)**: Implementation is in Foundational Phase 2 (because US2 depends on it); comprehensive auth tests are in Phase 4 (T082)

### Within Each User Story

- Models before services
- Services before handlers
- Repository before service
- Handlers before integration tests
- Integration tests before frontend consumption
- Backend complete before frontend for that story

### Parallel Opportunities

**Phase 1 — Setup** (all [P] tasks run in parallel):
- T002 (Cargo workspace) ‖ T003 (pnpm workspace) ‖ T004 (Docker Compose) ‖ T005 (backend env) ‖ T006 (frontend env)

**Phase 2 — Foundational**:
- T013 (ids.rs) ‖ T014 (error.rs) ‖ T015 (time.rs) — all different files in common-utils
- T031 (observability mod) ‖ T032 (logging.rs) ‖ T033 (metrics.rs)
- T039 (coordinate.ts) ‖ T040 (viewport.ts) ‖ T041 (marker.ts) — all different files in geo-models
- T050 (stations tests) ‖ T082 (auth tests) — different test files

**Phase 3 — US1**:
- T055 (vite config) ‖ T056 (index.html) ‖ T057 (App.tsx) ‖ T058 (apiClient.ts) ‖ T059 (map-theme constants)
- T065 (Expo config) ‖ T066 (mobile App.tsx)

**Phase 4 — US2**:
- T081 (admin CRUD tests) ‖ T082 (auth tests)
- T090 (Playwright driver test) ‖ T091 (Playwright admin test)

**Phase 5 — Polish**:
- T093 (CI workflow) ‖ T094 (OpenAPI diff workflow) ‖ T095 (globals.css)

---

## Parallel Example: Phase 2 — Foundational

```bash
# After T012 (common-utils Cargo.toml) completes, launch all source files in parallel:
Task: "Create ids.rs in backend/libs/common-utils/src/ids.rs"
Task: "Create error.rs in backend/libs/common-utils/src/error.rs"
Task: "Create time.rs in backend/libs/common-utils/src/time.rs"

# After T017-T019 (station-service main), launch auth + station modules in parallel:
Task: "Create auth/ module (T020-T025) — 6 files"
Task: "Create station/models.rs + filters.rs (T026-T028) — 3 files"

# After T038 (geo-models package.json), launch all model files in parallel:
Task: "Create coordinate.ts (T039)"
Task: "Create viewport.ts (T040)"
Task: "Create marker.ts (T041)"
```

## Parallel Example: Phase 3 — US1

```bash
# Backend US1 handlers + tests can proceed in parallel with frontend setup:
Task: "T044-T049 — Station repository, service, handlers for public reads"
Task: "T054-T059 — Admin portal package setup + config files"

# After backend US1 is done and API client is generated (T053):
Task: "T060-T063 — Web map feature components"
Task: "T064-T074 — Mobile app screens + components"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T006)
2. Complete Phase 2: Foundational (T007–T043)
3. Complete Phase 3: User Story 1 (T044–T074)
4. **STOP and VALIDATE**: Test US1 independently — driver can discover chargers on map + navigate
5. Deploy/demo if ready — this is the minimum viable product

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo (admin can manage data)
4. Polish → CI + perf baseline + FPS/cold-start profiling → Ready for validation cohort

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 backend (T044–T052) → then US1 web (T054–T063)
   - Developer B: US1 mobile (T064–T074) — starts after T053 (API client generated)
   - Developer C: US2 backend (T075–T082) — starts after T044–T049 are stable
3. US2 frontend (T083–T092) starts after US1 frontend patterns are established

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- TDD order: write test → see it fail → implement → see it pass (Principle V)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- `MOCK_ADMIN_USERNAMES` is never logged (FR-019)
- All `coord` fields are `[lng, lat]` — never `{lat, lng}` (Principle I)
- `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` enforced on all non-test Rust (Principle VII)
- `sqlx::query!` / `query_as!` everywhere — no dynamic SQL strings (Principle VI)
- **admin_patch SQL approach**: service layer reads current values via `get_by_id`, merges with patch fields, then calls repository with a full-row `sqlx::query!` UPDATE — avoids dynamic SQL (Principle VI)
- **name in viewport markers**: `name` is included in `StationMarker` (R5 exception) because pin tooltips require it; overhead is negligible
- **US3 priority**: P2 by user-facing impact, P1 by dependency order (US2 requires auth); implementation is in Phase 2 (Foundational)
- **Degraded-operations rule** (Constitution §Performance): deferred to MVP 4 with documented Complexity Tracking entry CT-001 in plan.md
- **Client-side admin search**: limited to currently loaded page; server-side `?q=` deferred to MVP 2
