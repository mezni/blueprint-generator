# Research: CI/CD Pipeline & Delivery Automation

**Phase**: 0 — Research & Resolution
**Date**: 2026-05-31

## Methodology

All decisions are drawn from the feature specification (spec.md), EPIC 3 document (docs/epic03.md), and project constitution. No external research needed — the spec is complete on all technical decisions after clarification.

## Design Decisions

### Decision 1: Workflow File Strategy

- **Decision**: Split into 6 dedicated workflow files rather than a single monolithic workflow
- **Rationale**: Each workflow has distinct triggers and purposes (PR, integration, build, deploy, release, audit). Separate files enable independent maintenance, clearer logging, and per-workflow concurrency settings. Merging would create complex conditional logic with `if` gates.
- **Alternatives considered**:
  - Single monolithic workflow — rejected: harder to maintain, debug, and exceeds GitHub Actions 256-job limit per workflow
  - Per-service workflows — rejected: adds orchestration complexity without benefit since all services share the same build pattern

### Decision 2: Deployment Approach (Constitution Deviation)

- **Decision**: Implement automated deployment via GitHub Actions workflow (manual dispatch trigger) with sequential rolling restart
- **Rationale**: The constitution states "No auto-deployment — only artifact generation" but EPIC 3 explicitly requires deployment automation (User Story 4). Manual GHCR pull + restart requires SSH access and is error-prone. Automated deployment provides audit trail, repeatability, and rollback capability.
- **Deviation justification**: The deployment workflow is manual-dispatch-only (not automatic on merge), preserving the constitution's intent of no automatic deployment while enabling one-click automated deployment from the GitHub UI.
- **Alternatives considered**:
  - Continue manual SSH deployment — rejected: error-prone, no audit trail, requires SSH key distribution
  - Automatic deploy on merge — rejected: violates constitution; manual dispatch is the safe compromise

### Decision 3: Monorepo-Aware Change Detection

- **Decision**: Use `dorny/paths-filter` GitHub Action to detect changed paths and conditionally run workspace-specific jobs
- **Rationale**: The monorepo has distinct backend (Rust), frontend (TypeScript), and shared package (Rust crates) workspaces. Building everything on every PR would waste runner minutes and exceed the 15-minute PR validation target (FR-017).
- **Alternatives considered**:
  - Build everything every time — rejected: exceeds 15-minute target for small changes
  - Manual workspace selection — rejected: error-prone, requires developer discipline

### Decision 4: Container Build Strategy

- **Decision**: Matrix build using `docker/build-push-action` with GHCR cache, triggered on main merge and version tags
- **Rationale**: Docker BuildX with GitHub Actions cache (`type=gha`) provides layer caching across workflow runs, dramatically reducing build time. Matrix strategy builds all 7 images (4 backend + 3 frontend) in parallel.
- **Alternatives considered**:
  - Sequential builds — rejected: would exceed 20-minute build target
  - Single Dockerfile with build args — rejected: matrix is simpler and more maintainable for distinct service Dockerfiles

### Decision 5: Security Audit Tooling

- **Decision**: Use `cargo audit` for Rust, `npm audit` for frontend, and `trivy` for container image scanning
- **Rationale**: Cargo audit and npm audit are the standard ecosystem tools for their respective platforms. Trivy is the de facto standard for container image scanning with zero configuration, GitHub Actions integration, and comprehensive CVE database.
- **Alternatives considered**:
  - Snyk — rejected: requires paid subscription and API token management
  - Grype — rejected: less mature GitHub Actions integration than Trivy

### Decision 6: Email Alerting Strategy

- **Rationale**: GitHub Actions has built-in email notifications for workflow failures when configured via `on: workflow_run` with `workflows: [...]` failure conditions. No additional tooling needed. Email alerts are sent to the commit author and repo watchers by default when workflows fail on the default branch.
- **Implementation**: Use GitHub's native failure notification + `actions/github-script` to send custom email if richer formatting is needed.
