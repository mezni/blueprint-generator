import React from 'react';
import { StatusBar } from 'expo-status-bar';
import { SafeAreaProvider, useSafeAreaInsets } from 'react-native-safe-area-context';
import { ErrorBoundary } from '@/components/ErrorBoundary';
import { AuthGate } from '@/components/AuthGate';
import { ThemeProvider, useTheme } from '@/hooks/useTheme';
import { DashboardPage } from '@/pages/DashboardPage';
import { useStations } from '@/hooks/useStations';
import { useAuth } from '@/hooks/useAuth';
import { useNetworkStatus } from '@/hooks/useNetworkStatus';
import { OfflineManager } from '@/services/offline-manager';
import { MockService } from '@/services/mock-service';

interface RootProps {
  children: React.ReactNode;
}

function AppContent({ children }: RootProps) {
  const { mode } = useTheme();
  const { isAuthenticated } = useAuth();
  const { isOnline, isOffline } = useNetworkStatus();
  const { data: stations, isLoading } = useStations();

  useEffect(() => {
    // Initialize mock services in development
    if (__DEV__) {
      MockService.initialize();
    }
  }, []);

  // Handle offline/online status changes
  useEffect(() => {
    if (isOffline && stations && stations.length > 0) {
      OfflineManager.cacheStations(stations);
    }
  }, [isOffline, stations]);

  return (
    <>
      {children}
      <StatusBar style={mode === 'dark' ? 'light' : 'dark'} />
    </>
  );
}

export default function App() {
  const insets = useSafeAreaInsets();

  return (
    <SafeAreaProvider style={{ flex: 1, paddingTop: insets.top }}>
      <ThemeProvider>
        <ErrorBoundary>
          <AuthGate>
            <AppContent>
              <DashboardPage />
            </AppContent>
          </AuthGate>
        </ErrorBoundary>
      </ThemeProvider>
    </SafeAreaProvider>
  );
}
