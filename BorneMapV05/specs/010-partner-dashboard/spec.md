# Sprint 10: Partner Dashboard

## Objective
Build an operational dashboard for charging station partners to manage their stations, chargers, and availability.

## Features

### US1: Station Management
- **List stations** with pagination, status, and availability badges
- **Create station** with name, address, coordinates (idempotent)
- **Update station** name, address, coordinates, status (optimistic concurrency via If-Match)
- **Delete station** (soft-delete)
- **Availability toggle** - quick inline status change (available/limited/unavailable)

### US2: Charger Management
- **List chargers** per station with type, power, status
- **Create charger** on a station (type, power, status)
- **Update charger** type, power, status (optimistic concurrency via If-Match)

### US3: Profile
- **View profile** - partner name, email, role

## Architecture

### Backend (already implemented in Sprint 5)
- Admin-service at `services/admin-service/src/routes/partner.rs`
- Endpoints under `/api/v1/partner/*` (full prefix, no Traefik strip)
- Auth- and role-gated (Partner role)

### Backend Change Required
- Add `/api/v1/partner` Traefik router → admin-service (no stripPrefix)

### Frontend
- `apps/partner-dashboard/` - Vite + React + Tailwind app
- Shared packages: `@bornemap/api-client`, `@bornemap/auth-client`, `@bornemap/design-tokens`
- Layout: Header + Sidebar(260px) + Main Content
- Pages: Stations, Station Detail (chargers), Profile
- Auth via Keycloak (check-sso, PKCE)
- React Query for server state
