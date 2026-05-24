# Amilcar — Architecture Reference

**Version:** 2.0.0 (constitution-aligned)
**Scope:** Cross-cutting, system-wide architecture for the Amilcar EV Charging Admin Portal.
**Companions:** [`constitution.md`](./constitution.md), [`plan.md`](./plan.md).
**Per-feature design docs:** `specs/00X-…/data-model.md`, `specs/00X-…/contracts/`.

> This is a **reference**. It explains *how* the platform is built. The constitution explains *what is allowed*. The plan explains *when things ship*. In any conflict, the constitution wins.

---

## 1. System Context (C4 L1)

```mermaid
flowchart TB
    driver["Driver<br/>(Web / Mobile)"]
    admin["Administrator<br/>(Web Portal)"]
    invitee["Invitee<br/>(Email recipient)"]
    osMaps["OS Map Provider<br/>(Apple/Google Maps)<br/>navigation only"]
    social["Social IdP<br/>(Google / Facebook)<br/>Phase 5+"]
    mailer["Mail Transport<br/>(SMTP / Provider)"]

    subgraph platform["Amilcar Platform"]
        bm["Amilcar<br/>EV Charging Admin Portal"]
    end

    driver -->|HTTPS| bm
    admin -->|HTTPS| bm
    invitee -->|HTTPS signup link| bm
    bm -->|federated OIDC| social
    bm -->|invitation email| mailer
    driver -.deep-link.-> osMaps
```

**Non-goals embodied here** (Constitution §2 / NG-1..NG-7): no OCPP, no payments, no smart grid, no telemetry from chargers, no in-app routing.

---

## 2. Container View (C4 L2)

```mermaid
flowchart TB
    subgraph dmz["PUBLIC DMZ"]
        web["Admin Portal<br/>React + Vite + Leaflet"]
        mob["Mobile App<br/>React Native + Expo"]
    end

    subgraph apps["APPS & RUNTIMES (PRIVATE)"]
        actix["bornemap-backend<br/>Actix Web · Modular Monolith<br/>Phases 1-5"]
        bff["BFF / API Gateway<br/>Phase 6+"]
        worker["bornemap-worker<br/>Phase 3+"]
        rmq[["RabbitMQ<br/>Phase 3+"]]
        redis[["Redis 7<br/>Phase 4+"]]
    end

    subgraph data["DATA STORAGE (ISOLATED)"]
        pg[("PostgreSQL 16<br/>+ PostGIS 3.4")]
        kc["Keycloak<br/>Phase 5+"]
        minio[("MinIO / S3<br/>Phase 3+")]
    end

    web -->|HTTPS · generated client| actix
    mob -->|HTTPS · generated client| actix
    actix -->|SQLx · GiST · ST_DWithin| pg
    actix -->|OIDC PKCE| kc
    actix -->|presigned URL| minio
    actix -->|publish| rmq
    actix -->|GET/SET| redis
    rmq --> worker
    worker --> pg

    bff -. Phase 6 .-> actix
```

**Phase transitions:**

- Phases 1–2: only `actix`, `pg` and frontends present.
- Phase 3: `rmq`, `worker`, `minio` come online.
- Phase 4: `redis` joins.
- Phase 5: `kc` replaces mock JWT issuance.
- Phase 6: `actix` is decomposed; `bff` becomes the public entrypoint; per-service databases replace the shared `pg`.

---

## 3. Component View (C4 L3) — Backend (Monolith)

```mermaid
flowchart LR
    subgraph H[Handlers]
        h1[station_handlers]
        h2[identity_handlers]
        h3[profile_handlers]
        h4[review_handlers]
        h5[event_handlers]
    end
    subgraph S[Services]
        s1[StationService]
        s2[IdentityService]
        s3[ProfileService]
        s4[ReviewService]
        s5[EventService]
    end
    subgraph R[Repositories]
        r1[StationRepo]
        r2[IdentityRepo]
        r3[ProfileRepo]
        r4[ReviewRepo]
        r5[EventRepo]
    end
    subgraph DB[(PostgreSQL + PostGIS)]
        sch1[station_domain.*]
        sch2[identity_domain.*]
        sch3[profile_domain.*]
        sch4[review_domain.*]
        sch5[event_domain.events]
    end

    h1 --> s1 --> r1 --> sch1
    h2 --> s2 --> r2 --> sch2
    h3 --> s3 --> r3 --> sch3
    h4 --> s4 --> r4 --> sch4
    h5 --> s5 --> r5 --> sch5

    s4 -. DTO via trait .-> s1
    s3 -. DTO via trait .-> s2
    s2 -. emits event .-> s5
```

**Invariants enforced (Constitution §III, §IV):**

- Handlers → Services → Repositories only. No reverse calls, no skip-layer calls.
- Cross-domain access goes through a public service trait (e.g., `pub trait StationService { fn marker_summary(...) -> ... }`). Repositories are crate-private.
- Each domain owns its schema (`station_domain`, `identity_domain`, …). Cross-schema reads are forbidden.

---

## 4. Domain Map

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

| Domain | Owns | Phase introduced |
| --- | --- | --- |
| `station_domain` | Companies, stations, chargers, GiST spatial index | 1 |
| `identity_domain` | Users, sessions, invitations, federation | 1 (mock) → 5 (Keycloak) |
| `profile_domain` | Driver profiles, avatars, favorites | 2 |
| `review_domain` | Ratings, feedback, moderation status | 2 |
| `event_domain` | Append-only telemetry log | 3 |
| `settings_domain` | Lookup tables, feature flags | as needed |

---

## 5. Request Lifecycle — Spatial Viewport Query

```mermaid
sequenceDiagram
    autonumber
    participant U as User Gesture
    participant MI as MapInteractionDomain
    participant RQ as React Query
    participant API as Actix Handler
    participant SVC as StationService
    participant REPO as StationRepository
    participant CACHE as Redis (Phase 4+)
    participant PG as PostGIS

    U->>MI: pan / zoom
    MI->>MI: debounce ≥ 300 ms (R1)
    MI->>MI: quantize bounds to 4 decimals (R2)
    MI->>RQ: lookup ['stations', quantized] (R6)
    alt React Query cache hit
        RQ-->>MI: cached marker summaries
    else cache miss
        RQ->>API: GET /api/v1/stations?bbox=...
        alt Redis hit (Phase 4+)
            API->>CACHE: GET stations:bbox:{q}
            CACHE-->>API: marker summaries
        else Redis miss / Phase < 4
            API->>SVC: list_in_viewport(bbox)
            SVC->>REPO: query(bbox)
            REPO->>PG: ST_DWithin(geog, envelope, 0)
            PG-->>REPO: rows (summary only)
            REPO-->>SVC: marker summaries
            SVC-->>API: DTO[]
            API->>CACHE: SETEX stations:bbox:{q} 60 (Phase 4+)
        end
        API-->>RQ: 200 OK
        RQ-->>MI: marker summaries
    end
    MI->>MI: clip off-viewport (R3)
    MI->>MI: cluster if density > 15 / 40px (R7)
    MI-->>U: render at 60 FPS
```

End-to-end enforcement of R1, R2, R3, R6, R7 (Constitution §III.VIII).

---

## 6. Authentication Architecture

### 6.1 Phase Evolution

```mermaid
flowchart LR
    p12["Phases 1-2<br/>Mock JWT HS256<br/>frozen claim shape"]
    p34["Phases 3-4<br/>internal identity_domain<br/>argon2 + RBAC"]
    p5["Phase 5+<br/>Keycloak OAuth2 + PKCE<br/>JWKS validation"]
    p12 --> p34 --> p5
    p5 -. federation .-> google((Google))
    p5 -. federation .-> facebook((Facebook))
```

**Frozen claim shape** (Constitution §6 / Phase 1+):

```json
{
  "sub": "user-uuid",
  "preferred_username": "alice",
  "realm_access": { "roles": ["driver"] },
  "iat": 1748000000,
  "exp": 1748003600
}
```

This shape never changes. Phase 5 replaces the **issuer** without altering downstream consumers.

### 6.2 Admin Invitation Flow

```mermaid
sequenceDiagram
    autonumber
    participant Admin as Admin (Web)
    participant Modal as InviteCollaborators
    participant API as BFF / API
    participant ID as identity_domain
    participant Mail as Mail Transport
    participant Inv as Invitee

    Admin->>Modal: enter email + role
    Modal->>API: POST /api/v1/admin/invitations
    API->>ID: createInvitation(email, role)
    ID->>ID: generate single-use signed token (24 h TTL)
    ID-->>API: invitation_id
    API->>Mail: send signup link {token}
    Mail-->>Inv: email
    Inv->>API: GET /signup?token=...
    API->>ID: validateInvitation(token)
    ID-->>API: ok + role claims
    API-->>Inv: provisioning UI
    Inv->>API: POST /api/v1/auth/accept-invitation
    API->>ID: create user (Keycloak in Phase 5+) + idempotent profile init
    ID-->>API: tokens
    API-->>Inv: signed in
```

The `InviteCollaborators` modal in the admin portal is the **only** path to admin account creation. Public admin signup is banned (Constitution §V security).

---

## 7. Data Architecture

### 7.1 Spatial Storage Rules

| Concern | Rule |
| --- | --- |
| Column type | `GEOGRAPHY(Point, 4326)` |
| Index | `CREATE INDEX … USING GIST (location);` |
| Coordinate format | `[lng, lat]` everywhere (wire, code, storage) |
| Primary predicate | `ST_DWithin` (indexed) |
| Secondary computation | `ST_Distance` (post-filter sort only) |
| Cache key normalization | 4-decimal quantization |
| Soft delete | `deleted_at TIMESTAMPTZ NULL` |
| Test isolation | `is_test BOOLEAN NOT NULL DEFAULT FALSE` |

### 7.2 Canonical Viewport Query

```sql
SELECT s.id, s.name,
       ST_X(s.location::geometry) AS lng,
       ST_Y(s.location::geometry) AS lat,
       s.is_active, s.under_maintenance
FROM station_domain.stations s
WHERE s.deleted_at IS NULL
  AND ST_DWithin(
        s.location,
        ST_MakeEnvelope($1, $2, $3, $4, 4326)::geography,
        0
      )
ORDER BY s.id
LIMIT 5000;
```

### 7.3 Idempotent Profile Initialization (Constitution Principle XI)

```sql
INSERT INTO profile_domain.driver_profiles (user_id, display_name, locale)
VALUES ($1, $2, $3)
ON CONFLICT (user_id) DO UPDATE
SET display_name = EXCLUDED.display_name,
    locale       = EXCLUDED.locale,
    updated_at   = NOW();
```

### 7.4 Outbox / Event Pattern (Phase 3+)

To avoid losing events when RabbitMQ is unavailable:

```sql
CREATE TABLE event_domain.outbox (
    id            BIGSERIAL PRIMARY KEY,
    event_id      UUID NOT NULL UNIQUE,
    event_type    TEXT NOT NULL,
    payload_json  JSONB NOT NULL,
    occurred_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at  TIMESTAMPTZ
);
CREATE INDEX outbox_pending_idx ON event_domain.outbox (published_at) WHERE published_at IS NULL;
```

The worker drains `outbox` in `published_at IS NULL` order, publishes to RabbitMQ, then sets `published_at = NOW()`.

---

## 8. Frontend Architecture

### 8.1 Layered Model (Web + Mobile)

```mermaid
flowchart TB
    pres[Presentation Components<br/>shadcn/ui · RN primitives]
    mi[MapInteractionDomain<br/>viewport · clustering · gestures · cache]
    rq[React Query Hooks<br/>server-state owner]
    api[Generated OpenAPI Client<br/>@bornemap/api-client]
    net((Backend API))

    pres --> mi
    pres --> rq
    mi --> rq
    rq --> api --> net
```

**Hard rules:**

- Presentation components never call the network directly.
- All server state is owned by React Query.
- All HTTP types live in `@bornemap/api-client` and are generated from `openapi.json`. Hand-written types for HTTP payloads are forbidden.

### 8.2 State Model

| State type | Owner | Examples |
| --- | --- | --- |
| Server | React Query | Stations, profiles, reviews, favorites |
| UI | React local / context | Modals, panels, active marker, gesture state |
| Draft | `react-hook-form` | Invitation form, station-edit form |

### 8.3 Shared Geospatial Models

`frontend/packages/geo-models/src/index.ts`:

```ts
export type CoordinateModel = readonly [lng: number, lat: number];

export interface StationMarkerModel {
  id: string;
  name: string;
  coord: CoordinateModel;
  is_active: boolean;
  under_maintenance: boolean;
}

export interface MapViewportModel {
  west: number;   // quantized to 4 decimals (R2)
  south: number;
  east: number;
  north: number;
  zoom: number;
}

export function quantizeBounds(b: MapViewportModel): MapViewportModel {
  const q = (n: number) => Math.round(n * 10_000) / 10_000;
  return { west: q(b.west), south: q(b.south), east: q(b.east), north: q(b.north), zoom: b.zoom };
}
```

### 8.4 Map Engineering Blueprint

#### 8.4.1 Web (React + Leaflet)

- Map handlers: bind `moveend` and `zoomend` **only**. Never bind `move` or `zoom`.
- Cluster wrapper: `react-leaflet-cluster`, options: `{ chunkedLoading: true, disableClusteringAtZoom: 18, maxClusterRadius: 40 }`, density threshold > 15 within 40 px (R7).
- Filter pills (`All`, `Available`, `Fast 50kW+`, `CCS`): pure local React Query selector — no refetch (R5).
- Search bar overlay: pans map immediately; the 300 ms debounce (R1) still applies to the resulting fetch.

```tsx
// frontend/admin-portal/src/features/map/useViewportStations.ts
export function useViewportStations(viewport: MapViewportModel) {
  const q = quantizeBounds(viewport);
  return useQuery({
    queryKey: ['stations', q.west, q.south, q.east, q.north],
    queryFn: () => apiClient.stations.list({ bbox: `${q.west},${q.south},${q.east},${q.north}` }),
    staleTime: 60_000,
    keepPreviousData: true,
  });
}
```

#### 8.4.2 Mobile (React Native + react-native-maps)

- `<Marker tracksViewChanges={false}>` on every static marker (R4).
- Filter the marker array against the active viewport **before** the `.map(...)` render loop (R3).
- Same 300 ms debounce + 4-decimal quantization rules (R1, R2).

```tsx
// frontend/mobile-app/src/screens/MapScreen.tsx
const visible = useMemo(
  () => markers.filter((m) => insideViewport(m.coord, viewport)),
  [markers, viewport]
);

return (
  <MapView onRegionChangeComplete={onRegionEnd /* debounced */} ...>
    {visible.map((m) => (
      <Marker
        key={m.id}
        coordinate={{ latitude: m.coord[1], longitude: m.coord[0] }}
        pinColor={pinColor(m)}
        tracksViewChanges={false}
      />
    ))}
  </MapView>
);
```

### 8.5 UI Tokens (Reference)

`frontend/admin-portal/src/styles/map-theme.constants.ts` (mirror in mobile):

```ts
export const AmilcarMapStyles = {
  overlays: {
    searchBar: {
      borderRadius: '9999px',
      boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
      border: '1px solid #E2E8F0',
    },
    modalCard: {
      background: '#FFFFFF',
      borderRadius: '12px',
      padding: '24px',
    },
  },
  pins: {
    activeGreen: '#22C55E',   // is_active && !under_maintenance
    inactiveRed: '#EF4444',   // under_maintenance || !is_active
  },
} as const;
```

`pinColor(marker)` is the **only** function that maps marker fields → color. It is colocated with the tokens and unit-tested.

### 8.6 Map Interaction Runtime Rules — Implementation Matrix

| Rule | Where enforced | How verified |
| --- | --- | --- |
| R1 Viewport Debounce | `MapInteractionDomain` debounce hook | Unit test: pan storm → ≤ 1 fetch per 300 ms |
| R2 Query Quantization | `quantizeBounds()` | Unit test on rounding; query-key regex check |
| R3 Marker Virtualization | render-time filter | Snapshot test: off-bounds absent from tree |
| R4 Gesture Priority | RN marker prop | ESLint rule banning `<Marker>` without `tracksViewChanges` |
| R5 Lazy Hydration | filter handlers + marker DTO shape | Network panel during user flow → 0 requests on filter click |
| R6 Bounds Cache Registry | React Query keying | Devtools verification; cache-hit unit test |
| R7 Cluster Threshold | cluster wrapper config | Visual test at synthetic density |

---

## 9. Cross-Cutting Concerns

### 9.1 API Contract & Codegen

```mermaid
flowchart LR
    rust[Rust handlers<br/>utoipa annotations] --> spec[openapi.json]
    spec --> codegen[openapi-typescript-codegen]
    codegen --> tsclient[@bornemap/api-client]
    spec --> ci{CI gate}
    ci -->|breaking diff| fail([Pipeline fail])
```

- The Rust handlers are the single source of truth.
- `@bornemap/api-client` is regenerated on every CI run; diffs are committed.
- Breaking API diffs require an explicit migration commit on the consumers.

### 9.2 Observability

| Signal | Tool | Path |
| --- | --- | --- |
| Logs | `tracing` + JSON formatter | stdout |
| Metrics | `prometheus` exposition | `GET /metrics` |
| Liveness | cheap check | `GET /health/live` |
| Readiness | DB + broker check | `GET /health/ready` |
| Distributed traces | OpenTelemetry | Phase 6 only |

Every request emits at least one log line with: `trace_id`, `request_id`, `method`, `path`, `status`, `latency_ms`, `domain`.

### 9.3 Caching

| Layer | Key | Used in |
| --- | --- | --- |
| React Query (client) | `['stations', quantizedBbox]` | All phases |
| Redis (server) | `stations:bbox:{w}:{s}:{e}:{n}` | Phase 4+ |
| PostGIS | GiST spatial index | All phases |

### 9.4 Security Zones (Detailed)

```text
PUBLIC DMZ                  APPS & RUNTIMES (PRIVATE)        DATA STORAGE (ISOLATED)
─────────────────           ──────────────────────────       ─────────────────────────
HTTPS · 443                 Internal DNS only                Internal DNS only
TLS termination             mTLS where supported             No public ingress
Admin Portal                bornemap-backend / BFF           PostgreSQL + PostGIS
Mobile clients              RabbitMQ + workers (P3+)         Keycloak (P5+)
                            Redis (P4+)                      MinIO (P3+)
```

Firewall rules summary:

- `dmz → apps`: HTTPS only.
- `apps → data`: domain-specific ports (5432, 8080 Keycloak, 9000 MinIO, 5672 RabbitMQ, 6379 Redis).
- `data → apps`: deny.
- `* → data` from public: deny.

---

## 10. Technology Decisions (Summary Table)

| Concern | Choice | Constitutional anchor |
| --- | --- | --- |
| Backend runtime | Rust + Actix Web v4 | §IV, §V |
| DB driver | `sqlx` (compile-time verified) | §VI |
| Spatial DB | PostgreSQL 16 + PostGIS 3.4 | §I |
| Spatial type | `GEOGRAPHY(Point, 4326)` + GiST | §I |
| API contract | OpenAPI 3.1 via `utoipa` | §III |
| Frontend (web) | React + Vite + Leaflet + React Query + shadcn/ui | §II, §VIII |
| Frontend (mobile) | React Native + Expo (managed) | §II, §VIII |
| Async transport | RabbitMQ via `lapin` | Phase 3+ |
| Object storage | MinIO via `aws-sdk-s3` | Phase 3+ |
| Cache | Redis 7 | Phase 4+ |
| Identity broker | Keycloak | Phase 5+ |
| Service interconnect | `tonic` (gRPC) | Phase 6+ |
| Errors | `thiserror` + RFC-7807 | §VII |
| Logging | `tracing` JSON | §IX |
| Metrics | `prometheus` on `/metrics` | §IX |

---

## 11. Phase Transition Architecture

```mermaid
flowchart TB
    subgraph p14["Phases 1-4 (Monolith)"]
        mono[bornemap-backend]
        pg1[(PostGIS)]
        rmq1[[RabbitMQ P3+]]
        red1[[Redis P4+]]
        mono --> pg1
        mono --> rmq1
        mono --> red1
    end

    subgraph p5["Phase 5 (Identity Hardened)"]
        mono2[bornemap-backend]
        kc[Keycloak]
        pg2[(PostGIS)]
        mono2 --> kc
        mono2 --> pg2
    end

    subgraph p6["Phase 6 (Extraction)"]
        bff[BFF / API Gateway]
        ss[station-service]
        is[identity-service]
        ps[profile-service]
        rs[review-service]
        es[event-service]
        pgss[(station_db)]
        pgis[(identity_db)]
        pgps[(profile_db)]
        pgrs[(review_db)]
        pges[(event_db)]
        bff -->|gRPC tonic| ss
        bff -->|gRPC tonic| is
        bff -->|gRPC tonic| ps
        bff -->|gRPC tonic| rs
        bff -->|gRPC tonic| es
        ss --> pgss
        is --> pgis
        ps --> pgps
        rs --> pgrs
        es --> pges
    end

    p14 --> p5 --> p6
```

**Invariants across all phases:**

- Token claim shape (frozen).
- Coordinate format `[lng, lat]` (frozen).
- OpenAPI as the contract (frozen).
- PostGIS as the spatial engine (frozen).

---

## 12. Repository Structure (Implementation-Ready)

```text
bornemap/
├── .github/workflows/
│   ├── ci.yml
│   └── openapi-diff.yml
├── .specify/
│   └── memory/constitution.md   # canonical Speckit constitution
├── backend/
│   ├── Cargo.toml               # workspace
│   ├── Cargo.lock
│   ├── api-gateway/             # BFF (Phase 6+)
│   ├── libs/
│   │   ├── common-utils/        # newtypes, errors, time helpers
│   │   └── openapi-spec/        # utoipa harness; emits openapi.json
│   └── services/
│       ├── station-service/
│       │   ├── src/
│       │   │   ├── handlers/    # Actix routes + utoipa annotations
│       │   │   ├── services/    # business logic; trait-based DI
│       │   │   ├── repositories/# sqlx queries
│       │   │   ├── models/      # DTOs + newtypes
│       │   │   └── lib.rs
│       │   └── migrations/      # *.sql per migration
│       ├── identity-service/
│       ├── profile-service/
│       ├── review-service/
│       └── event-service/
├── frontend/
│   ├── admin-portal/            # React + Vite
│   │   ├── src/
│   │   │   ├── features/
│   │   │   │   ├── map/
│   │   │   │   ├── stations/
│   │   │   │   ├── reviews/
│   │   │   │   └── invitations/
│   │   │   ├── styles/map-theme.constants.ts
│   │   │   └── main.tsx
│   │   └── vite.config.ts
│   ├── mobile-app/              # Expo React Native
│   │   ├── src/
│   │   │   ├── screens/
│   │   │   ├── components/
│   │   │   └── styles/map-theme.constants.ts
│   │   └── app.config.ts
│   └── packages/
│       ├── api-client/          # generated OpenAPI client
│       └── geo-models/          # CoordinateModel, quantizeBounds, etc.
├── infrastructure/
│   ├── docker-compose.local.yml
│   └── terraform/
├── docs/
│   ├── constitution.md
│   ├── plan.md
│   ├── architecture.md
│   └── adr/                     # one file per decision
└── specs/
    ├── 001-mvp1-geo-core/
    ├── 002-mvp2-user-interaction/
    ├── 003-mvp3-async-foundation/
    ├── 004-mvp4-performance/
    ├── 005-mvp5-identity/
    └── 006-mvp6-scale/
```

---

## 13. Deployment Topology (Production)

```text
[ PUBLIC WEB TRAFFIC ]
         │
         ▼ (443 / TLS termination at the edge)
┌────────────────────────────────────────────────────────────────────────┐
│ PUBLIC SUBNET (DMZ)                                                    │
│   Admin Portal (static · CDN)     Mobile Client (App Store / Play)     │
└─────────────────────────────────────┬──────────────────────────────────┘
                                      ▼ (internal routing)
┌─────────────────────────────────────┴──────────────────────────────────┐
│ APPS & RUNTIMES SUBNET (PRIVATE)                                       │
│   bornemap-backend / BFF      bornemap-worker      Redis      RabbitMQ │
└─────────────────────────────────────┬──────────────────────────────────┘
                                      ▼
┌─────────────────────────────────────┴──────────────────────────────────┐
│ DATA STORAGE SUBNET (ISOLATED)                                         │
│   Keycloak           MinIO / S3            PostgreSQL + PostGIS        │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 14. Local Developer Loop

`infrastructure/docker-compose.local.yml` (canonical for MVP 1):

```yaml
version: '3.8'

services:
  bornemap-database:
    image: postgis/postgis:16-3.4
    environment:
      POSTGRES_DB: bornemap_dev
      POSTGRES_USER: bornemap
      POSTGRES_PASSWORD: bornemap
    volumes:
      - pgdata:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  pgdata:
```

Backend `.env` (local):

```bash
DATABASE_URL=postgres://bornemap:bornemap@localhost:5432/bornemap_dev
RUST_LOG=info,bornemap=debug
MOCK_JWT_SECRET=dev-secret-do-not-use-in-prod
BIND_ADDR=0.0.0.0:8000
```

Frontend `.env`:

```bash
VITE_API_BASE_URL=http://localhost:8000
```

---

## 15. ADR Index (To Be Maintained)

Architectural Decision Records live under `docs/adr/`. Each MVP adds at least one ADR for any non-obvious choice. Suggested initial entries:

- `0001-postgis-as-spatial-engine.md`
- `0002-actix-web-and-sqlx.md`
- `0003-utoipa-openapi-source-of-truth.md`
- `0004-react-leaflet-vs-mapbox.md`
- `0005-expo-managed-vs-dev-client.md`
- `0006-rabbitmq-vs-kafka.md` (Phase 3 trigger)
- `0007-redis-spatial-cache-shape.md` (Phase 4 trigger)
- `0008-keycloak-deployment-topology.md` (Phase 5 trigger)
- `0009-microservice-extraction-justification.md` (Phase 6 trigger; required by constitution)

---

## 16. Non-Goals Restated

Per Constitution §2 / Non-Goals (NG-1..NG-7). Any feature crossing those lines requires a constitution amendment.
