export type Role = 'registered_driver' | 'partner' | 'admin';

export interface AuthenticatedUser {
  id: string;
  email: string;
  displayName: string;
  roles: Role[];
}

export interface TokenResponse {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
}

export interface AuthConfig {
  realm: 'ev-platform';
  clientId: 'driver-web' | 'admin-dashboard' | 'partner-dashboard' | 'driver-mobile';
  redirectUri: string;
  silentCheckSsoRedirectUri?: string;
}
