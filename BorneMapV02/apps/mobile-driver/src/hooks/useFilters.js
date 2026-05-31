import { useState, useEffect, useCallback, useRef } from 'react';
import { getFilters, setFilters } from '../services/api';
import { getSessionId } from '../services/session';
import { useAppContext } from '../context/AppContext';

const POLL_INTERVAL = 60000;

export function useFilters() {
  const { sessionId } = useAppContext();
  const [activeFilters, setActiveFilters] = useState(null);
  const [syncing, setSyncing] = useState(false);
  const pollRef = useRef(null);

  const fetchRemote = useCallback(async () => {
    try {
      const data = await getFilters(sessionId);
      if (data && data.filters) {
        setActiveFilters(data.filters);
      }
    } catch {
      // silent — poll will retry
    }
  }, [sessionId]);

  const update = useCallback(async (newFilters) => {
    setActiveFilters(newFilters);
    setSyncing(true);
    try {
      await setFilters(sessionId, newFilters);
    } catch {
      // last-writer-wins will resolve on next poll
    }
    setSyncing(false);
  }, [sessionId]);

  useEffect(() => {
    fetchRemote();
    pollRef.current = setInterval(fetchRemote, POLL_INTERVAL);
    return () => {
      if (pollRef.current) clearInterval(pollRef.current);
    };
  }, [fetchRemote]);

  useEffect(() => {
    const handle = () => fetchRemote();
    const subscription = require('react-native').AppState?.addEventListener?.('change', (nextState) => {
      if (nextState === 'active') fetchRemote();
    });
    return () => subscription?.remove();
  }, [fetchRemote]);

  return { activeFilters, setActiveFilters: update, syncing, refresh: fetchRemote };
}
