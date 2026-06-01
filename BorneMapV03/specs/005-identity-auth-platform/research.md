# Research: Identity, Authentication & Authorization Platform

**Phase**: 0 — Research & Resolution
**Date**: 2026-05-31

## Methodology

All decisions are drawn from the feature specification (spec.md), the EPIC 4 document (docs/epic04.md), the project constitution, and pre-existing EPIC 2 infrastructure decisions. No external research needed — all architectural constraints are resolved by the existing Keycloak integration in EPIC 2.

## Design Decisions

### Decision 1: Identity Provider

- **Decision**: Use Keycloak 25.0 as the centralized identity provider
- **Rationale**: Keycloak is already listed in the constitution (§System Architecture) as part of the platform runtime from EPIC 2. The realm export file already exists at `infra/keycloak/realm-export.json`. Reusing Keycloak avoids introducing a new identity system.
- **Alternatives considered**: Custom auth service (rejected — violates constitution), Auth0/Firebase (rejected — adds external dependency, no clear benefit over existing Keycloak)

### Decision 2: Dual Token Validation

- **Decision**: Validate tokens at both the Traefik gateway and each backend service
- **Rationale**: Gateway-level validation provides fast rejection of malformed/expired tokens before they reach services (latency optimization). Service-level re-validation provides defense in depth — auth enforcement survives gateway misconfiguration.
- **Alternatives considered**: Gateway-only validation (rejected — single point of failure), Service-only validation (rejected — allows malformed tokens to reach services)

### Decision 3: Shared Auth Package Approach

- **Decision**: Two shared auth packages — `crates/common-auth` (Rust middleware) and `packages/auth-client` (TypeScript adapter)
- **Rationale**: Backend services are Rust-based and need JWT validation middleware. Frontend apps are TypeScript-based and need Keycloak JS adapter wrappers. A single package cannot serve both platforms. Supporting both ecosystems independently avoids cross-language coupling.
- **Alternatives considered**: Single auth gateway service (rejected — adds network hop, violates stateless API constraint), FFI-based shared library (rejected — over-engineered for this use case)

### Decision 4: Client Registration Flows

- **Decision**: Authorization Code + PKCE for all public clients (web + mobile), Client Credentials for backend service client
- **Rationale**: PKCE is mandatory for public OAuth2 clients per RFC 7636 and provides CSRF protection without requiring a client secret. Client Credentials is the standard machine-to-machine flow and does not require user interaction.
- **Alternatives considered**: Implicit flow (rejected — deprecated in OAuth2.1), Resource Owner Password flow (rejected — anti-pattern, leaks credentials to client)

### Decision 5: Account Lifecycle States

- **Decision**: Three states — Active, Suspended, Deleted; email verification as a property flag
- **Rationale**: Three states cover the essential identity lifecycle. Using a flag (is_verified) rather than a full state avoids state machine complexity while meeting the self-registration verification requirement.
- **Alternatives considered**: Four states including PendingVerification (rejected — unnecessary state complexity), Two states Active/Deleted (rejected — no suspension mechanism)

### Decision 6: GDPR Compliance Scope

- **Decision**: Limit GDPR compliance to authentication-related personal data (identity, credentials, auth logs); provide account deletion and data export
- **Rationale**: Authentication data is inherently personal (email, name, auth history). The right to delete (FR-014) and right to data portability (FR-015) are GDPR requirements that directly affect the identity system. Broader GDPR compliance across the platform is a cross-cutting concern addressed separately.
- **Alternatives considered**: Full platform GDPR compliance (rejected — out of scope, requires all services), No GDPR compliance (rejected — legal risk)
