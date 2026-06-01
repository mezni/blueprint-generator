export { KeycloakAdapter } from './keycloak';
export { TokenStorage } from './token-storage';
export type { AuthConfig, AuthenticatedUser, Role, TokenResponse } from './types';

const SESSION_TIMEOUT_MS = 30 * 24 * 60 * 60 * 1000; // 30 days

let sessionStart: number | null = null;

export function startSessionTracking(): void {
  sessionStart = Date.now();
}

export function stopSessionTracking(): void {
  sessionStart = null;
}

export function isSessionExpired(): boolean {
  if (!sessionStart) return true;
  return Date.now() - sessionStart >= SESSION_TIMEOUT_MS;
}

export function getRemainingSessionTime(): number {
  if (!sessionStart) return 0;
  const elapsed = Date.now() - sessionStart;
  return Math.max(0, SESSION_TIMEOUT_MS - elapsed);
}
