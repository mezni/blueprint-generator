<!--
  SYNC IMPACT REPORT
  Version change: 1.0.0 → 2.0.0
  Modified principles:
    - I: "Spatial-First, PostGIS-Always" → "Real-Time Telemetry & OCPP Streaming"
    - IV: "Modular Monolith Until Justified" → "Modular Monolith, OCPP Gateway Isolation"
    - VIII: "Map Interaction Runtime Rules" → "Admin Portal Layout & Rendering Contract"
    - IX: "Backend Authority Over Pin Status" → "Backend Authority Over Charger Health & Status"
    - X: "Idempotent Profile Initialization" → folded into "Idempotent State Management" (Principle XI)
    - XII: "Non-Goals (Hard Boundaries)" → completely redefined for EV charging domain
  Added sections:
    - Principle VIII: "Admin Portal Layout & Rendering Contract" (codifies user-supplied Tailwind layout, KPI format, telemetry log spec, color-coded status scheme)
    - Principle IX: "Operational Queue Discipline" (charger lifecycle state machine, remediation queue)
  Removed sections:
    - Principle X: "Idempotent Profile Initialization" (absorbed into Principle XI)
    - "Frontend (Mobile App)" technology subsection (admin-only portal)
    - "Shared Frontend" geo-models subsection
  Templates requiring updates:
    - .specify/templates/plan-template.md → ⚠ pending (Constitution Check section should reference Portal Layout and OCPP telemetry principles)
    - .specify/templates/spec-template.md → ⚠ pending (no changes required — remains technology-agnostic)
    - .specify/templates/tasks-template.md → ⚠ pending (task types should cover telemetry pipeline, OCPP integration, queue operations)
  Follow-up TODOs: N/A
-->

# Amilcar Constitution

> **Canonical Speckit constitution.** Every `/speckit.*` command and every implementer (human or LLM) MUST treat this file as the binding source of truth. A human-readable mirror lives at `docs/constitution.md`. In any conflict, **this file wins**.

## Core Principles

### I. Real-Time Telemetry & OCPP Streaming (NON-NEGOTIABLE)

- The system MUST ingest streaming telemetry from EV chargers via WebSocket/OCPP as a first-class data path.
- The telemetry ingestion pipeline MUST sustain at least 10,000 events/second per gateway, with backpressure handling and a dead-letter queue for failed messages.
- Every telemetry event MUST carry: `station_id`, `event_type`, `timestamp` (ISO 8601), `severity` (info/warn/error/critical), and a structured JSON payload.
- OCPP status codes (e.g., `OCPP_InternalError`, `OCPP_ConnectorLockFailure`, `OCPP_HighTemperature`) MUST be propagated verbatim in telemetry logs and MUST NOT be translated or aliased in transit.
- Event log storage SHALL be append-only with configurable TTL-based retention (minimum 90 days). Audit-trail replay MUST be supported.
- The streaming pipeline SHALL use a message broker (RabbitMQ / Kafka) to decouple ingestion from processing. Synchronous handling of real-time telemetry in HTTP request handlers is banned.

### II. API-First Contract (NON-NEGOTIABLE)

- The OpenAPI 3.1 schema produced by `utoipa` from Rust handlers is the **single** source of truth for the API.
- Frontend HTTP clients are **generated** from `openapi.json` (e.g., `openapi-typescript-codegen`). Hand-written `interface ApiFoo {}` for HTTP payloads is banned.
- CI MUST fail the build on any **breaking** OpenAPI diff against the previous `main` snapshot.

### III. Backend Authority, Frontend Projection (NON-NEGOTIABLE)

- The backend is the sole authority for validation, authorization, business rules, spatial filtering, charger state mutation, and telemetry aggregation.
- The frontend is a pure projection layer. It MAY hold UI state, scroll position, form drafts, and WebSocket subscriptions; it MUST NOT hold business rules or compute charger health from raw telemetry.
- Filtering / structural query logic lives in the Repository Layer only. Frontend "filter-after-fetch" of server data is banned.
- Control flow direction is strictly: **Handlers → Service Layer → Repository Layer**. No reverse calls. No skip-layer calls.

### IV. Spatial-First, PostGIS-Always (NON-NEGOTIABLE)

- The canonical spatial store is **PostgreSQL 16 + PostGIS 3.4**. No alternative spatial engine may be introduced at any phase.
- Every coordinate column is `GEOGRAPHY(Point, 4326)`.
- Every spatial column **MUST** have a **GiST** index. Migrations that add a spatial column without a GiST index fail CI.
- The primary spatial predicate is `ST_DWithin`. `ST_Distance` is allowed **only** for post-filter sorting/ranking on already-bounded result sets.
- All coordinates on wire, in code, and in storage use the array form `[lng, lat]`. Any `{lat, lng}` JSON in code review is rejected.
- Viewport bounding boxes are quantized to **4 decimal places** before being used as cache keys or repository query parameters.
- Charger locations in the map viewport MUST be served via bounded spatial queries; unbounded "fetch all" endpoint is banned.

### V. Modular Monolith, OCPP Gateway Isolation

- Phases 1–5 are a single Rust binary (`amilcar-backend`) compiled from a Cargo workspace.
- Domain modules (`station`, `identity`, `telemetry`, `queue`, `settings`) communicate **only** through public service traits exchanging DTOs. No cross-domain repository imports. No cross-domain schema reads.
- The OCPP gateway adapter SHALL be isolated in its own crate (`amilcar-ocpp-gateway`) with a defined `TelemetryIngest` trait that the monolith depends on via a trait object. Direct OCPP message handling MUST NOT leak into domain logic.
- Microservice extraction (Phase 6) requires a written justification matching at least one of:
  - Sustained CPU or memory saturation that vertical scaling cannot resolve.
  - Per-domain traffic skew ≥ 10× across the monolith.
  - Regulatory, tenancy, or contractual isolation requirement for telemetry data.
- Any Phase 6 PR without that justification document is rejected.

### VI. Test-First with Real Infrastructure in CI (NON-NEGOTIABLE)

- TDD is mandatory for every backend domain change: failing test first, then implementation.
- Integration tests **MUST** run against real PostGIS and a real message broker via **Testcontainers**. Mocking PostGIS or the broker in integration tests is banned.
- Every new endpoint adds at least one contract test (request/response against generated OpenAPI types) and one integration test (real DB, real query).
- Every telemetry pipeline change adds a throughput test verifying ≥ 10,000 events/second per gateway.
- `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` are green before merge.

### VII. Type-Driven Safety (Rust)

- Use the newtype pattern for all domain identifiers: `pub struct StationId(pub Uuid);`, `pub struct ChargerId(pub Uuid);`, `pub struct UserId(pub Uuid);`, etc. Bare `Uuid` in handler or service signatures is rejected.
- All fallible paths return `Result<T, DomainError>`. `unwrap`, `expect`, and `panic!` in non-test code are banned.
- Public errors are serialized as **RFC-7807** `application/problem+json`.
- Telemetry event types MUST be modeled as a sealed `TelemetryEvent` enum; stringly-typed event discrimination is banned.

### VIII. Admin Portal Layout & Rendering Contract (NON-NEGOTIABLE)

The frontend SHALL enforce these layout rules. Lint / unit-test failures here block merge.

- **R1 — Fixed Viewport:** The main container MUST use `flex h-screen w-screen overflow-hidden bg-slate-900`. Scrolling is prohibited at the viewport level.
- **R2 — Persistent Sidebar:** The left sidebar MUST use `w-64 h-full flex-shrink-0 border-r border-slate-850` and remain fixed. Navigation links in the sidebar MUST NOT re-render the surrounding chrome.
- **R3 — Global Header:** Header row MUST contain global search, breadcrumbs (reflecting the current navigation depth), systemic health badges (aggregate online/offline/faulted counts), and the authenticated user's role badge.
- **R4 — Upper Workspace Split:** The workspace wrapper MUST use `flex flex-col flex-1 h-full overflow-hidden`. The upper region MUST use `grid grid-cols-5 gap-4 p-6 h-1/2`:
  - **Map pane** (`col-span-3`): Live spatial/geographic map tracking charging hardware nodes. Pins MUST be color-coded: **Available = green**, **Occupied = amber**, **Faulted = red**. Color values SHALL be `#22c55e` / `#eab308` / `#ef4444`.
  - **Telemetry pane** (`col-span-2`): Real-time streaming event feed consumed via WebSocket. Entries MUST auto-scroll to bottom. A manual pause/resume toggle MUST be provided.
- **R5 — Lower Operational Queue:** The lower region MUST use `flex-1 p-6 overflow-y-auto h-1/2` containing a tabular view of critical device states. Columns: Priority, Station ID, Fault Code, Status, Created At, Assigned To, Actions. Sortable by priority and created_at. Rows MUST be color-coded by severity.
- **R6 — Telemetry Log Format:** Every log row SHALL display: `{ISO-8601 timestamp} | {station_id} | {event_type} | {severity} | {structured JSON payload}`. Severity SHALL be visually demarcated (critical = red badge, error = orange, warn = yellow, info = gray).
- **R7 — KPI Table Format:** Any KPI table in the dashboard MUST contain exactly four columns: **Metric Component**, **Current Live Value**, **MoM Delta %**, **Engineering/Business Impact**. Delta % MUST show direction arrow (↑/↓) and color (green for positive, red for negative).
- **R8 — WebSocket Reconnection:** The telemetry WebSocket client MUST implement exponential backoff (1 s, 2 s, 4 s, 8 s, max 30 s) with jitter. Connection state MUST be reflected in the header health badge area.

### IX. Operational Queue Discipline

- Every charger SHALL have a defined lifecycle state machine: `Available → Occupied → Faulted → Maintenance → Available`. A state transition diagram MUST exist in `docs/operations/charger-lifecycle.md`.
- State transitions MUST be idempotent and logged with a before/after snapshot in the telemetry audit trail.
- The operational queue (lower workspace) is the source of truth for pending remediation actions.
- Each queue item SHALL have: priority (P1–P5), station_id, fault_code (from OCPP or system-defined), created_at (ISO 8601), assigned_to (nullable user ID), status (open/acknowledged/in_progress/resolved/closed).
- Queue items with status `faulted` for > 24 hours without acknowledgement MUST auto-escalate to P1 and notify the on-call engineering contact via the configured alerting channel.

### X. Backend Authority Over Charger Health & Status

- Pin color and health badge reflect **administrative** state (`is_active`, `under_maintenance`, `last_heartbeat_age`) computed by the backend and shipped in the marker DTO.
- Frontend MUST NOT recompute charger health, derive status from raw telemetry, or override the backend-provided color assignment.
- The backend SHALL derive aggregate health from the last N telemetry events per station using a sliding window of 5 minutes. A station with no heartbeat for > 120 seconds SHALL be marked `Faulted`.

### XI. Idempotent State Management

- First-login profile creation (federated or invitation-based) MUST use `INSERT … ON CONFLICT (user_id) DO UPDATE`. No client-side "check then create" flow.
- Charger state transitions MUST use conditional updates (`UPDATE … WHERE current_state = expected_previous_state`) to prevent race conditions in the telemetry pipeline.
- Queue item assignment MUST be idempotent: re-assigning the same user to an already-assigned item SHALL be a no-op.

### XII. Iterative Real-User Validation (NON-NEGOTIABLE)

- Every MVP ends with a **real-user validation cycle**: a defined cohort (≥ 5 admin operators), scripted tasks, quantitative metrics (task success rate, P50/P95 latency observed by users, error rate), and qualitative feedback (≥ 5 interviews per MVP).
- A proceed/kill gate evaluates the next MVP based on the previous MVP's validation report. The proceed gate is documented in `specs/00X-mvpN/spec.md` Success Criteria. If the gate fails, scope adjustments precede the next MVP.
- Telemetry pipeline throughput and queue resolution time MUST be tracked as validation metrics from MVP 1 onward.

### XIII. Non-Goals (Hard Boundaries)

The system **SHALL NOT** include:

- Billing, payments, subscription, or wallet integration (any phase).
- Smart-charging, load balancing, or grid energy optimization algorithms.
- Custom turn-by-turn navigation (navigation is delegated to OS map providers via deep links).
- Driver-facing mobile apps or consumer portal (admin-operators only).
- Real-time charger-to-vehicle communication (PWM signaling, PLC, ISO 15118).
- Distributed event streaming, microservices, or service mesh before Phase 6 is gated open by Principle V's justification rule.
- Hardware firmware management or OTA update orchestration.

Any new feature touching these areas requires a constitution amendment (Governance section) before any code is written.

## Technology Stack & Constraints

### Backend

- **Language:** Rust (stable, MSRV pinned in `rust-toolchain.toml`).
- **HTTP:** `actix-web` v4.
- **DB driver:** `sqlx` (Postgres feature, `runtime-tokio-rustls`).
- **Spatial DB:** PostgreSQL 16 + PostGIS 3.4.
- **Message broker (Phase 3+):** RabbitMQ via `lapin`; worker binary `amilcar-worker`.
- **OpenAPI:** `utoipa` + `utoipa-swagger-ui`.
- **Auth (Phases 1–2):** local mock JWT, **HS256**, dev-only secret.
- **Auth (Phases 3–4):** internal `identity_domain`, password hashing via `argon2`.
- **Auth (Phase 5+):** Keycloak (OIDC, OAuth2 Authorization Code + PKCE).
- **Cache (Phase 4+):** Redis 7 via `redis` crate or `deadpool-redis`.
- **gRPC (Phase 6+):** `tonic`.
- **Logging:** `tracing` + `tracing-subscriber` (JSON formatter).
- **Metrics:** `prometheus` exposition on `/metrics`.
- **Errors:** `thiserror` + custom Actix `ResponseError` impl emitting RFC-7807.

### Frontend (Admin Portal)

- **Stack:** React 18, Vite, TypeScript (`strict: true`).
- **Map engine:** `leaflet` + `react-leaflet` + `react-leaflet-cluster`.
- **Server state:** `@tanstack/react-query` v5.
- **Real-time:** Native `WebSocket` client with exponential-backoff reconnection.
- **Styling:** Tailwind CSS + `shadcn/ui`.
- **HTTP client:** generated from `openapi.json` via `openapi-typescript-codegen`.
- **Forms:** `react-hook-form` + `zod`.

### Tooling & CI

- **Pre-commit:** `cargo fmt`, `cargo clippy -- -D warnings`, ESLint + Prettier, `tsc --noEmit`.
- **CI:** GitHub Actions running:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `SQLX_OFFLINE=true cargo build --workspace`
  4. `cargo test --workspace` (with Testcontainers PostGIS + RabbitMQ)
  5. `pnpm -r lint && pnpm -r typecheck && pnpm -r test`
  6. OpenAPI breaking-diff check (compare generated `openapi.json` against `main` baseline).
- **Container build:** multi-stage Dockerfile, `gcr.io/distroless/cc` runtime, image ≤ 50 MB per service.
- **Image tag format:** `v[MAJOR].[MINOR].[PATCH]-[GIT_SHA]`.

### Performance Constraints

- 95% of `/api/v1/stations` viewport queries ≤ **200 ms** server-side at the documented data scale (see plan).
- Telemetry ingestion pipeline sustains ≥ **10,000 events/second** per gateway instance.
- 99% of `/health/ready` responses ≤ **50 ms**.
- Operational queue query (top 100 items) ≤ **100 ms**.
- WebSocket telemetry push latency ≤ **500 ms** end-to-end from broker receive to client render.
- Map sustains **60 FPS** during pan/zoom.
- Production uptime ≥ **99.5%**. Continuous outage > **3.65 h** in a calendar month is an SLA breach.
- **Degraded operations rule:** if average spatial query latency > **1500 ms** or telemetry throughput drops below **1,000 events/second** over any rolling 15-minute window, the frontend MUST display a cached static map snapshot and disable live fetches until metrics recover.

### Security & Network Zones

- **Public DMZ:** Frontends only. TLS-terminating reverse proxy.
- **Apps & Runtimes (private):** Actix backend, RabbitMQ, workers.
- **Data Storage (isolated):** PostgreSQL + PostGIS, Keycloak, MinIO (Phase 3+). No public ingress permitted.
- **OT Network (isolated):** OCPP gateway adapter receives WebSocket connections from chargers in a separate network segment. No direct path from OT to public DMZ.
- All inter-zone traffic uses internal DNS and mTLS where supported.
- Admin signup is **invitation-only** (single-use, role-scoped tokens generated by `identity_domain`). Public registration of admin accounts is banned.

## Development Workflow & Quality Gates

### Branching & Speckit Flow

1. Create a feature branch matching `NNN-short-name` (e.g., `001-mvp1-telemetry-core`).
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
- [ ] If touching telemetry pipeline: throughput test results attached.
- [ ] If touching map components: R1–R8 verification notes in PR description.
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

**Version**: 2.0.0 | **Ratified**: 2026-05-23 | **Last Amended**: 2026-05-24
