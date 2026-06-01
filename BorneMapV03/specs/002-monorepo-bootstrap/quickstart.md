# BorneMap Monorepo Quickstart

Prerequisites: Rust stable toolchain, Node.js 20+, npm, Expo CLI

```bash
# Clone and enter
git clone <repo-url> borne-map
cd borne-map

# Build everything
make build-all

# Run all tests
make test-all

# Lint everything
make lint-all

# Format everything
make format-all
```

## Manual Steps

```bash
# Build Rust workspace only
cargo build --workspace

# Build web apps
cd apps/driver-web && npm run build
cd apps/partner-dashboard && npm run build
cd apps/admin-dashboard && npm run build

# Check mobile app
cd apps/driver-mobile && expo doctor

# Type-check all TypeScript
npx tsc --noEmit --project tsconfig.base.json
```

## Directory Map

```
borne-map/
├── apps/          → Frontend apps (Vite + React / Expo)
├── services/      → Rust backend services
├── crates/        → Shared Rust libraries
├── packages/      → Shared TypeScript packages
├── infra/         → Docker + Compose scaffolding
├── scripts/       → Helper scripts
├── docs/          → Documentation
└── .github/       → CI/CD (EPIC 3)
```

## Next Steps After Bootstrap

1. Implement service business logic (EPIC 2+)
2. Set up Docker Compose with Traefik (EPIC 2)
3. Configure CI/CD pipelines (EPIC 3)
4. Populate contracts crate with full DTO implementations
