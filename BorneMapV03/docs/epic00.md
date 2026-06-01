# EPIC 0 — SYSTEM CONSTITUTION: Architecture, Contracts & Delivery Pipeline

## Epic Key

`ARCH-EPIC-0`

## Priority

Critical (Hard Blocker for all downstream work)

## Type

Foundation / System Constitution

---

## 1. PURPOSE

Define the complete deterministic contract layer of the EV platform before implementation.

This includes:

- Service architecture
- Data ownership (PostgreSQL schemas)
- Identity & RBAC model
- Event streaming system (Clickstream)
- CI/CD pipeline rules
- API standards
- Observability model
- Security model
- Caching strategy
- Migration governance
- Data lifecycle rules
- Environment model

This epic ensures the system is fully specified, reproducible, and operable before any code is written.

---

## 2. SYSTEM ARCHITECTURE CONTRACTS

### 2.1 Services

- Keycloak (Identity Provider)
- Admin Service (system of record for inventory)
- Driver Service (discovery + user actions)
- Clickstream Service (event ingestion)
- GIS Sync Worker (spatial projection system)

### 2.2 Communication Model

**Allowed:**
- REST (frontend ↔ services)
- RabbitMQ (async events)
- DB access only within owning service

**Forbidden:**
- cross-service DB access
- inventory writes outside Admin Service
- GIS writes outside worker

### 2.3 Data Ownership Matrix

| Schema | Owner Service |
|--------|--------------|
| `inventory` | Admin Service |
| `users` | Driver + Admin |
| `gis` | GIS Worker |
| `analytics` | Clickstream Service |

### 2.4 Architectural Invariant

> Inventory is the single source of truth for physical infrastructure. All other systems are derived projections.

---

## 3. IDENTITY & RBAC CONTRACT

### 3.1 Roles

- `registered_driver`
- `partner`
- `admin`

### 3.2 Enforcement layers

1. Keycloak (authentication + claims)
2. Service layer (authorization)
3. DB constraints (final enforcement)

### 3.3 Partner rule

> A user belongs to exactly one partner.

---

## 4. POSTGRESQL SCHEMA CONTRACTS

### 4.1 `inventory` (Admin-owned)

Tables:
- `partner`
- `station` (PostGIS POINT)
- `charger`
- `station_availability`

**Rules:**
- Admin Service is sole writer
- `station` is spatial root entity

### 4.2 `users` (Driver + Admin)

Tables:
- `user_account`
- `user_profile`
- `partner_membership`
- `favorite_station`
- `station_review`

**Rules:**
- favorites are composite PK
- reviews linked to station + user

### 4.3 `gis` (Derived only)

Tables:
- `roads`
- `boundaries`
- `station_geospatial_view`

**Rules:**
- derived from `inventory.station`
- fully rebuildable

### 4.4 `analytics` (Clickstream)

Tables:
- `raw_event` (partitioned)
- `daily_event_count`
- `station_daily_metric`
- `search_daily_metric`

**Rules:**
- append-only ingestion
- JSONB only for flexible payload
- partition by time

---

## 5. CLICKSTREAM EVENT CONTRACT

### 5.1 Event envelope

```json
{
  "event_id": "string",
  "event_type": "string",
  "timestamp": "ISO-8601",
  "session_id": "string",
  "actor_id": "string|null",
  "platform": "web|mobile|admin",
  "payload": {}
}
```

### 5.2 Event types (v1)

- `station_viewed`
- `station_searched`
- `map_moved`
- `favorite_added`
- `favorite_removed`
- `review_created`
- `review_deleted`
- `auth_login_success`
- `auth_login_failed`

### 5.3 Rules

- schema versioned (v1)
- no secrets in payload
- at-least-once delivery

---

## 6. API CONTRACT STANDARD

### 6.1 API rules

- REST only
- JSON only
- versioned endpoints `/v1/`

### 6.2 Pagination

- cursor-based only

### 6.3 Error format

```json
{
  "error_code": "string",
  "message": "string",
  "trace_id": "string"
}
```

---

## 7. CI/CD CONTRACT

### 7.1 Philosophy

- CI is mandatory
- no auto-deployment
- only artifact generation

### 7.2 Pipeline stages

1. Lint
2. Test
3. Build
4. Contract validation
5. Docker build
6. GHCR publish

### 7.3 Build rules

**Backend:**
- `cargo fmt`
- `clippy` (`-D warnings`)
- tests required

**Frontend:**
- lint
- build required

### 7.4 Artifact rules

- `ghcr.io/<service>:<git-sha>`
- deterministic builds required

### 7.5 Security rules

- no secrets in images
- GH secrets only
- no env leakage in logs

---

## 8. OBSERVABILITY CONTRACT (NEW)

### 8.1 Logging standard

Structured JSON logs.

Required fields:
- `service_name`
- `request_id`
- `trace_id`
- `user_id` (if exists)
- `event_type`

### 8.2 Metrics

- request latency (p95/p99)
- error rates
- GIS sync lag
- clickstream ingestion lag
- DB query latency

### 8.3 Tracing

- `trace_id` must propagate across services
- at minimum: Driver → Clickstream → Analytics

---

## 9. CACHING CONTRACT (NEW)

### 9.1 Cacheable data

- nearby stations
- station details
- map markers
- search results

### 9.2 Invalidation rules

- station update → invalidate station cache
- availability update → partial refresh
- GIS sync → regional invalidation

---

## 10. SECURITY CONTRACT (NEW)

### 10.1 Trust boundaries

- Keycloak = identity source
- Services = authorization enforcement
- DB = final enforcement layer

### 10.2 Token rules

- minimal JWT claims
- no sensitive data in token

### 10.3 Rate limiting

- public endpoints: strict limit
- auth endpoints: stricter
- ingestion endpoints: throttled

---

## 11. MIGRATION GOVERNANCE (NEW)

### 11.1 Rules

- only Admin Service writes inventory migrations
- no manual DB changes in production
- all migrations versioned

### 11.2 Safety

- backward-compatible changes only
- destructive migrations require explicit versioning

---

## 12. DATA LIFECYCLE CONTRACT (NEW)

### 12.1 Retention

- `raw_event`: 30–90 days hot retention
- logs: 7–14 days
- aggregates: permanent

### 12.2 Deletion model

- soft delete only for users/stations/reviews

---

## 13. ENVIRONMENT CONTRACT (NEW)

### 13.1 Environments

- local (docker compose)
- production (bare metal)

> NO staging environment.

### 13.2 Rules

- same images everywhere
- config only via env vars
- no environment branching in code

---

## 14. DATABASE CONTRACT DOCUMENTS

Must produce:

- `inventory` schema spec
- `users` schema spec
- `gis` schema spec
- `analytics` schema spec

---

## 15. CI/CD + ARCHITECTURE OUTPUTS

Required artifacts:

- `architecture-contract.md`
- `service-matrix.md`
- `event-spec-v1.md`
- `rbac-model.md`
- `id-strategy.md`
- `communication-rules.md`
- `ci-cd-contract.md`
- `database-schema-contract.md`

---

## 16. ACCEPTANCE CRITERIA

EPIC 0 is **COMPLETE ONLY IF**:

**Architecture**
- service boundaries finalized
- communication model defined
- invariants defined

**Data**
- all schemas defined
- ownership rules defined
- PostGIS usage defined
- partitioning strategy defined

**Identity**
- RBAC model finalized
- partner scoping defined

**Event system**
- clickstream envelope defined
- event types defined
- delivery rules defined

**CI/CD**
- full pipeline defined
- GHCR rules defined
- failure policy defined
- build rules defined

**Observability**
- logging standard defined
- metrics defined
- tracing rules defined

**Security**
- trust boundaries defined
- rate limiting defined
- token rules defined

**Caching**
- cache strategy defined
- invalidation rules defined

**Migration + lifecycle**
- migration governance defined
- retention rules defined
- soft delete rules defined

---

## FINAL ONE-LINE DEFINITION

> EPIC 0 defines the complete deterministic system constitution of the EV platform, covering architecture, data, identity, events, CI/CD, observability, security, caching, and lifecycle rules—ensuring zero ambiguity before implementation begins.
