# Frontend Auth Adapter Contract

**Enforced by**: `packages/auth-client` library
**Target runtime**: All frontend apps (driver-web, driver-mobile, admin-dashboard, partner-dashboard)

## Responsibilities

- Wrap Keycloak JS adapter for web clients
- Implement PKCE flow for mobile clients
- Manage token lifecycle (login, refresh, logout)
- Expose authenticated user context to consuming apps
- Handle silent token renewal for web clients
- Secure token storage (memory/httpOnly cookies for web; secure storage for mobile)

## API Surface

```typescript
interface AuthClient {
  login(redirectUri?: string): Promise<void>;
  logout(): Promise<void>;
  getToken(): Promise<string | null>;
  getUser(): Promise<AuthenticatedUser | null>;
  isAuthenticated(): boolean;
  hasRole(role: 'registered_driver' | 'partner' | 'admin'): boolean;
  onTokenExpired(callback: () => void): void;
}

interface AuthenticatedUser {
  id: string;           // NanoID USR-*
  email: string;
  displayName: string;
  roles: string[];
}
```

## Flow: Web Login

```
User clicks "Login"
  → AuthClient.login()
  → Redirect to Keycloak /auth endpoint
  → User enters credentials on Keycloak login page
  → Authorization code returned to app callback URL
  → Token exchange (code → access + refresh tokens)
  → AuthClient stores tokens in memory
  → UserContext available to app
```

## Flow: Token Refresh

```
Access token expires
  → AuthClient detects 401 or checks expiry
  → Silent token refresh via Keycloak iframe (web) or refresh token grant (mobile)
  → New access token issued
  → Original request retried with new token
  → User unaware of refresh
```

## Configuration

Each frontend app provides on initialization:

```typescript
interface AuthConfig {
  realm: 'ev-platform';
  clientId: 'driver-web' | 'admin-dashboard' | 'partner-dashboard';
  redirectUri: string;
  silentCheckSsoRedirectUri?: string;  // For web silent refresh
}
```
