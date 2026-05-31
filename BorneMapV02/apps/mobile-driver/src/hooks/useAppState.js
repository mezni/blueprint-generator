import { useEffect, useRef, useCallback } from 'react';
import { AppState } from 'react-native';

export function useAppState() {
  const appStateRef = useRef(AppState.currentState);

  const subscribe = useCallback((onForeground) => {
    const subscription = AppState.addEventListener('change', (nextState) => {
      if (appStateRef.current.match(/inactive|background/) && nextState === 'active') {
        onForeground();
      }
      appStateRef.current = nextState;
    });
    return () => subscription.remove();
  }, []);

  return { subscribe };
}
