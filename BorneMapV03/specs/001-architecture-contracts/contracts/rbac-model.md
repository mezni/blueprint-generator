# RBAC Model

## Purpose

Define the role-based access control model. Exactly three roles exist. No
additional roles may be introduced without explicit architectural approval.

## Version

1.0.0

## Roles

| Role | Authenticated | Keycloak Mapper | Default Scope |
|------|--------------|-----------------|---------------|
| `registered_driver` | Yes (Keycloak) | Direct role mapping | Own profile, favorites, reviews |
| `partner` | Yes (Keycloak) | Role + `partner_id` claim | Own stations, chargers, availability, scoped reports |
| `admin` | Yes (Keycloak) | Direct role mapping | Full platform access |

## Enforcement Layers

| Layer | Enforcement |
|-------|-------------|
| Keycloak | Authentication, token issuance, role claims in JWT |
| Service | JWT validation middleware, role extraction, route-level guards |
| Database | `partner_id` column constraints, RLS (row-level security) on `inventory` |

## Partner Rule

- One user belongs to exactly one partner organization.
- `partner_id` is embedded in the JWT as a custom claim by Keycloak.
- All `inventory` queries MUST filter by `partner_id` at the repository level.
- No API-layer logic may override or omit the `partner_id` filter.

## Public Access (Unauthenticated)

Not a role. Unauthenticated users can browse stations, view map markers,
search/filter, and view public ratings & reviews. All other capabilities
require authentication.
