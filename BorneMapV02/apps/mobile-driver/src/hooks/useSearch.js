import { useState, useRef, useCallback } from 'react';
import { searchStations } from '../services/api';

export function useSearch() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const [isSearching, setIsSearching] = useState(false);
  const [error, setError] = useState(null);
  const timerRef = useRef(null);
  const abortRef = useRef(null);

  const search = useCallback((q, filters) => {
    setQuery(q);

    if (timerRef.current) clearTimeout(timerRef.current);

    if (!q || q.trim().length < 2) {
      setResults([]);
      setError(null);
      return;
    }

    timerRef.current = setTimeout(() => {
      if (abortRef.current) abortRef.current.abort();
      const controller = new AbortController();
      abortRef.current = controller;

      setIsSearching(true);
      setError(null);

      searchStations({ query: q, filters, signal: controller.signal })
        .then((data) => {
          setResults(data || []);
          setIsSearching(false);
        })
        .catch((err) => {
          if (err?.name === 'CanceledError') return;
          setError('Search failed. Check your connection and try again.');
          setIsSearching(false);
        });
    }, 300);
  }, []);

  const clear = useCallback(() => {
    setQuery('');
    setResults([]);
    setError(null);
    if (abortRef.current) abortRef.current.abort();
  }, []);

  return { query, setQuery, results, isSearching, error, search, clear };
}
