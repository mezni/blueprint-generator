# BorneMap

EV charging station platform monorepo.

## Directory Map

```
borne-map/
├── apps/                    # Frontend applications
│   ├── driver-web/               # Vite + React — driver portal
│   ├── partner-dashboard/        # Vite + React — partner dashboard
│   ├── admin-dashboard/          # Vite + React — admin dashboard
│   └── driver-mobile/            # React Native + Expo — mobile app
├── services/                 # Rust backend services
│   ├── admin-service/             # Inventory CRUD
│   ├── driver-service/            # Station discovery, reviews
│   ├── clickstream-service/       # Event ingestion
│   └── gis-sync-worker/           # GIS enrichment
├── crates/                   # Shared Rust libraries
│   ├── contracts/                 # Cross-service DTOs, events, enums
│   ├── common-auth/               # Auth utilities
│   ├── common-config/             # Configuration loading
│   ├── common-db/                 # DB connection pool
│   ├── common-errors/             # Error types
│   └── common-types/              # Domain types
├── packages/                 # Shared TypeScript packages
│   ├── design-system/             # UI components
│   ├── api-client/                # Typed REST client
│   ├── analytics-client/          # Clickstream emitter
│   └── auth-client/               # OAuth management
├── infra/                    # Infrastructure
│   ├── docker/                    # Dockerfiles
│   └── compose/                   # Docker Compose
└── scripts/                  # Helper scripts
```

## Prerequisites

- Rust stable toolchain
- Node.js 20+
- npm
- Expo CLI (`npm install -g expo-cli`)

## Commands

```bash
make build-all    # Build Rust workspace + web apps
make test-all     # Run all Rust tests
make lint-all     # Clippy + ESLint
make format-all   # cargo fmt + prettier
```

## CI/CD Pipeline

Six automated GitHub Actions workflows:

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `pr-validation.yml` | Pull request | Lint, test, build, contract validation |
| `build-images.yml` | Push to main / version tag | Build & publish 7 container images to GHCR |
| `integration.yml` | Push to main / schedule | Full-stack Docker Compose boot + health checks |
| `deploy.yml` | Manual dispatch | One-click production deployment with rollback |
| `security-audit.yml` | Weekly (Sunday) | Rust/npm dependency audit + Trivy scan |
| `release.yml` | Version tag push | GitHub Release + artifact/image manifests |

See `specs/004-ci-cd-pipeline/` for full specification and `docs/epic03.md` for the EPIC definition.

## Architecture

See `docs/epic01.md` and `specs/001-architecture-contracts/contracts/`.
