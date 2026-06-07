# Docker Compose Service Contract: Sprint 0

**Date**: 2026-06-05  
**Plan**: [plan.md](../plan.md)  
**Purpose**: Define service interfaces and dependencies for local development

---

## Overview

Sprint 0 Docker Compose stack runs three services:

1. **PostgreSQL** — Database backend with PostGIS
2. **Driver Service** — Rust Actix-Web application
3. **pgAdmin** — Web UI for database inspection (dev only)

All services communicate via Docker Compose network (`bornemap-net` or default bridge).

---

## Service Definitions

### PostgreSQL

**Service Name**: `postgres`  
**Image**: `postgis/postgis:14-3.2`  
**Port**: 5432 (internal) → 5432 (host)

#### Environment Variables (Required)
```env
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres123          # Use strong password in production
POSTGRES_DB=platform_db                # Primary business database
POSTGRES_INITDB_ARGS="-c shared_preload_libraries=postgis"
```

#### Volumes
```yaml
volumes:
  - postgres_data:/var/lib/postgresql/data   # Persistent storage
```

#### Health Check
```yaml
healthcheck:
  test: ["CMD-SHELL", "pg_isready -U postgres"]
  interval: 10s
  timeout: 5s
  retries: 5
```

#### Network
- Connected to: `bornemap-net` (or default Docker Compose network)
- Accessible internally as: `postgres:5432`
- Accessible from host: `localhost:5432`

#### Initialization
- Migrations run automatically via driver-service on startup (once database is healthy)
- Or manually via: `docker compose exec postgres psql -U postgres -d platform_db -f /migrations/...`

---

### Driver Service

**Service Name**: `driver-service`  
**Build**: `./services/driver-service` (Dockerfile at Sprint 0: uses `cargo build`)  
**Port**: 8000 (internal) → 8000 (host)

#### Environment Variables (Required)
```env
DATABASE_URL=postgresql://postgres:postgres123@postgres:5432/platform_db
SERVICE_HOST=0.0.0.0
SERVICE_PORT=8000
RUST_LOG=info                          # Tracing/logging level
```

#### Depends On
```yaml
depends_on:
  postgres:
    condition: service_healthy         # Wait for PostgreSQL health check
```

#### Health Check
```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
  interval: 10s
  timeout: 5s
  retries: 5
  start_period: 30s                    # Allow time for migrations
```

#### Network
- Connected to: `bornemap-net`
- Accessible internally as: `driver-service:8000`
- Accessible from host: `localhost:8000`

#### Startup Behavior
1. Wait for PostgreSQL to be healthy
2. Run database migrations (`sqlx migrate run`)
3. Start Actix-Web server on 0.0.0.0:8000
4. Log startup completion

#### Shutdown Behavior
- Graceful shutdown: Signal handling in Actix-Web
- Close database connections
- Log shutdown completion

---

### pgAdmin (Development Only)

**Service Name**: `pgadmin`  
**Image**: `dpage/pgadmin4:latest`  
**Port**: 5050 (internal) → 5050 (host)

#### Environment Variables
```env
PGADMIN_DEFAULT_EMAIL=admin@localhost.local
PGADMIN_DEFAULT_PASSWORD=admin123       # Development only
PGADMIN_CONFIG_ENHANCED_COOKIE_PROTECTION=False
PGADMIN_CONFIG_COOKIE_SAMESITE='Lax'
```

#### Volumes (Optional)
```yaml
volumes:
  - pgadmin_data:/var/lib/pgadmin       # Persistent pgAdmin config
```

#### Depends On
```yaml
depends_on:
  - postgres                            # Non-essential; pgAdmin gracefully handles offline DB
```

#### Network
- Connected to: `bornemap-net`
- Accessible from host: `http://localhost:5050`

#### Access
- **Email**: admin@localhost.local
- **Password**: admin123
- **PostgreSQL Server**: postgres:5432 (auto-discovered on first login)

#### Purpose
- Inspect database tables and data (development convenience only)
- Run ad-hoc SQL queries
- Verify migrations executed correctly

**Note**: pgAdmin is **development-only** and is removed from production deployments.

---

## Network Topology

```
┌─────────────────────────────────────────────┐
│         Docker Compose Network              │
│           (bornemap-net or default)         │
├─────────────────────────────────────────────┤
│                                             │
│  ┌──────────────────┐                      │
│  │   PostgreSQL     │                      │
│  │   Port 5432      │                      │
│  └────────┬─────────┘                      │
│           │                                │
│           │ (internal: postgres:5432)      │
│           │                                │
│  ┌────────▼──────────────────┐             │
│  │   Driver Service (Actix)   │             │
│  │   Port 8000                │             │
│  └───────────────────────────┘             │
│           │                                │
│           │ (internal: driver-service:8000)│
│           │                                │
│  ┌────────▼──────────────────┐             │
│  │   pgAdmin (dev only)       │             │
│  │   Port 5050                │             │
│  └───────────────────────────┘             │
│                                             │
└─────────────────────────────────────────────┘
         │
         │ (host bindings)
         │
    ┌────▼────────────────┐
    │   Host Machine       │
    ├──────────────────────┤
    │ localhost:5432   → postgres
    │ localhost:8000   → driver-service
    │ localhost:5050   → pgAdmin
    └──────────────────────┘
```

---

## Volumes

### Persistent Data

```yaml
volumes:
  postgres_data:
    driver: local                       # Local filesystem
  pgadmin_data:
    driver: local
```

**Note**: Volumes persist between `docker compose down` and `docker compose up`. To reset:

```bash
docker compose down -v                  # -v removes all volumes
```

---

## Environment Configuration

### Development (.env file)

```env
# PostgreSQL
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres123
POSTGRES_DB=platform_db

# Driver Service
SERVICE_HOST=0.0.0.0
SERVICE_PORT=8000
DATABASE_URL=postgresql://postgres:postgres123@postgres:5432/platform_db
RUST_LOG=info

# pgAdmin
PGADMIN_DEFAULT_EMAIL=admin@localhost.local
PGADMIN_DEFAULT_PASSWORD=admin123
```

### Production (future)

Production deployments override via environment variables (not Docker Compose):
- No hardcoded passwords
- Managed credentials via Vault or cloud provider secrets
- Separate database instances per environment
- pgAdmin disabled

---

## Usage

### Start Stack
```bash
docker compose up -d                    # Start in background
docker compose up                       # Start in foreground (see logs)
```

### Verify Health
```bash
curl http://localhost:8000/health      # Should return 200 OK
docker compose ps                       # Check service status
```

### View Logs
```bash
docker compose logs -f driver-service   # Follow Driver Service logs
docker compose logs postgres            # PostgreSQL logs
```

### Stop Stack
```bash
docker compose down                     # Stop and remove containers
docker compose down -v                  # Also remove volumes (reset DB)
```

### Run Migrations Manually
```bash
docker compose exec postgres psql -U postgres -d platform_db \
  < db/migrations/0001_extensions.sql
```

### Connect to Database
```bash
# Via psql
docker compose exec postgres psql -U postgres -d platform_db

# Via pgAdmin
# Open http://localhost:5050
# Login: admin@localhost.local / admin123
```

---

## Startup Sequence

1. **Docker Compose reads docker-compose.yml**
   - Defines services, volumes, networks, environment

2. **PostgreSQL starts**
   - Initializes database `platform_db`
   - PostGIS extension auto-loaded (`shared_preload_libraries`)
   - Health check begins polling

3. **Driver Service waits for PostgreSQL**
   - `depends_on: condition: service_healthy` ensures PG is ready
   - Attempts database connection via `DATABASE_URL`

4. **Driver Service runs migrations**
   - `sqlx migrate run` executes all SQL migrations in order
   - Creates `inventory` and `gis` schemas
   - Creates `_sqlx_migrations` table to track applied migrations

5. **Driver Service starts HTTP server**
   - Listens on 0.0.0.0:8000
   - `/health` endpoint responds with 200
   - All 4 expected handlers registered

6. **pgAdmin starts (if included)**
   - Web UI available at localhost:5050
   - Auto-discovers PostgreSQL service via DNS

7. **Stack is ready**
   - All health checks passing
   - All services can communicate internally
   - Developer can query localhost:8000, localhost:5432, localhost:5050

---

## Failure Modes & Recovery

### PostgreSQL fails to start
**Symptom**: `driver-service` restarts indefinitely, health check fails
**Fix**:
```bash
docker compose down -v                  # Remove volume
docker compose up                       # Recreate from scratch
```

### Driver Service migration fails
**Symptom**: `driver-service` exits with error, no /health response
**Fix**:
1. Check logs: `docker compose logs driver-service`
2. Identify migration error
3. Fix SQL in `db/migrations/`
4. Restart: `docker compose restart driver-service`

### Port 5432 already in use
**Symptom**: `postgres` fails to bind port 5432
**Fix**:
```bash
# Find what's using port 5432
lsof -i :5432

# Either kill the process or change docker-compose.yml:
# ports:
#   - "15432:5432"  # Bind to different host port
docker compose up
```

### Network connectivity issues
**Symptom**: `driver-service` cannot resolve `postgres` hostname
**Fix**:
- Ensure all services are on same network: `docker network inspect`
- Verify service names match in compose file
- Restart: `docker compose down && docker compose up`

---

## Monitoring & Debugging

### Check Service Status
```bash
docker compose ps
```

Expected output:
```
NAME                    COMMAND                  SERVICE         STATUS
bornemap-postgres-1     "docker-entrypoint.s…"  postgres        Up (healthy)
bornemap-driver-1       "cargo run"              driver-service  Up (healthy)
bornemap-pgadmin-1      "/entrypoint.sh"        pgadmin         Up
```

### View Real-Time Logs
```bash
docker compose logs -f                  # All services
docker compose logs -f driver-service   # Specific service
docker compose logs postgres --tail=50  # Last 50 lines
```

### Execute Commands in Running Container
```bash
docker compose exec postgres psql -U postgres -d platform_db -c "SELECT count(*) FROM inventory.station;"
docker compose exec driver-service curl http://localhost:8000/health
```

### Inspect Database
```bash
docker compose exec postgres pg_dump -U postgres platform_db > backup.sql
```

---

## Related Documentation

- **Configuration Template**: `infra/env/.env.example`
- **Docker Compose File**: `infra/compose/docker-compose.yml` (Sprint 0)
- **Database Migrations**: `db/migrations/`
- **Driver Service**: `services/driver-service/`
