# CI/CD Contract

## Purpose

Define the continuous integration pipeline, artifact publishing rules, and
deployment process. CI is mandatory. No automatic production deployment.

## Version

1.0.0

## Pipeline Stages

```text
Lint → Test → Build → Contract Validation → Docker Build → GHCR Publish
```

| Stage | Backend | Frontend |
|-------|---------|----------|
| Lint | `cargo fmt --check`, `cargo clippy -D warnings` | `eslint`, `prettier --check` |
| Test | `cargo test` | `vitest` or `jest` |
| Build | `cargo build --release` | `npm run build` |
| Contract Validation | Schema + contract doc review | — |
| Docker Build | `docker build -t ghcr.io/... .` | `docker build -t ghcr.io/... .` |
| GHCR Publish | `docker push ghcr.io/...` | `docker push ghcr.io/...` |

## Artifact Rules

- Image tag: `ghcr.io/<org>/<service>:<git-sha>`
- Deterministic builds required (lock files, pinned base images)
- `latest` tag for dev branch only

## Security Rules

- No secrets in Docker images (build args only, multi-stage builds)
- GitHub Secrets only — no env files in CI
- No environment variable leakage in logs
- SCA (software composition analysis) scan on dependencies

## Deployment Process

Manual only:

1. Operator pulls images from GHCR
2. Runs DB migrations
3. Restarts services via Docker Compose
4. Executes smoke tests
