import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { View, StyleSheet } from "react-native";
import { useOsFloorCheck } from "./hooks/useOsFloorCheck";
import { OsUpdateRequiredScreen } from "./screens/OsUpdateRequiredScreen";
import { MapScreen } from "./screens/MapScreen";
import { ErrorBoundary } from "./lib/ErrorBoundary";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60_000,
      networkMode: "offlineFirst",
    },
  },
});

export default function App() {
  const { isSupported } = useOsFloorCheck();

  if (!isSupported) {
    return <OsUpdateRequiredScreen />;
  }

  return (
    <ErrorBoundary>
      <QueryClientProvider client={queryClient}>
        <View style={styles.container}>
          <MapScreen />
        </View>
      </QueryClientProvider>
    </ErrorBoundary>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1 },
});
