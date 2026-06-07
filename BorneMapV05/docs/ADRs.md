Architecture Decision Records (ADR v1.0)
ADR-001 — Monorepo Architecture (Rust + TypeScript Hybrid)
Status

Accepted

Context

The platform includes:

Backend services (Rust)
Frontend apps (React + React Native)
Shared contracts (events, APIs, types)

We need a unified structure that avoids cross-repo fragmentation.

Decision

We adopt a single monorepo containing:

Rust workspace (/services, /crates)
TypeScript workspace (/apps, /packages)
Shared contracts (event-taxonomy, api-contracts)
Consequences
Positive
Single source of truth
Easier cross-service refactoring
Shared type safety between backend and frontend
Negative
CI complexity increases
Larger repository size
ADR-002 — Backend Language Choice (Rust for Core Services)
Status

Accepted

Context

Services require:

high performance (GIS, event ingestion)
memory safety
predictable latency
Decision

Use Rust for all backend services:

Driver Service
Admin Service
Clickstream Service
GIS Worker
Analytics Writer
Consequences
Positive
High reliability
Strong type safety
Low runtime overhead
Negative
Higher initial development complexity
Steeper learning curve
ADR-003 — Authentication System (Keycloak as Single Identity Provider)
Status

Accepted

Context

We need:

OAuth2 / OpenID Connect
social login (Google, Facebook)
role-based access control
Decision

Use Keycloak as the only identity provider

No custom auth system in platform DB
JWT-based validation only
Consequences
Positive
Centralized authentication
Standard OAuth compliance
Reduced security risk
Negative
External dependency complexity
Operational overhead for Keycloak
ADR-004 — Database Separation Strategy (3 DB Model)
Status

Accepted

Context

The system requires separation between:

transactional data
identity data
analytics data
Decision

Use 3 PostgreSQL databases:

keycloak_db → identity only
platform_db → business + GIS
analytics_db → event analytics
Consequences
Positive
Clear separation of concerns
Independent scaling per domain
Reduced cross-domain corruption risk
Negative
Cross-db joins impossible
Requires duplication in analytics layer
ADR-005 — Event-Driven Architecture (RabbitMQ Backbone)
Status

Accepted

Context

System requires:

async processing (GIS sync, analytics)
decoupled services
reliable event ingestion
Decision

Use RabbitMQ as the event backbone

Event flow:

Clickstream → RabbitMQ → Analytics Writer
Admin/Driver → Outbox → GIS Worker
Consequences
Positive
Decoupled services
Scalable ingestion pipeline
Fault isolation
Negative
Event consistency complexity
Requires idempotency everywhere
ADR-006 — Clickstream System as Canonical Event Source
Status

Accepted

Context

We need a unified analytics model.

Decision

All frontend apps emit events to:

👉 Clickstream Service

It is responsible for:

validation
normalization
publishing to RabbitMQ
Consequences
Positive
Centralized event governance
Schema enforcement point
Easier analytics consistency
Negative
Single ingestion bottleneck (mitigated via scaling)
ADR-007 — GIS System Using Outbox Pattern
Status

Accepted

Context

Station and charger updates must reflect in spatial layer.

Decision

Use Outbox Pattern + GIS Worker

Flow:

DB mutation
outbox event inserted
GIS Worker consumes
updates spatial layer
Consequences
Positive
eventual consistency
retry-safe updates
decoupled GIS processing
Negative
eventual consistency delays
more infrastructure complexity
ADR-008 — API Style (Pure REST Only)
Status

Accepted

Context

System must support:

4 frontend apps
mobile + web
simple integration
Decision

Use REST only APIs

No GraphQL.

Consequences
Positive
simple caching
predictable endpoints
easier debugging
Negative
potential over-fetching
multiple endpoints required
ADR-009 — Frontend Stack (React + Vite, No Next.js)
Status

Accepted

Context

We need:

fast UI iteration
map-heavy UI (Leaflet)
multi-app architecture
Decision

Use:

React + Vite (web apps)
React Native Expo (mobile)

No Next.js.

Consequences
Positive
simpler architecture
full control of routing and state
better separation between apps
Negative
no SSR (not required for this use case)
ADR-010 — Map System (Leaflet as Core Mapping Engine)
Status

Accepted

Context

Platform requires:

station clustering
viewport-based loading
GIS interaction
Decision

Use Leaflet for all web mapping

Consequences
Positive
lightweight
highly customizable
stable ecosystem
Negative
less advanced than Mapbox GL for 3D (not needed here)
ADR-011 — Tenant Isolation Model (Partner Scoped Access)
Status

Accepted

Context

Partners must only access their own stations.

Decision

Enforce server-side tenant isolation only

Rule:

partner_id derived from membership
never accepted from client
Consequences
Positive
strong security boundary
prevents privilege escalation
Negative
requires strict backend enforcement everywhere
ADR-012 — Soft Delete Strategy (All Business Entities)
Status

Accepted

Context

Data must be recoverable and auditable.

Decision

Use soft deletes only for:

stations
partners
reviews
Consequences
Positive
auditability
rollback capability
safer analytics consistency
Negative
increased query complexity
ADR-013 — Analytics Storage Model (Partitioned Event Tables)
Status

Accepted

Context

Event volume must scale over time.

Decision
store events in analytics_db.raw_event
partition by month
deduplicate via event_id
Consequences
Positive
scalable ingestion
efficient queries by time range
Negative
operational complexity in partition management
ADR-014 — Configuration Model (Env-Driven, No Dynamic Config)
Status

Accepted

Context

Need predictable runtime behavior.

Decision
environment variables only
no runtime config service
no remote feature flag system (MVP)
Consequences
Positive
predictable deployments
simpler debugging
Negative
no real-time config changes
ADR-015 — Deployment Model (Docker Compose + Bare Metal)
Status

Accepted

Context

No Kubernetes in current phase.

Decision
Docker Compose orchestration
bare metal deployment
Traefik as ingress
Consequences
Positive
simple ops model
low infrastructure overhead
Negative
manual scaling required
ADR-016 — Error Handling Model (Strict Typed Errors)
Status

Accepted

Decision

All APIs use:

{
  "success": false,
  "error": {
    "code": "...",
    "message": "..."
  }
}

No mixed formats.

Consequences
consistent frontend handling
easier observability
📌 Summary

This ADR set establishes:

strict service boundaries
event-driven architecture backbone
GIS + analytics separation
REST-first API model
Rust-based backend consistency
Keycloak-only identity model
bare-metal operational simplicity
