# CI/CD Pipeline Contract

**Enforced by**: GitHub Actions workflows
**Target runtime**: EPIC 2 Docker Compose platform

## Workflow Contracts

### 1. PR Validation (`pr-validation.yml`)

**Guarantees**:
- Blocks merge if any stage fails
- Runs only for affected workspaces (backend/frontend/shared)
- Cancels in-progress runs on new push to same PR
- Completes within 15 minutes

**Required checks (branch protection)**:
- `lint` (formatting + clippy/clippy + eslint)
- `test` (cargo test --workspace)
- `build` (cargo build + npm run build)
- `contract-validation` (DTO drift detection)

### 2. Integration Validation (`integration.yml`)

**Guarantees**:
- Full platform stack boots via Docker Compose
- All 11 services report healthy
- All `/api/v1/*` routes are reachable
- Cross-service communication passes (DB, RabbitMQ, Keycloak)

**Failure mode**: Blocks main if failing; 1 retry for flaky tests.

### 3. Container Build & Publish (`build-images.yml`)

**Guarantees**:
- All 7 images built via matrix strategy
- Images tagged: `sha-<commit>`, `latest`, (and `vX.Y.Z` for version tags)
- Published to `ghcr.io/<repo>/<service>:<tag>`
- Build caching via `type=gha` for fast rebuilds
- No partial publish on failure

### 4. Deployment (`deploy.yml`)

**Guarantees**:
- Manual dispatch only (not automatic)
- Configuration validated before restart
- Rolling restart: infra → auth → backend → frontend
- Health verification between each group
- Post-deploy validation: health endpoints + gateway routing + API compliance
- Rollback on post-deploy failure
- Sequential queuing — no concurrent deploys

### 5. Release Creation (`release.yml`)

**Guarantees**:
- GitHub Release created with auto-generated notes
- Artifact manifest included (SHA hashes of all artifacts)
- Image manifest included (registry paths + tags)
- Fails on version tag conflict

### 6. Security Audit (`security-audit.yml`)

**Guarantees**:
- Rust: `cargo audit` — CVE check on all dependencies
- Frontend: `npm audit` — vulnerability report
- Container images: `trivy` scan — base image CVE check
- Weekly schedule (Sunday 00:00)
- Email alert on critical findings

## Tagging Scheme

| Build Type | Tags | Example |
|------------|------|---------|
| Commit build | `sha-<git-sha>`, `latest` | `sha-a1b2c3d`, `latest` |
| Release build | `sha-<git-sha>`, `vX.Y.Z`, `latest` | `sha-a1b2c3d`, `v1.2.3`, `latest` |

## Concurrency Rules

| Scope | Policy | Group Key |
|-------|--------|-----------|
| Per PR | Cancel outdated | `pr-${{ github.head_ref }}` |
| Main branch | Cancel outdated | `ci-main` |
| Deployment | Queue sequentially | `deploy-production` |
| Release | Cancel outdated | `release-${{ github.ref }}` |
