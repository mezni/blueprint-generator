# Implementation Plan: CI/CD Pipeline & Delivery Automation

**Branch**: `004-ci-cd-pipeline` | **Date**: 2026-05-31 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/004-ci-cd-pipeline/spec.md`

## Summary

Build a complete automated CI/CD system for the BorneMap platform — 6 GitHub Actions workflows covering PR validation, integration testing, container image build/publish, deployment automation, release creation, and security auditing. Zero-touch pipeline from commit to production deployment, with deterministic builds, monorepo-aware execution, and email alerting on failures.

## Technical Context

**Language/Version**: YAML (GitHub Actions workflow syntax), shell script (deployment/health checks)

**Primary Dependencies**: GitHub Actions (ubuntu-latest), Docker BuildX, cargo/Rust toolchain (stable), Node.js 20, Docker Compose v2

**Storage**: GitHub Container Registry (GHCR) — image storage; GitHub Actions artifacts — build artifact retention

**Testing**: `docker compose config --quiet` for syntax validation; `curl`-based health endpoint checks; workflow self-validation via action status checks

**Target Platform**: GitHub Actions CI runners (ubuntu-latest); production Docker host from EPIC 2

**Project Type**: Infrastructure automation — CI/CD pipeline workflows + deployment scripts

**Performance Goals**: PR validation < 15 minutes (SC-001); full integration validation < 25 minutes (SC-002); container image builds < 20 minutes (SC-003); deployment < 30 minutes (SC-004)

**Constraints**: GitHub Actions only (FR-001); monorepo-aware execution (FR-002); deterministic builds with pinned toolchains and lockfile enforcement (FR-003); branch protection requiring passing checks (FR-005); concurrency — cancel outdated per-PR, queue deployments sequentially; no Kubernetes; no staging environment; email alerts on pipeline failures (FR-016)

**Scale/Scope**: 6 workflows; 11 services across 2 networks (from EPIC 2); 2 environments (local, production); 4 backend + 3 frontend Docker images published to GHCR

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Rationale |
|-----------|--------|-----------|
| I. Pragmatic Architecture | ✅ PASS | CI/CD is a single pipeline system — no fragmentation across multiple CI providers or tools |
| II. Clear Ownership Boundaries | ✅ PASS | Pipeline validates service boundaries but does not create new ones; each workflow has explicit scope |
| III. Operational Simplicity | ⚠ GATE CAUTION | Constitution mandates "No auto-deployment — only artifact generation" but EPIC 3 explicitly defines deployment automation (US4). This is a deliberate expansion of scope. See Complexity Tracking. |
| IV. Evolution over Complexity | ⚠ SEE TRACKING | 6 workflows require justification (see Complexity Tracking) |
| V. Data Separation | ✅ N/A | CI/CD pipeline does not manage database schemas |

**GATE RESULT**: PASS with complexity tracking note. The auto-deployment deviation from constitution Section 3 (CI/CD) is deliberate and user-requested via EPIC 3 specification.

### Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Auto-deployment (contradicts constitution "No auto-deployment") | Production deployment automation ensures repeatable, auditable releases without SSH access — directly requested in EPIC 3 User Story 4 | Continuing manual deployment from GHCR would require SSH access and manual pull/restart steps, introducing human error risk |
| 6 workflow files | Each workflow has a distinct trigger and purpose (PR, integration, build, deploy, release, audit); merging would create monolithic files with complex conditional logic | Single monolithic workflow would be harder to maintain, debug, and would violate separation of concerns |
| Monorepo-aware execution | The monorepo has 4 backend services, 3 frontend apps, and shared packages — only affected workspaces should be rebuilt | Building everything on every PR would exceed the 15-minute PR validation target (FR-017) |

## Project Structure

### Documentation (this feature)

```text
specs/004-ci-cd-pipeline/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
borne-map/
├── .github/
│   └── workflows/
│       ├── pr-validation.yml       # PR validation (lint, test, build, contract check)
│       ├── integration.yml          # Integration validation (full stack boot + contract verify)
│       ├── build-images.yml         # Container build & publish to GHCR
│       ├── deploy.yml               # Deployment automation (manual trigger)
│       ├── release.yml              # Release creation (version tag → release)
│       └── security-audit.yml       # Security & dependency audit (weekly)
├── scripts/
│   └── ci/
│       ├── deploy.sh               # Deployment orchestration script
│       ├── health-check.sh         # Post-deploy health verification
│       └── manifest.sh             # Artifact manifest generator
├── .github/                    # Existing from EPIC 2
│   └── workflows/
│       └── ci.yml              # Will be replaced/extended by new workflows
└── specs/
    └── 004-ci-cd-pipeline/     # This feature's documentation
```

**Structure Decision**: Six workflow files under `.github/workflows/` for clear separation of concerns. Supporting scripts under `scripts/ci/` for deployment and verification logic that is shared across workflows. The existing `ci.yml` from EPIC 2 will be replaced by the new `pr-validation.yml` and `build-images.yml` workflows.
