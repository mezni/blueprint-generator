# EPIC 3 Specification — CI/CD Pipeline & Delivery Automation

## Epic Metadata

| Field | Value |
|-------|-------|
| **Epic ID** | EPIC-3 |
| **Epic Name** | Continuous Integration, Delivery & Deployment Automation |
| **Priority** | Critical |
| **Status** | Planned |
| **Depends On** | EPIC 1 — Monorepo & Workspace Foundation, EPIC 2 — Runtime Infrastructure & Gateway Platform |
| **Blocks** | All service deployment epics, Production rollout, Release governance |

---

## 1. Objective

Establish a fully automated CI/CD system that guarantees:

- deterministic builds
- automated validation
- artifact generation
- container publishing
- deployment orchestration
- quality gates
- release traceability

This epic defines the complete automation lifecycle from commit to production deployment.

---

## 2. Business Outcome

After completion:

Every code change will automatically:
- validate
- build
- test
- package
- publish artifacts
- deploy to target runtime

with zero manual build steps.

---

## 3. Architectural Scope

This epic covers:

### 3.1 Continuous Integration
Automated validation on every change.

### 3.2 Artifact Production
Build outputs for all platform components.

### 3.3 Container Registry Publication
Versioned image publishing.

### 3.4 Deployment Automation
Runtime deployment execution.

### 3.5 Quality Enforcement
Blocking invalid merges.

### 3.6 Release Governance
Version tagging and release promotion.

---

## 4. Core Architectural Constraints

### 4.1 GitHub Actions Mandatory
All CI/CD automation must run through GitHub Actions. No alternative CI provider permitted.

### 4.2 Monorepo-Aware Execution
Pipelines must execute only for affected workspaces. Changes to frontend trigger frontend jobs only; backend service changes trigger relevant service jobs; shared packages trigger dependent builds.

### 4.3 Deterministic Build Rule
All builds must be reproducible. Required: lockfile enforced, pinned toolchain versions, immutable dependency resolution.

### 4.4 Deployment Model Constraint
Deployment target: single-node Docker runtime defined in EPIC 2. No Kubernetes orchestration.

### 4.5 Branch Protection Rule
No code may merge into main unless all required checks pass.

---

## 5. Pipeline Architecture

```
Developer Push
      |
Source Validation
      |
Static Analysis
      |
Unit Tests
      |
Workspace Build
      |
Container Build
      |
Artifact Publish
      |
Deployment Validation
      |
Release / Deploy
```

---

## 6. Workflow Definitions

The platform requires six primary workflows.

---

## 7. Workflow 1 — Pull Request Validation

| Field | Value |
|-------|-------|
| **File** | `.github/workflows/pr-validation.yml` |
| **Trigger** | `pull_request`, `manual dispatch` |
| **Purpose** | Validate all incoming changes before merge. |

### Required Stages

#### 7.1 Checkout
Repository checkout.

#### 7.2 Toolchain Setup
Install: Rust toolchain, Node.js, pnpm, cargo dependencies.

#### 7.3 Dependency Verification
Validate lockfile consistency. Fail if drift detected.

#### 7.4 Formatting Validation
Rust: `cargo fmt --check`. Frontend: `pnpm format:check`.

#### 7.5 Static Analysis
Rust: `cargo clippy -- -D warnings`. TypeScript: `pnpm lint`.

#### 7.6 Unit Tests
Execute all affected tests.

#### 7.7 Build Validation
Compile affected workspaces.

### Acceptance Criteria
PR blocked if any validation fails.

---

## 8. Workflow 2 — Integration Validation

| Field | Value |
|-------|-------|
| **File** | `.github/workflows/integration.yml` |
| **Trigger** | `push to main`, `nightly schedule` |
| **Purpose** | Validate full-system integration. |

### Required Stages

- **Runtime Boot**: Start full stack: `docker compose up -d`
- **Service Health Validation**: Verify Traefik, APIs, PostgreSQL, RabbitMQ, Keycloak
- **API Contract Validation**: Validate all endpoints exposed under `/api/v1/*`. Reject any drift.
- **Cross-Service Communication Tests**: Validate DB connectivity, message broker publish/consume, auth token verification.

---

## 9. Workflow 3 — Container Build & Publish

| Field | Value |
|-------|-------|
| **File** | `.github/workflows/build-images.yml` |
| **Trigger** | `merge to main`, `version tag` |
| **Purpose** | Build and publish versioned images. |

### Required Images

**Infrastructure-adjacent:** gateway config bundle

**Backend:** admin-service, driver-service, clickstream-service, gis-sync-worker

**Frontend:** driver-web, admin-dashboard, partner-dashboard

### Registry Target
GitHub Container Registry (GHCR).

### Tagging Scheme
- Commit builds: `sha-<commit>`
- Release builds: `vX.Y.Z`
- Latest: `latest`

---

## 10. Workflow 4 — Deployment Automation

| Field | Value |
|-------|-------|
| **File** | `.github/workflows/deploy.yml` |
| **Trigger** | Manual release dispatch |
| **Purpose** | Deploy release images to production runtime. |

### Deployment Steps

1. **Pull Latest Images**
2. **Validate Runtime Configuration**: Check env completeness, compose integrity, image availability
3. **Rolling Restart**: Deploy services in order: infra → auth → backend → frontend
4. **Post-Deploy Validation**: Verify health endpoints, gateway routing, API version compliance

---

## 11. Workflow 5 — Release Creation

| Field | Value |
|-------|-------|
| **File** | `.github/workflows/release.yml` |
| **Trigger** | Version tag push |
| **Responsibilities** | Generate release notes, artifact manifest, image manifest, deployment changelog |

---

## 12. Workflow 6 — Dependency Audit

| Field | Value |
|-------|-------|
| **File** | `.github/workflows/security-audit.yml` |
| **Trigger** | Weekly schedule |
| **Required Checks** | Rust: `cargo audit`. Frontend: `pnpm audit`. Container base images: security scan. |

---

## 13. Artifact Strategy

### 13.1 Required Artifacts
- **Backend**: Compiled binaries
- **Frontend**: Static bundles
- **Infrastructure**: Compose manifests
- **Release Metadata**: Version manifests

### 13.2 Artifact Retention
- PR builds: 7 days
- Main builds: 30 days
- Release builds: indefinite

---

## 14. Versioning Strategy

### 14.1 Semantic Versioning Mandatory
Format: `MAJOR.MINOR.PATCH`

### 14.2 Release Rules
- **Major**: Breaking API/runtime changes. Requires potential `/api/v2`.
- **Minor**: Backward-compatible features.
- **Patch**: Fixes only.

---

## 15. Deployment Environment Model

Supported targets:
- **Local**: Developer validation
- **Production**: Primary deployment target

No staging environment.

---

## 16. Quality Gates

Every merge must pass:

- **Code Quality**: formatting, linting, static analysis
- **Functional Quality**: unit tests, integration tests
- **Runtime Quality**: compose boot validation, health checks
- **API Governance**: `/api/v1` contract enforcement

---

## 17. Secrets Management

CI/CD secrets must include:

- **Registry**: `GHCR_TOKEN`
- **Deployment**: `DEPLOY_HOST`, `DEPLOY_USER`, `DEPLOY_KEY`
- **Runtime**: Environment-specific secret injection

**Forbidden**: Secrets committed to repository.

---

## 18. Performance Requirements

- **PR Validation**: Target < 15 minutes
- **Full Integration**: Target < 25 minutes
- **Image Build**: Target < 20 minutes

---

## 19. Failure Handling Rules

Pipeline failures must:
- fail fast
- produce actionable logs
- block deployment
- preserve diagnostic artifacts

---

## 20. Required Deliverables

This epic must produce:

### Workflow Files
- `pr-validation.yml`
- `integration.yml`
- `build-images.yml`
- `deploy.yml`
- `release.yml`
- `security-audit.yml`

### Supporting Scripts
- deployment scripts
- health verification scripts
- artifact manifest generator

### Documentation
- CI/CD operating guide

---

## 21. Acceptance Criteria

- **CI Validation**: PR validation blocks invalid merges; lint + tests enforced; affected-only builds work
- **Integration**: full stack boot tested automatically; `/api/v1` validated
- **Delivery**: images published automatically; artifacts versioned
- **Deployment**: production deployment automated; post-deploy validation succeeds
- **Governance**: semantic versioning enforced; releases traceable

---

## 22. Definition of Done

EPIC 3 is complete when:

A developer can:
1. merge validated code
2. automatically build artifacts
3. publish containers
4. deploy runtime
5. verify operational health

without manual build orchestration.

---

## 23. Dependency Graph

```
EPIC 1
   ↓
EPIC 2
   ↓
EPIC 3
   ↓
Service Delivery Epics
```

---

## 24. Epic Summary

EPIC 3 establishes the complete automated software delivery pipeline for the platform, enforcing deterministic builds, strict quality gates, versioned artifact publication, automated deployment to the Docker runtime, and full `/api/v1` contract validation across every release.
