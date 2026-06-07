/**
 * useAuth Hook
 * Provides access to authentication context
 * Must be used within an AuthProvider
 */

import { useContext } from 'react';
import { AuthContext } from '@/context/AuthContext';
import { AuthContextType } from '@/types/auth';

/**
 * Hook to access authentication context
 * Provides auth state and methods
 *
 * @throws {Error} If used outside of AuthProvider
 * @returns {AuthContextType} Authentication context value
 *
 * @example
 * const { user, isAuthenticated, login, logout } = useAuth();
 */
export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);

  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }

  return context;
}
