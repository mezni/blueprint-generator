import axios from 'axios';
import { Platform } from 'react-native';

function getBaseUrl() {
  if (process.env.EXPO_PUBLIC_API_URL) return process.env.EXPO_PUBLIC_API_URL;
  if (Platform.OS === 'android') return 'http://10.0.2.2:8080/api/v1';
  return 'http://localhost:8080/api/v1';
}

const API_BASE_URL = getBaseUrl();

console.log(`📡 API base URL: ${API_BASE_URL}`);

export const fetchNearbyStations = async ({ lat, lng, showStaged = false, signal }) => {
  try {
    const response = await axios.get(`${API_BASE_URL}/stations/nearby`, {
      params: { lat, lng, show_staged: showStaged },
      timeout: 5000,
      signal,
    });
    return response.data;
  } catch (error) {
    if (axios.isCancel(error)) return;
    console.error(`🚨 API Service integration handshake failure: ${error.message}`);
    throw error;
  }
};

export const searchStations = async ({ query, filters, signal }) => {
  try {
    const params = { q: query };
    if (filters) params.filters = JSON.stringify(filters);
    const response = await axios.get(`${API_BASE_URL}/search`, {
      params,
      timeout: 5000,
      signal,
    });
    return response.data;
  } catch (error) {
    if (axios.isCancel(error)) return;
    throw error;
  }
};

export const getStationDetail = async (stationId) => {
  const response = await axios.get(`${API_BASE_URL}/stations/${stationId}`, {
    timeout: 5000,
  });
  return response.data;
};

export const getFilters = async (sessionId) => {
  const response = await axios.get(`${API_BASE_URL}/filters`, {
    params: { session_id: sessionId },
    timeout: 5000,
  });
  return response.data;
};

export const setFilters = async (sessionId, filters) => {
  const response = await axios.put(`${API_BASE_URL}/filters`, filters, {
    params: { session_id: sessionId },
    timeout: 5000,
  });
  return response.data;
};
