# Quickstart: Sprint 11 — Admin Dashboard

## Setup

```bash
# 1. Install dependencies
cd apps/admin-dashboard
npm install

# 2. Verify the app scaffold exists (should already be scaffolded from Sprint 1)
ls src/main.tsx src/App.tsx src/index.css src/lib/utils.ts
```

## Required Packages

The following dependencies need to be added to `apps/admin-dashboard/package.json`:

```
@bornemap/api-client
@bornemap/auth-client
@bornemap/design-tokens
@bornemap/api-contracts
@bornemap/event-taxonomy
@bornemap/shared-types
@tanstack/react-query
react-router
keycloak-js
class-variance-authority
```

These exist in the monorepo; install via:
```bash
npm install @bornemap/api-client @bornemap/auth-client @bornemap/design-tokens @bornemap/api-contracts @bornemap/event-taxonomy @bornemap/shared-types @tanstack/react-query react-router keycloak-js class-variance-authority
```

## Development

```bash
# Start the admin dashboard dev server (requires backend stack running)
npm run dev
```

The Vite dev server runs on its default port (typically 5173). API calls are proxied to `localhost:80` (Traefik) — verify the proxy config in `vite.config.ts`.

## Backend Prerequisites

The following must be running (via `docker compose up` in `infra/compose/`):

- Traefik (port 80)
- Keycloak (auth)
- admin-service (serves `/api/v1/admin/*`)
- PostgreSQL (platform_db)

## Verification

### Endpoint Check (Unauthenticated)
```bash
curl -v http://localhost/api/v1/admin/partners
# Expected: 401 Unauthenticated
```

### Endpoint Check (Authenticated)
```bash
# Get token from Keycloak (admin credentials)
TOKEN=$(curl -s -X POST http://localhost:8080/realms/bornemap/protocol/openid-connect/token \
  -d "client_id=bornemap-api" \
  -d "username=admin@bornemap.tn" \
  -d "password=admin" \
  -d "grant_type=password" | jq -r '.access_token')

curl -H "Authorization: Bearer $TOKEN" http://localhost/api/v1/admin/partners
# Expected: 200 with partners list (or empty array)
```

### Frontend Verification
```bash
open http://localhost:5173
# Should redirect to Keycloak login
# After login with admin credentials, should see dashboard overview
```

## Troubleshooting

- **401 on all requests**: Verify Keycloak is running and `admin` role exists
- **CORS errors**: Verify Vite proxy config points to `localhost:80`
- **API returns 404**: Verify Traefik admin route is configured in `routes.yml`
- **Missing packages**: Run `npm install` from `apps/admin-dashboard/`
- **TypeScript errors**: Run `npx tsc --noEmit` to check types
