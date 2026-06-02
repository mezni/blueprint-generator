# Research: Monorepo Foundation

**Phase**: 0 — Outline & Research

**Date**: 2026-06-01

## Overview

This sprint establishes the engineering foundation for the Bornemap platform.
No NEEDS CLARIFICATION items were identified because the constitution and
user-provided task checklist fully specify all requirements.

## Technology Decisions

### Rust Edition

- **Decision**: Rust edition 2021, minimum version 1.70
- **Rationale**: Latest stable edition with broad crate ecosystem support;
  1.70+ provides stable 1.75 features needed for future work
- **Alternatives considered**: Edition 2018 (too old for current best practices)

### TypeScript Configuration

- **Decision**: TypeScript 5.x with strict mode for all packages and apps
- **Rationale**: Strict mode is consistent across the monorepo, preventing type
  leaks between packages; aligns with constitution's contract-driven approach
- **Alternatives considered**: Loose mode (would allow type unsafety)
  — rejected per constitution principle IV

### React Router Version

- **Decision**: React Router v6 with data loaders pattern
- **Rationale**: Industry standard for React SPAs; data loaders align with
  future API integration needs
- **Alternatives considered**: React Router v5 (outdated), TanStack Router
  (too early, less ecosystem)

### Expo SDK

- **Decision**: Expo SDK 50+ with development builds
- **Rationale**: Expo is the constitution-mandated mobile framework; SDK 50+
  provides latest stability and TypeScript support
- **Alternatives considered**: Expo SDK 49 (would work but older)

### Docker Compose Version

- **Decision**: Docker Compose v2 (compose plugin, not standalone v1)
- **Rationale**: Compose v2 is the current standard; v1 is deprecated
- **Alternatives considered**: Docker Compose v1 (deprecated)

### CI Platform

- **Decision**: GitHub Actions
- **Rationale**: Standard CI for GitHub-hosted projects; community workflows
  for Rust and Node.js are mature
- **Alternatives considered**: GitLab CI, CircleCI (no repo hosted there)

## Dependency Considerations

### Rust Crate Recommendations

- **axum** or **actix-web** for HTTP services (both well-suited; choice
  deferred to implementation)
- **serde** + **serde_json** for JSON serialization (standard in Rust ecosystem)
- **tokio** for async runtime (ecosystem standard)
- **thiserror** for error types (common-errors crate)
- **tracing** for structured logging (common-observability crate)
- **sqlx** for future DB access (common-db stub for now)
- **jsonwebtoken** for future JWT handling (common-auth stub)

### Node.js Package Recommendations

- **@tanstack/react-query** for data fetching (future compatibility)
- **zod** for runtime type validation (shared contracts)
- **axios** or native fetch for API client

## Build Tooling

### Rust

- `cargo build` from workspace root compiles all crates
- `cargo check` for fast type checking during development
- `cargo test` runs all tests across workspace
- `cargo clippy` for linting

### TypeScript / Frontend

- Vite dev server per app (`npm run dev`)
- `tsc --noEmit` for type checking across all packages
- `vitest` for unit tests (future)
- `npm run build` for production builds

### Monorepo Tooling

- Rust: Cargo workspace (`Cargo.toml` at root)
- TypeScript: npm workspaces (`package.json` at root)
- Root-level scripts for unified commands (e.g., `npm run build:all`)
