# BorneMap — Constitution (Human Mirror)

**Version:** 1.0.0
**Ratified:** 2026-05-23
**Canonical file:** [`.specify/memory/constitution.md`](../.specify/memory/constitution.md) — that file is what Speckit reads. This document mirrors it for human review. In any conflict, the canonical file wins.

---

## 0. How To Read This

This is a binding rulebook for everyone working on BorneMap, including LLM implementers. Each rule is written to be **directly testable**: a reviewer (or `/speckit.analyze`) can mechanically verify compliance. If a rule sounds aspirational, it is broken and must be tightened.

---

## 1. Mission

BorneMap is a geospatial EV-charging discovery platform for Tunisia. Drivers find chargers on an interactive map and leave reviews; administrators manage station data and monitor usage. The product is optimized for **viewport-driven map exploration**, **fast mobile interactions**, and **iterative validation with real users**.

The core spatial engine is PostGIS — across every phase, forever.

---

## 2. Non-Goals (Hard Boundaries)

The system **shall not** include:

| # | Excluded capability |
| --- | --- |
| NG-1 | EV charging session control (OCPP / OPP / hardware signaling) |
| NG-2 | Billing, payments, or wallets |
| NG-3 | Smart-charging or grid energy optimization |
| NG-4 | Real-time hardware telemetry from chargers |
| NG-5 | Custom routing / turn-by-turn navigation (delegated to OS map providers) |
| NG-6 | Distributed event streaming or microservices before Phase 6 is justified |
| NG-7 | Infrastructure scale-out before real-world user validation demands it |

Any feature touching NG-1..NG-7 requires a constitutional amendment **before** code is written.

---

## 3. Core Principles

### I. Spatial-First, PostGIS-Always (NON-NEGOTIABLE)

- Database: PostgreSQL 16 + PostGIS 3.4 — no substitutes.
- Spatial column type: `GEOGRAPHY(Point, 4326)` — no exceptions.
- Index: every spatial column has a GiST index. Migrations without one **fail CI**.
- Primary predicate: `ST_DWithin`. `ST_Distance` is allowed only as a secondary, post-filter sort.
- Coordinate format on the wire and in code: `[lng, lat]`. `{lat, lng}` JSON is rejected in review.
- Viewport bounding boxes are quantized to **4 decimal places** before cache or query use.

### II. Backend Authority, Frontend Projection (NON-NEGOTIABLE)

- All validation, authorization, business rules, and filtering live on the backend.
- The frontend holds UI state and form drafts only — never business rules.
- Server collections must not be filtered client-side after fetch.
- Layer flow is strict: **Handlers → Services → Repositories**.

### III. API-First Contract (NON-NEGOTIABLE)

- `utoipa` emits OpenAPI 3.1 from the Rust handlers. That JSON is the single source of API truth.
- Frontend HTTP types are **generated** (`openapi-typescript-codegen` or equivalent). Hand-rolled interfaces for HTTP payloads are banned.
- CI fails on a breaking OpenAPI diff.

### IV. Modular Monolith Until Justified

- Phases 1–5: one Rust binary, Cargo workspace, isolated domain modules.
- Domain modules communicate **only** through public service traits passing DTOs.
- Microservice extraction (Phase 6) requires a written justification matching one of:
  - sustained CPU/memory saturation,
  - per-domain traffic skew ≥ 10×,
  - regulatory / tenancy / contractual isolation requirement.

### V. Test-First with PostGIS in CI (NON-NEGOTIABLE)

- TDD: failing test, then code.
- Integration tests run against real PostGIS via **Testcontainers** — spatial mocks are banned.
- Every endpoint ships with a contract test and an integration test.

### VI. SQLx Compile-Time Verified

- All SQL goes through `sqlx::query!` / `query_as!`. No string concatenation.
- `.sqlx/` query metadata is committed; CI runs `SQLX_OFFLINE=true`.

### VII. Type-Driven Safety (Rust)

- Newtype IDs (`StationId`, `UserId`, etc.). Bare `Uuid` in handler/service signatures is rejected.
- All fallible paths return `Result<T, DomainError>`.
- `unwrap` / `expect` / `panic!` are banned outside tests.
- Errors are serialized as **RFC-7807** `application/problem+json`.

### VIII. Map Interaction Runtime Rules (CI-Blocking)

The dedicated frontend module `MapInteractionDomain` enforces these rules. Violations block merge.

| # | Rule | Mechanical check |
| --- | --- | --- |
| **R1** | **Viewport Debounce.** Fetches fire only after the map is settled ≥ 300 ms; bind to `moveend`/`zoomend`. | Unit test: pan storm → ≤ 1 fetch per 300 ms. |
| **R2** | **Query Quantization.** Bounding-box coordinates are rounded to 4 decimal places before cache keys / network. | `quantizeBounds()` unit test; key regex `-?\d+\.\d{4}`. |
| **R3** | **Marker Virtualization.** Off-viewport markers are filtered out before render. | Snapshot test: off-bounds coords absent from rendered tree. |
| **R4** | **Gesture Priority.** RN `<Marker tracksViewChanges={false}>` for static markers; web markers do not re-render on pan. | ESLint rule + perf trace. |
| **R5** | **Lazy Hydration.** Marker payloads are summary-only; details fetch lazily. Filter pills mutate local state only when current viewport is cached. | Network panel: filter click → 0 requests. |
| **R6** | **Bounds Cache Registry.** React Query keys: `['stations', quantizedBbox]`; cached within `staleTime` (60 s default) reused. | Devtools: repeated viewports = cache hits. |
| **R7** | **Cluster Threshold.** Clustering activates at > 15 markers within a 40 px radius. | Visual test at synthetic density. |

### IX. Status Projection Rule

- Pin color reflects backend-computed `is_active` + `under_maintenance` only.
- Frontends never override pin status from any other source.
- Real-time hardware telemetry remains a non-goal (NG-4).

### X. Idempotent Profile Initialization

- First-login profile creation uses `INSERT … ON CONFLICT (user_id) DO UPDATE`. No "check-then-create" anywhere.

### XI. Iterative Real-User Validation (NON-NEGOTIABLE)

- Every MVP ends with a real-user validation cycle: defined cohort, scripted tasks, quantitative metrics, ≥ 5 qualitative interviews.
- A proceed/kill gate (documented in the MVP's `spec.md` Success Criteria) decides whether the next MVP starts.

### XII. Non-Goals (See §2)

Listed above as NG-1..NG-7. Amendments required before crossing those lines.

---

## 4. Domain Model

```text
Company ───► Station ───► Charger
                │
                └───► Review (Driver)

Identity ───► User Lifecycle (Invitations Portal)
                  │
                  ▼
         Driver Profile Metadata
                  │
                  ▼
            MinIO / S3 Storage
```

| Domain | Owns | Does not own |
| --- | --- | --- |
| `station_domain` | Companies, stations, chargers, spatial indexing | User identity, ratings text |
| `identity_domain` | Auth state, sessions, invitations, federation | Display preferences, avatars |
| `profile_domain` | Avatars, display metadata, localization, preferences | Authentication state |
| `review_domain` | Ratings, feedback, interaction matrices | Station metadata |
| `event_domain` | Append-only telemetry log | Business decisions |
| `settings_domain` | Lookup tables, feature flags | All of the above |

---

## 5. Authentication Phases

| Phase | Mechanism | Notes |
| --- | --- | --- |
| 1–2 | Mock JWT (HS256, dev secret) | Claim shape **frozen** from day 1: `sub`, `preferred_username`, `realm_access.roles`, `iat`, `exp`. |
| 3–4 | Internal `identity_domain` + argon2 password hashing | RBAC server-side; invitation-only admin signup. |
| 5+ | Keycloak (OIDC, OAuth2 Authorization Code + PKCE) | Tokens proxied through BFF; social federation optional. |

The claim shape is **immutable** across all phases — moving to Keycloak must not require a downstream code change outside the identity layer.

### Administrative invitation flow

1. Admin enters email + role in the `InviteCollaborators` modal.
2. Frontend `POST /admin/invitations` (BFF gateway).
3. `identity_domain` generates a single-use, role-scoped, signed token bound to email + expiry.
4. Mail transport delivers the link.
5. Invitee opens `/signup?token=…`; backend validates token, runs idempotent profile insert (Principle X), provisions session.

---

## 6. Repository Structure

```text
bornemap/
├── .github/workflows/        # CI pipelines
├── .specify/                 # Speckit canonical files
├── backend/                  # Cargo workspace
│   ├── Cargo.toml
│   ├── api-gateway/          # BFF (Phase 6+)
│   ├── libs/
│   │   ├── common-utils/
│   │   └── openapi-spec/     # utoipa harness
│   └── services/
│       ├── station-service/
│       ├── identity-service/
│       ├── profile-service/
│       ├── review-service/
│       └── event-service/
├── frontend/
│   ├── admin-portal/         # React + Vite
│   ├── mobile-app/           # React Native + Expo
│   └── packages/
│       ├── api-client/       # generated OpenAPI client
│       └── geo-models/       # CoordinateModel et al.
├── infrastructure/
│   ├── docker-compose.local.yml
│   └── terraform/
├── docs/                     # Human-readable docs
└── specs/                    # Per-feature Speckit specs
```

---

## 7. CI/CD Contract

Pipeline stages (all must pass):

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `SQLX_OFFLINE=true cargo build --workspace`
4. `cargo test --workspace` (Testcontainers PostGIS)
5. `pnpm -r lint && pnpm -r typecheck && pnpm -r test`
6. OpenAPI breaking-diff check
7. Multi-stage Docker build; image ≤ 50 MB per service
8. Tag images `v[MAJOR].[MINOR].[PATCH]-[GIT_SHA]`

---

## 8. Deployment Topology

Three logical zones:

```text
PUBLIC DMZ            APPS & RUNTIMES (PRIVATE)         DATA STORAGE (ISOLATED)
─────────────         ──────────────────────────        ────────────────────────
Admin Portal          Actix backend / BFF                Keycloak (Phase 5+)
Mobile clients        RabbitMQ + workers (Phase 3+)      MinIO (Phase 3+)
                                                         PostgreSQL + PostGIS
```

- Frontends are the only zone reachable from the public internet.
- No public path reaches the data-storage zone. Keycloak admin endpoints are private-only.

---

## 9. Observability

- **Logs:** `tracing` JSON output with `trace_id` per request lifecycle.
- **Metrics:** Prometheus exposition on `/metrics` (DB pool, memory, HTTP rate, query latency histograms).
- **Health:** `/health/live` (cheap) and `/health/ready` (verifies DB + broker).
- **Distributed tracing:** OpenTelemetry from Phase 6 onward.

---

## 10. Performance & SLA

| Target | Value |
| --- | --- |
| Spatial query latency (server) | P95 ≤ 200 ms |
| Identity sync hook | ≤ 500 ms |
| Map rendering | ≥ 60 FPS on reference mid-range Android |
| Availability | ≥ 99.5% monthly uptime |
| Continuous outage budget | ≤ 3.65 h / month |
| Degraded-ops fallback | Static map snapshot when avg spatial latency > 1500 ms over any 15-minute window |

---

## 11. Governance

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

## 12. Constitution Gate (per-PR checklist)

- [ ] No spatial column added without GiST index.
- [ ] No `unwrap` / `expect` / `panic!` in non-test Rust.
- [ ] No hand-written HTTP type definitions on the frontend.
- [ ] No frontend filter-after-fetch of server collections.
- [ ] No cross-domain repository imports.
- [ ] `cargo clippy -- -D warnings` green.
- [ ] `.sqlx/` committed when SQL changed.
- [ ] OpenAPI snapshot updated; breaking-change report attached if any.
- [ ] Map components: R1–R7 verification notes attached.
- [ ] If shipping an MVP: real-user validation report attached.
