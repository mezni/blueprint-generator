# Phase 0: Research

## Technology Validation

### Map Tile Provider: CartoDB Positron
- **Chosen**: CartoDB Positron (free tier, no API key, attribution required)
- **Fallback**: OpenStreetMap tiles (tile.openstreetmap.org) if CDN unreachable
- **Reasoning**: Positron is the standard light map for admin panels — clean, readable,
  matches shadcn/ui neutral aesthetic
- **Edge case**: In China, both CDNs may be blocked — Phase 5 will add a tile proxy

### Mobile Runtime: Expo Go (Managed)
- Expo SDK 51 with managed workflow — no `expo eject` or `expo prebuild`
- Sufficient for Phase 0-1 prototyping
- EAS Build reserved for Phase 2+ (native plugins, App Store deployment)
- Limitation: `expo-location` requires `ACCESS_FINE_LOCATION` permission
  which Expo Go supports

### PostGIS Docker Image
- `postgis/postgis:16-3.4` — PostgreSQL 16 with PostGIS 3.4
- Exposed on port 5432, mounted volume for persistence
- Health check via `pg_isready`

### shadcn/ui Setup
- `npx shadcn@latest init` generates `components/ui/` directory
- Components added individually with `npx shadcn@latest add button card`
- Design tokens in `globals.css` via CSS custom properties
- CartoDB Positron map background matches shadcn neutral color palette

### FastAPI + asyncpg
- SQLAlchemy 2.x async engine with asyncpg driver
- `AsyncSession` factory yields sessions per request
- Session-per-request pattern via FastAPI `Depends()`
- Connection pooling via `create_async_engine(pool_size=5, max_overflow=10)`

### CI Pipeline
- Two workflows: `backend.yml` and `frontend.yml`
- Backend: Ruff lint → Black format check → pytest (with docker service postgis)
- Frontend: two jobs — `web` (`npm ci` → `tsc --noEmit` → `npm run build`) and `mobile` (`pnpm install --frozen-lockfile` → `pnpm typecheck` → `pnpm test`)
- Mobile uses pnpm 9+ via `pnpm/action-setup@v4`
- Both triggered on PR to `main`

## Unresolved Questions

None — spec is complete for Phase 0 scope.

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| PoSTGIS Docker image version drift | Low | Medium | Pin to `16-3.4` in docker-compose |
| Expo SDK breaking managed runtime | Low | High | Pin to SDK 51, test before upgrade |
| Leaflet tile CDN downtime | Medium | Low | OSM fallback + warning banner |
| CI minutes exhaustion | Low | Medium | Local lint/typecheck before push |
