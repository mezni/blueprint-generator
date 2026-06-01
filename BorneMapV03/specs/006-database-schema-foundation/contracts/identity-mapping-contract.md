# Contract: Identity Mapping

## Status

**Adopted**: 2026-05-31
**Scope**: Keycloak ↔ application user synchronization across all services
**Owner**: Platform Architecture

## Principle

Keycloak is the single source of truth for authentication and roles. The application database maintains a 1:1 mapping from Keycloak `sub` (subject) to `user_account.id` for application-level data ownership. Roles are NEVER stored in the application database.

## Mapping

```
Keycloak User (sub: "a1b2c3d4-...")
  │
  │ 1:1
  ▼
users.user_account (keycloak_id: "a1b2c3d4-...")
  │
  │ owns
  ▼
Application data (favorites, reviews, etc.)
```

## Provisioning Rules

1. **First authentication**: When a valid JWT is presented and `users.user_account.keycloak_id` does not match the JWT `sub` claim, the service automatically inserts a new `user_account` record.
2. **Returning authentication**: When `keycloak_id` matches, the existing record is reused. The `last_login_at` timestamp is updated.
3. **Role synchronization**: Roles are read from `realm_access.roles` in the JWT on every request. No application-level role storage or caching.
4. **Account deletion**: When a user is deleted in Keycloak, the `user_account` record enters `orphaned` status. Cleanup is admin-triggered only (no automatic cleanup).

## Enforcement Layers

| Layer | Mechanism | Enforces |
|-------|-----------|----------|
| 1. Keycloak | JWT issuance, realm roles | Authentication + role assignment |
| 2. Auth middleware | JWT validation, `sub` extraction | Token validity |
| 3. Identity mapping | DB check + auto-provision | 1:1 mapping |
| 4. DB constraint | UNIQUE on `keycloak_id` | No duplicate mappings |

## Provisioning SQL

```sql
INSERT INTO users.user_account (id, keycloak_id, email, display_name)
VALUES (:id, :keycloak_id, :email, :display_name)
ON CONFLICT (keycloak_id) DO UPDATE
SET last_login_at = NOW(),
    email = COALESCE(EXCLUDED.email, user_account.email),
    display_name = COALESCE(EXCLUDED.display_name, user_account.display_name)
RETURNING id;
```

## Critical Rules

- The backend MUST NEVER override roles independently of Keycloak.
- No user-facing API endpoint exists to create or delete `user_account` records directly.
- `user_account` deletion is an admin-only operation (admin CLI or admin API with elevated privileges).
