import { apiClient } from './api';
import { ApiEndpoints } from './api-endpoints';

export class StationService {
  /**
   * Get all stations
   */
  static async getStations(): Promise<any[]> {
    try {
      const response = await apiClient.get(ApiEndpoints.STATIONS);
      return response.data;
    } catch (error) {
      console.error('Failed to get stations:', error);
      throw error;
    }
  }

  /**
   * Get station by ID
   */
  static async getStation(id: string): Promise<any> {
    try {
      const response = await apiClient.get(ApiEndpoints.STATION_DETAIL.replace(':id', id));
      return response.data;
    } catch (error) {
      console.error('Failed to get station:', error);
      throw error;
    }
  }

  /**
   * Get nearby stations
   */
  static async getNearbyStations(lat: number, lng: number, radiusKm: number): Promise<any[]> {
    try {
      const response = await apiClient.get(ApiEndpoints.STATION_NEARBY, {
        params: { lat, lng, radius: radiusKm },
      });
      return response.data;
    } catch (error) {
      console.error('Failed to get nearby stations:', error);
      throw error;
    }
  }

  /**
   * Get station chargers
   */
  static async getStationChargers(stationId: string): Promise<any[]> {
    try {
      const response = await apiClient.get(ApiEndpoints.STATION_CHARGERS.replace(':id', stationId));
      return response.data;
    } catch (error) {
      console.error('Failed to get station chargers:', error);
      throw error;
    }
  }

  /**
   * Search stations by name or location
   */
  static async searchStations(query: string, lat: number | null = null, lng: number | null = null): Promise<any[]> {
    try {
      const response = await apiClient.get(ApiEndpoints.STATIONS, {
        params: { q: query, lat, lng },
      });
      return response.data;
    } catch (error) {
      console.error('Failed to search stations:', error);
      throw error;
    }
  }
}

export default StationService;
