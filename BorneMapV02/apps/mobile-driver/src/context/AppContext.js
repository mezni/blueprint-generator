import React, { createContext, useContext, useState, useCallback } from 'react';
import { getSessionId } from '../services/session';

const AppContext = createContext(null);

export function AppProvider({ children }) {
  const [activeTab, setActiveTab] = useState('map');
  const [searchQuery, setSearchQuery] = useState('');
  const [filters, setFilters] = useState(null);
  const [viewport, setViewport] = useState({ center: { lat: 36.8065, lng: 10.1815 }, zoom: 13 });
  const [selectedStation, setSelectedStation] = useState(null);
  const sessionId = getSessionId();

  const value = {
    activeTab,
    setActiveTab,
    searchQuery,
    setSearchQuery,
    filters,
    setFilters,
    viewport,
    setViewport,
    selectedStation,
    setSelectedStation,
    sessionId,
  };

  return (
    <AppContext.Provider value={value}>
      {children}
    </AppContext.Provider>
  );
}

export function useAppContext() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error('useAppContext must be used within AppProvider');
  return ctx;
}
