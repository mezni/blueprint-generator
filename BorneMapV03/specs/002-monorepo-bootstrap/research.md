# Research: Monorepo Bootstrap

**Phase**: 0 — Research & Resolution
**Date**: 2026-05-31

## Methodology

All decisions are drawn from the feature specification (spec.md), the project constitution, and EPIC 0 architecture contracts. No external research was needed — the spec was unambiguous.

## Design Decisions

### Decision 1: Rust Workspace Strategy

- **Decision**: Single-root `Cargo.toml` workspace with glob-based member discovery (`services/*`, `crates/*`)
- **Rationale**: Simplest Cargo workspace setup; all services and crates share the same lockfile; single `cargo build --workspace` compiles everything
- **Alternatives considered**:
  - Per-service workspaces — rejected: would require separate lockfiles and cross-workspace path deps
  - Git submodules per service — rejected: adds overhead, no benefit for co-located services

### Decision 2: TypeScript Package Manager

- **Decision**: npm workspaces (no pnpm/yarn)
- **Rationale**: Zero additional tooling; Node.js 20+ built-in; all CI runners support it natively
- **Alternatives considered**:
  - pnpm — rejected: adds tooling dependency for marginal disk-space benefit at monorepo scale
  - yarn — rejected: no advantage over npm workspaces for 4 packages

### Decision 3: Shared TypeScript Config

- **Decision**: Single `tsconfig.base.json` at root; all apps/packages extend it
- **Rationale**: Enforces consistent strictness, module resolution, and target across all TS code
- **Alternatives considered**:
  - Per-package standalone tsconfig — rejected: would allow drift; base config ensures uniformity

### Decision 4: Container Scaffold Format

- **Decision**: Multi-stage Dockerfile placeholder (no build logic) per service; single `docker-compose.dev.yml`
- **Rationale**: CI-ready without requiring actual containerization logic in EPIC 1; services and tools can `docker compose up` against empty stubs
- **Alternatives considered**:
  - Single Dockerfile — rejected: each service has unique build context
  - No Docker scaffolding — rejected: FR-015/FR-016 explicitly require it

### Decision 5: Makefile Target Layout

- **Decision**: Four top-level targets (build-all, test-all, lint-all, format-all) that delegate to Cargo + npm
- **Rationale**: Single entry point for CI; each target maps to a pipeline stage
- **Alternatives considered**:
  - Shell scripts per target — rejected: Makefile provides dependency tracking and parallelism
  - Justfile — rejected: adds tooling dependency
