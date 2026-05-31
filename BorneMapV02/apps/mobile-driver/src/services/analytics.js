import axios from 'axios';
import { Platform } from 'react-native';
import { getSessionId } from './session';

function getBaseUrl() {
  if (process.env.EXPO_PUBLIC_API_URL) return process.env.EXPO_PUBLIC_API_URL;
  if (Platform.OS === 'android') return 'http://10.0.2.2:8080/api/v1';
  return 'http://localhost:8080/api/v1';
}

const ANALYTICS_ENDPOINT = `${getBaseUrl()}/analytics/connect`;

export function sendEvent(eventName, properties = {}) {
  const payload = {
    event_id: `evt-${Math.random().toString(16).slice(2, 10)}`,
    event_name: eventName,
    platform: Platform.OS === 'web' ? 'desktop_web' : 'mobile_app',
    session_id: getSessionId(),
    timestamp: new Date().toISOString(),
    properties,
  };

  axios.post(ANALYTICS_ENDPOINT, payload).catch(() => {
    console.log('analytics event dropped silently');
  });
}
