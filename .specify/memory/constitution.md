# BorneMap Constitution

> **Canonical Speckit constitution.** Every `/speckit.*` command and every implementer (human or LLM) MUST treat this file as the binding source of truth. A human-readable mirror lives at `docs/constitution.md`. In any conflict, **this file wins**.

## Core Principles

### I. Spatial-First, PostGIS-Always (NON-NEGOTIABLE)

- The canonical spatial store is **PostgreSQL 16 + PostGIS 3.4**. No alternative spatial engine may be introduced at any phase.
- Every coordinate column is `GEOGRAPHY(Point, 4326)`.
- Every spatial column **MUST** have a **GiST** index. Migrations that add a spatial column without a GiST index fail CI.
- The primary spatial predicate is `ST_DWithin`. `ST_Distance` is allowed **only** for post-filter sorting/ranking on already-bounded result sets.
- All coordinates on wire, in code, and in storage use the array form `[lng, lat]`. Any `{lat, lng}` JSON in code review is rejected.
- Viewport bounding boxes are quantized to **4 decimal places** before being used as cache keys or repository query parameters.

### II. Backend Authority, Frontend Projection (NON-NEGOTIABLE)

- The backend is the sole authority for validation, authorization, business rules, spatial filtering, and state mutation.
- The frontend is a pure projection layer. It MAY hold UI state and form drafts; it MUST NOT hold business rules.
- Filtering / structural query logic lives in the Repository Layer only. Frontend "filter-after-fetch" of server data is banned.
- Control flow direction is strictly: **Handlers → Service Layer → Repository Layer**. No reverse calls. No skip-layer calls.

### III. API-First Contract (NON-NEGOTIABLE)

- The OpenAPI 3.1 schema produced by `utoipa` from Rust handlers is the **single** source of truth for the API.
- Frontend HTTP clients are **generated** from `openapi.json` (e.g., `openapi-typescript-codegen`). Hand-written `interface ApiFoo {}` for HTTP payloads is banned.
- CI fails the build on any **breaking** OpenAPI diff against the previous `main` snapshot.

### IV. Modular Monolith Until Justified

- Phases 1–5 are a single Rust binary (`bornemap-backend`) compiled from a Cargo workspace.
- Domain modules (`station`, `identity`, `profile`, `review`, `event`, `settings`) communicate **only** through public service traits exchanging DTOs. No cross-domain repository imports. No cross-domain schema reads.
- Microservice extraction (Phase 6) requires a written justification matching at least one of:
  - Sustained CPU or memory saturation that vertical scaling cannot resolve.
  - Per-domain traffic skew ≥ 10× across the monolith.
  - Regulatory, tenancy, or contractual isolation requirement.
- Any Phase 6 PR without that justification document is rejected.

### V. Test-First with PostGIS in CI (NON-NEGOTIABLE)

- TDD is mandatory for every backend domain change: failing test first, then implementation.
- Integration tests **MUST** run against a real PostGIS instance via **Testcontainers**. Mocking PostGIS in spatial tests is banned.
- Every new endpoint adds at least one contract test (request/response against generated OpenAPI types) and one integration test (real DB, real query).
- `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` are green before merge.

### VI. SQLx Compile-Time Verified

- All SQL is executed via `sqlx::query!` or `sqlx::query_as!`. Dynamically concatenated SQL strings are banned.
- CI runs with `SQLX_OFFLINE=true` against committed `.sqlx/` query metadata. Schema-affecting PRs MUST commit refreshed metadata.

### VII. Type-Driven Safety (Rust)

- Use the newtype pattern for all domain identifiers: `pub struct StationId(pub Uuid);`, `pub struct UserId(pub Uuid);`, etc. Bare `Uuid` in handler or service signatures is rejected.
- All fallible paths return `Result<T, DomainError>`. `unwrap`, `expect`, and `panic!` in non-test code are banned.
- Public errors are serialized as **RFC-7807** `application/problem+json`.

### VIII. Map Interaction Runtime Rules (CI-Blocking)

The frontend `MapInteractionDomain` module enforces these rules. Lint / unit-test failures here block merge.

- **R1 — Viewport Debounce:** Viewport-triggered fetches fire only after the map has been settled for ≥ **300 ms**. Bind to `moveend` / `zoomend`, never to intra-gesture events.
- **R2 — Query Quantization:** Bounding-box coordinates are rounded to **4 decimal places** before generating React Query keys or hitting the network.
- **R3 — Marker Virtualization:** Markers outside the active quantized viewport are not instantiated. Off-viewport entries are filtered out of the array passed to the renderer.
- **R4 — Gesture Priority:** On React Native, static `<Marker>` components set `tracksViewChanges={false}`. Web markers must not re-render on pan.
- **R5 — Lazy Hydration:** Initial marker payloads contain only summary fields (`id`, `coord`, `is_active`, `under_maintenance`, `name`). Full details fetch lazily on user interaction. Quick-filter pills mutate local state only — they MUST NOT issue HTTP requests if the current quantized viewport is cached.
- **R6 — Bounds Cache Registry:** React Query keys use `['stations', quantizedBbox]`. Cached entries within `staleTime` (default 60 s) are reused without refetch.
- **R7 — Cluster Threshold:** Clustering activates at density > **15 markers within a 40 px radius**.

### IX. Backend Authority Over Pin Status (Status Projection Rule)

- Pin color reflects **administrative** state (`is_active`, `under_maintenance`) computed by the backend and shipped in the marker DTO. Frontends MUST NOT recompute or override status from any other source. Real-time hardware telemetry is explicitly out of scope.

### X. Idempotent Profile Initialization

- First-login profile creation (federated or invitation-based) MUST use `INSERT … ON CONFLICT (user_id) DO UPDATE`. No client-side "check then create" flow.

### XI. Iterative Real-User Validation (NON-NEGOTIABLE)

- Every MVP ends with a **real-user validation cycle**: a defined cohort, scripted tasks, quantitative metrics (task success rate, P50/P95 latency observed by users, error rate), and qualitative feedback (≥ 5 interviews per MVP).
- A proceed/kill gate evaluates the next MVP based on the previous MVP's validation report. The proceed gate is documented in `specs/00X-mvpN/spec.md` Success Criteria. If the gate fails, scope adjustments precede the next MVP.

### XII. Non-Goals (Hard Boundaries)

The system **SHALL NOT** include:

- EV charging session control (OCPP / OPP / hardware signaling).
- Billing, payments, or wallet integration.
- Smart-charging or grid energy optimization.
- Real-time hardware telemetry from chargers.
- Custom routing / turn-by-turn navigation (navigation is delegated to OS map providers via deep links).
- Distributed event streaming, microservices, or service mesh before Phase 6 is gated open by Principle IV's justification rule.

Any new feature touching these areas requires a constitution amendment (Governance section) before any code is written.

## Technology Stack & Constraints

### Backend

- **Language:** Rust (stable, MSRV pinned in `rust-toolchain.toml`).
- **HTTP:** `actix-web` v4.
- **DB driver:** `sqlx` (Postgres feature, `runtime-tokio-rustls`).
- **Spatial DB:** PostgreSQL 16 + PostGIS 3.4.
- **OpenAPI:** `utoipa` + `utoipa-swagger-ui`.
- **Auth (Phases 1–2):** local mock JWT, **HS256**, dev-only secret.
- **Auth (Phases 3–4):** internal `identity_domain`, password hashing via `argon2`.
- **Auth (Phase 5+):** Keycloak (OIDC, OAuth2 Authorization Code + PKCE).
- **Async (Phase 3+):** `lapin` for RabbitMQ; worker binary `bornemap-worker`.
- **Object storage (Phase 3+):** MinIO (S3-compatible) via `aws-sdk-s3`. Presigned URLs only.
- **Cache (Phase 4+):** Redis 7 via `redis` crate or `deadpool-redis`.
- **gRPC (Phase 6+):** `tonic`.
- **Logging:** `tracing` + `tracing-subscriber` (JSON formatter).
- **Metrics:** `prometheus` exposition on `/metrics`.
- **Errors:** `thiserror` + custom Actix `ResponseError` impl emitting RFC-7807.

### Frontend (Admin Portal)

- **Stack:** React 18, Vite, TypeScript (`strict: true`).
- **Map engine:** `leaflet` + `react-leaflet` + `react-leaflet-cluster`.
- **Server state:** `@tanstack/react-query` v5.
- **Styling:** Tailwind CSS + `shadcn/ui`.
- **HTTP client:** generated from `openapi.json` via `openapi-typescript-codegen`.
- **Forms:** `react-hook-form` + `zod`.

### Frontend (Mobile App)

- **Stack:** React Native via **Expo SDK (Managed Workflow)** until a documented blocker forces a Dev Client migration.
- **Map engine:** `react-native-maps` (Apple Maps on iOS, Google Maps on Android).
- **Server state:** `@tanstack/react-query` v5.
- **HTTP client:** same generated client as web (shared package `@bornemap/api-client`).

### Shared Frontend

- Monorepo package `@bornemap/geo-models` exports `CoordinateModel`, `StationMarkerModel`, `MapViewportModel`. Coordinate alias is `readonly [lng: number, lat: number]`.

### Tooling & CI

- **Pre-commit:** `cargo fmt`, `cargo clippy -- -D warnings`, ESLint + Prettier, `tsc --noEmit`.
- **CI:** GitHub Actions running:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `SQLX_OFFLINE=true cargo build --workspace`
  4. `cargo test --workspace` (with Testcontainers PostGIS)
  5. `pnpm -r lint && pnpm -r typecheck && pnpm -r test`
  6. OpenAPI breaking-diff check (compare generated `openapi.json` against `main` baseline).
- **Container build:** multi-stage Dockerfile, `gcr.io/distroless/cc` runtime, image ≤ 50 MB per service.
- **Image tag format:** `v[MAJOR].[MINOR].[PATCH]-[GIT_SHA]`.

### Performance Constraints

- 95% of `/api/v1/stations` viewport queries ≤ **200 ms** server-side at the documented data scale (see plan).
- 99% of `/health/ready` responses ≤ **50 ms**.
- Identity sync hook (Phase 5+) ≤ **500 ms**.
- Map sustains **60 FPS** during pan/zoom on the reference mobile device (mid-range Android, 2022).
- Production uptime ≥ **99.5%**. Continuous outage > **3.65 h** in a calendar month is an SLA breach.
- **Degraded operations rule:** if average spatial query latency > **1500 ms** over any rolling 15-minute window, the frontend MUST display a cached static map snapshot and disable live fetches until latency recovers.

### Security & Network Zones

- **Public DMZ:** Frontends only. TLS-terminating reverse proxy.
- **Apps & Runtimes (private):** Actix backend, BFF (Phase 6+), RabbitMQ, workers.
- **Data Storage (isolated):** PostgreSQL + PostGIS, Keycloak, MinIO. No public ingress permitted.
- All inter-zone traffic uses internal DNS and mTLS where supported.
- Admin signup is **invitation-only** (single-use, role-scoped tokens generated by `identity_domain`). Public registration of admin accounts is banned.

## Development Workflow & Quality Gates

### Branching & Speckit Flow

1. Create a feature branch matching `NNN-short-name` (e.g., `001-mvp1-geo-core`).
2. Run `/speckit.specify "..."` to populate `specs/NNN-.../spec.md`.
3. Run `/speckit.clarify` to resolve all `[NEEDS CLARIFICATION]` markers.
4. Run `/speckit.plan` to produce `plan.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`.
5. Run `/speckit.tasks` to produce `tasks.md` (granular, ordered).
6. Run `/speckit.implement` (or hand off `tasks.md` to an implementer LLM).
7. PR runs `/speckit.analyze` for constitutional compliance before merge.

### Constitution Gate (per PR)

Every PR MUST tick:

- [ ] No new spatial column without GiST index.
- [ ] No `unwrap` / `expect` / `panic!` in non-test Rust code.
- [ ] No hand-written HTTP type definitions on frontend.
- [ ] No frontend filter-after-fetch of server collections.
- [ ] No cross-domain repository imports.
- [ ] `cargo clippy -- -D warnings` green.
- [ ] `SQLX_OFFLINE=true cargo build` green; `.sqlx/` committed.
- [ ] OpenAPI snapshot updated; breaking-change report attached if any.
- [ ] If touching map components: R1–R7 verification notes in PR description.
- [ ] If shipping an MVP: real-user validation report attached.

### Documentation Rules

- Specs are authored under `specs/NNN-feature-name/`.
- Architecture decisions that affect more than one MVP are recorded as ADRs under `docs/adr/NNNN-title.md` (created on demand).
- The plan in `docs/plan.md` is the cross-MVP roadmap. Per-MVP detail lives under `specs/`.

## Governance

This constitution supersedes every other practice, style guide, or convention in the repository.

**Amendment procedure:**

1. Open a PR titled `constitution: <change>` modifying both `.specify/memory/constitution.md` and `docs/constitution.md` in the same commit.
2. Include a `Rationale` section in the PR description referencing affected sections by ID.
3. Bump version per SemVer:
   - **MAJOR**: breaking change to a NON-NEGOTIABLE principle or removal of a principle.
   - **MINOR**: new principle, new mandatory rule, or expanded scope.
   - **PATCH**: clarifications, typo fixes, non-semantic edits.
4. Provide a migration note for any in-flight MVP whose scope or acceptance criteria are affected.
5. Require explicit approval from the platform owner of record.
6. Update the footer line (Version / Ratified / Last Amended).

**Compliance:**

- `/speckit.analyze` runs the constitution gate for every PR.
- Any violation must either be fixed before merge or accompanied by a Complexity Tracking entry in the feature's `plan.md` justifying the deviation.

**Version**: 1.0.0 | **Ratified**: 2026-05-23 | **Last Amended**: 2026-05-23
