import { StatusBar } from 'expo-status-bar';
import { Stack } from 'expo-router';
import { useTheme } from '@/hooks/useTheme';

export default function RootLayout() {
  const { mode } = useTheme();

  return (
    <>
      <StatusBar style={mode === 'dark' ? 'light' : 'dark'} />
      <Stack screenOptions={{ headerShown: false }}>
        <Stack.Screen name="dashboard" options={{ headerShown: false }} />
        <Stack.Screen name="station-detail" options={{ headerShown: false }} />
        <Stack.Screen name="favorites" options={{ headerShown: false }} />
        <Stack.Screen name="review-form" options={{ headerShown: false }} />
      </Stack>
    </>
  );
}
