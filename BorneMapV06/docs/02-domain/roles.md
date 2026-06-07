# Roles (Strict Set)

## Allowed Roles

| Role | Keycloak Role | Scope |
|------|---------------|-------|
| Registered Driver | `registered_driver` | Own data only |
| Partner | `partner` | Own organization only |
| Admin | `admin` | Global |

## Rules

- No additional roles are allowed beyond these three.
- Partner users are strictly scoped to one organization.
- Role assignment is managed exclusively through Keycloak.
- The system MUST reject any unrecognized role.

## Enforcement

Roles are enforced at multiple layers:

1. **JWT validation** — every authenticated request carries the role in claims
2. **API middleware** — each endpoint checks required role
3. **Partner scope** — partner endpoints filter data by organization ID
4. **Database queries** — scoped queries prevent cross-organization data leaks
