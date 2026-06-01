# API Versioning Contract

**Status**: Adopted (2026-05-31)
**Scope**: All HTTP APIs exposed by the BorneMap platform

## 1. Canonical Rule

All HTTP APIs MUST be exposed and consumed under the unified versioned prefix:

```
/api/v1
```

## 2. Route Mapping

| Path | Target | Service |
|------|--------|---------|
| `/api/v1/admin/*` | Admin Service | Backend |
| `/api/v1/driver/*` | Driver Service | Backend |
| `/api/v1/events/*` | Clickstream Service | Backend |
| `/auth/*` | Keycloak | Identity (separate prefix) |
| `/` | `driver-web` | Frontend |
| `/admin` | `admin-dashboard` | Frontend |
| `/partner` | `partner-dashboard` | Frontend |

## 3. Service-Side Enforcement

Every backend service MUST prefix all routes internally with `/api/v1`.

**Allowed example** (Driver Service):
```
GET  /api/v1/driver/stations/nearby
GET  /api/v1/driver/stations/:id
POST /api/v1/driver/favorites
```

**Forbidden** (unversioned endpoints):
```
/stations
/favorites
/login
```

## 4. Scope

Versioning applies to:
- All REST APIs
- All internal service APIs exposed via Traefik
- All mobile/web API calls

NOT applied to:
- Health checks (`/health`)
- Metrics (`/metrics`)
- Internal DB connections

## 5. Version Lifecycle

- **v1**: Initial (Phase 1, this epic)
- **v2+**: Future breaking changes
- No unversioned endpoints permitted at any time

## 6. Frontend Client Rule

All API clients MUST assume `baseURL = "/api/v1"`. No client may hardcode an unversioned path.

## 7. Enforcement

- **Gateway (Traefik)**: Rejects non-`/api/v1` requests to backend services
- **Code review**: Blocks any PR introducing unversioned routes
- **CI**: Contract validation checks path prefixes
