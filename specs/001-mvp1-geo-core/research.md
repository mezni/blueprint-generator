# Phase 0 — Research: MVP 1 Geo Core

**Branch**: `001-mvp1-geo-core` | **Date**: 2026-05-23 | **Plan**: [`plan.md`](./plan.md)

This document records the technology decisions and best-practice patterns that back MVP 1. No `NEEDS CLARIFICATION` markers remain after `/speckit.clarify`; the items below are best-practice consolidation only.

---

## R-001 — PostGIS viewport query pattern

**Decision**: Use `ST_DWithin(location, ST_MakeEnvelope(west, south, east, north, 4326)::geography, 0)` as the indexed primary predicate. Order by `id` for stable pagination; do **not** compute `ST_Distance` server-side in MVP 1 (no proximity sort yet).

**Rationale**:
- `ST_DWithin` with a distance of `0` against an envelope leverages the GiST index for a bounding-box overlap test, equivalent to `ST_Intersects` but conventionally faster and aligned with the constitution's "primary predicate" rule.
- Casting the envelope to `geography` matches the `GEOGRAPHY(Point, 4326)` column type and avoids implicit SRID conversion in the predicate path.
- Stable `ORDER BY id` keeps test snapshots reproducible.
- Hard `LIMIT 5000` matches FR-001/edge case and prevents pathological responses.

**Alternatives considered**:
- `ST_Intersects(location::geometry, envelope)` — would force a cast on every row; the constitution mandates `ST_DWithin` anyway (Principle I).
- `<->` operator (KNN) — appropriate for proximity sort (deferred to MVP 2's "Nearest" feature), not viewport filtering.
- Pre-clustered DB-side aggregation (e.g., `ST_ClusterKMeans`) — over-engineered for 500-50k station scale; client-side clustering meets R7.

---

## R-002 — Bounding-box quantization

**Decision**: Quantize every viewport input to 4 decimal places (≈ 11 m) at the **edge** (HTTP handler `extract` step) before reaching the service layer. Round-half-away-from-zero. Reflect the quantized bbox in the response envelope so clients can confirm cache-key alignment.

**Rationale**:
- 4 decimals gives ≈ 11.1 m resolution at the equator — far below pin radius — and yields a cache-key cardinality compatible with 60 s `staleTime`.
- Quantizing at the edge guarantees frontend and backend keys match (constitutional R2 and R6 alignment).
- Round-half-away-from-zero matches `Math.round(n * 10_000) / 10_000` in TypeScript and `(n * 10_000.0).round() / 10_000.0` in Rust.

**Alternatives considered**:
- Truncation (floor) — produces asymmetric cells; can drop a marker that exists on a cell boundary into a neighboring bbox.
- 3 decimals (≈ 111 m) — too coarse; visible "snapping" of viewport extents at high zoom.
- 5 decimals (≈ 1.1 m) — cache cardinality balloons with no UX benefit.

---

## R-003 — Mock JWT structure (HS256, frozen claim shape)

**Decision**: Use HS256 with a dev-only secret read from `MOCK_JWT_SECRET`. Claim shape locked from MVP 1:

```json
{
  "sub": "<uuid-v4>",
  "preferred_username": "<from request>",
  "realm_access": { "roles": ["admin"] },
  "iat": <unix>,
  "exp": <unix + 3600>
}
```

**Rationale**:
- HS256 keeps the implementation tiny while preserving the canonical claim shape that Keycloak (MVP 5) will continue to emit.
- 1-hour expiry mirrors Keycloak defaults and is short enough to exercise the 401 refresh path during validation.
- `sub` is a freshly generated UUID v4 per login (no user persistence in MVP 1).

**Alternatives considered**:
- RS256 with an embedded test key — adds key-management ceremony with no security benefit at this phase.
- Opaque session cookie — diverges from the eventual OIDC bearer flow and would force a rewrite at MVP 5.

---

## R-004 — Admin allowlist semantics

**Decision**: `MOCK_ADMIN_USERNAMES` is a comma-separated, trimmed, case-sensitive list. Server refuses to start if it is unset or empty. The variable is masked in logs and never echoed in HTTP responses.

**Rationale**:
- Fail-closed start prevents accidental "anyone can be admin" deployments.
- Case-sensitive avoids quietly accepting `Alice` when `alice` was intended.
- Masking in logs blocks credential exposure even at TRACE level.

**Alternatives considered**:
- Case-insensitive — friendlier UX, but the JWT's `preferred_username` claim must be reproducible; case-sensitive removes ambiguity.
- DB-persisted seed list — moves the source of truth into a migration, harder to rotate per environment.

---

## R-005 — Opening-hours validation

**Decision**: Validate `opening_hours_osm` strings with the Rust `opening_hours` crate (OSM-spec parser) on `POST` / `PATCH`. Reject invalid strings with HTTP 422 and an RFC-7807 problem detail citing the parse position and a short hint. Empty / `NULL` is allowed and represents "Hours unknown".

**Rationale**:
- The OSM `opening_hours` grammar is the de-facto industry standard for charger operators and OSM contributors.
- Parser-based validation is cheap (sub-millisecond) and gives precise error messages.
- The same string format is parseable on the web (`opening_hours.js`) and mobile via the JS engine, avoiding duplication.

**Alternatives considered**:
- Structured weekly JSON — heavier admin UI, more code, no library reuse with the OSM ecosystem.
- Free text only — fastest but unverifiable; admins will silently disagree on syntax.

---

## R-006 — Connector enum + DB constraint

**Decision**: Persist `chargers.connector` as `TEXT` with a `CHECK (connector IN ('Type2','CCS','CHAdeMO','Type2_Tethered'))` constraint. Export the same four constants from `@bornemap/api-client` as a TS string-literal union derived from the OpenAPI schema.

**Rationale**:
- `CHECK` is enforced by the DB and survives ad-hoc inserts.
- A string-literal union in TS (rather than a Postgres `ENUM`) keeps migrations simple — adding a value is a single `ALTER TABLE … DROP CONSTRAINT / ADD CONSTRAINT` instead of `ALTER TYPE`.
- Frontend share is generated, so no drift between server and client allowed values.

**Alternatives considered**:
- Postgres `ENUM` type — `ALTER TYPE … ADD VALUE` is non-transactional in older versions; harder to roll back during migrations.
- Lookup table — over-engineered for 4 stable values; introduces a join.

---

## R-007 — Soft delete and `is_test` flag

**Decision**: All non-immutable entities carry `deleted_at TIMESTAMPTZ NULL` and `is_test BOOLEAN NOT NULL DEFAULT FALSE`. Every repository read filters `WHERE deleted_at IS NULL`. Test fixtures set `is_test = TRUE`; production reads filter `AND is_test = FALSE` only in environments where `BORNEMAP_HIDE_TEST_ROWS=true` (default true in prod, false in dev/staging).

**Rationale**:
- Soft delete preserves history for the admin "show deleted" toggle and for the future audit log (MVP 3 events).
- `is_test` separates synthetic seed rows from real operator data, allowing the same DB to host the validation cohort and dev test rows without contamination.

**Alternatives considered**:
- Hard delete — destroys audit trail; FR-015 forbids.
- Separate `*_archive` tables — doubles schema surface; admin queries get harder.

---

## R-008 — OpenAPI generation and client codegen

**Decision**: Backend handlers are annotated with `utoipa::path` and DTOs with `utoipa::ToSchema`. A small bin `openapi-spec` walks the registry and prints the spec to `openapi.json`. The frontend uses `openapi-typescript-codegen` to emit `@bornemap/api-client`. CI regenerates and `git diff --exit-code` blocks the build on drift.

**Rationale**:
- Single source of truth in Rust handlers.
- Generated client carries response/request types and operation IDs, eliminating hand-rolled interfaces (constitution Principle III, FR-014).
- CI drift check makes manual changes to the spec impossible.

**Alternatives considered**:
- Hand-authored `openapi.yaml` — fast at the start, drifts immediately, defeats Principle III.
- `paperclip` or `okapi` — less active maintenance and weaker Actix Web integration than `utoipa`.

---

## R-009 — Frontend state architecture

**Decision**: All server state lives in React Query (`@tanstack/react-query` v5). UI state stays in component state / context. No Redux, no Zustand. Web uses `react-router-dom` for routes; mobile uses `react-navigation` v6 stack.

**Rationale**:
- React Query already owns cache, retries, devtools, and SSR-style hydration. Adding a second store creates two sources of truth.
- For MVP 1 the only non-server state is the active viewport, the active marker selection, and form drafts — all component-scoped.

**Alternatives considered**:
- Zustand for the viewport — premature abstraction; a single `useMapInteraction()` hook with `useState` + `useEffect` suffices.
- Apollo Client / SWR — comparable; React Query is already the team baseline per the constitution.

---

## R-010 — Map engine choice per platform

**Decision**: Web uses Leaflet via `react-leaflet` + `react-leaflet-cluster`. Mobile uses `react-native-maps` (Apple Maps on iOS, Google Maps on Android by default).

**Rationale**:
- Leaflet is free, dependency-light, and matches the constitution's `docs/architecture.md` choice.
- `react-leaflet-cluster` is configurable to the R7 threshold (15 markers / 40 px radius).
- `react-native-maps` is a first-class Expo SDK 50 module — no Dev Client needed, satisfying the Managed Framework Rule.
- Pin color is computed via `pinColor()` from `BorneMapMapStyles.pins`.

**Alternatives considered**:
- Mapbox GL JS / Mapbox GL Native — requires a paid token at scale and a Dev Client on mobile (forbidden in MVP 1).
- Apple MapKit JS / Google Maps JS — non-OSS, token-gated, and would force a per-platform divergence.

---

## R-011 — Performance reference devices

**Decision**: Measure SC-002 (60 FPS pan/zoom) on **Samsung Galaxy A33 5G** (Android 12, 6 GB RAM, mid-2022) and **iPhone 11** (iOS 15+). Compatibility floor: Android 10 (API 29) and iOS 15.

**Rationale**:
- Galaxy A33 5G is mid-2022 mid-range and broadly representative of EV-owning households in Tunisia.
- iPhone 11 is the oldest device that still receives current iOS updates.
- Hard floors below the perf targets avoid silently degraded UX on unsupported devices.

**Alternatives considered**:
- Flagship-only targets — easier to hit 60 FPS but misrepresents the cohort.
- Single Android target without an iOS counterpart — leaves SC-002 unmeasurable on iOS.

---

## R-012 — Navigation deep link

**Decision**: Use a single URL pattern `https://www.google.com/maps/dir/?api=1&destination={lat},{lng}` opened via `window.location.assign()` (web) and `Linking.openURL()` (mobile). The OS routes the URL to its default map app; falls back to the browser tab if none is installed.

**Rationale**:
- Single implementation across platforms.
- Honors NG-5 (no bundled routing) and FR-016.
- Universally supported by Apple Maps, Google Maps, Waze (which intercepts the URL), and major web browsers.

**Alternatives considered**:
- `maps://` (iOS) + `geo:` (Android) — two implementations, more code, no UX benefit.
- User-selectable provider — defer to MVP 2+.

---

## R-013 — Testing strategy

**Decision**:
- **Unit**: pure functions (`quantizeBounds`, `pinColor`, claim builder).
- **Contract**: per-handler tests assert request/response shapes match generated OpenAPI types (using `serde_json::Value` against the schema).
- **Integration**: Testcontainers spins up `postgis/postgis:16-3.4`, applies migrations, seeds 50 fixture stations marked `is_test=TRUE`, asserts repository and service behavior.
- **E2E**: Playwright (web) drives US 1 and US 2 against a staging deploy; Detox skeleton lives in `frontend/mobile-app/e2e/` and runs only in the nightly job.

**Rationale**:
- Testcontainers gives a real PostGIS for every test run, satisfying Principle V.
- Splitting contract from integration keeps the OpenAPI contract honest while integration covers DB behavior.
- E2E is intentionally narrow to keep CI fast.

**Alternatives considered**:
- Mocked Postgres — banned by Principle V.
- Cypress instead of Playwright — comparable; team baseline is Playwright per existing docs.

---

## R-014 — CI pipeline shape

**Decision**: Single workflow `.github/workflows/ci.yml` running on PR + push to `main`:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `SQLX_OFFLINE=true cargo build --workspace --release`
4. `cargo test --workspace` (with Testcontainers, Docker-in-Docker)
5. `pnpm -r install --frozen-lockfile`
6. `pnpm -r lint && pnpm -r typecheck && pnpm -r test`
7. Generate `openapi.json` from `openapi-spec` bin; `git diff --exit-code frontend/packages/api-client/openapi.json`

A separate workflow `.github/workflows/openapi-diff.yml` (manual trigger) runs `oasdiff` against the latest `main` baseline and reports breaking changes.

**Rationale**:
- One workflow is fastest at MVP 1 size; matches the constitution's enumerated stages.
- Drift gate prevents manual edits to the generated artifact.

**Alternatives considered**:
- Separate Rust and Node workflows — adds matrix complexity without speed benefit at this size.

---

## R-015 — Synthetic seed dataset

**Decision**: Seed migration `20260523_0004_seed_synthetic.sql` inserts 500 stations distributed across Tunisia using a hand-curated CSV (`infrastructure/seed/stations.csv`). Each station gets 1–4 chargers drawn from the four connector types. `is_test = TRUE` on all rows.

**Rationale**:
- 500 stations is enough to validate SC-001 (P95 ≤ 200 ms with 50 RPS) and to populate dense Tunis-center viewports.
- A CSV source is easy to regenerate later (MVP 4 will need 10k-50k rows; same pipeline scales).
- `is_test = TRUE` keeps the seed out of any future "real data only" view.

**Alternatives considered**:
- Random uniform distribution — produces unrealistic densities and bad demos.
- API-driven seeding — adds operational overhead for no benefit.

---

## Open Items

None. All Phase 0 questions resolved.
