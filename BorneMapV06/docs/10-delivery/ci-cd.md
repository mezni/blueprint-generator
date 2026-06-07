# CI/CD Pipeline

## GitHub Actions Workflows

| Workflow | Trigger | Actions |
|----------|---------|---------|
| `ci-rust.yml` | PR to main | `cargo test`, `cargo clippy`, `cargo fmt` |
| `ci-frontend.yml` | PR to main | `pnpm install`, `pnpm lint`, `pnpm test` |
| `build-driver-service.yml` | Tag, main push | Build + push Docker image |
| `build-admin-service.yml` | Tag, main push | Build + push Docker image |
| `build-clickstream-service.yml` | Tag, main push | Build + push Docker image |
| `build-gis-sync-worker.yml` | Tag, main push | Build + push Docker image |
| `build-driver-web.yml` | Tag, main push | Build + push Docker image |
| `build-partner-dashboard.yml` | Tag, main push | Build + push Docker image |
| `build-admin-dashboard.yml` | Tag, main push | Build + push Docker image |
| `build-driver-mobile.yml` | Tag, main push | Build + push (OTA/artifact) |

## Key Rules

- No automated production deployment from CI
- Docker images are built and pushed to registry
- Manual deployment pulls images and deploys to production
- All tests must pass for PR to merge
- Linting must pass before tests run
