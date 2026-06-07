# Quickstart: Sprint 0 Foundation

**Date**: 2026-06-05  
**Purpose**: Developer onboarding guide for running Sprint 0 locally  
**Audience**: Team developers setting up their environment for MVP01 work

---

## Prerequisites

Before starting, ensure you have:

### Required
- **Git** (any recent version)
- **Docker** & **Docker Compose** (v2.0+)
- **Rust** (1.70+, install via [rustup](https://rustup.rs/))
- **Node.js** (18+) and **pnpm** (8+)
- **PostgreSQL client tools** (optional, for manual queries)

### Verify Installation

```bash
# Check Rust
rustc --version          # Should show 1.70+
cargo --version

# Check Node.js and pnpm
node --version           # Should show 18+
pnpm --version           # Should show 8+

# Check Docker
docker --version
docker compose --version # Compose v2.0+ (not docker-compose)
```

---

## Setup (First Time Only)

### 1. Clone Repository

```bash
git clone https://github.com/mezni/BorneMap.git
cd BorneMap
git checkout 003-sprint-0-foundation
```

### 2. Install Rust Dependencies

```bash
# Navigate to repo root
cargo build --release

# This compiles:
# - services/driver-service
# - crates/ev-core
# - crates/ev-geo
# - crates/ev-db
# 
# First build takes ~2-3 minutes. Subsequent builds are faster.
```

**Troubleshooting**:
- If `cargo build` fails with "cannot find" errors, ensure Rust is updated:
  ```bash
  rustup update
  ```
- If you get linker errors on macOS, you may need to install Xcode Command Line Tools:
  ```bash
  xcode-select --install
  ```

### 3. Install Node.js Dependencies

```bash
# At repo root
pnpm install

# This installs dependencies for:
# - apps/driver-web
# - apps/driver-mobile
# - apps/admin-dashboard
# - apps/partner-dashboard
# - packages/ui
# - packages/api-client
```

**Troubleshooting**:
- If `pnpm install` fails with version errors, clear cache and retry:
  ```bash
  pnpm store prune
  pnpm install
  ```

### 4. Set Up Environment Variables

```bash
# Copy example .env file
cp infra/env/.env.example .env

# Edit .env if needed (defaults work for local development):
# POSTGRES_PASSWORD=postgres123
# SERVICE_PORT=8000
```

The `.env` file is gitignored for security. Each developer maintains their own local copy.

---

## Daily Development Flow

### Start the Stack

```bash
# From repo root, start Docker Compose
docker compose -f infra/compose/docker-compose.yml up -d

# Alternatively, just:
docker compose up -d         # If docker-compose.yml is in root

# Watch startup logs:
docker compose logs -f driver-service
```

**Expected output** (after ~20 seconds):
```
driver-service-1  | Listening on 0.0.0.0:8000
driver-service-1  | [GET /health] registered
```

### Verify Services Are Healthy

```bash
# Check service status
docker compose ps

# Test health endpoint
curl http://localhost:8000/health
# Should respond: { "status": "ok" }

# Access pgAdmin (optional)
# Open http://localhost:5050 in browser
# Login: admin@localhost.local / admin123
```

### Run Rust Tests

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p ev-core
cargo test -p ev-geo
cargo test -p ev-db

# Run with output printed
cargo test -- --nocapture
```

### Run Frontend Dev Servers

```bash
# Driver Web App
cd apps/driver-web
pnpm dev
# Opens http://localhost:5173

# Driver Mobile App
cd apps/driver-mobile
pnpm start
# or
expo start
```

### Stop the Stack

```bash
# Stop containers (data persists)
docker compose down

# Stop and remove all data (fresh start)
docker compose down -v
```

---

## Common Tasks

### View Database

#### Option 1: pgAdmin (Web UI)
```
1. Open http://localhost:5050
2. Login: admin@localhost.local / admin123
3. Navigate to Servers > PostgreSQL > platform_db > Schemas
```

#### Option 2: psql Command Line
```bash
docker compose exec postgres psql -U postgres -d platform_db

# List schemas
\dn

# List tables in inventory schema
\dt inventory.*

# Query data
SELECT * FROM inventory.partner LIMIT 10;

# Exit
\q
```

#### Option 3: Programmatically
```bash
docker compose exec postgres psql -U postgres -d platform_db \
  -c "SELECT count(*) FROM information_schema.tables WHERE table_schema='inventory';"
```

### View Service Logs

```bash
# Driver Service only
docker compose logs driver-service

# Follow real-time logs
docker compose logs -f driver-service

# Last 50 lines
docker compose logs --tail=50 driver-service

# Combine with timestamp
docker compose logs --timestamps driver-service
```

### Reset Database

```bash
# Stop stack and remove volumes
docker compose down -v

# Restart stack (migrations run automatically)
docker compose up -d
```

### Run Migrations Manually

```bash
# View pending migrations
docker compose exec driver-service sqlx migrate info

# Run all pending migrations
docker compose exec driver-service sqlx migrate run

# Reset and re-run (careful!)
docker compose exec driver-service sqlx database reset -y
```

### Debug Driver Service

```bash
# Increase logging
docker compose down
export RUST_LOG=debug
docker compose up -d driver-service

# View debug logs
docker compose logs -f driver-service
```

### Check Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Format JavaScript
pnpm run format

# Lint JavaScript
pnpm run lint
```

### Rebuild Services

```bash
# Rebuild Rust (without Docker rebuild)
cargo build --release

# Rebuild Docker image
docker compose build --no-cache driver-service

# Restart service
docker compose restart driver-service
```

---

## Troubleshooting

### Port Already in Use

```
ERROR: driver-service-1  | bind: address already in use
```

**Solution**:
```bash
# Find what's using port 8000
lsof -i :8000

# Kill the process
kill -9 <PID>

# Or change port in .env:
# SERVICE_PORT=8001
docker compose up -d
```

### Database Connection Failed

```
ERROR: driver-service-1  | could not connect to database at postgresql://postgres:postgres123@postgres:5432/platform_db
```

**Solution**:
```bash
# Ensure PostgreSQL is healthy
docker compose ps       # Check Status column

# Check PostgreSQL logs
docker compose logs postgres

# If stuck, reset:
docker compose down -v
docker compose up -d
sleep 10
docker compose logs driver-service
```

### Migrations Failed

```
ERROR: driver-service-1  | Migration failed: ...
```

**Solution**:
```bash
# View migration error details
docker compose logs driver-service | grep -A 10 "Migration"

# Check migration files for syntax errors
ls -la db/migrations/

# Manually run migration to see error
docker compose exec postgres psql -U postgres -d platform_db \
  < db/migrations/0001_extensions.sql

# If database is corrupted, reset
docker compose down -v
docker compose up -d
```

### Rust Compilation Failed

```
error: could not find `tokio` in dependency tree
```

**Solution**:
```bash
# Update Rust
rustup update

# Clear build cache
cargo clean

# Rebuild
cargo build
```

### Node Dependencies Conflict

```
error: peer dep missing: "react@^18"
```

**Solution**:
```bash
# Clear pnpm cache
pnpm store prune

# Reinstall
rm -rf node_modules pnpm-lock.yaml
pnpm install
```

---

## Architecture Overview

### Directory Structure

```
BorneMap/
├── services/driver-service/     # Main HTTP service (Actix-Web)
├── crates/                       # Shared Rust libraries
│   ├── ev-core/                 # IDs, types, enums
│   ├── ev-geo/                  # Spatial math
│   └── ev-db/                   # Database utilities
├── apps/                         # Frontend applications
│   ├── driver-web/              # React web app
│   ├── driver-mobile/           # React Native mobile app
│   ├── admin-dashboard/         # Admin panel (stub)
│   └── partner-dashboard/       # Partner panel (stub)
├── packages/                     # Shared Node.js libraries
│   ├── ui/                      # Design tokens & components
│   └── api-client/              # HTTP API client
├── db/                          # Database migrations
│   └── migrations/              # SQL migration files
├── infra/                       # Infrastructure config
│   ├── compose/                 # Docker Compose setup
│   ├── env/                     # Environment templates
│   └── osm/                     # OSM import scripts (Sprint 1)
├── docs/                        # Project documentation
└── specs/003-sprint-0-foundation/  # This sprint's specification
```

### Service Architecture

```
┌──────────────────────────────────────────┐
│      Docker Compose (Local Dev)          │
├──────────────────────────────────────────┤
│                                          │
│  PostgreSQL 14+PostGIS                   │
│  └─ platform_db                          │
│     ├─ inventory schema (partner, station, charger)
│     └─ gis schema (osm_*, roads, boundaries)
│                                          │
│  Driver Service (Actix-Web)              │
│  └─ GET /health                          │
│  └─ GET /stations/nearby (stub)          │
│  └─ GET /stations/markers (stub)         │
│  └─ GET /stations/search (stub)          │
│  └─ GET /stations/:id (stub)             │
│                                          │
│  pgAdmin (Web UI, dev only)              │
│  └─ localhost:5050                       │
│                                          │
└──────────────────────────────────────────┘
```

### Development Workflow

```
1. Clone & setup (once)
   └─ cargo build
   └─ pnpm install

2. Daily development (every session)
   └─ docker compose up
   └─ cargo test / pnpm dev
   └─ Make changes
   └─ Verify with curl / browser

3. Before committing
   └─ cargo fmt && cargo clippy
   └─ pnpm run format && pnpm run lint
   └─ git add & git commit

4. Before push
   └─ Run full test suite
   └─ Verify Stack starts cleanly
   └─ Check documentation
```

---

## Testing

### Unit Tests (Rust)

```bash
# All tests
cargo test

# Specific crate
cargo test -p ev-core

# With output
cargo test -- --nocapture

# Run a specific test
cargo test test_nanoid_generation
```

### Integration Tests (Rust)

```bash
# Integration tests (require database)
cargo test --test integration

# (These are in services/driver-service/tests/)
```

### Component Tests (JavaScript)

```bash
cd apps/driver-web
pnpm test          # Jest/Vitest tests (if configured)
```

---

## Performance

### Build Time

**First build**: 2-3 minutes (Rust compilation)  
**Incremental build**: 10-30 seconds  
**pnpm install**: 1-2 minutes

### Runtime

**Docker Compose startup**: ~30 seconds  
**Migrations**: <5 seconds  
**Health check response**: <100ms  
**/stations/nearby query** (once implemented): <100ms (target)

---

## Next Steps

After Sprint 0 setup is complete:

1. **Read Documentation**
   - `docs/03-architecture/clean-architecture.md` — Layer structure
   - `docs/10-delivery/mvp01/README.md` — Sprint plan

2. **Explore Code**
   - `services/driver-service/src/main.rs` — Service entrypoint
   - `crates/ev-core/src/ids.rs` — NanoID implementation
   - `apps/driver-web/src/App.jsx` — Frontend scaffold

3. **Sprint 1 Prep** (next sprint)
   - OSM data import
   - Database seeds
   - First real endpoint implementation

---

## Support

**Issues**:
- Check logs: `docker compose logs -f`
- Verify prerequisites: Run prerequisite checks again
- Search existing issues: GitHub Issues tab
- Ask team in Slack/Discord

**Documentation**:
- API: `specs/003-sprint-0-foundation/contracts/api.md`
- Data Model: `specs/003-sprint-0-foundation/data-model.md`
- Docker: `specs/003-sprint-0-foundation/contracts/docker-compose.md`

---

## Quick Reference

```bash
# Setup (first time)
git checkout 003-sprint-0-foundation
cargo build
pnpm install
cp infra/env/.env.example .env

# Daily work
docker compose up -d
cargo test
pnpm dev

# Cleanup
docker compose down
docker compose down -v    # Remove data

# Useful commands
curl http://localhost:8000/health
docker compose ps
docker compose logs -f driver-service
docker compose exec postgres psql -U postgres -d platform_db
```

---

**Last Updated**: 2026-06-05  
**Status**: Sprint 0 Foundation  
**Next**: Sprint 1 — OSM Schema + Data Import
