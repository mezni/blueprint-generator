import React from 'react'
import { View, Text, StyleSheet } from 'react-native'

export default function App() {
  return (
    <View style={styles.container}>
      <View style={styles.icon}>
        <Text style={styles.iconText}>⚡</Text>
      </View>
      <Text style={styles.title}>BorneMap</Text>
      <Text style={styles.subtitle}>Driver Mobile App</Text>
      <Text style={styles.footer}>Sprint 0 — Foundation</Text>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#F8FAF6',
    alignItems: 'center',
    justifyContent: 'center',
    padding: 16,
  },
  icon: {
    width: 48,
    height: 48,
    borderRadius: 12,
    backgroundColor: '#007943',
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: 16,
  },
  iconText: {
    fontSize: 24,
    color: '#FFFFFF',
  },
  title: {
    fontSize: 24,
    fontWeight: '900',
    color: '#007943',
    letterSpacing: -0.5,
  },
  subtitle: {
    fontSize: 14,
    color: '#6B7280',
    marginTop: 8,
  },
  footer: {
    fontSize: 12,
    color: '#6B7280',
    marginTop: 32,
  },
})
