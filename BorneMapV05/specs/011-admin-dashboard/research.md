# Research: Sprint 11 — Admin Dashboard

## Decisions

### Decision: Frontend-Only Sprint — No Backend Changes Required
- **Decision**: Admin dashboard is a pure frontend sprint. Admin API endpoints (`/api/v1/admin/*`) are already implemented in admin-service from Sprint 5.
- **Rationale**: The admin-service already exposes partner CRUD, station CRUD, review moderation, user listing, and report endpoints. No new backend code is needed.
- **Alternatives considered**: Building new admin-specific backend endpoints. Rejected because existing Sprint 5 endpoints already cover the required functionality.

### Decision: Same Layout as Partner Dashboard (260px Sidebar + Main Content)
- **Decision**: Reuse the same sidebar layout pattern as the partner dashboard.
- **Rationale**: Platform consistency reduces cognitive load for users who may switch between admin and partner dashboards. Reuses existing design system components.
- **Alternatives considered**: Full-width top nav (less dense for admin data density needs); minimal icon sidebar (insufficient for 5+ nav items).

### Decision: Reports Page Deferred to Sprint 14/15
- **Decision**: The Reports page (US6, FR-014) is out of scope for this sprint.
- **Rationale**: Reports require analytics data from the analytics pipeline, which isn't active until Sprint 14. Building with mock data creates maintenance debt.
- **Alternatives considered**: Build with mock/placeholder data; build a simplified overview with non-analytics counts only. Both add scope without delivering real value.

### Decision: Clickstream Events Wired Now
- **Decision**: Emit clickstream events from admin dashboard following the same pattern as partner-dashboard (`@bornemap/event-taxonomy` + `api-client` POST to `/api/v1/clickstream`).
- **Rationale**: The event taxonomy defines admin events (`admin_station.created`, `admin_review.moderated`). Wired properly, events are safe to emit even if the clickstream pipeline isn't fully active (events are dropped at ingress if service is down).
- **Alternatives considered**: Deferring all clickstream work. Creates a second pass through every component in Sprint 13.

### Decision: Station Editing Includes Coordinates with Confirmation
- **Decision**: The station edit form includes latitude/longitude fields. Changing coordinates triggers a confirmation dialog before saving.
- **Rationale**: Coordinates are part of the station data model. Admins should be able to correct them. GIS resync handles the update automatically via the existing outbox pattern. Confirmation prevents accidental changes.
- **Alternatives considered**: Excluding coordinates (admins can't fix bad data); including without confirmation (risk of accidental changes).

### Decision: Chargers Shown as Inline Detail in Station Views
- **Decision**: Charger count appears as a column in the station list. Clicking a station row expands to show charger details (type, power_kw, status).
- **Rationale**: Follows the same pattern as the partner dashboard. No separate charger management page is needed for admin.
- **Alternatives considered**: Separate Chargers management page (overkill for admin needs); no charger visibility (admins need to see what's installed).

### Decision: Reuse Existing Shared Packages
- **Decision**: Use `@bornemap/api-client`, `@bornemap/auth-client`, `@bornemap/design-tokens`, `@bornemap/api-contracts`, `@bornemap/event-taxonomy`, and `@bornemap/shared-types` from the monorepo packages.
- **Rationale**: These packages are already consumed by the partner dashboard and driver web. Reuse ensures API contract consistency and respects the monorepo structure.
- **Alternatives considered**: Building admin-specific API/auth clients. Duplicates effort and creates drift risk.

### Decision: React Query for Server State
- **Decision**: Use `@tanstack/react-query` for all server state management (queries and mutations).
- **Rationale**: Same pattern as partner dashboard and driver web. Provides caching, deduplication, optimistic updates, and error handling out of the box.
- **Alternatives considered**: Raw fetch calls (no caching); Redux (overkill for this scope).

### Decision: Keycloak PKCE Auth (check-sso)
- **Decision**: Use `@bornemap/auth-client` with `keycloak-js` for silent token check (`check-sso`). Login redirects to Keycloak login page; token refresh is automatic.
- **Rationale**: Same pattern as partner dashboard. Silent check avoids redirect on page load if already authenticated.

## Key Findings

- The admin-service Traefik route (`/api/v1/admin/*`) with `stripPrefix` was noted in the partner dashboard sprint as potentially broken. This sprint's backend verification should confirm admin routes respond correctly (401 for unauthenticated).
- The `admin` role must exist in the Keycloak realm and be assigned to admin users. This was configured in Sprint 3.
- For testing admin endpoints: `curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/admin/partners` should return the partners list.
