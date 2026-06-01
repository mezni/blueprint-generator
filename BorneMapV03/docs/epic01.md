# EPIC 1 — MONOREPO & WORKSPACE BOOTSTRAP

## Epic Key

`PLAT-EPIC-1`

## Title

Monorepo, Workspace & Development Foundation Setup

## Priority

Critical (Blocks all implementation work)

## Depends On

EPIC 0 — System Constitution (must be completed and frozen)

## Blocks

- EPIC 2 (Runtime Infrastructure)
- EPIC 3 (CI/CD Execution)
- All backend and frontend development

---

## 1. PURPOSE

Establish a fully standardized, reproducible monorepo workspace that supports:

- Rust backend services
- React + Vite web apps
- React Native Expo mobile app
- shared TypeScript + Rust contracts
- CI/CD integration
- consistent tooling across all environments

This epic ensures:

> "Every developer runs the same system, builds the same artifacts, and shares the same contracts."

---

## 2. SCOPE

This epic includes:

### 2.1 Repository structure

- Monorepo layout
- service isolation boundaries
- shared packages

### 2.2 Rust workspace initialization

- multi-service workspace
- shared crates

### 2.3 Frontend workspace initialization

- React + Vite apps
- Expo mobile app

### 2.4 Shared contract system

- API DTOs
- event schemas
- RBAC enums
- ID format definitions

### 2.5 Tooling standardization

- linting
- formatting
- type-checking
- build orchestration

### 2.6 Root automation layer

- Makefile / task runner
- CI alignment preparation

---

## 3. REPOSITORY ARCHITECTURE CONTRACT

### 3.1 Final monorepo structure (MANDATORY)

```
bornemap/
├── apps/
│   ├── driver-web/
│   ├── partner-dashboard/
│   ├── admin-dashboard/
│   ├── driver-mobile/
│
├── services/
│   ├── admin-service/
│   ├── driver-service/
│   ├── clickstream-service/
│   ├── gis-sync-worker/
│
├── crates/
│   ├── common-auth/
│   ├── common-config/
│   ├── common-db/
│   ├── common-errors/
│   ├── common-types/
│   ├── contracts/          ← CRITICAL (shared system contract)
│
├── packages/
│   ├── design-system/
│   ├── api-client/
│   ├── analytics-client/
│   ├── auth-client/
│
├── infra/
│   ├── docker/
│   ├── traefik/
│   ├── compose/
│
├── scripts/
├── docs/
├── .github/
├── Makefile
├── docker-compose.yml
└── README.md
```

### 3.2 Ownership rules

- `/services/*` → backend runtime services only
- `/apps/*` → frontend applications only
- `/crates/*` → Rust shared logic ONLY
- `/packages/*` → TypeScript shared logic ONLY
- `/infra/*` → deployment + runtime config only

---

## 4. RUST WORKSPACE SETUP

### 4.1 Workspace definition

All backend services must be part of a single Cargo workspace:

```toml
[workspace]
members = [
  "services/admin-service",
  "services/driver-service",
  "services/clickstream-service",
  "services/gis-sync-worker",
  "crates/common-*",
  "crates/contracts"
]
```

### 4.2 Required shared crates

| Crate | Responsibility |
|-------|----------------|
| `common-types` | Shared structs, enums, primitives |
| `common-auth` | JWT parsing, role extraction |
| `common-errors` | Unified error format |
| `common-db` | DB connection abstraction, Postgres helpers |
| `contracts` (CRITICAL) | API DTOs, event schemas, RBAC definitions, ID formats |

### 4.3 Constraint

- NO service defines its own duplicate DTOs
- ALL cross-service types must come from `/contracts`

---

## 5. FRONTEND WORKSPACE SETUP

### 5.1 Web apps (React + Vite)

Required apps:

- `driver-web`
- `partner-dashboard`
- `admin-dashboard`

### 5.2 Shared frontend packages

| Package | Responsibility |
|---------|----------------|
| `design-system` | UI components, tokens, layout primitives |
| `api-client` | Typed API wrapper, generated from contracts |
| `auth-client` | Keycloak integration, JWT handling |
| `analytics-client` | Clickstream event emitter |

### 5.3 Constraint

- NO app defines its own API types
- ALL types come from `contracts` or `api-client`

---

## 6. MOBILE APP SETUP (Expo)

### 6.1 App

- `driver-mobile` (React Native Expo)

### 6.2 Shared constraints

- uses same `api-client`
- uses same `auth-client`
- uses same event contract definitions

### 6.3 Rule

Mobile must be API-consistent with web apps at type level.

---

## 7. SHARED CONTRACT SYSTEM (CRITICAL CORE)

### 7.1 `contracts` crate responsibilities

Must define:

- **API DTOs**: `StationDTO`, `UserDTO`, `PartnerDTO`, `ReviewDTO`
- **Event schemas**: `ClickstreamEventEnvelope`, `event_type` enum
- **RBAC definitions**: `registered_driver`, `partner`, `admin`
- **ID formats**: `USR-*`, `STN-*`, `PRT-*`, `CHG-*`, `REV-*`

### 7.2 Rule (VERY IMPORTANT)

Contracts are the **ONLY** source of truth for cross-service data structures.

---

## 8. TOOLING & QUALITY GATES

### 8.1 Rust tooling

Required:

- `cargo fmt`
- `cargo clippy -- -D warnings`
- `cargo test`

### 8.2 Frontend tooling

Required:

- `eslint`
- `prettier`
- `tsc --noEmit`
- `vite build`

### 8.3 Mobile tooling

Required:

- `expo doctor`
- `tsc --noEmit`

---

## 9. ROOT AUTOMATION LAYER

### 9.1 Makefile (MANDATORY)

Must include:

- `build-all`
- `test-all`
- `lint-all`
- `format-all`

### 9.2 Task consistency rule

All developers must be able to run full system checks with a single command.

---

## 10. CI/CD READINESS (PREPARATION ONLY)

This epic does NOT implement CI/CD but prepares it.

Must ensure:

- services are independently buildable
- contracts compile standalone
- frontend builds succeed independently
- workspace is deterministic

---

## 11. DEPENDENCY GRAPH

```
EPIC 0
  ↓
EPIC 1 (THIS)
  ↓
EPIC 2 (Runtime)
  ↓
EPIC 3 (CI/CD Execution)
```

---

## 12. ACCEPTANCE CRITERIA

EPIC 1 is **COMPLETE ONLY IF**:

**Repository**
- Monorepo structure created exactly as specified
- Ownership boundaries enforced

**Rust workspace**
- Multi-service workspace compiles
- Shared crates integrated
- `contracts` crate exists and is used

**Frontend**
- All 3 web apps scaffolded
- Expo mobile app initialized
- Shared packages functional

**Contracts system**
- DTOs centralized
- event schema defined
- RBAC definitions centralized
- ID formats centralized

**Tooling**
- linting works across repo
- formatting works across repo
- full build works from root

**Automation**
- Makefile (or equivalent) works
- single-command build/test works

**Constraint validation**
- no duplicate DTO definitions outside contracts
- no service defines its own RBAC enums
- no frontend defines API types independently

---

## 13. OUTCOMES

After EPIC 1 completion:

You have:

- a fully deterministic monorepo
- shared contract system in place
- reproducible builds across backend + frontend
- CI-ready structure (without CI yet)

---

## 14. ONE-LINE SUMMARY

> EPIC 1 establishes the deterministic monorepo workspace with shared contract-driven architecture across backend, frontend, and mobile, ensuring all future development is type-safe, consistent, and CI-ready.
