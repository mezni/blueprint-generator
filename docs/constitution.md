# Amilcar — Constitution (Human Mirror)

**Version:** 2.0.0
**Ratified:** 2026-05-23
**Last Amended:** 2026-05-24
**Canonical file:** [`.specify/memory/constitution.md`](../.specify/memory/constitution.md) — that file is what Speckit reads. This document mirrors it for human review. In any conflict, the canonical file wins.

---

## 0. How To Read This

This is a binding rulebook for everyone working on Amilcar, including LLM implementers. Each rule is written to be **directly testable**: a reviewer (or `/speckit.analyze`) can mechanically verify compliance. If a rule sounds aspirational, it is broken and must be tightened.

---

## 1. Mission

Amilcar is an EV Charging Admin Portal for monitoring, managing, and remediating charging infrastructure. Operators view live charger status on a spatial map, consume real-time OCPP/WebSocket telemetry, and manage operational queues for fault remediation. The product is optimized for **full-screen non-scrollable dashboards**, **high-throughput telemetry pipelines**, and **iterative validation with real admin operators**.

The core data paths are OCPP streaming telemetry and PostGIS spatial queries — across every phase, forever.

---

## 2. Non-Goals (Hard Boundaries)

The system **shall not** include:

| # | Excluded capability |
| --- | --- |
| NG-1 | Billing, payments, or wallet integration |
| NG-2 | Smart-charging or grid energy optimization algorithms |
| NG-3 | Custom turn-by-turn navigation (delegated to OS map providers) |
| NG-4 | Driver-facing mobile apps or consumer portal (admin-operators only) |
| NG-5 | Real-time charger-to-vehicle communication (PWM, PLC, ISO 15118) |
| NG-6 | Distributed event streaming or microservices before Phase 6 is justified |
| NG-7 | Hardware firmware management or OTA update orchestration |

Any feature touching NG-1..NG-7 requires a constitutional amendment **before** code is written.

---

## 3. Core Principles

### I. Real-Time Telemetry & OCPP Streaming (NON-NEGOTIABLE)

- System MUST ingest streaming telemetry from EV chargers via WebSocket/OCPP as a first-class data path.
- Pipeline MUST sustain ≥ 10,000 events/second per gateway with backpressure and dead-letter queue.
- Every event carries: `station_id`, `event_type`, `timestamp` (ISO 8601), `severity` (info/warn/error/critical), structured JSON payload.
- OCPP status codes (e.g., `OCPP_InternalError`) MUST propagate verbatim — no translation or aliasing.
- Event storage is append-only with TTL-based retention (≥ 90 days). Audit replay MUST be supported.
- Pipeline uses a message broker (RabbitMQ / Kafka) for decoupling. Synchronous telemetry handling in HTTP handlers is banned.

### II. API-First Contract (NON-NEGOTIABLE)

- `utoipa` emits OpenAPI 3.1 from the Rust handlers. That JSON is the single source of API truth.
- Frontend HTTP types are **generated** (`openapi-typescript-codegen` or equivalent). Hand-rolled interfaces for HTTP payloads are banned.
- CI fails on a breaking OpenAPI diff.

### III. Backend Authority, Frontend Projection (NON-NEGOTIABLE)

- All validation, authorization, business rules, spatial filtering, charger state mutation, and telemetry aggregation live on the backend.
- The frontend holds UI state, scroll position, form drafts, and WebSocket subscriptions only — never business rules.
- Server collections must not be filtered client-side after fetch.
- Layer flow is strict: **Handlers → Services → Repositories**.

### IV. Spatial-First, PostGIS-Always (NON-NEGOTIABLE)

- Database: PostgreSQL 16 + PostGIS 3.4 — no substitutes.
- Spatial column type: `GEOGRAPHY(Point, 4326)` — no exceptions.
- Index: every spatial column has a GiST index. Migrations without one **fail CI**.
- Primary predicate: `ST_DWithin`. `ST_Distance` is allowed only as a secondary, post-filter sort.
- Coordinate format on the wire and in code: `[lng, lat]`. `{lat, lng}` JSON is rejected in review.
- Viewport bounding boxes are quantized to **4 decimal places** before cache or query use.
- Charger locations in map viewport MUST be served via bounded spatial queries. "Fetch all" unbounded endpoint is banned.

### V. Modular Monolith, OCPP Gateway Isolation

- Phases 1–5: one Rust binary, Cargo workspace, isolated domain modules.
- Domain modules (`station`, `identity`, `telemetry`, `queue`, `settings`) communicate **only** through public service traits passing DTOs.
- OCPP gateway adapter is isolated in `amilcar-ocpp-gateway` crate with a `TelemetryIngest` trait.
- Microservice extraction (Phase 6) requires a written justification matching one of:
  - sustained CPU/memory saturation,
  - per-domain traffic skew ≥ 10×,
  - regulatory / tenancy / contractual isolation requirement for telemetry data.

### VI. Test-First with Real Infrastructure in CI (NON-NEGOTIABLE)

- TDD: failing test, then code.
- Integration tests run against real PostGIS + message broker via **Testcontainers** — mocks are banned in integration tests.
- Every endpoint ships with a contract test and an integration test.
- Every telemetry pipeline change adds a throughput test verifying ≥ 10,000 events/second per gateway.

### VII. Type-Driven Safety (Rust)

- Newtype IDs (`StationId`, `UserId`, `ChargerId`). Bare `Uuid` in handler/service signatures is rejected.
- All fallible paths return `Result<T, DomainError>`.
- `unwrap` / `expect` / `panic!` are banned outside tests.
- Errors are serialized as **RFC-7807** `application/problem+json`.
- Telemetry event types modeled as sealed `TelemetryEvent` enum — stringly-typed discrimination is banned.

### VIII. Admin Portal Layout & Rendering Contract (NON-NEGOTIABLE)

The frontend enforces these layout rules. Violations block merge.

| # | Rule | Implementation |
| --- | --- | --- |
| **R1** | **Fixed Viewport.** Main container: `flex h-screen w-screen overflow-hidden bg-slate-900`. No viewport-level scrolling. | CSS audit |
| **R2** | **Persistent Sidebar.** `w-64 h-full flex-shrink-0 border-r border-slate-850`. Navigation does not re-render chrome. | Snapshot test |
| **R3** | **Global Header.** Contains global search, breadcrumbs, systemic health badges, user role badge. | Component test |
| **R4** | **Upper Workspace Split.** `grid grid-cols-5 gap-4 p-6 h-1/2`. Map `col-span-3`, Telemetry `col-span-2`. Pins: Available=`#22c55e`, Occupied=`#eab308`, Faulted=`#ef4444`. | Layout test + color audit |
| **R5** | **Lower Operational Queue.** `flex-1 p-6 overflow-y-auto h-1/2`. Columns: Priority, Station ID, Fault Code, Status, Created At, Assigned To, Actions. Sortable. | Grid snapshot |
| **R6** | **Telemetry Log Format.** `{timestamp} \| {station_id} \| {event_type} \| {severity} \| {payload}`. Severity badge: critical=red, error=orange, warn=yellow, info=gray. | Format validation test |
| **R7** | **KPI Table Format.** Exactly 4 columns: Metric Component, Current Live Value, MoM Delta %, Engineering/Business Impact. Delta % shows direction arrow (↑/↓) + color. | Schema validation |
| **R8** | **WebSocket Reconnection.** Exponential backoff (1 s → 30 s max) with jitter. Connection state reflected in header badges. | Integration test |

### IX. Operational Queue Discipline

- Charger lifecycle state machine: `Available → Occupied → Faulted → Maintenance → Available`. State diagram at `docs/operations/charger-lifecycle.md`.
- State transitions are idempotent, logged with before/after snapshot in telemetry audit trail.
- Queue items carry: priority (P1–P5), station_id, fault_code, created_at (ISO 8601), assigned_to, status (open/acknowledged/in_progress/resolved/closed).
- Items faulted > 24 h without acknowledgement auto-escalate to P1 with on-call alert.

### X. Backend Authority Over Charger Health & Status

- Pin color and badge reflect backend-computed `is_active`, `under_maintenance`, `last_heartbeat_age`.
- Frontend never overrides charger status or derives health from raw telemetry.
- Backend derives aggregate health from 5-minute sliding window. No heartbeat > 120 s → `Faulted`.

### XI. Idempotent State Management

- First-login profile creation uses `INSERT … ON CONFLICT (user_id) DO UPDATE`. No "check-then-create".
- Charger state transitions use `UPDATE … WHERE current_state = expected_previous_state`.
- Queue item re-assignment to same user is a no-op.

### XII. Iterative Real-User Validation (NON-NEGOTIABLE)

- Every MVP ends with a real-user validation cycle: defined cohort (≥ 5 operators), scripted tasks, quantitative metrics (task success rate, P50/P95 latency, error rate), ≥ 5 qualitative interviews.
- A proceed/kill gate (documented in the MVP's `spec.md` Success Criteria) decides whether the next MVP starts.
- Telemetry pipeline throughput and queue resolution time tracked as validation metrics from MVP 1.

### XIII. Non-Goals (See §2)

Listed above as NG-1..NG-7. Amendments required before crossing those lines.

---

## 4. Technology Stack

### Backend

| Layer | Technology |
| --- | --- |
| Language | Rust (stable, MSRV pinned) |
| HTTP | actix-web v4 |
| DB driver | sqlx (Postgres, `runtime-tokio-rustls`) |
| Spatial DB | PostgreSQL 16 + PostGIS 3.4 |
| Message broker | RabbitMQ via `lapin` (Phase 3+) |
| OpenAPI | utoipa + utoipa-swagger-ui |
| Cache | Redis 7 (Phase 4+) |
| Auth (Phases 1–2) | Mock JWT HS256 |
| Auth (Phases 3–4) | Internal identity_domain, argon2 |
| Auth (Phase 5+) | Keycloak (OIDC, OAuth2 PKCE) |
| Logging | tracing + tracing-subscriber (JSON) |
| Metrics | Prometheus on `/metrics` |
| Errors | thiserror, RFC-7807 problem+json |

### Frontend (Admin Portal)

| Layer | Technology |
| --- | --- |
| Stack | React 18, Vite, TypeScript strict |
| Map engine | Leaflet + react-leaflet + react-leaflet-cluster |
| Server state | @tanstack/react-query v5 |
| Real-time | Native WebSocket client, exponential-backoff reconnection |
| Styling | Tailwind CSS + shadcn/ui |
| HTTP client | Generated from openapi.json |
| Forms | react-hook-form + zod |

### Tooling & CI

- Pre-commit: cargo fmt, clippy, ESLint + Prettier, `tsc --noEmit`.
- CI pipeline: fmt → clippy → `SQLX_OFFLINE=true cargo build` → cargo test (Testcontainers) → pnpm lint/typecheck/test → OpenAPI diff check → Docker build (distroless/cc, ≤ 50 MB) → tag `vMAJ.MIN.PATCH-GIT_SHA`.

### Performance Constraints

| Target | Value |
| --- | --- |
| Spatial query latency (P95) | ≤ 200 ms |
| Telemetry ingestion | ≥ 10,000 events/s per gateway |
| Health check (P99) | ≤ 50 ms |
| Queue query (top 100) | ≤ 100 ms |
| WebSocket push latency | ≤ 500 ms end-to-end |
| Map rendering | ≥ 60 FPS |
| Availability | ≥ 99.5% monthly uptime |
| Continuous outage budget | ≤ 3.65 h / month |
| Degraded-ops fallback | Static map when spatial latency > 1500 ms or throughput < 1000 events/s over 15-minute window |

---

## 5. Security & Network Zones

```
PUBLIC DMZ            APPS & RUNTIMES (PRIVATE)         DATA STORAGE (ISOLATED)
─────────────         ──────────────────────────        ────────────────────────
Admin Portal          Actix backend / RabbitMQ           PostgreSQL + PostGIS
                      Workers / OCPP gateway             Keycloak (Phase 5+)
                                                         MinIO (Phase 3+)

OT NETWORK (ISOLATED):
OCPP gateway adapter receives WebSocket connections from chargers
```

- Frontends are the only zone reachable from the public internet.
- No public path reaches the data-storage zone. OCPP gateway in separate OT network segment.
- Admin signup is invitation-only. Single-use, role-scoped tokens.
- All inter-zone traffic uses internal DNS + mTLS where supported.

---

## 6. Observability

- **Logs:** `tracing` JSON output with `trace_id` per request lifecycle.
- **Metrics:** Prometheus on `/metrics` (DB pool, memory, HTTP rate, query latency, telemetry throughput, queue depth).
- **Health:** `/health/live` (cheap) and `/health/ready` (verifies DB + broker + OCPP gateway connectivity).
- **Distributed tracing:** OpenTelemetry from Phase 6 onward.

---

## 7. Governance

This constitution supersedes all other practices.

**Amendment process:**

1. Open a PR titled `constitution: <change>` modifying **both** `.specify/memory/constitution.md` and `docs/constitution.md`.
2. Include a Rationale section referencing affected sections by ID.
3. Bump version: MAJOR (breaking NON-NEGOTIABLE), MINOR (additive), PATCH (clarification).
4. Provide a migration note for any in-flight MVP whose scope is affected.
5. Require explicit approval from the platform owner of record.

**Compliance:**

- `/speckit.analyze` runs the constitution gate per PR.
- Violations require either a fix before merge or a Complexity Tracking entry in the feature's `plan.md`.

---

## 8. Constitution Gate (per-PR checklist)

- [ ] No spatial column added without GiST index.
- [ ] No `unwrap` / `expect` / `panic!` in non-test Rust.
- [ ] No hand-written HTTP type definitions on the frontend.
- [ ] No frontend filter-after-fetch of server collections.
- [ ] No cross-domain repository imports.
- [ ] `cargo clippy -- -D warnings` green.
- [ ] `.sqlx/` committed when SQL changed.
- [ ] OpenAPI snapshot updated; breaking-change report attached if any.
- [ ] If touching telemetry pipeline: throughput test results attached.
- [ ] If touching map components: R1–R8 verification notes attached.
- [ ] If shipping an MVP: real-user validation report attached.
