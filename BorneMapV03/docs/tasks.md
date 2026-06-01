# BorneMap Platform — Task Breakdown

## EPIC 0 — Architecture Freeze & Contract Definition

**Goal:** Lock all system contracts before any implementation begins
**Depends on:** None
**Blocks:** Everything else
**Status:** ✅ COMPLETE — All 8 contract documents finalized at `specs/001-architecture-contracts/contracts/`

### Tickets

- **ARCH-1** — Finalize service boundary contracts
  - Define Admin / Driver / Clickstream / GIS Worker responsibilities
  - Lock "single writer per schema" rule
- **ARCH-2** — Define PostgreSQL schema contracts
  - `inventory` / `users` / `gis` / `analytics` definitions
  - ownership rules per schema
- **ARCH-3** — Define event system contract (Clickstream v1)
  - event envelope structure
  - event types list
  - JSONB payload rules
- **ARCH-4** — Define RBAC model in Keycloak
  - `registered_driver`
  - `partner`
  - `admin`
  - scope rules per role
- **ARCH-5** — Define ID strategy standard
  - USR / PRT / STN / CHG / REV prefixes
  - NanoID format rules
- **ARCH-6** — Define service communication rules
  - sync vs async
  - RabbitMQ usage rules
  - no direct DB cross-service writes

---

## EPIC 1 — Monorepo + Workspace Bootstrap

**Goal:** Establish codebase structure
**Depends on:** EPIC 0

### Tickets

- **MONO-1** — Initialize monorepo structure
  - `apps/`
  - `services/`
  - `crates/`
  - `packages/`
- **MONO-2** — Initialize Rust workspace
  - `driver-service`
  - `admin-service`
  - `clickstream-service`
  - `gis-sync-worker`
- **MONO-3** — Initialize frontend apps (Vite + Expo)
  - `driver-web`
  - `partner-dashboard`
  - `admin-dashboard`
  - `driver-mobile`
- **MONO-4** — Create shared contracts crate
  - DTOs
  - event schema types
  - role enums
- **MONO-5** — Setup linting + formatting pipeline
  - rustfmt
  - clippy
  - eslint
  - prettier
- **MONO-6** — Root scripts + Makefile
  - build all
  - test all
  - lint all

---

## EPIC 2 — Runtime Infrastructure (Docker + Traefik)

**Goal:** Local + bare-metal runtime foundation
**Depends on:** EPIC 1

### Tickets

- **RUNTIME-1** — Create Docker Compose baseline
  - Traefik
  - Keycloak
  - PostgreSQL + PostGIS
  - RabbitMQ
- **RUNTIME-2** — Configure Traefik routing
  - single entrypoint
  - route per service
  - TLS strategy (dev self-signed)
- **RUNTIME-3** — Internal network segmentation
  - backend network
  - public network rules
- **RUNTIME-4** — Environment configuration system
  - `.env` model
  - dev/prod separation rules (no staging)
- **RUNTIME-5** — Containerization of all services
  - `admin-service`
  - `driver-service`
  - `clickstream-service`
  - `gis-worker`

---

## EPIC 3 — CI/CD (Build + GHCR only)

**Goal:** Automated build and artifact publishing
**Depends on:** EPIC 1, EPIC 2

### Tickets

- **CI-1** — Rust CI pipeline
  - fmt check
  - clippy
  - test
  - build
- **CI-2** — Frontend CI pipeline
  - install
  - lint
  - build
- **CI-3** — Docker image build pipeline
  - multi-service build
  - GHCR push
- **CI-4** — Versioning strategy
  - git SHA tagging
  - `latest` tag for dev only

---

## EPIC 4 — Identity & Access (Keycloak Integration)

**Goal:** Authentication and RBAC foundation
**Depends on:** EPIC 2

### Tickets

- **AUTH-1** — Keycloak realm setup
  - roles creation
  - clients for web/mobile/admin
- **AUTH-2** — Social login federation
  - Google OAuth
  - Facebook OAuth
- **AUTH-3** — JWT validation middleware (Rust)
  - token parsing
  - role extraction
- **AUTH-4** — First login provisioning system
  - user creation in `users` schema
  - role mapping sync
- **AUTH-5** — Partner scope enforcement model
  - `partner_id` binding
  - DB constraint rules

---

## EPIC 5 — PostgreSQL Core Foundation (All Schemas)

**Goal:** Data backbone
**Depends on:** EPIC 4

### Tickets

- **DB-1** — Initialize PostGIS + base DB setup
  - extensions
  - migration system
- **DB-2** — Create `inventory` schema
  - `partner`
  - `station`
  - `charger`
  - `station_availability`
- **DB-3** — Add inventory constraints & indexes
  - partner ownership constraint
  - station geometry index
  - charger FK integrity
- **DB-4** — Create `users` schema
  - `user_account`
  - `user_profile`
  - `favorite_station`
  - `station_review`
  - `partner_membership`
- **DB-5** — Add user constraints
  - unique favorites
  - review uniqueness rules
- **DB-6** — Create `gis` schema
  - `roads`
  - `boundaries`
  - station projection tables
- **DB-7** — Create `analytics` schema
  - `raw_event`
  - `daily_event_count`
  - `station_daily_metric`
  - `search_daily_metric`
- **DB-8** — Enable partitioning for `raw_event`
  - time-based partitions
- **DB-9** — Implement ID system utilities
  - USR/PRT/STN prefix generators

---

## EPIC 6 — Admin Service (FIRST BUSINESS SERVICE)

**Goal:** System of record (inventory creation)
**Depends on:** EPIC 5

### Tickets

- **ADMIN-1** — Admin service bootstrap (Rust)
  - Axum setup
  - auth middleware integration
- **ADMIN-2** — Partner CRUD (scoped)
  - partner create/update/delete
  - `partner_id` enforcement
- **ADMIN-3** — Station CRUD
  - station creation
  - geometry storage
- **ADMIN-4** — Charger CRUD
  - charger management
  - station linking
- **ADMIN-5** — Availability updates
  - `station_availability` writes
- **ADMIN-6** — Review moderation (admin side)
  - delete/hide reviews
- **ADMIN-7** — Reporting foundation endpoints
  - raw counts from `inventory`/`users`

---

## EPIC 7 — GIS Sync Worker

**Goal:** Derived spatial system
**Depends on:** EPIC 6

### Tickets

- **GIS-1** — Worker bootstrap
  - RabbitMQ consumer setup
- **GIS-2** — Outbox pattern implementation
  - inventory change tracking
- **GIS-3** — Station → GIS projection sync
  - station geometry transformation
- **GIS-4** — OSM Tunisia import pipeline
  - roads import
  - boundaries import
- **GIS-5** — Idempotency + retry system
  - safe reprocessing

---

## EPIC 8 — Driver Service (Discovery Layer)

**Goal:** Public + authenticated driver APIs
**Depends on:** EPIC 6, EPIC 7

### Tickets

- **DRIVER-1** — Service bootstrap
  - Axum setup
- **DRIVER-2** — Public station discovery API
  - nearby stations
  - map markers
- **DRIVER-3** — Station details aggregation
  - `inventory` + `gis` join
- **DRIVER-4** — Search and filter system
  - indexed queries
- **DRIVER-5** — Favorites system (`users` schema)
  - add/remove favorites
- **DRIVER-6** — Reviews system
  - create/update/delete reviews
- **DRIVER-7** — Public vs authenticated separation
  - route guards

---

## EPIC 9 — Shared Web Platform (Design System + API Client)

**Goal:** Reusable frontend foundation
**Depends on:** EPIC 8

### Tickets

- **WEB-1** — Design system foundation
  - tokens
  - components
- **WEB-2** — Auth client (Keycloak integration)
  - JWT handling
  - role detection
- **WEB-3** — API client layer
  - typed SDK
- **WEB-4** — Layout system
  - dashboards layout
  - public layout

---

## EPIC 10 — Driver Web App

**Depends on:** EPIC 9

### Tickets

- **WEB-DRIVER-1** — Map + station browsing
- **WEB-DRIVER-2** — Station details page
- **WEB-DRIVER-3** — Search/filter UI
- **WEB-DRIVER-4** — Favorites UI
- **WEB-DRIVER-5** — Reviews UI
- **WEB-DRIVER-6** — Auth flows (login/register)

---

## EPIC 11 — Partner Dashboard

**Depends on:** EPIC 9, EPIC 6

### Tickets

- **WEB-PARTNER-1** — Partner station management
- **WEB-PARTNER-2** — Charger management UI
- **WEB-PARTNER-3** — Availability updates UI
- **WEB-PARTNER-4** — Partner reporting view

---

## EPIC 12 — Admin Dashboard

**Depends on:** EPIC 9, EPIC 6

### Tickets

- **WEB-ADMIN-1** — Partner management UI
- **WEB-ADMIN-2** — Station management UI
- **WEB-ADMIN-3** — User management UI
- **WEB-ADMIN-4** — Review moderation UI
- **WEB-ADMIN-5** — Reporting dashboard

---

## EPIC 13 — Driver Mobile App (Expo)

**Depends on:** EPIC 8, EPIC 9

### Tickets

- **MOBILE-1** — App bootstrap
- **MOBILE-2** — Map + nearby stations
- **MOBILE-3** — Station details
- **MOBILE-4** — Favorites
- **MOBILE-5** — Reviews
- **MOBILE-6** — Auth flows

---

## EPIC 14 — Clickstream System

**Goal:** Analytics ingestion pipeline
**Depends on:** EPIC 8

### Tickets

- **ANALYTICS-1** — Clickstream ingestion API
- **ANALYTICS-2** — Event schema validation
- **ANALYTICS-3** — RabbitMQ publisher
- **ANALYTICS-4** — Analytics consumer worker
- **ANALYTICS-5** — `raw_event` persistence
- **ANALYTICS-6** — Aggregation jobs (daily metrics)

---

## EPIC 15 — Reporting Layer (Admin Service Extension)

**Depends on:** EPIC 14

### Tickets

- **REPORT-1** — Inventory reporting queries
- **REPORT-2** — User analytics reporting
- **REPORT-3** — Clickstream dashboards queries
- **REPORT-4** — Partner scoped reporting

---

## EPIC 16 — Hardening & Production Readiness

**Depends on:** EPIC 10–15

### Tickets

- **PROD-1** — Manual deployment runbook
- **PROD-2** — Backup & restore procedures
- **PROD-3** — Performance tuning (PostGIS + analytics)
- **PROD-4** — Observability setup (logs + metrics)
- **PROD-5** — Security audit (RBAC + scopes)
- **PROD-6** — Load testing (discovery APIs)

---

## Dependency Graph (simplified)

```
ARCHITECTURE
   ↓
MONOREPO
   ↓
RUNTIME + CI
   ↓
AUTH + DB
   ↓
ADMIN SERVICE
   ↓
GIS WORKER
   ↓
DRIVER SERVICE
   ↓
WEB PLATFORM
   ↓
MOBILE APP
   ↓
CLICKSTREAM
   ↓
REPORTING
   ↓
HARDENING
```
