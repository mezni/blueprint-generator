# Setup Guide

## Prerequisites

- Docker & Docker Compose
- Rust toolchain (for backend services)
- Node.js 20+ & pnpm (for frontend apps)

## Quick Start

```bash
# 1. Clone the repository
git clone <repo-url> && cd ev-platform

# 2. Start infrastructure
docker compose -f infra/compose/docker-compose.yml up -d

# 3. Run database migrations
# (apply migrations from db/migrations/ in order)

# 4. Start services
cargo run -p driver-service &
cargo run -p admin-service &

# 5. Start frontend
cd apps/driver-web && pnpm dev
```

## Docker Compose Services

The compose file starts:
- PostgreSQL (with PostGIS)
- RabbitMQ
- Keycloak
- Traefik (optional, for full stack)
