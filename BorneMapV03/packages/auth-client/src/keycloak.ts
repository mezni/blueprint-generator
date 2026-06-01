import Keycloak from 'keycloak-js';
import type { AuthConfig, AuthenticatedUser, Role } from './types';
import { TokenStorage } from './token-storage';
import { startSessionTracking, stopSessionTracking, isSessionExpired } from './index';

type AuditEventType = 'login_success' | 'login_failure' | 'logout' | 'token_refresh' | 'role_change';
type AuditOutcome = 'success' | 'failure';

function emitAuditEvent(
  eventType: AuditEventType,
  userId?: string,
  clientId?: string,
  outcome: AuditOutcome = 'success',
  details?: Record<string, unknown>,
): void {
  const event = {
    event_type: eventType,
    user_id: userId || null,
    client_id: clientId || null,
    ip_address: null,
    outcome,
    timestamp: new Date().toISOString(),
    details: details || null,
  };
  console.debug('AUDIT:', JSON.stringify(event));
}

export class KeycloakAdapter {
  private keycloak: Keycloak;
  private storage: TokenStorage;
  private config: AuthConfig;

  constructor(config: AuthConfig, storage: TokenStorage) {
    this.config = config;
    this.storage = storage;
    this.keycloak = new Keycloak({
      url: '/auth',
      realm: config.realm,
      clientId: config.clientId,
    });
  }

  async login(redirectUri?: string): Promise<void> {
    if (isSessionExpired()) {
      await this.handleSessionExpired();
      return;
    }

    try {
      await this.keycloak.init({
        onLoad: 'login-required',
        redirectUri: redirectUri || this.config.redirectUri,
        silentCheckSsoRedirectUri: this.config.silentCheckSsoRedirectUri,
        pkceMethod: 'S256',
      });
      this.syncTokens();
      startSessionTracking();
      emitAuditEvent('login_success', this.keycloak.subject, this.config.clientId);
    } catch (err) {
      emitAuditEvent('login_failure', undefined, this.config.clientId, 'failure', {
        error: String(err),
      });
      throw err;
    }
  }

  async logout(): Promise<void> {
    const userId = this.keycloak.subject;
    try {
      await this.keycloak.logout({
        redirectUri: this.config.redirectUri,
      });
      emitAuditEvent('logout', userId, this.config.clientId);
    } catch {
      emitAuditEvent('logout', userId, this.config.clientId, 'failure');
    }
    this.storage.clear();
    stopSessionTracking();
  }

  async getToken(): Promise<string | null> {
    if (isSessionExpired()) {
      await this.handleSessionExpired();
      return null;
    }

    if (this.keycloak.authenticated) {
      try {
        const refreshed = await this.keycloak.updateToken(30);
        if (refreshed) {
          this.syncTokens();
          emitAuditEvent('token_refresh', this.keycloak.subject, this.config.clientId);
        }
      } catch {
        try {
          await this.keycloak.init({
            onLoad: 'check-sso',
            silentCheckSsoRedirectUri: this.config.silentCheckSsoRedirectUri,
            pkceMethod: 'S256',
          });
          if (!this.keycloak.authenticated) {
            return null;
          }
          this.syncTokens();
        } catch {
          return null;
        }
      }
      return this.keycloak.token || null;
    }
    return this.storage.getAccessToken();
  }

  getUser(): AuthenticatedUser | null {
    if (this.keycloak.authenticated && this.keycloak.tokenParsed) {
      return {
        id: this.keycloak.subject || '',
        email: this.keycloak.tokenParsed.email || '',
        displayName: this.keycloak.tokenParsed.name || '',
        roles: (this.keycloak.tokenParsed.realm_access?.roles as Role[]) || [],
      };
    }
    return this.storage.getUser();
  }

  isAuthenticated(): boolean {
    return this.keycloak.authenticated || false;
  }

  hasRole(role: Role): boolean {
    return this.keycloak.hasRealmRole(role);
  }

  async refreshToken(): Promise<boolean> {
    try {
      await this.keycloak.updateToken(0);
      this.syncTokens();
      emitAuditEvent('token_refresh', this.keycloak.subject, this.config.clientId);
      return true;
    } catch {
      return false;
    }
  }

  private async handleSessionExpired(): Promise<void> {
    const userId = this.keycloak.subject;
    try {
      await this.keycloak.logout();
      emitAuditEvent('logout', userId, this.config.clientId, 'success', {
        reason: 'session_expired',
      });
    } catch {
      // Ignore
    }
    this.storage.clear();
    stopSessionTracking();
  }

  private syncTokens(): void {
    if (this.keycloak.token && this.keycloak.refreshToken) {
      this.storage.setTokens({
        accessToken: this.keycloak.token,
        refreshToken: this.keycloak.refreshToken,
        expiresIn: this.keycloak.tokenParsed?.exp
          ? this.keycloak.tokenParsed.exp - Math.floor(Date.now() / 1000)
          : 900,
      });
    }
  }
}

