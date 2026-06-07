# Data Model: Identity & RBAC

This document defines the data entities required for the identity and authorization system. Existing entities from `crates/common-types` (Role, EntityPrefix) are referenced but not duplicated.

## Entity: CurrentUser (Runtime Request Context)

Represents the authenticated user making a request. Not a persisted entity — populated by auth middleware and carried in request extensions.

| Field | Type | Source | Description |
|-------|------|--------|-------------|
| `user_id` | `String` (ULID: `USR-*`) | `user_account.id` | Platform user ID |
| `keycloak_user_id` | `String` | JWT `sub` claim | Keycloak user UUID |
| `email` | `String` | JWT `email` claim or `user_account.email` | User email |
| `role` | `Role` | JWT `realm_access.roles` | `registered_driver`, `partner`, or `admin` |
| `partner_id` | `Option<String>` | `partner_membership.partner_id` | Partner org ID if user is a partner role |

**Validation Rules**:
- If `role == Partner`, `partner_id` MUST be `Some`
- `user_id` MUST exist in `user_account` table
- `keycloak_user_id` MUST match a unique `user_account.keycloak_user_id`

---

## Entity: UserAccount (Persisted — schema `users`)

Maps a Keycloak identity to a platform user. Created on first login (first-login provisioning).

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | `TEXT PK` | `USR-<ULID>` | Platform user ID |
| `keycloak_user_id` | `TEXT` | `UNIQUE NOT NULL` | JWT `sub` — the only identity bridge |
| `email` | `TEXT` | | User email from Keycloak |
| `status` | `TEXT` | `active` \| `disabled` | Account status |
| `last_login_at` | `TIMESTAMPTZ` | | Last successful login timestamp |
| `created_at` | `TIMESTAMPTZ` | `NOT NULL` | Audit |
| `updated_at` | `TIMESTAMPTZ` | `NOT NULL` | Audit |

**Indexes**: `UNIQUE(keycloak_user_id)`

**Lifecycle**:
- `not_provisioned` (Keycloak user exists, no user_account) → `active` (first login)
- `active` ↔ `disabled` (admin action in Keycloak — synced on next login)

---

## Entity: PartnerMembership (Persisted — schema `users`)

Associates a user with a partner organization and defines their role within it.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `user_id` | `TEXT` | `UNIQUE FK → user_account.id` | Platform user |
| `partner_id` | `TEXT` | `FK → inventory.partner.id` | Partner organization |
| `role` | `TEXT` | `owner` \| `manager` \| `operator` \| `viewer` | Role within the partner |

**Constraints**: 
- `UNIQUE(user_id)` — one membership per user
- `partner_id` MUST reference an existing partner in `inventory.partner`
- `role` values correspond to `PartnerRole` enum in `common-types`

**Lifecycle**:
- Created on first login if Keycloak user has pre-configured `partner_id` attribute
- Updated via admin API when partner membership changes
- Never deleted — user can be reassigned to a different partner or have role changed

---

## Entity: UserProfile (Persisted — schema `users`, optional)

Additional user metadata. Safe to delete without affecting auth.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `user_id` | `TEXT` | `PK FK → user_account.id` | Platform user |
| `display_name` | `TEXT` | | Public display name |
| `avatar_url` | `TEXT` | | Profile picture URL |
| `preferred_language` | `TEXT` | | Language preference |
| `preferences` | `JSONB` | | Client-specific preferences |

---

## Entity: JWT Claims (Token Contract)

The JWT payload structure that the platform validates. These are claims issued by Keycloak.

| Field | Type | Source | Description |
|-------|------|--------|-------------|
| `sub` | `String` | Keycloak | Unique Keycloak user ID |
| `iss` | `String` | Keycloak | Issuer URL (must match `AUTH_ISSUER` env) |
| `aud` | `String` \| `Vec<String>` | Keycloak | Audience (must include `bornemap-api`) |
| `exp` | `i64` | Keycloak | Expiration timestamp (UNIX) |
| `iat` | `i64` | Keycloak | Issued at timestamp (UNIX) |
| `email` | `String` | Keycloak | User email |
| `realm_access.roles` | `Vec<String>` | Keycloak | Realm roles (contains platform role) |

**Validation Rules**:
- `iss` MUST equal configured `AUTH_ISSUER`
- `aud` MUST contain configured `AUTH_AUDIENCE`
- `exp` MUST be in the future
- `realm_access.roles` MUST contain exactly one of: `registered_driver`, `partner`, `admin`
- Token MUST be signed by a key in the configured JWKS

---

## Entity: JWK (JSON Web Key)

A public key from Keycloak's JWKS endpoint used to verify token signatures.

| Field | Type | Description |
|-------|------|-------------|
| `kid` | `String` | Key ID (matches JWT header `kid`) |
| `kty` | `String` | Key type (e.g., `RSA`) |
| `alg` | `String` | Algorithm (e.g., `RS256`) |
| `n` | `String` | RSA modulus (base64url-encoded) |
| `e` | `String` | RSA exponent (base64url-encoded) |
| `use` | `String` | `sig` for signature keys |

**Lifecycle**: Cached in memory with TTL; evicted and refreshed on expiry or when a signature verification fails with a cached key.
