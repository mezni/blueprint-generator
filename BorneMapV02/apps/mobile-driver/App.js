import React from 'react';
import { Platform } from 'react-native';
import { SafeAreaProvider, SafeAreaView } from 'react-native-safe-area-context';
import { StatusBar } from 'expo-status-bar';
import { AppProvider } from './src/context/AppContext';
import NavigationProvider from './src/context/NavigationProvider';
import MapPortal from './src/components/MapPortal';
import MapScreen from './src/screens/MapScreen';

export default function App() {
  return (
    <SafeAreaProvider>
      <AppProvider>
        <NavigationProvider>
          <StatusBar style="dark" />
          {Platform.OS === 'web' ? <DesktopApp /> : <MobileApp />}
        </NavigationProvider>
      </AppProvider>
    </SafeAreaProvider>
  );
}

function DesktopApp() {
  return <MapPortal />;
}

function MobileApp() {
  return (
    <SafeAreaView style={{ flex: 1, backgroundColor: '#FFFFFF' }}>
      <MapScreen />
    </SafeAreaView>
  );
}
