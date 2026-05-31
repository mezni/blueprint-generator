import React from 'react';
import { TouchableOpacity, Text, StyleSheet, Platform } from 'react-native';
import theme from '../styles/theme';
import { isDesktop } from '../utils/platform';

export default function FAB({ onPress, label = 'Navigate', disabled = false }) {
  if (isDesktop) {
    return (
      <button
        onClick={onPress}
        disabled={disabled}
        style={styles.desktopFab}
        aria-label={label}
      >
        {label}
      </button>
    );
  }

  return (
    <TouchableOpacity
      style={[styles.mobileFab, disabled && styles.disabled]}
      onPress={onPress}
      disabled={disabled}
      accessibilityLabel={label}
      accessibilityRole="button"
    >
      <Text style={styles.fabText}>{label}</Text>
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  desktopFab: {
    position: 'absolute',
    bottom: 24,
    left: '50%',
    transform: 'translateX(-50%)',
    backgroundColor: theme.colors.primary,
    color: '#FFFFFF',
    border: 'none',
    borderRadius: 24,
    paddingHorizontal: 24,
    paddingVertical: 12,
    fontSize: 14,
    fontWeight: '600',
    cursor: 'pointer',
    zIndex: 30,
    boxShadow: '0 4px 12px rgba(0,122,255,0.3)',
    minHeight: 44,
    minWidth: 44,
  },
  mobileFab: {
    position: 'absolute',
    bottom: 24,
    alignSelf: 'center',
    backgroundColor: theme.colors.primary,
    paddingHorizontal: 24,
    paddingVertical: 12,
    borderRadius: 24,
    elevation: 6,
    shadowColor: theme.colors.primary,
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.3,
    shadowRadius: 8,
    zIndex: 30,
    minHeight: 44,
    minWidth: 44,
    justifyContent: 'center',
    alignItems: 'center',
  },
  disabled: {
    opacity: 0.4,
  },
  fabText: {
    color: '#FFFFFF',
    fontSize: 14,
    fontWeight: '600',
  },
});
