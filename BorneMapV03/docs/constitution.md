# BorneMap Platform Constitution (v5.1)

## 1. Purpose

BorneMap is an EV charging station discovery and management platform initially built for Tunisia.

### Supported capabilities

- Public station discovery (web + mobile)
- Map visualization + markers
- Nearby station search (GIS-based)
- Station search and filtering
- Station details
- Ratings and reviews
- Favorites (registered users)
- Partner station & charger management
- Manual availability updates
- Reporting (admin/partner scoped)
- GIS synchronization pipeline
- Clickstream analytics pipeline

### Explicitly out of scope (MVP)

- OCPP protocol integration
- Charging sessions
- Payments / billing
- Real-time charging control
- Routing/navigation engine

## 2. Core Principles

### 2.1 Pragmatic architecture

- Minimize deployable services
- Avoid premature microservices fragmentation

### 2.2 Clear ownership boundaries

Every component must have a single source of truth and explicit responsibility.

### 2.3 Operational simplicity (Phase 1)

- Bare metal deployment
- Docker Compose
- Traefik as reverse proxy
- Manual deployment
- GitHub Container Registry (GHCR)

### 2.4 Evolution over complexity

- Optimize for fast iteration
- Introduce complexity only when justified by real load or domain needs

### 2.5 Data separation in PostgreSQL

Even within one database instance:

- **inventory** → transactional business data
- **users** → identity-linked user data
- **gis** → geospatial enrichment layer
- **analytics** → event + aggregation layer

## 3. Product Access Model

### 3.1 Public Driver (anonymous)

Not a role — this is unauthenticated access.

**Can:**

- Browse stations
- View map markers
- Search/filter stations
- View station details
- View public ratings & reviews

**Cannot:**

- Favorite stations
- Write reviews
- Access personal data

### 3.2 Registered Driver (`registered_driver`)

Authenticated via Keycloak.

**Can:**

- All public capabilities
- Manage favorites
- Create/update/delete own reviews
- Manage profile

### 3.3 Partner (`partner`)

Authenticated via Keycloak.

**Constraints:**

- Belongs to exactly one partner organization
- Strict tenant isolation (`partner_id` enforced everywhere)

**Can:**

- Manage own stations
- Manage chargers under owned stations
- Update availability (manual)
- Access partner dashboard
- View scoped analytics/reporting

> ⚠ Partner type (`business` | `private`) is metadata only, NOT authorization.

### 3.4 Admin (`admin`)

**Can:**

- Full platform access
- Manage users, partners, stations, chargers
- Moderate reviews
- Access global analytics
- Trigger GIS synchronization

## 4. Role System Rules

System has exactly three roles:

- `registered_driver`
- `partner`
- `admin`

**Hard rule**: No additional roles may be introduced without explicit architectural approval.

## 5. Frontend Applications

### 5.1 Applications

- Driver Web App (React + Vite)
- Driver Mobile App (React Native + Expo)
- Partner Dashboard (React + Vite)
- Admin Dashboard (React + Vite)

### 5.2 Shared frontend system

Shared across web apps:

- Design tokens
- UI component library
- API client conventions
- Auth/session handling
- Shared TypeScript types

Mobile shares:

- Design tokens
- API client patterns
- Core domain types

## 6. Deployment Model

### 6.1 Runtime

- Bare metal infrastructure
- Docker Compose
- Traefik (only public entrypoint)

### 6.2 Networking rule

Only Traefik exposes public ports. All services remain internal.

### 6.3 CI/CD

GitHub Actions handles:

- lint
- test
- build
- Docker image build
- GHCR push

> 🚫 No automatic production deployment.

### 6.4 Deployment process (manual)

Operator:

1. Pulls images from GHCR
2. Runs DB migrations
3. Restarts services via Docker Compose
4. Executes smoke tests

## 7. Repository Architecture

### 7.1 Monorepo

Single repository for entire system.

### 7.2 Rust workspace

All backend services + shared crates in one workspace.

### 7.3 Central migrations

Single migration system for PostgreSQL.

> 🚫 No per-service migration ownership.

### 7.4 Shared crates

- auth
- config
- errors
- id generation
- observability
- shared types
- API clients (frontend)

## 8. Backend Services

Core services:

- Keycloak (identity provider)
- Driver Service
- Admin Service
- Clickstream Service
- GIS Sync Worker
- Traefik (edge)

### 8.1 Keycloak

**Owns:**

- Authentication
- Tokens
- Sessions
- OAuth login providers
- Role assignment

**Does NOT own:**

- Profiles
- Favorites
- Reviews
- Partner business logic

### 8.2 Driver Service

**Supports:**

- Public station discovery
- Nearby search
- Station details
- Favorites
- Reviews
- User profile

**Rule:** Separate public vs authenticated endpoints explicitly.

### 8.3 Admin Service

**Handles:**

- Partner management
- Station management
- Charger management
- Moderation
- Reporting
- GIS sync triggers

**Supports:**

- Partner-scoped operations
- Admin-global operations

### 8.4 Clickstream Service

**Responsible for:**

- Event ingestion
- Validation
- RabbitMQ publishing

> 🚫 Must NOT handle business data.

### 8.5 GIS Sync Worker

**Responsible for:**

- Reading `inventory.station`
- Computing GIS enrichments
- Updating `gis` schema artifacts

> 🚫 Never source of truth.

## 9. PostgreSQL Architecture

### Schemas (fixed set)

- `inventory`
- `users`
- `gis`
- `analytics`

> 🚫 No new schemas allowed without approval.

### 9.1 `inventory` (source of truth)

**Contains:**

- `partner`
- `station`
- `charger`
- `station_availability`

**Rules:**

- `inventory.station` is canonical truth
- Includes geometry (`geom`)
- Never duplicated in GIS schema

### 9.2 `users`

**Contains:**

- `user_account`
- `user_profile`
- `partner_membership`
- `favorite_station`
- `station_review`

**Rules:**

- Public users do not exist here
- One partner membership per partner user
- Favorites and reviews belong here

### 9.3 `gis`

**Contains:**

- OSM imports
- roads
- boundaries
- derived spatial layers
- materialized views

**Rule:** GIS is enrichment layer only.

### 9.4 `analytics`

**Contains:**

- clickstream raw events
- aggregates
- reporting tables

**Rules:**

- Partitioned by time
- JSONB for flexible payloads
- Must never affect transactional schemas

## 10. Analytics Architecture

**Pipeline:**

Frontend → Clickstream Service → RabbitMQ → Analytics Consumers → PostgreSQL (`analytics`)

**Rules:**

- No MongoDB
- Event-driven ingestion
- Decoupled storage

## 11. GIS Synchronization

**Trigger:** station create/update in `inventory.station`

**Flow:**

1. Commit business transaction
2. Emit event (outbox pattern preferred)
3. GIS worker processes asynchronously
4. Update `gis` artifacts

**Rules:**

- Idempotent processing required
- GIS never writes back to inventory

## 12. Identity Rules

- Keycloak is the only identity provider
- `users.user_account.keycloak_user_id` is canonical link
- Provisioning on login:
  - Create `user_account` if missing
  - Create `user_profile` if missing

## 13. Partner Isolation (Critical Rule)

All partner queries MUST enforce `partner_id` filter at **repository level**. No exception at API layer.

**Violation = architectural defect.**

## 14. Availability Model

- Manual updates only (MVP)
- Stored in `inventory.station_availability`
- Future automation allowed but must preserve API contract.

## 15. Identifier System

All entities use NanoID with prefixes:

- `USR-`
- `PRT-`
- `STN-`
- `CHG-`
- `REV-`

Must appear consistently in: DB, APIs, logs, events, UI.

## 16. Geographic Scope

- Initial scope: Tunisia
- OSM import: batch only
- No real-time sync

## 17. Routing

> 🚫 Explicitly out of scope for MVP

## 18. Engineering Standards

### Clean Architecture (Rust)

**Layers:**

- domain
- application
- infrastructure
- interfaces

**Rules:**

- `domain` cannot depend on frameworks
- `infrastructure` adapts external systems
- `interfaces` handles HTTP/worker entrypoints

## 19. Testing Requirements

**Mandatory:**

- unit tests
- integration tests
- auth tests
- DB query tests
- smoke tests

## 20. Security Rules

- Traefik only public entrypoint
- JWT required for protected routes
- Partner isolation mandatory
- No secrets in repo
- Public endpoints explicitly declared

## 21. Observability

Each service must provide:

- structured logs
- health endpoints
- readiness endpoints

## 22. Definition of Done

A feature is complete only if:

- tests included
- auth rules validated
- migrations included
- CI passes
- no secrets exposed
- documentation updated

## 23. Non-Negotiable Rules

- `inventory.station` is source of truth
- `gis` is not authoritative
- `analytics` is isolated schema
- RabbitMQ used for clickstream
- Traefik is only public gateway
- Partner scoping is mandatory everywhere
- No routing in MVP
- Minimal service count enforced
- Keycloak is the only identity boundary
