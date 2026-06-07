# Bornemap — Detailed Execution Plan (LLM-Executable)

> **Audience**: a coding LLM (including smaller/cheaper models) that will implement
> the Bornemap platform sprint-by-sprint. This file is **self-contained**: it
> restates every rule, schema, contract, and command needed so you do **not** have to
> infer anything. When in doubt, follow this file literally. The authority order is:
> `.specify/memory/constitution.md` > this file > other `docs/*.md`.

---

## 0. How To Use This Document

**Rules for the implementing LLM (read first, follow always):**

1. **Do exactly what each step says.** Do not invent features, endpoints, fields,
   roles, or services that are not listed here.
2. **One sprint at a time.** Finish a sprint's Exit Criteria before starting the next.
   Sprints are dependency-gated (see §2).
3. **Never break a NON-NEGOTIABLE rule** (listed in §1). If a task seems to require
   breaking one, stop and re-read; the rule wins.
4. **Always run the verification commands** at the end of each step. If they fail,
   fix before proceeding. Never mark a task done on intent alone.
5. **Use the exact names** in §3 (directories, services, schemas, IDs, env vars).
6. **Commit** after each logical task group using Conventional Commits
   (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`). Never commit secrets,
   `node_modules`, or build artifacts.
7. **Ask nothing of the user mid-sprint** unless a NON-NEGOTIABLE conflict appears.

**Definition of Done (every feature):** unit tests exist, integration coverage
exists, authorization tested, partner isolation tested, data correctness validated,
soft-delete behavior tested, performance impact understood, security reviewed.

---

## 1. NON-NEGOTIABLE Rules (Memorize These)

These come from the Constitution and apply to **every** sprint.

### 1.1 Identity & Roles
- **Keycloak is the ONLY authentication system.** The platform NEVER stores
  passwords, sessions, or replicates identity state.
- **Exactly 3 roles**: `registered_driver`, `partner`, `admin`. No others, ever.
- **Public users** are anonymous: not stored in DB, not Keycloak users, not a role.
- **Identity bridge (the ONLY one)**:
  `platform_db.users.user_account.keycloak_user_id = JWT.sub`.

### 1.2 Authorization
- Enforce auth at **backend (primary)** AND **repository/data-access layer
  (mandatory)**. Frontend auth is **UX only, never secure**.
- **`partner_id` is NEVER accepted from the client.** It is derived only from
  `users.partner_membership.partner_id`.
- A partner can NEVER read/modify/infer another partner's data.
- A user belongs to **at most ONE** partner.

### 1.3 Data
- **Source of truth**: `platform_db` for business data. GIS is **derived, never
  authoritative**. Analytics never affects business logic.
- **Soft delete only** for `station`, `partner`, `review`. NEVER hard delete in prod.
- **Station visibility** = `is_live = true AND deleted_at IS NULL AND status =
  'active' AND is_public = true`.
- **ID strategy** = ULID + prefix: `USR-`, `PRT-`, `STN-`, `CHG-`, `REV-`, `EVT-`,
  events use `CLK-` envelope id.
- **Audit fields** on all mutable entities: `created_at`, `updated_at`, `created_by`,
  `updated_by`, `deleted_at`.

### 1.4 APIs
- **Pure REST only.** No GraphQL, no RPC, no BFF. URL versioning only (`/api/v1`).
- **Standard envelopes** (always):
  - Success: `{ "success": true, "data": {}, "meta": {} }`
  - Error: `{ "success": false, "error": { "code": "STRING", "message": "STRING", "details": {} } }`
- All list endpoints **MUST paginate**. No full-table scans.

### 1.5 Events / GIS
- **RabbitMQ** is the event backbone. **At-least-once** delivery; consumers MUST
  deduplicate (`event_id` for analytics, entity id for GIS).
- All GIS + event processing MUST be **idempotent and replay-safe**.
- GIS uses the **outbox pattern**: DB mutation → outbox row → queue → worker.
- Analytics events are **immutable, append-only, no PII**, never used for auth/business.
- Required emitters: station created/updated/deleted, review created/updated/deleted,
  availability changed.

### 1.6 Deployment
- **Only Traefik** (and Keycloak auth endpoint) is publicly exposed. Everything else
  internal-only, no host port exposure.
- **Docker Compose** on bare metal/VM. No Kubernetes, no service mesh, no registry
  dependency. Migrations run **before** service startup, never auto at runtime.
- **Env vars only**, fail-fast on invalid/missing config, secrets never in Git
  (only `.env.example` committed).

---

## 2. Sprint Map & Dependency Graph

Sprint length = 1 week, solo developer, strict dependency gating.

```
0 → 1 → 2 → 3 → 4
            ↓
            5 → 6 → 7
                    ↓
          8 → 9 → 10 → 11 → 12
                            ↓
                        13 → 14 → 15
                                ↓
                               16
```

| Sprint | Name | Output |
|--------|------|--------|
| 0 | Architecture Freeze | docs frozen, repo initialized |
| 1 | Monorepo + Tooling | all apps/crates compile, empty shells |
| 2 | Runtime Infra (Compose) | `docker compose up` works, `/health` ok |
| 3 | Identity & Auth | JWT validated, RBAC enforced |
| 4 | Core DB Schema | migrations run, indexes, relationships |
| 5 | Admin Service MVP | partner/station/charger CRUD + GIS outbox |
| 6 | GIS Sync v1 | station→geometry sync, idempotent |
| 7 | Driver Service MVP | discovery, favorites, reviews, profile |
| 8 | Design System | tokens, shadcn/ui, RTL-ready |
| 9 | Driver Web App | full driver journey |
| 10 | Partner Dashboard | station/charger/availability mgmt |
| 11 | Admin Dashboard | global control, moderation |
| 12 | Mobile App (Expo) | parity with driver web core |
| 13 | Clickstream System | events frontend→DB, dedup, validation |
| 14 | Analytics Writer | raw_event ingestion, partitioning, aggregation |
| 15 | Reporting Layer | partner + admin KPIs |
| 16 | Hardening | load test, RBAC audit, RTL/WCAG, rollback |

---

## 3. Canonical Names (Use These Exactly)

### 3.1 Monorepo Layout
```
bornemap/
  apps/                       # frontend applications
    driver-web/               # React + Vite
    partner-dashboard/        # React + Vite
    admin-dashboard/          # React + Vite
    driver-mobile/            # React Native Expo
  services/                   # backend Rust service crates (binaries)
    driver-service/
    admin-service/
    clickstream-service/
    gis-worker/
    analytics-writer/
  crates/                     # shared Rust library crates (no binaries)
    common-types/
    common-errors/
    common-auth/
    common-db/
  packages/                   # shared TypeScript packages
    shared-types/
    api-client/
    auth-client/
    design-tokens/
    event-taxonomy/
    api-contracts/
  infra/                      # docker compose, traefik, env templates
    compose/
    traefik/
    env/
  docs/                       # this folder
```
**Naming**: everything kebab-case. Rust Cargo package name = directory name.
npm package name = directory name. Commit conventionally.

### 3.2 Services & Responsibilities
| Service | Responsibility | Public? |
|---------|----------------|---------|
| `driver-service` | station discovery (read) + user actions (favorites, reviews, profile); public + authenticated | via Traefik `/api/v1/driver/*` |
| `admin-service` | partner/station/charger CRUD, moderation, reporting; hosts `/api/v1/admin/*` and `/api/v1/partner/*` | via Traefik |
| `clickstream-service` | event ingestion (HTTP in), validation, publish to RabbitMQ | internal only |
| `gis-worker` | outbox consumption, station geometry sync to GIS layer | internal only |
| `analytics-writer` | consume events from RabbitMQ, write/aggregate to `analytics_db` | internal only |

### 3.3 Databases
- `keycloak_db` — identity only (Keycloak-owned, no business data).
- `platform_db` — business + GIS. Schemas: `inventory`, `users`, `gis`. PostGIS on.
- `analytics_db` — schema `analytics`. Partitioned events.

---

## 4. Data Model (platform_db) — Exact Schemas

> All mutable tables include audit fields: `created_at`, `updated_at`, `created_by`,
> `updated_by`, `deleted_at` (nullable). IDs are `TEXT` ULID with prefix.

### 4.1 schema `inventory`

**`inventory.partner`**
| column | type | notes |
|--------|------|-------|
| id | TEXT PK | `PRT-<ULID>` |
| name | TEXT | required |
| type | TEXT | `business` \| `private` |
| status | TEXT | `active` \| `suspended` |
| created_at / updated_at / created_by / updated_by | — | audit |
| deleted_at | TIMESTAMPTZ NULL | soft delete |

Indexes: `BTREE(id)`, `BTREE(status)`.
Rules: can exist without stations; soft delete only; **delete blocked if active
stations exist** (`ACTIVE_STATIONS_EXIST`).

**`inventory.station`** (CORE)
| column | type | notes |
|--------|------|-------|
| id | TEXT PK | `STN-<ULID>` |
| partner_id | TEXT FK→`inventory.partner.id` | required, owner |
| name | TEXT | |
| description | TEXT | |
| latitude | DOUBLE PRECISION | |
| longitude | DOUBLE PRECISION | |
| geom | GEOGRAPHY(Point,4326) | `ST_SetSRID(ST_MakePoint(lng,lat),4326)` |
| status | TEXT | `active` \| `inactive` \| `maintenance` (also `draft` in lifecycle) |
| is_live | BOOLEAN | |
| is_public | BOOLEAN | |
| city | TEXT | |
| country | TEXT | |
| audit + deleted_at | — | |

Constraints: partner ownership enforced; `geom` MUST match lat/lng.
Indexes: `GIST(geom)`, `BTREE(partner_id)`, `BTREE(status)`,
`BTREE(is_live, is_public)`, `BTREE(city)`.
Lifecycle: `draft → active → inactive → deleted`.
Every change → GIS outbox event + analytics event + cache invalidation.

**`inventory.charger`**
| column | type | notes |
|--------|------|-------|
| id | TEXT PK | `CHG-<ULID>` |
| station_id | TEXT FK→station | required; inherits partner via station |
| type | TEXT | `CCS` \| `Type2` \| `CHAdeMO` |
| power_kw | NUMERIC | |
| status | TEXT | `available` \| `offline` \| `fault` |
| audit + deleted_at | — | |

Indexes: `BTREE(station_id)`, `BTREE(status)`.

**`inventory.station_availability`** (mutable operational projection, NOT truth)
| column | type | notes |
|--------|------|-------|
| id | TEXT PK | |
| station_id | TEXT FK→station | |
| status | TEXT | `available` \| `limited` \| `unavailable` |
| source | TEXT | `manual_partner` \| `system_sync` \| `admin` |
| updated_at | TIMESTAMPTZ | |

Index: `BTREE(station_id)`.

### 4.2 schema `users`

**`users.user_account`** (identity bridge)
| column | type | notes |
|--------|------|-------|
| id | TEXT PK | `USR-<ULID>` |
| keycloak_user_id | TEXT UNIQUE | = JWT.sub; the only bridge |
| email | TEXT | |
| status | TEXT | `active` \| `disabled` |
| created_at | TIMESTAMPTZ | |
| last_login_at | TIMESTAMPTZ | |

Index: `UNIQUE(keycloak_user_id)`. Maps 1:1 with Keycloak. No business ownership here.

**`users.user_profile`** (optional, safe to delete)
| column | type |
|--------|------|
| user_id | TEXT FK→user_account |
| display_name | TEXT |
| avatar_url | TEXT |
| preferred_language | TEXT |
| preferences | JSONB |

**`users.partner_membership`** (CRITICAL, strict 1:1)
| column | type | notes |
|--------|------|-------|
| user_id | TEXT UNIQUE FK→user_account | one membership per user |
| partner_id | TEXT FK→`inventory.partner` | required for partner role |
| role | TEXT | `owner` \| `manager` \| `operator` \| `viewer` |

Constraint: `UNIQUE(user_id)`. Partner identity derived ONLY from this, never JWT alone.

**`users.favorite_station`**
| column | type | notes |
|--------|------|-------|
| user_id | TEXT | |
| station_id | TEXT | |
| created_at | TIMESTAMPTZ | |

PK: `(user_id, station_id)`. Only `registered_driver` can create.

**`users.station_review`**
| column | type | notes |
|--------|------|-------|
| id | TEXT PK | `REV-<ULID>` |
| user_id | TEXT | |
| station_id | TEXT | |
| rating | INT | 1–5 |
| comment | TEXT | |
| status | TEXT | `published` \| `hidden` \| `flagged` \| `deleted` |
| created_at / updated_at | — | |

Constraint: `UNIQUE(user_id, station_id)` (one review per user per station).
Indexes: `BTREE(station_id)`, `BTREE(user_id)`. Owner modifies; admin moderates
status only. Lifecycle: `submitted → published → flagged → hidden → deleted`.

### 4.3 schema `gis`

**`gis.sync_queue`** (outbox)
| column | type | notes |
|--------|------|-------|
| id | TEXT PK | |
| entity_type | TEXT | `station` \| `charger` |
| entity_id | TEXT | |
| operation | TEXT | `insert` \| `update` \| `delete` |
| payload | JSONB | |
| status | TEXT | `pending` \| `processing` \| `done` \| `failed` \| `dead_letter` |
| created_at / processed_at | — | |

Indexes: `BTREE(status)`, `BTREE(entity_type, entity_id)`.
Also store OSM base layers (Tunisia first) + station geometry layer.

### 4.4 analytics_db — schema `analytics`

**`analytics.raw_event`** (partitioned by month, e.g. `raw_event_2026_06`)
| column | type |
|--------|------|
| event_id | TEXT (dedup key) |
| event_name | TEXT |
| session_id | TEXT |
| user_id | TEXT NULL |
| anonymous_id | TEXT |
| actor_role | TEXT |
| occurred_at | TIMESTAMPTZ |
| ingested_at | TIMESTAMPTZ |
| path | TEXT |
| payload | JSONB |
| metadata | JSONB |

Indexes: `BTREE(event_name, occurred_at)`, `BTREE(user_id)`, `BTREE(session_id)`.
**`analytics.event_dead_letter`** — stores invalid events.

---

## 5. API Contract (Full)

Base URL `/api/v1`. Auth: `Authorization: Bearer <JWT>` (Keycloak-issued).
Standard envelopes from §1.4. Pagination meta:
```json
{ "page": 1, "size": 20, "total": 100, "total_pages": 5, "has_next": true, "has_prev": false }
```

### 5.1 Canonical Error Codes
`UNAUTHENTICATED`, `FORBIDDEN`, `TOKEN_EXPIRED`, `PARTNER_SCOPE_VIOLATION`,
`INSUFFICIENT_ROLE`, `NOT_FOUND`, `ALREADY_EXISTS`, `SOFT_DELETED`,
`VALIDATION_FAILED`, `INVALID_COORDINATES`, `INVALID_STATE_TRANSITION`,
`ACTIVE_STATIONS_EXIST`, `REVIEW_STATE_INVALID`, plus `409 CONFLICT` for optimistic
locking (`If-Match: <version>`).

### 5.2 Driver APIs (`/api/v1/driver/*`)
| Method | Path | Auth | Rules |
|--------|------|------|-------|
| GET | `/driver/stations` | public | params: `lat,lng,radius_km,bbox,filters,page,size`; `is_live=true`, exclude deleted, use GIST index |
| GET | `/driver/stations/{station_id}` | public | returns station + chargers + availability + reviews summary |
| GET | `/driver/stations/search` | public | params: `q,city,connector_type,availability` |
| POST | `/driver/favorites/{station_id}` | registered_driver | |
| DELETE | `/driver/favorites/{station_id}` | registered_driver | |
| POST | `/driver/reviews` | registered_driver | body `{station_id,rating(1-5),comment}`; one per user/station |
| PATCH | `/driver/reviews/{id}` | owner only | |
| DELETE | `/driver/reviews/{id}` | owner only | soft delete |
| GET | `/driver/me` | registered_driver | |
| PATCH | `/driver/me` | registered_driver | |

GIS response requirement on station objects:
```json
{ "distance_km": 1.2, "geom": { "lat": 0, "lng": 0 } }
```

### 5.3 Partner APIs (`/api/v1/partner/*`, hosted by admin-service)
| Method | Path | Rules |
|--------|------|-------|
| GET | `/partner/me` | returns `partner_id`, `role`, membership info |
| GET | `/partner/stations` | OWNED ONLY (mandatory), optional soft-deleted flag |
| POST | `/partner/stations` | header `Idempotency-Key` required |
| PATCH | `/partner/stations/{id}` | triggers GIS sync + analytics event |
| DELETE | `/partner/stations/{id}` | soft delete ONLY |
| GET | `/partner/chargers` | |
| POST | `/partner/chargers` | |
| PATCH | `/partner/chargers/{id}` | |
| PATCH | `/partner/stations/{id}/availability` | |
| GET | `/partner/reports/overview` | |

**Every partner query is scoped by `partner_id` from membership. Never client-supplied.**

### 5.4 Admin APIs (`/api/v1/admin/*`)
| Method | Path | Rules |
|--------|------|-------|
| GET | `/admin/users` | |
| GET | `/admin/partners` | |
| POST | `/admin/partners` | |
| PATCH | `/admin/partners/{id}` | |
| DELETE | `/admin/partners/{id}` | blocked if active stations exist |
| GET | `/admin/stations` | |
| PATCH | `/admin/stations/{id}` | |
| DELETE | `/admin/stations/{id}` | soft delete |
| GET | `/admin/reviews` | |
| PATCH | `/admin/reviews/{id}/status` | states: published/hidden/flagged/deleted |
| GET | `/admin/reports/overview` | |
| GET | `/admin/reports/top-stations` | |
| GET | `/admin/reports/search-analytics` | |

### 5.5 Cross-cutting API rules
- GIS queries MUST support bbox + radius and use spatial index.
- Rate limits: public 60/min, authenticated 300/min, admin 1000/min.
- Station search ≤ 200ms p95. Pagination mandatory. No analytics exposed via API.
- Breaking change → `/v2`. No header versioning.

---

## 6. Event Taxonomy (Clickstream)

Applies ONLY to the 4 frontend apps. Excludes system/infra logs, DB/GIS domain
events, backend telemetry. Events = user behavior only. **Forbidden**: UI-style
names (`button.clicked`, `modal.opened`). **Allowed**: business meaning
(`station.favorited`, `search.performed`).

### 6.1 Canonical Envelope
```json
{
  "event_id": "CLK-01ABCDEF",
  "event_version": 1,
  "schema_namespace": "clickstream",
  "event_name": "station.opened",
  "occurred_at": "2026-06-01T12:00:00Z",
  "ingested_at": "2026-06-01T12:00:01Z",
  "channel": "driver_web",
  "session_id": "sess-123",
  "correlation_id": "flow-xyz",
  "anonymous_id": "anon-123",
  "user_id": "usr-123",
  "actor_role": "registered_driver",
  "path": "/stations/STN-123",
  "payload": {},
  "metadata": {}
}
```
Required: `event_id`, `event_name`, `occurred_at`, `ingested_at`, `channel`,
`session_id`. Identity: `user_id` nullable (anonymous), `anonymous_id` required if no
user, `actor_role` from JWT or anonymous. Channels: `driver_web`, `driver_mobile`,
`partner_dashboard`, `admin_dashboard`. Naming: `<domain>.<action>`, lowercase,
dot-separated.

### 6.2 Event Catalog (implement exactly these)
- **Navigation**: `page.viewed`, `map.loaded`, `map.viewport_changed`
- **Discovery**: `search.performed`, `stations.nearby.viewed`, `filter.applied`
- **Station**: `station.marker_clicked`, `station.opened`, `charger.opened`
- **Favorites**: `favorite_station.added`, `favorite_station.removed`
- **Reviews**: `review.submitted`, `review.updated`
- **Auth**: `auth.started`, `auth.succeeded`, `auth.failed`
- **Partner**: `partner_station.created`, `partner_station.updated`,
  `partner_availability.updated`
- **Admin**: `admin_station.created`, `admin_review.moderated`
- **Failures**: `search.failed`, `station.load_failed`

### 6.3 Validation (clickstream-service MUST enforce)
- `event_name` exists in taxonomy; `event_id` unique; payload valid JSON;
  `session_id` present; `actor_role` matches JWT.
- Payload safety — FORBIDDEN: passwords, tokens, emails, phone numbers, raw auth.
- Delivery: at-least-once, dedup by `event_id`, ordering NOT guaranteed.
- Versioning: breaking change → bump `event_version`; additive fields no bump.
- Performance: ingestion > 100/sec baseline, write < 50ms avg, indexed dedup lookup.

---

## 7. Sprint-by-Sprint Tasks

> Each sprint lists: Goal → Tasks → Verification (commands/checks) → Exit Criteria.

### Sprint 0 — Architecture Freeze
- **Goal**: lock boundaries; no design drift afterward.
- **Tasks**: confirm `docs/*.md` frozen; confirm service list (the 5 in §3.2),
  role set (3), ID strategy, event envelope. Initialize empty repo.
- **Exit**: no open design questions; all boundaries explicit; `git` initialized.

### Sprint 1 — Monorepo + Tooling
- **Goal**: full engineering skeleton + shared contracts.
- **Tasks**:
  - Create layout in §3.1. Init Rust workspace (root `Cargo.toml` with `services/*`,
    `crates/*` members). Init pnpm/npm workspace for `apps/*` + `packages/*`.
  - Scaffold Rust crates: `common-types`, `common-errors`, `common-auth` (stub),
    `common-db` (stub). Scaffold service binaries with a `main` that prints/serves nothing yet.
  - Scaffold React+Vite apps: `driver-web`, `partner-dashboard`, `admin-dashboard`.
    Scaffold Expo app `driver-mobile`.
  - Create TS packages: `shared-types`, `api-client`, `auth-client` (stub),
    `design-tokens` (empty), `event-taxonomy` (locked schema from §6),
    `api-contracts` (envelopes + error codes from §5).
  - Empty Docker Compose structure + base Traefik config in `infra/`.
- **Verify**: `cargo build` (workspace) succeeds; each frontend `npm run build`
  succeeds; shared types import cross-stack.
- **Exit**: all apps compile; empty screens render; Rust workspace builds.

### Sprint 2 — Runtime Infrastructure (Docker Compose v1)
- **Goal**: bring full system online locally.
- **Tasks**:
  - Compose services: Traefik, Keycloak, PostgreSQL (create 3 DBs:
    `keycloak_db`, `platform_db` with PostGIS, `analytics_db`), RabbitMQ.
  - Compose the 5 backend services serving an empty HTTP `/health` (validates DB
    connection + dependency reachability + internal state).
  - Internal Docker network; **only Traefik exposed**; no host ports on others.
  - Per-service `.env` (see §8); Traefik routing rules for domains.
- **Verify**: `docker compose up` brings everything up; `curl` each `/health` returns
  ok; Keycloak reachable; RabbitMQ reachable; DB connectivity validated.
- **Exit**: full stack online; services respond `/health`.

### Sprint 3 — Identity & RBAC
- **Goal**: auth + authorization backbone.
- **Tasks**:
  - Keycloak realm `bornemap`; roles `registered_driver`, `partner`, `admin`;
    OAuth providers (Google/Facebook) config.
  - Rust JWT validation middleware in `common-auth`: validate issuer/JWKS/audience
    (§8); extract role; auth-guard framework.
  - First-login provisioning: on first valid JWT, create `users.user_account`
    (`keycloak_user_id = JWT.sub`); map `partner_membership` if applicable.
  - **Implement rule**: `partner_id` NEVER from client — always from membership.
- **Verify**: login issues JWT; protected endpoints reject missing/expired tokens
  (`UNAUTHENTICATED`/`TOKEN_EXPIRED`); role-gated endpoints reject wrong role
  (`INSUFFICIENT_ROLE`); first login creates `user_account` exactly once.
- **Exit**: JWT validated everywhere; RBAC active.

### Sprint 4 — Core Database Schema
- **Goal**: stable data model before business logic.
- **Tasks**: write migrations for all schemas/tables/indexes in §4 (inventory, users,
  gis, analytics stub). Enforce soft delete columns, station visibility rule helpers,
  GIST indexing on `geom`. Seed data + spatial query smoke test.
- **Verify**: migrations run clean and are idempotent on re-run; relationships/FKs
  enforced; indexes created (`\d+` inspection); a sample bbox spatial query returns
  results using GIST.
- **Exit**: migrations stable; seed works; spatial queries functional.

### Sprint 5 — Admin Service MVP (Inventory Write API)
- **Goal**: first real business system (backend only).
- **Tasks**: implement `/api/v1/admin/*` and `/api/v1/partner/*` from §5.3–5.4:
  partner CRUD, station CRUD, charger CRUD, availability update.
  - **Hard rules**: full partner isolation (every query scoped by membership
    `partner_id`); no cross-partner queries; soft delete only; partner delete blocked
    if active stations (`ACTIVE_STATIONS_EXIST`).
  - **On station change**: insert `gis.sync_queue` outbox row (operation insert/
    update/delete) and emit analytics event.
- **Verify**: integration tests prove a partner cannot read/modify another partner's
  station (`PARTNER_SCOPE_VIOLATION`); soft delete sets `deleted_at`, no hard delete;
  station mutation creates an outbox row.
- **Exit**: admin can fully manage inventory; DB consistency + RBAC enforced.

### Sprint 6 — GIS Sync System v1
- **Goal**: spatial layer operational.
- **Tasks**: implement `gis-worker`: poll/consume `gis.sync_queue` (or RabbitMQ),
  process states `pending → processing → done|failed → dead_letter`. Convert station
  lat/lng to `geom = ST_SetSRID(ST_MakePoint(lng,lat),4326)`. OSM Tunisia import
  (basic). Idempotent + replay-safe + retry/backoff.
- **Verify**: a station insert/update reflects in GIS layer; replaying the same
  outbox row twice yields identical state (idempotency test); map bbox queries work.
- **Exit**: station→geometry sync works; idempotency verified.

### Sprint 7 — Driver Service MVP (Discovery Core)
- **Goal**: public + authenticated user API.
- **Tasks**: implement all `/api/v1/driver/*` from §5.2.
  - Public: stations (bbox/radius), detail, markers, search.
  - Authenticated: favorites, reviews (one per user/station; owner-only modify),
    profile (`/me`).
  - **Rules**: enforce station visibility rule (§1.3); exclude soft-deleted; Tunisia
    fallback center (`MAP_DEFAULT_LAT=36.8065`, `MAP_DEFAULT_LNG=10.1815`,
    `MAP_DEFAULT_RADIUS_KM=10`, `MAP_MAX_RADIUS_KM=50`); include `distance_km`+`geom`.
- **Verify**: bbox query returns only visible stations and uses GIST (EXPLAIN shows
  index scan); duplicate review rejected (`REVIEW_STATE_INVALID`/`ALREADY_EXISTS`);
  non-owner cannot edit review (`FORBIDDEN`); search ≤ 200ms p95 on seed.
- **Exit**: full discovery works; mobile-ready APIs stable.

### Sprint 8 — Design System Foundation
- **Goal**: UI consistency layer.
- **Tasks**: finalize `design-tokens` package (raw values); Tailwind theme mapping;
  integrate shadcn/ui; build primitives: Button, Input, Card, Modal, Map container
  shell. RTL-ready foundation. **Forbidden**: inline hex, arbitrary spacing, duplicate
  components across apps.
- **Verify**: all apps consume the same token/theme base; a sample RTL toggle flips
  layout direction.
- **Exit**: reusable UI system exists; RTL ready; tokens enforced.

### Sprint 9 — Driver Web App
- **Goal**: first full UX product.
- **Tasks**: implement map-first discovery (Leaflet), station detail, search/filter,
  favorites, reviews. Use React Query; progressive auth (login modal only on gated
  action — favorite, review). Map state machine: Idle → Viewport Active (bbox query
  + cluster) → Station Selected. Loading = skeletons (not spinners). Emit clickstream
  events (§6) via `api-client`.
- **Verify**: full driver journey works; public browsing without login; gated actions
  trigger login then resume; map movement debounced 300–500ms and refetches.
- **Exit**: driver journey works; web mirrors mobile-style UX.

### Sprint 10 — Partner Dashboard
- **Goal**: operational dashboard (density-first).
- **Tasks**: station management, charger management, availability updates via
  `/api/v1/partner/*`. Sidebar(260px) + Main + Context Panel layout.
- **Verify**: partner sees ONLY owned stations; CRUD works; isolation enforced
  client-and-server.
- **Exit**: partner isolation verified; CRUD functional.

### Sprint 11 — Admin Dashboard
- **Goal**: system control interface.
- **Tasks**: global station view, partner management, moderation (review status
  transitions), system overview via `/api/v1/admin/*`.
- **Verify**: admin global access works but still respects soft-delete/audit;
  moderation transitions follow review lifecycle.
- **Exit**: admin has full platform control; RBAC enforced.

### Sprint 12 — Mobile App (Expo)
- **Goal**: driver mobile experience.
- **Tasks**: map discovery, station details, favorites, reviews, login flow. Reuse
  `shared-types`, `api-client`, `auth-client`. RTL stable, offline-safe UI.
- **Verify**: parity with driver web core; smooth map UX; RTL works.
- **Exit**: parity achieved.

### Sprint 13 — Clickstream System
- **Goal**: event ingestion backbone.
- **Tasks**: `clickstream-service` HTTP ingest (single + batch); validate against
  taxonomy (§6.3); dedup by `event_id`; publish to RabbitMQ
  (`clickstream.topic` → `clickstream.raw`, DLQ `clickstream.dlq`). Accept anonymous.
- **Verify**: valid event flows frontend → RabbitMQ; invalid event rejected and/or
  routed to DLQ; duplicate `event_id` deduped; no PII passes validation.
- **Exit**: events flowing frontend → pipeline.

### Sprint 14 — Analytics Writer
- **Goal**: turn events into stored data.
- **Tasks**: `analytics-writer` consumes RabbitMQ; writes `analytics.raw_event`
  (monthly partitions); dedup; aggregation pipeline; invalid →
  `analytics.event_dead_letter`. Append-only.
- **Verify**: `raw_event` populated, partition routing correct; duplicate ingestion
  handled; aggregation queries operational.
- **Exit**: analytics DB populated; queries work.

### Sprint 15 — Reporting Layer
- **Goal**: business insights.
- **Tasks**: Partner reports (station performance, engagement). Admin reports
  (platform KPIs, top stations, search trends) via `/admin/reports/*`. Default window
  `REPORTING_DEFAULT_WINDOW_DAYS=30`.
- **Verify**: report endpoints return correct aggregates from `analytics_db`; never
  expose raw analytics or PII.
- **Exit**: reports functional for partner + admin.

### Sprint 16 — Hardening + Production Readiness
- **Goal**: production-safe.
- **Tasks**: load test (<100 events/sec baseline), RBAC audit, GIS stress + consistency
  checks, retry/failure simulation, rollback drills, DB backup validation, RTL (Arabic)
  audit, WCAG 2.1 AA check, mobile weak-network testing.
- **Verify**: pre-release gates pass (all unit/integration/contract pass, critical E2E
  pass, no perf regression, security checks pass); post-deploy smoke tests pass.
- **Exit**: production-ready checklist passes.

---

## 8. Configuration Reference (.env per service)

Rules: each service owns its `.env`; no cross-service env import; fail-fast on
missing/invalid; log resolved config with secrets redacted; only `.env.example`
committed. `APP_ENV=local|dev|prod`. Prefixes: `APP_*`, `LOG_*`, `AUTH_*`, `DB_*`,
`RABBITMQ_*`, `GIS_*`, `CLICKSTREAM_*`, `ANALYTICS_*`.

**Traefik**: `TRAEFIK_HTTP_PORT=80`, `TRAEFIK_HTTPS_PORT=443`, `TRAEFIK_TLS_ENABLED=true`,
`TRAEFIK_DOMAIN_DRIVER=driver.example.tn`, `..._PARTNER=partner.example.tn`,
`..._ADMIN=admin.example.tn`, `..._API=api.example.tn`, `..._AUTH=auth.example.tn`.

**Keycloak**: `KEYCLOAK_HTTP_PORT=8080`, `KEYCLOAK_REALM=bornemap`,
`KEYCLOAK_PUBLIC_URL=https://auth.example.tn`, admin bootstrap + `KEYCLOAK_DB_*`.

**platform_db**: `PLATFORM_DB_HOST=postgres.internal`, `PLATFORM_DB_NAME=platform_db`,
`PLATFORM_DB_USER`, `PLATFORM_DB_PASSWORD`, `PLATFORM_DB_SSL_MODE=disable`,
`PLATFORM_DB_MAX_CONNECTIONS=20`. Used by driver-service, admin-service, gis-worker.

**analytics_db**: `ANALYTICS_DB_*`. Used by analytics-writer, clickstream-service.

**RabbitMQ**: `RABBITMQ_HOST=rabbitmq.internal`, `RABBITMQ_PORT=5672`,
`RABBITMQ_USER`, `RABBITMQ_PASSWORD`, `RABBITMQ_VHOST=/bornemap`,
`RABBITMQ_EXCHANGE_CLICKSTREAM=clickstream.topic`,
`RABBITMQ_QUEUE_CLICKSTREAM_RAW=clickstream.raw`,
`RABBITMQ_QUEUE_CLICKSTREAM_DLQ=clickstream.dlq`.

**driver-service**: `DRIVER_SERVICE_PORT=8081`,
`AUTH_ISSUER=https://auth.example.tn/realms/bornemap`,
`AUTH_JWKS_URL=.../protocol/openid-connect/certs`, `AUTH_AUDIENCE=bornemap-api`,
`MAP_DEFAULT_LAT=36.8065`, `MAP_DEFAULT_LNG=10.1815`, `MAP_DEFAULT_RADIUS_KM=10`,
`MAP_MAX_RADIUS_KM=50`.

**admin-service**: `ADMIN_SERVICE_PORT=8082`,
`PARTNER_DELETE_BLOCK_ACTIVE_STATIONS=true`, `REPORTING_DEFAULT_WINDOW_DAYS=30`.

**gis-worker**: `GIS_WORKER_POLL_INTERVAL_MS=5000`, `GIS_WORKER_BATCH_SIZE=50`,
`GIS_DEFAULT_SRID=4326`.

**clickstream-service**: `CLICKSTREAM_PORT=8083`, `CLICKSTREAM_BATCH_SIZE=100`,
`CLICKSTREAM_ACCEPT_ANONYMOUS=true`, `CLICKSTREAM_ENFORCE_EVENT_ID=true`.

**analytics-writer**: `ANALYTICS_BATCH_SIZE=200`, `ANALYTICS_FLUSH_INTERVAL_MS=2000`,
`ANALYTICS_RETENTION_DAYS=90`.

**Web (Vite)**: `VITE_API_BASE_URL=https://api.example.tn`,
`VITE_AUTH_BASE_URL=https://auth.example.tn`, `VITE_REALM=bornemap`,
`VITE_SUPPORTED_LANGUAGES=ar,fr`, `VITE_MAP_LAT=36.8065`, `VITE_MAP_LNG=10.1815`.

**Mobile (Expo)**: `EXPO_PUBLIC_API_BASE_URL`, `EXPO_PUBLIC_AUTH_BASE_URL`,
`EXPO_PUBLIC_REALM=bornemap`, `EXPO_PUBLIC_LANGUAGES=ar,fr`.

**Observability (all)**: `LOG_LEVEL=info`, `LOG_FORMAT=json`,
`REQUEST_ID_HEADER=x-request-id`. Structured logs + request correlation, no PII.

**Feature flags**: `FF_ENABLE_REVIEWS=true`, `FF_ENABLE_GIS_SYNC=true`,
`FF_ENABLE_ANALYTICS=true` (explicit only, no dynamic remote config in MVP).

**Startup order (strict)**: Postgres → RabbitMQ → Keycloak → Traefik → backend
services → workers → frontends.

---

## 9. Deployment Workflow

Host layout: `/opt/bornemap/{compose,env,artifacts/{images,releases},logs,backups}`.
Artifacts as `service-name.tar` (prebuilt image, SHA digest). Release manifest pins
every image SHA; partial mismatch = deployment FAILURE.

1. **Preflight**: validate host/Docker, env files, DB + RabbitMQ connectivity.
2. **Load artifacts**: `docker load -i <service>.tar`.
3. **Validate manifest**: verify image SHAs; reject mismatches.
4. **Run migrations**: `platform_db`, `analytics_db`, `keycloak_db` (if needed) —
   BEFORE service startup.
5. **Start infra**: Traefik, Keycloak.
6. **Start services**: backend services, then workers.
7. **Start frontends** via Traefik.
8. **Smoke tests**: auth login, station fetch, GIS sync check, event ingestion check.

**Rollback**: L1 service redeploy previous image; L2 full stack restart; L3 DB restore
from backup. Always preplanned. Each service exposes `/health` validating DB +
dependencies + internal state. Backups before every prod release (`platform_db`,
`analytics_db`, optional `keycloak_db`).

---

## 10. Testing Strategy (apply per sprint)

6 levels: Unit, Integration, Contract/API, E2E, Performance, Security; plus Smoke
(post-deploy) and Operational. Use real PostgreSQL schema with migrations + isolated
datasets + full reset between runs; isolated RabbitMQ + test Keycloak realm; parallel
per service.

**Must-cover (CRITICAL)**:
- **Authorization**: public vs authenticated; driver permissions; **partner tenant
  isolation cannot be bypassed**; admin global scope.
- **Data correctness** across `platform_db` (truth), `analytics_db` (derived), GIS
  (derived).
- **Event integrity**: schema enforced, `event_id` dedup, RabbitMQ delivery.
- **GIS**: idempotency (duplicate/replayed/out-of-order outbox events), failure
  recovery, bbox queries stay indexed (no full-table scans).
- **Reliability**: service restart, queue backpressure, partial DB failure, delayed
  GIS sync, retry storms.
- **Security**: SQL injection, malformed/oversized JSON, event replay/tampering,
  rate limiting, brute-force resistance.
- **UX**: mobile-first parity, RTL Arabic / French LTR, WCAG 2.1 AA.

**Pre-release gate (blocks release)**: all unit + integration + contract pass;
critical E2E pass; no perf regression; security checks pass.

---

## 11. Architecture Decisions (Context — Do Not Re-Litigate)

- Monorepo (Rust + TS). Backend = Rust (all 5 services). Identity = Keycloak only,
  JWT validation only. 3 PostgreSQL DBs (no cross-DB joins). RabbitMQ backbone.
  Clickstream is the canonical event source. GIS via outbox pattern. Pure REST (no
  GraphQL). React + Vite (no Next.js), React Native Expo. Leaflet maps. Server-side
  tenant isolation (`partner_id` from membership, never client). Soft delete for
  station/partner/review. Partitioned analytics events, dedup by `event_id`.
  Env-driven config (no dynamic/remote config in MVP). Docker Compose + bare metal +
  Traefik. Strict typed error envelope.

---

## 12. Final Reminders

- The domain model is the source of truth; API/UI/events/GIS/analytics are
  projections of it.
- The system is **data-first, contract-driven, ownership-enforced** — not event-,
  GIS-, or UI-driven.
- If a task tempts you to: store passwords, add a 4th role, accept `partner_id` from a
  client, hard-delete a station/partner/review, expose a non-Traefik port, put PII in
  events/logs, or make GIS authoritative — **STOP. That violates the Constitution.**
