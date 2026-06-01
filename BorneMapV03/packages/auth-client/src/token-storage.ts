import type { AuthenticatedUser, TokenResponse } from './types';

interface TokenStore {
  accessToken: string | null;
  refreshToken: string | null;
  user: AuthenticatedUser | null;
  expiresAt: number | null;
}

export class TokenStorage {
  private store: TokenStore = {
    accessToken: null,
    refreshToken: null,
    user: null,
    expiresAt: null,
  };

  setTokens(response: TokenResponse): void {
    this.store.accessToken = response.accessToken;
    this.store.refreshToken = response.refreshToken;
    this.store.expiresAt = Date.now() + response.expiresIn * 1000;
  }

  setUser(user: AuthenticatedUser): void {
    this.store.user = user;
  }

  getAccessToken(): string | null {
    return this.store.accessToken;
  }

  getRefreshToken(): string | null {
    return this.store.refreshToken;
  }

  getUser(): AuthenticatedUser | null {
    return this.store.user;
  }

  isAuthenticated(): boolean {
    return this.store.accessToken !== null && !this.isExpired();
  }

  isExpired(): boolean {
    if (!this.store.expiresAt) return true;
    return Date.now() >= this.store.expiresAt;
  }

  clear(): void {
    this.store = {
      accessToken: null,
      refreshToken: null,
      user: null,
      expiresAt: null,
    };
  }
}
