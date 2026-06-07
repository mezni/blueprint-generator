/**
 * Authentication Service
 * Handles authentication operations with proper error handling and type safety
 */

import AsyncStorage from '@react-native-async-storage/async-storage';
import { AuthResponse, LoginCredentials, UserData, TokenData } from '@/types/auth';
import { AuthenticationError, StorageError, ValidationError } from '@/lib/errors';
import { Logger } from '@/services/logger';

const AUTH_TOKEN_KEY = 'auth_token';
const USER_DATA_KEY = 'user_data';
const TOKEN_EXPIRY_KEY = 'token_expiry';

/**
 * Authentication service with proper error handling and validation
 */
export class AuthService {
  /**
   * Login with credentials
   * @param credentials - User login credentials
   * @returns Auth response with token and user data
   * @throws {ValidationError} If credentials are invalid
   * @throws {AuthenticationError} If login fails
   */
  static async login(credentials: LoginCredentials): Promise<AuthResponse> {
    try {
      // Validate input
      if (!credentials.email || !credentials.password) {
        throw new ValidationError('Email and password are required');
      }

      if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(credentials.email)) {
        throw new ValidationError('Invalid email format', 'email', credentials.email);
      }

      if (credentials.password.length < 6) {
        throw new ValidationError('Password must be at least 6 characters', 'password');
      }

      // TODO: Implement actual login via auth client
      // const response = await authClient.login(credentials);

      // Mock login for development
      const user: UserData = {
        id: 'user-123',
        name: 'Driver User',
        email: credentials.email,
        createdAt: new Date().toISOString(),
      };

      const token = `mock_token_${Date.now()}`;
      const expiresAt = new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(); // 24 hours

      // Store credentials
      await this.storeToken(token, expiresAt);
      await this.storeUser(user);

      Logger.info('User logged in successfully', { userId: user.id });

      return { token, user, expiresAt };
    } catch (error) {
      if (error instanceof ValidationError || error instanceof AuthenticationError) {
        throw error;
      }
      Logger.error('Login failed', error);
      throw new AuthenticationError('Failed to login', { cause: error });
    }
  }

  /**
   * Logout user and clear stored credentials
   * @throws {AuthenticationError} If logout fails
   */
  static async logout(): Promise<void> {
    try {
      await this.clearAuth();
      Logger.info('User logged out successfully');
    } catch (error) {
      Logger.error('Logout failed', error);
      throw new AuthenticationError('Failed to logout', { cause: error });
    }
  }

  /**
   * Get stored auth token
   * @returns Auth token or null if not found
   */
  static async getToken(): Promise<string | null> {
    try {
      const token = await AsyncStorage.getItem(AUTH_TOKEN_KEY);

      if (!token) {
        return null;
      }

      // Check if token is expired
      const expiry = await AsyncStorage.getItem(TOKEN_EXPIRY_KEY);
      if (expiry && new Date(expiry) < new Date()) {
        await this.clearAuth();
        Logger.warn('Token expired, clearing auth');
        return null;
      }

      return token;
    } catch (error) {
      Logger.error('Failed to get auth token', error);
      throw new StorageError('Failed to retrieve auth token', AUTH_TOKEN_KEY, { cause: error });
    }
  }

  /**
   * Get stored user data
   * @returns User data or null if not found
   */
  static async getUser(): Promise<UserData | null> {
    try {
      const userData = await AsyncStorage.getItem(USER_DATA_KEY);
      if (!userData) {
        return null;
      }
      return JSON.parse(userData) as UserData;
    } catch (error) {
      Logger.error('Failed to get user data', error);
      return null;
    }
  }

  /**
   * Store auth token with expiry
   */
  private static async storeToken(token: string, expiresAt: string): Promise<void> {
    try {
      await AsyncStorage.multiSet([
        [AUTH_TOKEN_KEY, token],
        [TOKEN_EXPIRY_KEY, expiresAt],
      ]);
    } catch (error) {
      Logger.error('Failed to store token', error);
      throw new StorageError('Failed to store auth token', AUTH_TOKEN_KEY, { cause: error });
    }
  }

  /**
   * Store user data
   */
  private static async storeUser(user: UserData): Promise<void> {
    try {
      await AsyncStorage.setItem(USER_DATA_KEY, JSON.stringify(user));
    } catch (error) {
      Logger.error('Failed to store user data', error);
      throw new StorageError('Failed to store user data', USER_DATA_KEY, { cause: error });
    }
  }

  /**
   * Clear all authentication data
   */
  private static async clearAuth(): Promise<void> {
    try {
      await AsyncStorage.multiRemove([AUTH_TOKEN_KEY, USER_DATA_KEY, TOKEN_EXPIRY_KEY]);
    } catch (error) {
      Logger.error('Failed to clear auth data', error);
      throw new StorageError('Failed to clear authentication data', undefined, { cause: error });
    }
  }
}

export default AuthService;
