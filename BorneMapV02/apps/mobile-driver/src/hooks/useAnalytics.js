import { useCallback, useRef } from 'react';
import { sendEvent } from '../services/analytics';
import { useAppContext } from '../context/AppContext';

export function useAnalytics() {
  const { viewport } = useAppContext();
  const eventQueue = useRef([]);

  const track = useCallback((eventName, properties = {}) => {
    const payload = { ...properties };

    if (eventName === 'zoom_in' || eventName === 'zoom_out') {
      payload.zoom_level = viewport.zoom;
      payload.viewport_center = viewport.center;
    }

    sendEvent(eventName, payload);
  }, [viewport]);

  return { track };
}
