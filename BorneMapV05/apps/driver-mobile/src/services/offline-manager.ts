import AsyncStorage from '@react-native-async-storage/async-storage';
import StationService from '../services/station-service';

const OFFLINE_DATA_KEY = 'offline_stations';
const LAST_SYNC_KEY = 'last_sync_time';

export class OfflineManager {
  /**
   * Cache stations for offline access
   */
  static async cacheStations(stations: any[]): Promise<void> {
    try {
      await AsyncStorage.setItem(OFFLINE_DATA_KEY, JSON.stringify(stations));
      await AsyncStorage.setItem(LAST_SYNC_KEY, new Date().toISOString());
    } catch (error) {
      console.error('Failed to cache stations:', error);
      throw error;
    }
  }

  /**
   * Get cached stations
   */
  static async getCachedStations(): Promise<any[]> {
    try {
      const data = await AsyncStorage.getItem(OFFLINE_DATA_KEY);
      return data ? JSON.parse(data) : [];
    } catch (error) {
      console.error('Failed to get cached stations:', error);
      return [];
    }
  }

  /**
   * Check if offline data is available
   */
  static async hasOfflineData(): Promise<boolean> {
    try {
      const data = await AsyncStorage.getItem(OFFLINE_DATA_KEY);
      return !!data;
    } catch (error) {
      console.error('Failed to check offline data:', error);
      return false;
    }
  }

  /**
   * Get last sync time
   */
  static async getLastSyncTime(): Promise<string | null> {
    try {
      return await AsyncStorage.getItem(LAST_SYNC_KEY);
    } catch (error) {
      console.error('Failed to get last sync time:', error);
      return null;
    }
  }

  /**
   * Clear offline data
   */
  static async clearOfflineData(): Promise<void> {
    try {
      await AsyncStorage.removeItem(OFFLINE_DATA_KEY);
      await AsyncStorage.removeItem(LAST_SYNC_KEY);
    } catch (error) {
      console.error('Failed to clear offline data:', error);
    }
  }

  /**
   * Sync offline data with server
   */
  static async syncWithServer(): Promise<void> {
    try {
      const cachedStations = await this.getCachedStations();
      
      // TODO: Implement actual sync logic
      // const serverStations = await StationService.getStations();
      
      // For now, just return cached data
      console.log('Syncing offline data with server...');
      await this.cacheStations(cachedStations);
    } catch (error) {
      console.error('Failed to sync with server:', error);
      throw error;
    }
  }

  /**
   * Check if data needs refresh
   */
  static async needsRefresh(): Promise<boolean> {
    try {
      const cachedTime = await this.getLastSyncTime();
      if (!cachedTime) return true;

      const cacheDate = new Date(cachedTime);
      const now = new Date();
      const hoursSinceLastSync = (now.getTime() - cacheDate.getTime()) / (1000 * 60 * 60);

      return hoursSinceLastSync > 24; // Refresh if older than 24 hours
    } catch (error) {
      console.error('Failed to check refresh needed:', error);
      return true;
    }
  }
}

export default OfflineManager;
