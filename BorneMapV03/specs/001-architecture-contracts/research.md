# Research: Architecture Contracts

## Overview

This research validates technology and architecture decisions for the BorneMap
platform contract layer. All findings derive from the existing platform
constitution (`docs/constitution.md`, `docs/epic00.md`) and the clarified
specification (`spec.md`).

## Technology Decisions

### Backend Language: Rust (latest stable)

- **Decision**: Rust with async runtime (Tokio), Axum HTTP framework
- **Rationale**: Constitution mandates Rust for all backend services.
  Axum chosen for its ergonomic async handlers, tower middleware ecosystem,
  and strong typing for API contracts.
- **Alternatives considered**: None (constitution-enforced)

### Frontend: TypeScript + React (Vite) / React Native (Expo)

- **Decision**: Vite for web apps, Expo for mobile
- **Rationale**: Constitution section 5 specifies these explicitly.
  Shared packages for design tokens and UI components.
- **Alternatives considered**: None (constitution-enforced)

### Identity: Keycloak

- **Decision**: Keycloak as sole identity provider
- **Rationale**: Self-hosted, OIDC/OAuth2 compliant, supports social login
  federation (Google, Facebook), realm-based role management.
- **Alternatives considered**: Auth0 (SaaS, not aligned with bare-metal ops),
  Firebase Auth (vendor lock-in), custom JWT (operational overhead)

### Message Broker: RabbitMQ

- **Decision**: RabbitMQ for async event delivery
- **Rationale**: Lightweight, AMQP protocol, proven for clickstream ingestion.
  Constitution mandates RabbitMQ for clickstream pipeline.
- **Alternatives considered**: Kafka (overkill for MVP volume of <100K events/day),
  Redis Streams (less mature for guaranteed delivery)

### Database: PostgreSQL 16+ with PostGIS

- **Decision**: Single PostgreSQL instance with PostGIS extension
- **Rationale**: PostGIS provides native geospatial indexing (GIST indexes on
  POINT geometry), avoiding a separate geo-service. Single instance simplifies
  operations per "no premature microservices" principle. Schema-level isolation
  via four schemas.
- **Alternatives considered**: TimescaleDB (time-series focus not needed),
  MongoDB GeoSpatial (would violate "no MongoDB" rule), standalone geo-service
  (premature complexity)

### Reverse Proxy: Traefik

- **Decision**: Traefik as sole public entrypoint
- **Rationale**: Automatic TLS, Docker Compose label-based routing, health
  check integration. Constitution mandates Traefik-only public gateway.
- **Alternatives considered**: Nginx (manual config, no native Docker discovery),
  Caddy (less ecosystem support for Compose)

## Architecture Decisions from Clarification Session

### Data Volume & Scale

- **Decision**: Tunisia-wide, <500 stations, <50K users, <100K events/day MVP
- **Rationale**: Sufficient for initial rollout; modular growth path
- **Alternatives considered**: Smaller pilot (too limited for useful validation),
  global platform (premature)

### Performance Targets

- **Decision**: <500ms p95 discovery listings, <2s p99 geo-queries
- **Rationale**: Mobile-map UX standards; achievable with PostGIS spatial indexes
  + Redis cache layer without over-engineering
- **Alternatives considered**: <200ms aggressive (would require dedicated geo-cache),
  no targets (unacceptable for discovery UX)

### Compliance

- **Decision**: Tunisia Law 2004-63 on data protection
- **Rationale**: Initial scope is Tunisia; soft-delete and retention model aligns
- **Alternatives considered**: GDPR (adds consent/export/erasure — future scope)

### Station Lifecycle

- **Decision**: Active / Inactive (partner toggle) + soft-deleted (admin)
- **Rationale**: Simplest model for MVP; maintenance state can be added later
- **Alternatives considered**: 4-state model (Active/ Maintenance/ Inactive/ Retired)
  over-engineered for MVP

### Horizontal Scaling

- **Decision**: Stateless design, single-instance MVP, Compose scale-out later
- **Rationale**: "Evolution over complexity" principle — build stateless from
  day one, defer multi-replica until load requires it
- **Alternatives considered**: Multi-replica from start (premature ops complexity),
  stateful design (blocks future scaling)

## Contract Documents: Scope Confirmation

Based on EPIC 0 requirements (`docs/epic00.md` sections 14-15), the following
8 contract documents must be produced:

| Document | Content |
|----------|---------|
| `architecture-contract.md` | Service boundaries, communication model, invariants |
| `service-matrix.md` | Service descriptions, owned tables, tech stack |
| `event-spec-v1.md` | Clickstream event envelope, types, validation rules |
| `rbac-model.md` | Three roles, enforcement layers, partner isolation |
| `id-strategy.md` | NanoID prefixes, format rules |
| `communication-rules.md` | REST + RabbitMQ rules, sync vs async |
| `ci-cd-contract.md` | Pipeline stages, GHCR rules, build requirements |
| `database-schema-contract.md` | Four schemas, tables, constraints, partitioning |

No unresolved unknowns remain. All findings consolidated from spec + constitution.
