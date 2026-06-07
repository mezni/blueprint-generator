import React from 'react';
import { StyleSheet, View, Text } from 'react-native';

export default function DashboardPage() {
  return (
    <View style={styles.container}>
      <Text style={styles.title}>Driver Mobile App</Text>
      <Text style={styles.subtitle}>Welcome to Bornemap Driver Mobile</Text>
      <Text style={styles.description}>
        This is a placeholder for the driver mobile app implementation.
      </Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#F3F4F6',
    alignItems: 'center',
    justifyContent: 'center',
    padding: 20,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#111827',
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#6B7280',
    marginBottom: 16,
  },
  description: {
    fontSize: 14,
    color: '#6B7280',
    textAlign: 'center',
  },
});
