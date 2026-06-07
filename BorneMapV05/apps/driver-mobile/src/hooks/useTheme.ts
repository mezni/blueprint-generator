import React, { createContext, useContext, useState, useEffect } from 'react';
import { useColorScheme } from 'react-native';

type ThemeMode = 'light' | 'dark' | 'rtl';

interface ThemeContextType {
  mode: ThemeMode;
  toggleTheme: () => void;
  isRTL: boolean;
}

const ThemeContext = createContext<ThemeContextType | null>(null);

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [mode, setMode] = useState<ThemeMode>('light');
  const [isRTL, setIsRTL] = useState(false);
  const colorScheme = useColorScheme();

  useEffect(() => {
    // Determine theme mode based on device preference
    setMode(colorScheme === 'dark' ? 'dark' : 'light');
  }, [colorScheme]);

  const toggleTheme = () => {
    setMode(prev => prev === 'light' ? 'dark' : 'light');
  };

  return (
    <ThemeContext.Provider value={{ mode, toggleTheme, isRTL }}>
      {children}
    </ThemeContext.Provider>
  );
}

export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return context;
}
