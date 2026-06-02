# Quickstart: Monorepo Foundation

**Phase**: 1 — Design & Contracts

**Date**: 2026-06-01

## Prerequisites

- Rust toolchain (1.70+) — install via `rustup`
- Node.js 18+ — install via `nvm` or package manager
- npm 9+
- Docker Engine + Docker Compose v2
- Expo CLI — `npm install -g expo-cli`
- iOS development requires macOS + Xcode
- Android development requires Android Studio + SDK

## Setup Steps

### 1. Clone Repository

```bash
git clone <repo-url>
cd Bornemap
git checkout 001-monorepo-foundation
```

### 2. Rust Workspace

```bash
# Build all Rust crates and services
cargo build

# Run tests
cargo test

# Type-check only (faster)
cargo check

# Lint
cargo clippy
```

### 3. Frontend Apps

```bash
# Install all npm dependencies (workspace)
npm install

# Start driver web app
npm run dev --workspace=apps/driver-web

# Start partner dashboard
npm run dev --workspace=apps/partner-dashboard

# Start admin dashboard
npm run dev --workspace=apps/admin-dashboard

# TypeScript type-check across all packages
npx tsc --noEmit
```

### 4. Mobile App

```bash
# Start Expo dev server
cd apps/driver-mobile
npx expo start

# Run on iOS simulator
npx expo start --ios

# Run on Android emulator
npx expo start --android
```

### 5. Docker Compose

```bash
# Validate configuration
docker compose -f infra/compose/docker-compose.yml config

# Start all services (future — placeholders currently)
docker compose -f infra/compose/docker-compose.yml up -d
```

### 6. Verify Health Endpoints

```bash
# Each service exposes /health on its configured port
curl http://localhost:<port>/health
# Expected: {"success":true,"data":{"status":"ok","service":"<name>","version":"0.1.0"}}
```

## CI Pipeline

GitHub Actions workflows:

- `.github/workflows/rust-build.yml` — Rust workspace build
- `.github/workflows/frontend-build.yml` — Frontend build + type-check
- `.github/workflows/docker-build.yml` — Docker build (placeholder)

Push to any branch to trigger all workflows.

## Directory Reference

| Path | Purpose |
|------|---------|
| `services/` | Rust service crates |
| `crates/` | Rust shared crates |
| `apps/` | Frontend web and mobile apps |
| `packages/` | TypeScript shared packages |
| `infra/compose/` | Docker Compose configuration |
| `infra/env/` | Per-service environment files |
| `docs/` | Project documentation |

## Common Issues

### Rust build fails with missing dependencies

Ensure you're in the workspace root (`Cargo.toml` at repo root) and run
`cargo build` — it will fetch and compile all dependencies automatically.

### npm install fails

Ensure Node.js 18+ is active (`node --version`). Delete `node_modules`
and `package-lock.json`, then retry `npm install`.

### Docker Compose config fails

Ensure Docker Desktop / Docker Engine is running. Run `docker compose version`
to verify Compose v2 is available.

### Expo build fails on first run

First-time Expo builds download SDK dependencies and may take several minutes.
Ensure Metro bundler completes before testing on device.
