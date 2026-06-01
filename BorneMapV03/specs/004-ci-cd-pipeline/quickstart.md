# Quickstart: CI/CD Pipeline

## Prerequisites

- GitHub repository access with admin permissions (to set branch protection and secrets)
- GitHub Container Registry (GHCR) enabled for the repository
- Production host with Docker Engine 24+ and Compose v2 (from EPIC 2)

## Setup

### 1. Configure Branch Protection

In GitHub repository Settings > Branches > Add rule for `main`:

- Require status checks to pass before merging
- Require branches to be up to date
- Status checks: `lint`, `test`, `build`, `contract-validation`

### 2. Set Repository Secrets

Go to Settings > Secrets and variables > Actions and add:

| Secret | Description |
|--------|-------------|
| `GHCR_TOKEN` | GitHub token with `write:packages` scope |
| `DEPLOY_HOST` | Production server hostname/IP |
| `DEPLOY_USER` | SSH user for deployment |
| `DEPLOY_KEY` | SSH private key for deployment |

### 3. Verify Workflow Triggers

Push a branch to verify PR validation triggers. Merge to main to trigger integration and build workflows.

## Development Workflow

```
git checkout -b my-feature
# make changes
git push
# PR created → PR validation runs automatically
# Fix any failures → push again → old run cancelled, new run starts
git checkout main
git merge my-feature
# Push to main triggers integration validation + container build
```

## Deployment

```
# Navigate to GitHub Actions > Deploy > Run workflow
# Select branch (usually main)
# Click "Run workflow"
# Monitor deployment progress
# Post-deploy validation runs automatically
```

## Release

```bash
git tag v1.2.3
git push origin v1.2.3
# Release workflow triggers automatically
# GitHub Release created with notes + manifests
```

## Security Audit

Audit runs automatically every Sunday. View results in GitHub Actions > Security audit > Latest run.

## Verification

```bash
# Check PR validation
git checkout -b test-pr-ci
echo "// test" >> apps/driver-web/src/main.ts
git add . && git commit -m "test: trigger PR validation"
git push -u origin test-pr-ci
# Open PR → verify pipeline runs and blocks/fails

# Check image publishing
# Merge to main → verify images appear at:
# https://github.com/orgs/<org>/packages?repo_name=<repo>

# Check deployment
# Manual dispatch in Actions > Deploy workflow
```

## Shutdown

No shutdown needed — this is CI/CD configuration, not a running service.
