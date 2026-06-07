# Phase 0: Research — Sprint 10 Partner Dashboard

## Decisions

### 1. Frontend Framework & Tooling
- **Decision**: Vite + React 19 + Tailwind CSS 4
- **Rationale**: Matches existing driver-web app; minimal new tooling; shared design tokens package already exists
- **Alternatives considered**: Next.js (rejected: constitution forbids it), plain HTML/CSS (rejected: need React state management)

### 2. Server State Management
- **Decision**: `@tanstack/react-query` v5
- **Rationale**: Already used by driver-web; provides caching, background refetch, optimistic updates; no new dependency
- **Alternatives considered**: Redux Toolkit (rejected: overkill for this use case), Zustand (rejected: no built-in caching)

### 3. Auth Integration
- **Decision**: `keycloak-js` with PKCE, `check-sso` mode
- **Rationale**: Matches driver-web pattern; `check-sso` avoids login wall for unauthenticated users; PKCE for SPA security
- **Alternatives considered**: Direct OIDC fetch (rejected: keycloak-js handles token refresh and silent check-sso)

### 4. Routing
- **Decision**: `react-router` v7
- **Rationale**: Matches driver-web; file-system routing not needed for 3 pages; simple `Routes`/`Route` setup
- **Alternatives considered**: TanStack Router (overkill for 3 routes)

### 5. API Client
- **Decision**: Extend existing `@bornemap/api-client` with `headers` parameter
- **Rationale**: Avoids adding another HTTP client; partner endpoints require `Idempotency-Key` and `If-Match` headers
- **Changes**: Added optional `headers` param to `request()`, `post()`, `patch()`, `delete()` methods

### 6. Partner API Routing (Traefik)
- **Decision**: No stripPrefix middleware on `/api/v1/partner`
- **Rationale**: Admin-service partner routes register with full prefix (`/api/v1/partner/{resource}`); stripping would break path matching
- **Alternatives considered**: Adding stripPrefix + removing prefix from route registrations (would change existing backend code)

### 7. Admin-service Binary Deployment
- **Decision**: Host-compiled debug binary copied into container
- **Rationale**: Full Docker Rust rebuild takes 30+ min; debug binary build takes ~3s; container environment supports it
- **Caveat**: Binary must be recompiled if host libc differs from container; migration files mounted as volume

### 8. Database Migrations
- **Decision**: Non-fatal migration failures (warn instead of panic)
- **Rationale**: Database already seeded from initial deployment; re-running seed migrations conflicts on unique constraints; existing data must not be lost
- **Fix**: Changed `ON CONFLICT (id)` to `ON CONFLICT (keycloak_user_id)` for user insert in `0016_seed_data.up.sql`
