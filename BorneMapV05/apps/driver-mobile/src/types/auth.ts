/**
 * Authentication types and interfaces
 * Defines all authentication-related data structures
 */

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface UserData {
  id: string;
  name: string;
  email: string;
  avatar?: string;
  createdAt: string;
}

export interface AuthResponse {
  token: string;
  user: UserData;
  expiresAt?: string;
}

export interface AuthContextType {
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (credentials: LoginCredentials) => Promise<boolean>;
  logout: () => Promise<void>;
  user: UserData | null;
}

export interface TokenData {
  access_token: string;
  token_type: string;
  expires_in: number;
  refresh_token?: string;
}
