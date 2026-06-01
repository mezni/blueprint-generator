# Data Model: BorneMap Monorepo

**Phase**: 1 — Design & Contracts
**Date**: 2026-05-31
**Source Spec**: [spec.md](../spec.md)

This document defines the structural entities of the BorneMap monorepo. Zero runtime entities exist at this stage (scaffolding only).

## Directory Structure Entities

### Top-Level Directories

| Directory | Purpose | Contains |
|-----------|---------|----------|
| `apps/` | Frontend applications | 3 web apps + 1 mobile app |
| `services/` | Backend Rust services | 4 Cargo binary packages |
| `crates/` | Shared Rust libraries | 6 Cargo library crates |
| `packages/` | Shared TypeScript packages | 4 npm packages |
| `infra/` | Infrastructure scaffolding | Docker + Compose placeholders |
| `scripts/` | Helper scripts | Shell scripts |
| `docs/` | Project documentation | Markdown files |
| `.github/` | CI/CD workflows | GitHub Actions configs |

### Service Entities

| Service | Cargo Type | Purpose |
|---------|-----------|---------|
| `admin-service` | Binary (`main.rs`) | Inventory CRUD, partner/station/charger management |
| `driver-service` | Binary (`main.rs`) | Station discovery, favorites, reviews, user profile |
| `clickstream-service` | Binary (`main.rs`) | Event ingestion, validation, RabbitMQ publish |
| `gis-sync-worker` | Binary (`main.rs`) | GIS enrichment from inventory data |

### Shared Crate Entities

| Crate | Cargo Type | Purpose |
|-------|-----------|---------|
| `contracts` | Library (`lib.rs`) | Cross-service DTOs, event schemas, RBAC enums, ID formats |
| `common-auth` | Library (`lib.rs`) | Auth/authorization utilities, token validation |
| `common-config` | Library (`lib.rs`) | Configuration loading from env/file |
| `common-db` | Library (`lib.rs`) | DB connection pool, migration management |
| `common-errors` | Library (`lib.rs`) | Shared error types, error code definitions |
| `common-types` | Library (`lib.rs`) | Shared domain types, value objects |

### Application Entities

| App | Platform | Framework | Purpose |
|-----|----------|-----------|---------|
| `driver-web` | Web | Vite + React | Driver portal — station discovery, charging |
| `partner-dashboard` | Web | Vite + React | Partner dashboard — station management, analytics |
| `admin-dashboard` | Web | Vite + React | Admin panel — moderation, system management |
| `driver-mobile` | Mobile | React Native + Expo | Driver mobile app — on-the-go station access |

### Shared Package Entities

| Package | Type | Purpose |
|---------|------|---------|
| `design-system` | npm package | Reusable UI components, design tokens, layout primitives |
| `api-client` | npm package | Typed REST client wrapping Rust API contracts |
| `analytics-client` | npm package | Clickstream event emitter (contract-defined envelopes) |
| `auth-client` | npm package | OAuth token management, session handling |

## Workspace Configurations

### Cargo Workspace (`Cargo.toml`)

- **Members**: `services/*`, `crates/*`
- **Resolver**: v2 (default since Rust 2021 edition)
- **Edition**: 2021
- **Shared profile**: dev, release, test

### npm Workspace (`package.json`)

- **Workspaces**: `apps/*`, `packages/*`
- **Shared config**: `tsconfig.base.json` (extends pattern)
- **Root scripts**: `build`, `lint`, `format`, `test` (delegating per-app)

## Validation Rules

| Rule | Source | Enforced By |
|------|--------|-------------|
| No duplicate DTOs outside contracts crate | FR-011 | Code review + CI |
| All workspace members compile together | FR-007 | `cargo build --workspace` |
| All npm packages type-check | FR-012 | `tsc --noEmit` |
| All services have Dockerfile placeholder | FR-015 | Directory structure audit |
| Makefile must have all 4 targets | FR-014 | `make help` / listing |
