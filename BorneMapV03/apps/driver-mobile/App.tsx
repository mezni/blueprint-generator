import React, { useState, useEffect } from 'react';
import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, View, TouchableOpacity, ActivityIndicator } from 'react-native';
import { MobileAuthService } from './src/auth-service';

const authService = new MobileAuthService({
  realm: 'ev-platform',
  clientId: 'driver-mobile',
  redirectUri: 'bornemap://callback',
});

type AuthState = 'loading' | 'authenticated' | 'unauthenticated';

export default function App() {
  const [authState, setAuthState] = useState<AuthState>('loading');
  const [userName, setUserName] = useState('');

  useEffect(() => {
    checkAuth();
  }, []);

  async function checkAuth() {
    const token = await authService.getToken();
    if (token) {
      const user = authService.getUser();
      setUserName(user?.displayName || user?.email || 'User');
      setAuthState('authenticated');
    } else {
      setAuthState('unauthenticated');
    }
  }

  async function handleLogin() {
    try {
      await authService.login();
      const user = authService.getUser();
      setUserName(user?.displayName || user?.email || 'User');
      setAuthState('authenticated');
    } catch (err) {
      console.error('Login failed:', err);
    }
  }

  async function handleLogout() {
    await authService.logout();
    setAuthState('unauthenticated');
    setUserName('');
  }

  if (authState === 'loading') {
    return (
      <View style={styles.container}>
        <ActivityIndicator size="large" color="#aa3bff" />
        <Text style={styles.text}>Loading...</Text>
        <StatusBar style="auto" />
      </View>
    );
  }

  if (authState === 'unauthenticated') {
    return (
      <View style={styles.container}>
        <Text style={styles.title}>BorneMap Driver</Text>
        <Text style={styles.text}>Please log in to continue</Text>
        <TouchableOpacity style={styles.button} onPress={handleLogin}>
          <Text style={styles.buttonText}>Log In</Text>
        </TouchableOpacity>
        <StatusBar style="auto" />
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Welcome, {userName}</Text>
      <Text style={styles.text}>You are authenticated</Text>
      <TouchableOpacity style={styles.button} onPress={handleLogout}>
        <Text style={styles.buttonText}>Log Out</Text>
      </TouchableOpacity>
      <StatusBar style="auto" />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
    gap: 16,
  },
  title: {
    fontSize: 24,
    fontWeight: '600',
    color: '#08060d',
  },
  text: {
    fontSize: 16,
    color: '#6b6375',
  },
  button: {
    backgroundColor: '#aa3bff',
    paddingHorizontal: 24,
    paddingVertical: 12,
    borderRadius: 8,
  },
  buttonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
});
