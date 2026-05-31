import { useState, useCallback, useRef } from 'react';
import { getStationDetail } from '../services/api';

export function useStationDetail() {
  const [station, setStation] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);
  const [sheetMode, setSheetMode] = useState('closed');
  const debounceRef = useRef(null);
  const abortRef = useRef(null);

  const open = useCallback(async (stationId) => {
    if (debounceRef.current) clearTimeout(debounceRef.current);

    debounceRef.current = setTimeout(async () => {
      if (abortRef.current) abortRef.current.abort();
      const controller = new AbortController();
      abortRef.current = controller;

      setIsLoading(true);
      setError(null);

      try {
        const data = await getStationDetail(stationId);
        setStation(data);
        setIsLoading(false);
        setSheetMode('expanded');
      } catch {
        setError('Failed to load station details');
        setIsLoading(false);
      }
    }, 500);
  }, []);

  const close = useCallback(() => {
    setStation(null);
    setSheetMode('closed');
    setError(null);
  }, []);

  const retry = useCallback(async () => {
    if (!station) return;
    setIsLoading(true);
    setError(null);
    try {
      const data = await getStationDetail(station.station_id);
      setStation(data);
      setIsLoading(false);
    } catch {
      setError('Failed to load station details');
      setIsLoading(false);
    }
  }, [station]);

  return { station, isLoading, error, sheetMode, setSheetMode, open, close, retry };
}
