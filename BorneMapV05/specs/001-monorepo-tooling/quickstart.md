# Quickstart: Monorepo + Tooling Foundation

> Follow these steps to set up the Bornemap development environment after cloning.

## Prerequisites

- **Rust**: Install via `rustup` (stable toolchain, edition 2024 compatible)
- **Node.js**: Version 22 LTS (use `nvm` or install directly)
- **npm**: Bundled with Node.js 22
- **Docker & Docker Compose**: For infrastructure preview

## Setup

```bash
# 1. Ensure the correct Node.js version
nvm use         # reads .nvmrc

# 2. Install Rust workspace
cargo build --workspace

# 3. Install npm dependencies
npm install

# 4. Build all TypeScript apps and packages
npm run build --workspaces

# 5. (Optional) Validate Docker Compose skeleton
cd infra/compose
docker compose config
```

## Expected Outcomes

| Target | Command | Expected Result |
|--------|---------|----------------|
| Rust workspace | `cargo build --workspace` | All 9 crates compile, exit 0 |
| Web apps (3) | `npm run build -w apps/driver-web` | Each app builds, exit 0 |
| Mobile app | `npx expo export --platform web` | Expo bundle succeeds, exit 0 |
| TS packages (6) | `npm run build -w packages/shared-types` | Each package builds, exit 0 |
| Docker Compose | `docker compose config` | Valid YAML output, exit 0 |

## Development Servers

```bash
# Run a web app dev server
npm run dev -w apps/driver-web

# Run the mobile app
cd apps/driver-mobile && npx expo start
```

## What's Included

- 5 Rust service binaries (empty main functions)
- 4 Rust shared library crates (type definitions, stubs)
- 3 React + Vite web application shells
- 1 React Native Expo mobile app shell
- 6 shared TypeScript packages (types, contracts, clients)
- Docker Compose skeleton with Traefik routing
- Per-service `.env.example` files

## What's NOT Included (Later Sprints)

- Business logic, database migrations, API endpoints
- CI/CD configuration, pre-commit hooks
- Runtime infrastructure (PostgreSQL, RabbitMQ, Keycloak)
- Test frameworks and test suites
