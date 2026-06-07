import { ApiClient } from '@bornemap/api-client';
import * as auth from '@bornemap/auth-client';

// Get API base URL from Expo environment variables
const API_BASE_URL = (process.env.EXPO_PUBLIC_API_BASE_URL as string) ?? 'https://api.example.tn';
const DRIVER_API_URL = `${API_BASE_URL}/api/v1/driver`;

/**
 * API Client for Driver Mobile App
 * Configured with automatic token management via auth-client
 */
export const apiClient = new ApiClient({
  baseUrl: DRIVER_API_URL,
  getToken: async () => {
    try {
      const token = await auth.getToken();
      return token;
    } catch (error) {
      console.error('Failed to get auth token:', error);
      return null;
    }
  },
});
