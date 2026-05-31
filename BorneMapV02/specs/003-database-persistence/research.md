# Research: Database Persistence & Spatial Query Engine

## Technology Decisions

### Database Driver: sqlx vs. diesel

**Decision**: sqlx 0.8 (async, compile-time checked queries)

**Rationale**:
- sqlx provides `query!` macro that validates SQL against the live database at compile time — catches schema mismatches early
- Native async support aligns with Actix-web's async runtime
- Direct SQL control needed for PostGIS spatial functions (`ST_DWithin`, `ST_MakePoint`) — ORMs like diesel add complexity for spatial queries
- Lighter dependency footprint than diesel for this use case

**Alternatives considered**:
- diesel: Requires custom PostGIS support types; more boilerplate for spatial queries
- tokio-postgres: Lower level, no compile-time checking, more manual mapping

### PostGIS Docker Image

**Decision**: postgis/postgis:15-3.3

**Rationale**:
- PostgreSQL 15 with PostGIS 3.3 — stable, well-supported combination
- Official PostGIS Docker image maintained by the community
- Matches constitution's requirement for PostGIS spatial computations

**Alternatives considered**:
- postgres:15 + manual PostGIS extension: More setup steps, no advantage
- TimescaleDB: Overkill for <2,000 stations

### Migration Strategy

**Decision**: Raw SQL migration files in `backend/db/migrations/`, applied manually via `psql` or sqlx CLI

**Rationale**:
- Simplest approach for a small team and <5 migrations
- Full control over DDL including PostGIS-specific statements
- No need for a migration framework when migration count is low

**Alternatives considered**:
- sqlx migrate: Built-in but requires additional CLI setup
- diesel migration: Coupled to diesel ORM

### Spatial Query Approach

**Decision**: `ST_DWithin` on `GEOGRAPHY(Point, 4326)` with meter-based distance

**Rationale**:
- `ST_DWithin` on `GEOGRAPHY` uses meters directly — no coordinate system math needed
- GiST index on `geom` column ensures performant queries even at 2,000 stations
- Simple, standard PostGIS pattern

**Alternatives considered**:
- `ST_Distance` + `HAVING` clause: Less efficient, requires full table scan before filtering
- `ST_Within` + envelope: More complex, requires building a bounding box

### Health Check Pattern

**Decision**: Simple `GET /health` endpoint that runs `SELECT 1` against the database pool

**Rationale**:
- Lightweight, fast (sub-100ms)
- Validates actual database connectivity (not just process liveness)
- Standard Actix-web pattern

### API Security (from Clarification Q1)

**Decision**: Fully open (no auth) with network-level controls

**Rationale**:
- Eliminates auth implementation overhead for v1
- Network-level security (firewall, VPN) sufficient for internal/local deployments
- Rate limiting can be added as middleware when needed

### Status Update Mechanism (from Clarification Q2)

**Decision**: Manual `PATCH /api/v1/stations/{id}/status` endpoint

**Rationale**:
- Simple to implement and test
- Allows both manual operator updates and script-based updates
- Can be extended with auth later without restructuring

### Observability (from Clarification Q3)

**Decision**: Request logging (method, path, status, duration) + health check

**Rationale**:
- Covers the basics for debugging and operations
- No additional infrastructure dependencies (no Prometheus, no metrics server)
- Log output goes to stderr for container environments

### Data Volume (from Clarification Q4)

**Decision**: Design for <500 stations at launch, <2,000 in first year

**Rationale**:
- Simple GiST indexing on geometry column is sufficient
- No partitioning needed
- Connection pool of 5-10 connections adequate

### Rate Limiting (from Clarification Q5)

**Decision**: None in v1; architecture supports adding middleware later

**Rationale**:
- Not needed at current scale with network-level controls
- Actix-web middleware chain supports adding rate limiting without code restructure
