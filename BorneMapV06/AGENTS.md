<!-- SPECKIT START -->
**Current Implementation Plan**: `/specs/004-sprint-1-foundation/plan.md`

**Feature Branch**: `004-sprint-1-foundation`

**Sprint**: 1 — OSM Data & Station Discovery

**Key Artifacts**:
- Specification: `/specs/004-sprint-1-foundation/spec.md`
- Implementation Plan: `/specs/004-sprint-1-foundation/plan.md`
- Research: `/specs/004-sprint-1-foundation/research.md`
- Data Model: `/specs/004-sprint-1-foundation/data-model.md`
- API Contracts: `/specs/004-sprint-1-foundation/contracts/`
- Quickstart: `/specs/004-sprint-1-foundation/quickstart.md`
- Tasks: `/specs/004-sprint-1-foundation/tasks.md` (generated during `/speckit.tasks`)

**Project Structure**: Rust monorepo with Clean Architecture per service:
- `crates/driver-service/` — Station discovery & favorites (domain/application/infrastructure/interface)
- `crates/partner-service/` — Partner station management
- `crates/gis-worker/` — Async GIS sync worker (outbox pattern)
- `crates/ev-geo/` — Spatial utilities (Haversine distance)
- `crates/ev-auth/` — Keycloak JWT validation
- `crates/ev-domain/` — Shared domain models & identifiers

**Key Technology Decisions**:
- OSM Import: osm2pgsql (standard, battle-tested)
- Spatial Queries: PostGIS ST_DWithin with GIST indexes
- GIS Sync: Outbox pattern + Last-Write-Wins (no blocking)
- Rate Limiting: IP-based (100 req/min on public discovery)
- Authentication: Keycloak JWT (fail-secure if unavailable)
- Favorite Deletion: Hard delete (ephemeral data, no audit needed)

**Constitution Compliance**: ✅ All checks passed
- Clean Architecture per service ✅
- Keycloak-only identity ✅
- GIS as derived layer (no blocking) ✅
- Partner scope at API layer ✅
- Soft deletes for stations (hard for favorites) ✅

**For additional context about technologies, project structure, implementation decisions, and architecture patterns, read the plan at the path above.**
<!-- SPECKIT END -->
