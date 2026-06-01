# RBAC Contract

**Crate**: `crates/contracts`
**File**: `src/rbac.rs`
**Status**: Scaffold — roles defined

## Role Enum

```rust
enum Role {
    RegisteredDriver,
    Partner,
    Admin,
}
```

## ID Prefixes

| Entity | Prefix | NanoID Length | Example |
|--------|--------|---------------|---------|
| User | `USR-` | 21 chars | `USR-a1b2c3d4e5f6g7h8i9j0` |
| Partner | `PRT-` | 21 chars | `PRT-a1b2c3d4e5f6g7h8i9j0` |
| Station | `STN-` | 21 chars | `STN-a1b2c3d4e5f6g7h8i9j0` |
| Charger | `CHG-` | 21 chars | `CHG-a1b2c3d4e5f6g7h8i9j0` |
| Review | `REV-` | 21 chars | `REV-a1b2c3d4e5f6g7h8i9j0` |

## Enforcement Layers

1. **Keycloak**: Authentication + role claims in JWT
2. **Service layer**: Authorization checks per endpoint
3. **DB constraints**: Row-level `partner_id` filtering (repository-level enforcement)
