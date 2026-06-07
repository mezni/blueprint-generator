/**
 * Authentication Context
 * Provides authentication state and methods to the application
 * Separate from the useAuth hook to follow separation of concerns
 */

import React, { createContext, ReactNode, useState, useCallback } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { AuthContextType, UserData, LoginCredentials } from '@/types/auth';
import { AuthenticationError } from '@/lib/errors';
import { Logger } from '@/services/logger';

export const AuthContext = createContext<AuthContextType | null>(null);

const AUTH_TOKEN_KEY = 'auth_token';
const USER_DATA_KEY = 'user_data';

export interface AuthProviderProps {
  children: ReactNode;
}

/**
 * Authentication Provider
 * Manages authentication state and provides auth methods
 *
 * @example
 * <AuthProvider>
 *   <App />
 * </AuthProvider>
 */
export function AuthProvider({ children }: AuthProviderProps) {
  const [isAuthenticated, setIsAuthenticated] = React.useState(false);
  const [isLoading, setIsLoading] = React.useState(true);
  const [user, setUser] = React.useState<UserData | null>(null);

  // Initialize auth state on mount
  React.useEffect(() => {
    checkAuthStatus();
  }, []);

  /**
   * Check if user is already authenticated (from stored token)
   */
  const checkAuthStatus = useCallback(async () => {
    try {
      const token = await AsyncStorage.getItem(AUTH_TOKEN_KEY);
      const userData = await AsyncStorage.getItem(USER_DATA_KEY);

      if (token && userData) {
        try {
          setUser(JSON.parse(userData));
          setIsAuthenticated(true);
          Logger.info('User restored from storage');
        } catch (parseError) {
          Logger.error('Failed to parse stored user data', parseError);
          await clearAuth();
        }
      }
    } catch (error) {
      Logger.error('Auth check failed', error);
      setIsAuthenticated(false);
      setUser(null);
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Login with email and password
   * @param credentials - User login credentials
   * @returns true if login successful, false otherwise
   */
  const login = useCallback(async (credentials: LoginCredentials): Promise<boolean> => {
    try {
      // Validate input
      if (!credentials.email || !credentials.password) {
        throw new ValidationError('Email and password are required');
      }

      // TODO: Implement actual login via auth client
      // const response = await authClient.login(credentials);

      // Mock login for development
      const userData: UserData = {
        id: 'user-123',
        name: 'Driver User',
        email: credentials.email,
        createdAt: new Date().toISOString(),
      };

      const token = 'mock_token_' + Date.now();

      // Store credentials
      await AsyncStorage.setItem(AUTH_TOKEN_KEY, token);
      await AsyncStorage.setItem(USER_DATA_KEY, JSON.stringify(userData));

      setUser(userData);
      setIsAuthenticated(true);
      Logger.info('User logged in successfully', { userId: userData.id });

      return true;
    } catch (error) {
      Logger.error('Login failed', error);
      if (error instanceof ValidationError) {
        throw error;
      }
      throw new AuthenticationError('Failed to login', { cause: error });
    }
  }, []);

  /**
   * Logout user and clear stored credentials
   */
  const logout = useCallback(async (): Promise<void> => {
    try {
      await clearAuth();
      Logger.info('User logged out');
    } catch (error) {
      Logger.error('Logout failed', error);
      throw new AuthenticationError('Failed to logout', { cause: error });
    }
  }, []);

  /**
   * Clear all authentication data
   */
  const clearAuth = useCallback(async (): Promise<void> => {
    try {
      await AsyncStorage.multiRemove([AUTH_TOKEN_KEY, USER_DATA_KEY]);
      setIsAuthenticated(false);
      setUser(null);
    } catch (error) {
      Logger.error('Failed to clear auth data', error);
      throw error;
    }
  }, []);

  const value: AuthContextType = {
    isAuthenticated,
    isLoading,
    login,
    logout,
    user,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

// Import for validation error
import { ValidationError } from '@/lib/errors';
