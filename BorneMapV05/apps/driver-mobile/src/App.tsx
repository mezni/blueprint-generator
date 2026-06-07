import React from 'react';
import { StatusBar } from 'expo-status-bar';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { ErrorBoundary } from '@/components/ErrorBoundary';
import { AuthGate } from '@/components/AuthGate';
import { DashboardPage } from '@/pages/DashboardPage';
import { ThemeProvider, useTheme } from './hooks/useTheme';

function AppContent() {
  const { theme } = useTheme();
  
  return (
    <SafeAreaProvider>
      <StatusBar style="auto" />
      <ErrorBoundary>
        <AuthGate>
          <DashboardPage />
        </AuthGate>
      </ErrorBoundary>
    </SafeAreaProvider>
  );
}

export default function App() {
  return (
    <ThemeProvider>
      <AppContent />
    </ThemeProvider>
  );
}
