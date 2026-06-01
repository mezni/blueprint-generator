# EPIC 4 Specification — Identity, Authentication & Access Control Platform

## Epic Metadata

| Field | Value |
|-------|-------|
| **Epic ID** | EPIC-4 |
| **Epic Name** | Identity, Authentication & Authorization Platform |
| **Priority** | Critical |
| **Status** | Planned |
| **Depends On** | EPIC 1 — Monorepo & Workspace Foundation, EPIC 2 — Runtime Infrastructure & Gateway Platform, EPIC 3 — CI/CD Pipeline & Delivery Automation |
| **Blocks** | Driver application authentication, Admin platform access control, Partner portal access, Protected API access, Role-governed platform operations |

---

## 1. Objective

Establish the complete identity and access management foundation for the EV platform using centralized authentication and authorization.

This epic defines:

- user identity lifecycle
- authentication flows
- authorization enforcement
- token governance
- role-based access control
- session security
- service-to-service trust boundaries

This is the security foundation for all protected platform functionality.

---

## 2. Business Outcome

After completion, the platform must support:

- secure user authentication
- role-based platform access
- protected API consumption
- frontend authentication flows
- mobile authentication
- centralized identity governance
- secure service authorization validation

---

## 3. Architectural Scope

This epic covers:

### 3.1 Identity Provider Integration

Centralized identity authority.

### 3.2 Authentication Flows

User login and token issuance.

### 3.3 Authorization Platform

Role and permission enforcement.

### 3.4 Session Lifecycle Management

Token lifecycle and renewal.

### 3.5 API Security Enforcement

Protected access to `/api/v1/*`.

### 3.6 Service Trust Model

Backend token validation.

---

## 4. Core Architectural Constraints

### 4.1 Identity Provider Mandate

All authentication must be managed through **Keycloak**. No custom auth implementation allowed.

### 4.2 Authentication Authority Rule

Keycloak is the single source of truth for:

- identity
- credentials
- roles
- token issuance
- session lifecycle

### 4.3 API Protection Rule

Protected platform APIs must only accept authenticated requests under `/api/v1/*`, except explicitly public endpoints.

### 4.4 Authorization Model

Authorization must use **RBAC**. No ACL-per-resource implementation in phase 1.

### 4.5 Stateless API Rule

Backend APIs must remain stateless. Session state is token-based only.

---

## 5. Identity Architecture

```
User
   |
Frontend / Mobile
   |
Authentication Redirect
   |
Keycloak
   |
JWT Access Token
   |
Traefik Gateway
   |
Protected /api/v1 Services
```

---

## 6. Realm Specification

| Property | Value |
|----------|-------|
| **Realm Name** | `ev-platform` |
| **Realm Responsibilities** | User identities, Role assignments, Client registrations, Token issuance, Session policies |

---

## 7. Role Model

The platform must define three primary roles.

### 7.1 Registered Driver

**Purpose**: Consumer-facing application access

**Permissions**:
- view charging stations
- manage favorites
- access personal account
- submit events

### 7.2 Partner

**Purpose**: Partner portal access

**Permissions**:
- manage assigned infrastructure
- monitor operational data
- view partner analytics

### 7.3 Admin

**Purpose**: Administrative platform control

**Permissions**:
- full platform administration
- infrastructure management
- system oversight
- operational controls

---

## 8. Client Registration Specification

Required clients:

| Client | Purpose | Flow |
|--------|---------|------|
| **Driver Web Client** | Driver SPA authentication | Authorization Code + PKCE |
| **Driver Mobile Client** | Mobile app authentication | Authorization Code + PKCE |
| **Admin Dashboard Client** | Admin dashboard access | Authorization Code + PKCE |
| **Partner Dashboard Client** | Partner portal access | Authorization Code + PKCE |
| **Backend Service Client** | Machine-to-machine authentication | Client Credentials |

---

## 9. Authentication Flows

Three distinct authentication flows defined below.

---

## 10. Flow 1 — User Interactive Login

Applies to: driver web, admin dashboard, partner dashboard

```
User
 → Redirect to Keycloak
 → Credential Validation
 → Authorization Code
 → Token Exchange
 → Access Token + Refresh Token
 → Authenticated Session
```

---

## 11. Flow 2 — Mobile Login

Applies to: driver mobile

**Requirements**:
- PKCE mandatory
- secure token storage
- refresh support

---

## 12. Flow 3 — Service Authentication

Applies to: backend service communication

**Flow**: Client Credentials Grant

---

## 13. Token Specification

### 13.1 Access Token

| Property | Value |
|----------|-------|
| **Format** | JWT |
| **Required claims** | `sub`, `exp`, `iss`, `aud`, `realm_access.roles` |

### 13.2 Refresh Token

Required for interactive clients.

### 13.3 Token Expiration

| Token | Lifetime |
|-------|----------|
| **Access Token** | 15 minutes |
| **Refresh Token** | 30 days |

---

## 14. API Authorization Enforcement

All protected endpoints under `/api/v1/*` must enforce:

- token presence
- signature validation
- issuer validation
- expiration validation
- role validation

---

## 15. Public Endpoint Rules

Allowed unauthenticated endpoints:

| Category | Paths |
|----------|-------|
| **Health** | `/health`, `/ready` |
| **Authentication Gateway** | `/auth/*` |
| **Explicit Public APIs** | Must be explicitly documented |

No implicit public routes.

---

## 16. Service Authorization Middleware

Every backend service must implement shared auth middleware.

**Responsibilities**:
- bearer extraction
- JWT validation
- claim parsing
- role extraction
- request context injection

---

## 17. Shared Authentication Library

EPIC 4 must produce: `packages/shared-auth`

**Responsibilities**:
- token validation helpers
- role extraction
- middleware primitives
- auth error handling

---

## 18. Authorization Matrix

| Route Namespace | Required Role |
|-----------------|---------------|
| `/api/v1/driver/*` | `registered_driver` |
| `/api/v1/partner/*` | `partner` |
| `/api/v1/admin/*` | `admin` |

---

## 19. Frontend Authentication Requirements

### 19.1 Driver Web

Must support: login, logout, token refresh, silent renewal.

### 19.2 Admin Dashboard

Must enforce: immediate auth gate before app access.

### 19.3 Partner Dashboard

Must enforce: role validation on entry.

---

## 20. Logout Specification

Logout must:

- revoke active session
- invalidate refresh token
- clear client state
- redirect appropriately

---

## 21. Session Security Requirements

| Required | Forbidden |
|----------|-----------|
| HTTPS token transport | `localStorage` token persistence for sensitive clients |
| Secure refresh storage | Custom password storage |
| CSRF-safe auth flow | Plaintext token transport |
| PKCE for public clients | |

---

## 22. Identity Provisioning

| Method | Description |
|--------|-------------|
| **Self Registration** | Driver accounts |
| **Admin Provisioning** | Admin-created partner accounts |
| **Platform Provisioning** | Administrative role assignment |

---

## 23. Audit Requirements

Authentication events must emit audit events for:

- login success
- login failure
- logout
- token refresh
- role assignment changes

Published to: `events.exchange`

---

## 24. Runtime Integration

EPIC 4 must integrate with EPIC 2 runtime through:

- Traefik route: `/auth/*`
- Service validation against Keycloak issuer

---

## 25. CI/CD Requirements

EPIC 4 must extend EPIC 3 with authentication validation tests for:

- token issuance
- protected endpoint rejection
- role enforcement
- refresh flow

---

## 26. Deliverables

EPIC 4 must produce:

| Category | Artifact |
|----------|----------|
| **Identity Configuration** | Realm export, Client definitions, Role definitions |
| **Shared Code** | Shared auth library, Middleware package |
| **Frontend Integration** | Auth adapters for all clients |
| **Test Assets** | Authentication integration tests |

---

## 27. Acceptance Criteria

| Category | Criteria |
|----------|----------|
| **Identity** | Keycloak realm operational, Required roles exist, Required clients registered |
| **Authentication** | Interactive login works, Mobile auth works, Refresh flow operational |
| **Authorization** | Role restrictions enforced, Invalid tokens rejected, Expired tokens rejected |
| **API Security** | Protected `/api/v1/*` secured, Public routes remain accessible |
| **Frontend** | All clients authenticate correctly, Logout flow works |
| **Audit** | Auth events emitted to RabbitMQ |

---

## 28. Definition of Done

EPIC 4 is complete when: a user can authenticate through Keycloak, receive valid tokens, access only authorized `/api/v1` routes based on role, refresh sessions securely, and all services consistently enforce centralized authorization.
