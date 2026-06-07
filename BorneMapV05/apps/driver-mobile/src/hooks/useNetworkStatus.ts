import { useState, useEffect } from 'react';
import { AppState, AppStateStatus } from 'react-native';

export function useNetworkStatus() {
  const [isOnline, setIsOnline] = useState(true);
  const [appState, setAppState] = useState<AppStateStatus>(AppState.currentState);

  useEffect(() => {
    const handleNetworkChange = () => {
      setIsOnline(true);
    };

    const handleNetworkLost = () => {
      setIsOnline(false);
    };

    window.addEventListener('online', handleNetworkChange);
    window.addEventListener('offline', handleNetworkLost);

    return () => {
      window.removeEventListener('online', handleNetworkChange);
      window.removeEventListener('offline', handleNetworkLost);
    };
  }, []);

  useEffect(() => {
    const handleAppStateChange = (nextAppState: AppStateStatus) => {
      setAppState(nextAppState);
    };

    const subscription = AppState.addEventListener('change', handleAppStateChange);

    return () => {
      subscription.remove();
    };
  }, []);

  const isOffline = !isOnline && appState === 'active';

  return { isOnline, isOffline, appState };
}

export function useOnlineStatus() {
  const [isOnline, setIsOnline] = useState(true);
  const [isChecking, setIsChecking] = useState(false);

  useEffect(() => {
    const checkOnlineStatus = async () => {
      setIsChecking(true);
      try {
        // Check if we can reach the API endpoint
        const response = await fetch(`${process.env.EXPO_PUBLIC_API_BASE_URL || 'https://api.example.tn'}/health`, {
          method: 'HEAD',
          timeout: 5000,
        });
        setIsOnline(response.ok || true);
      } catch (error) {
        setIsOnline(false);
      } finally {
        setIsChecking(false);
      }
    };

    // Check initial status
    checkOnlineStatus();

    // Check every 30 seconds
    const interval = setInterval(checkOnlineStatus, 30000);

    return () => clearInterval(interval);
  }, []);

  return { isOnline, isChecking };
}
