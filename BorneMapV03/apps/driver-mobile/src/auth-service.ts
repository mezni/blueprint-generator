import { Platform } from 'react-native';
import * as SecureStore from 'expo-secure-store';
import {
  AuthConfig,
  AuthenticatedUser,
  TokenResponse,
} from 'auth-client';

function base64URLEncode(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let binary = '';
  bytes.forEach((b) => { binary += String.fromCharCode(b); });
  return btoa(binary)
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}

async function sha256(plain: string): Promise<ArrayBuffer> {
  const encoder = new TextEncoder();
  const data = encoder.encode(plain);
  return crypto.subtle.digest('SHA-256', data);
}

function generateCodeVerifier(): string {
  const array = new Uint8Array(64);
  crypto.getRandomValues(array);
  return base64URLEncode(array.buffer);
}

export class MobileAuthService {
  private config: AuthConfig;
  private tokenResponse: TokenResponse | null = null;
  private user: AuthenticatedUser | null = null;
  private baseUrl: string;

  constructor(config: AuthConfig) {
    this.config = config;
    this.baseUrl = '/auth/realms/ev-platform';
  }

  async login(): Promise<void> {
    const codeVerifier = generateCodeVerifier();
    const codeChallenge = base64URLEncode(await sha256(codeVerifier));

    const redirectUri = 'bornemap://callback';

    const authUrl =
      `${this.baseUrl}/protocol/openid-connect/auth?` +
      `client_id=${this.config.clientId}` +
      `&redirect_uri=${encodeURIComponent(redirectUri)}` +
      `&response_type=code` +
      `&scope=openid` +
      `&code_challenge=${codeChallenge}` +
      `&code_challenge_method=S256`;

    // In a real mobile app, open authUrl in system browser
    // For now, use direct token endpoint with Resource Owner Password Grant (dev only)
    await this.tokenExchangeWithPassword(codeVerifier);
  }

  private async tokenExchangeWithPassword(codeVerifier: string): Promise<void> {
    const response = await fetch(`${this.baseUrl}/protocol/openid-connect/token`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams({
        client_id: this.config.clientId,
        grant_type: 'authorization_code',
        code: 'mock-code',
        redirect_uri: 'bornemap://callback',
        code_verifier: codeVerifier,
      }).toString(),
    });

    if (!response.ok) {
      throw new Error('Login failed');
    }

    const data = await response.json();
    this.tokenResponse = {
      accessToken: data.access_token,
      refreshToken: data.refresh_token,
      expiresIn: data.expires_in,
    };

    await SecureStore.setItemAsync('access_token', this.tokenResponse.accessToken);
    await SecureStore.setItemAsync('refresh_token', this.tokenResponse.refreshToken);

    this.user = this.parseUser(data);
  }

  async logout(): Promise<void> {
    try {
      if (this.tokenResponse?.refreshToken) {
        await fetch(`${this.baseUrl}/protocol/openid-connect/logout`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
          body: new URLSearchParams({
            client_id: this.config.clientId,
            refresh_token: this.tokenResponse.refreshToken,
          }).toString(),
        });
      }
    } catch {
      // Ignore logout errors
    }

    await SecureStore.deleteItemAsync('access_token');
    await SecureStore.deleteItemAsync('refresh_token');
    this.tokenResponse = null;
    this.user = null;
  }

  async getToken(): Promise<string | null> {
    if (this.tokenResponse?.accessToken) {
      const expiresAt = Date.now() - (this.tokenResponse.expiresIn * 1000);
      if (Date.now() < expiresAt) {
        return this.tokenResponse.accessToken;
      }
      await this.refreshToken();
      return this.tokenResponse?.accessToken || null;
    }

    const stored = await SecureStore.getItemAsync('access_token');
    return stored;
  }

  async refreshToken(): Promise<boolean> {
    const refreshToken = this.tokenResponse?.refreshToken
      || await SecureStore.getItemAsync('refresh_token');

    if (!refreshToken) return false;

    try {
      const response = await fetch(`${this.baseUrl}/protocol/openid-connect/token`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body: new URLSearchParams({
          client_id: this.config.clientId,
          grant_type: 'refresh_token',
          refresh_token: refreshToken,
        }).toString(),
      });

      if (!response.ok) return false;

      const data = await response.json();
      this.tokenResponse = {
        accessToken: data.access_token,
        refreshToken: data.refresh_token,
        expiresIn: data.expires_in,
      };

      await SecureStore.setItemAsync('access_token', this.tokenResponse.accessToken);
      await SecureStore.setItemAsync('refresh_token', this.tokenResponse.refreshToken);
      return true;
    } catch {
      return false;
    }
  }

  getUser(): AuthenticatedUser | null {
    return this.user;
  }

  isAuthenticated(): boolean {
    return this.tokenResponse !== null;
  }

  private parseUser(data: any): AuthenticatedUser {
    return {
      id: data.user_id || '',
      email: data.email || '',
      displayName: data.name || '',
      roles: data.realm_access?.roles || [],
    };
  }
}
