import React from 'react';
import { View, Text } from 'react-native';
import theme from '../styles/theme';

export default function ProfileScreen() {
  return (
    <View style={{ flex: 1, justifyContent: 'center', alignItems: 'center', backgroundColor: theme.colors.background }}>
      <Text style={{ fontSize: 24, marginBottom: 8 }}>👤</Text>
      <Text style={{ fontSize: theme.typography.title, fontWeight: '600', color: theme.colors.textPrimary }}>Profile</Text>
      <Text style={{ fontSize: theme.typography.body, color: theme.colors.textSecondary, marginTop: 4 }}>Manage your account</Text>
    </View>
  );
}
