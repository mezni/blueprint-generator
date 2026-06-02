# Data Model: Monorepo Foundation

**Phase**: 1 — Design & Contracts

**Date**: 2026-06-01

## Overview

This sprint does not introduce business entities (per spec Out of Scope).
The data model described here covers the structural types and contracts
established by the foundation workspace.

## Structural Model

### Service Skeleton

A compile-ready Rust crate with health endpoint scaffolding.

| Element | Description |
|---------|-------------|
| Name | Cargo package name (e.g., `driver-service`) |
| Dependencies | Rust workspace crates (common-types, common-errors, etc.) |
| Entry point | `src/main.rs` or `src/lib.rs` |
| Health endpoint | `GET /health` returning standard envelope |

### Shared Crate (Rust)

A reusable library crate consumed by service crates.

| Element | Description |
|---------|-------------|
| Name | `common-types`, `common-errors`, `common-auth`, `common-db`, `common-observability` |
| Purpose | Shared type definitions, error handling, auth middleware stub, DB stub, logging/config stub |
| Visibility | `pub` where consumed; crate-internal otherwise |

### Shared Package (TypeScript)

An npm package consumed by frontend apps.

| Element | Description |
|---------|-------------|
| Name | `shared-types`, `api-client`, `auth-client`, `design-tokens`, `event-taxonomy` |
| Entry | `src/index.ts` (barrel exports) |
| Build output | `dist/` compiled to CommonJS or ESM |

### Design Token Package

Primitive design values with no runtime logic.

| Element | Description |
|---------|-------------|
| `colors.ts` | Named color constants |
| `spacing.ts` | Spacing scale values |
| `typography.ts` | Font size, weight, line-height definitions |

### Event Taxonomy Package (Stub)

Canonical event envelope structure.

| Field | Type | Description |
|-------|------|-------------|
| `event_id` | string (ULID) | Unique event identifier for deduplication |
| `event_version` | number | Schema version for evolution |
| `event_name` | string | Dot-separated domain.action name |
| `occurred_at` | ISO timestamp | Client-side event timestamp |
| `channel` | string | Source application identifier |
| `session_id` | string | User session tracking |
| `payload` | Record<string, unknown> | Event-specific data (flexible) |

## API Envelope Contract

All backend services share this response envelope.

### Success Response

```json
{
  "success": true,
  "data": {},
  "meta": {}
}
```

### Error Response

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable description"
  }
}
```

### Health Response

```json
{
  "success": true,
  "data": {
    "status": "ok",
    "service": "driver-service",
    "version": "0.1.0"
  }
}
```

## CI Pipeline Model

| Job | Trigger | Requirements |
|-----|---------|--------------|
| Rust build check | Every push to feature branch | `cargo build` passes |
| Frontend build check | Every push to feature branch | `npm run build` passes for all apps |
| TypeScript typecheck | Every push to feature branch | `tsc --noEmit` passes |
| Docker build (placeholder) | Every push to feature branch | Docker build validates (future) |
