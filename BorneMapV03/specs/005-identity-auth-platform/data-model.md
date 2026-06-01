# Data Model: Identity, Authentication & Authorization Platform

**Phase**: 1 — Design & Contracts
**Date**: 2026-05-31
**Source Spec**: [spec.md](../spec.md)

This document defines the data entities and relationships for the BorneMap identity and access control system.

## Entity: User Identity

A registered platform user with credentials, role assignment, and lifecycle status.

| Field | Type | Description | Constraints |
|-------|------|-------------|-------------|
| `id` | NanoID (`USR-*`) | Unique user identifier | Prefix `USR-`, per constitution identifier standard |
| `email` | String | User email address (also login identifier) | Unique, validated on registration |
| `password_hash` | String | Bcrypt/argon2 hash of user password | Never exposed in API responses |
| `display_name` | String | User-facing display name | Optional, max 100 chars |
| `role` | Enum | `registered_driver`, `partner`, `admin` | Exactly one role per user |
| `status` | Enum | `active`, `suspended`, `deleted` | Non-deleted status required for authentication |
| `is_verified` | Boolean | Email verification flag | Default false; required for active access |
| `partner_id` | NanoID (`PRT-*`) | Associated partner (if role=partner) | Required when role is `partner` |
| `failed_attempts` | Integer | Consecutive failed login counter | Reset on successful login; lock at threshold |
| `locked_until` | Timestamp | Account lockout expiration | Null when not locked |
| `created_at` | Timestamp | Account creation timestamp | Auto-set on registration |
| `updated_at` | Timestamp | Last update timestamp | Auto-updated |
| `deleted_at` | Timestamp | Soft-delete timestamp | Set when status transitions to `deleted` |

**State transitions**: `active` ↔ `suspended` (admin action), `active`/`suspended` → `deleted` (user or admin action, soft-delete with 30-day grace period).

## Entity: Client Registration

An OAuth2 client registered with the identity provider.

| Field | Type | Description | Constraints |
|-------|------|-------------|-------------|
| `client_id` | String | OAuth2 client identifier | Unique per realm |
| `client_name` | String | Human-readable client name | e.g., "Driver Web", "Backend Service" |
| `client_type` | Enum | `public` (web/mobile) or `confidential` (backend) | Determines secret requirement |
| `flow` | Enum | `authorization_code_pkce`, `client_credentials` | Per spec §8 |
| `redirect_uris` | String[] | Allowed post-login redirect destinations | Validated on every auth request |
| `public` | Boolean | Whether this is a public client | Public clients use PKCE; confidential use Client Credentials |

**Registered clients**: Driver Web, Driver Mobile, Admin Dashboard, Partner Dashboard (public, PKCE), Backend Service Client (confidential, Client Credentials).

## Entity: Role

A named permission set governing API access.

| Field | Type | Description |
|-------|------|-------------|
| `name` | Enum | `registered_driver`, `partner`, `admin` |
| `description` | String | Purpose of the role |

**Route mapping** (from spec §18):
| Role | Accessible Namespaces |
|------|----------------------|
| `registered_driver` | `/api/v1/driver/*` |
| `partner` | `/api/v1/partner/*` |
| `admin` | `/api/v1/admin/*` (plus all below) |

## Entity: Access Token

A short-lived JWT credential issued upon authentication.

| Claim | Value | Description |
|-------|-------|-------------|
| `sub` | User NanoID (`USR-*`) | Subject — the authenticated user |
| `iss` | `https://keycloak:8080/realms/ev-platform` | Token issuer |
| `aud` | Client ID (`backend-service`) | Intended audience |
| `exp` | Timestamp (`iat + 900s`) | Expiration (15 minutes) |
| `realm_access.roles` | String[] | User's granted roles |
| `iat` | Timestamp | Issued at time |

## Entity: Audit Event

A timestamped record of an authentication action.

| Field | Type | Description |
|-------|------|-------------|
| `event_type` | Enum | `login_success`, `login_failure`, `logout`, `token_refresh`, `role_change` |
| `user_id` | NanoID (`USR-*`) | Affected user (null for anonymous events) |
| `client_id` | String | OAuth2 client used |
| `ip_address` | String | Originating IP address |
| `outcome` | Enum | `success`, `failure` |
| `timestamp` | Timestamp | ISO 8601 UTC |
| `details` | JSON | Additional context (error reason, changed-from/changed-to role) |

**Routing**: Published to `events.exchange` (RabbitMQ) per EPIC 2 messaging infrastructure.

## Entity: Realm Configuration

The identity provider realm governing all identities, clients, and policies.

| Property | Value |
|----------|-------|
| `realm_name` | `ev-platform` |
| `access_token_lifespan` | 15 minutes (900 seconds) |
| `refresh_token_lifespan` | 30 days (2,592,000 seconds) |
| `login_theme` | BorneMap branded theme |
| `registration_allowed` | True (self-registration for drivers) |
| `reset_password_allowed` | True |
| `brute_force_protection` | Enabled (N failed attempts → temporary lockout) |

## Relationships

```
Realm ──has_many──> Client Registration
Realm ──has_many──> Role
Role ──assigned_to──> User Identity
User Identity ──issued──> Access Token
User Identity ──issued──> Refresh Token
User Identity ──generates──> Audit Event
Client Registration ──uses──> Authentication Flow (PKCE / Client Credentials)
```
