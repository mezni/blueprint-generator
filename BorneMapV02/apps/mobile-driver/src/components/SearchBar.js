import React, { useState, useCallback } from 'react';
import { View, TextInput, TouchableOpacity, Text, ActivityIndicator, StyleSheet, Platform } from 'react-native';
import theme from '../styles/theme';

export default function SearchBar({ onSearch, results, isSearching, error, query, setQuery, onClear }) {
  const handleChange = useCallback((text) => {
    setQuery(text);
    onSearch(text);
  }, [onSearch, setQuery]);

  return (
    <View style={styles.container}>
      <View style={styles.inputRow}>
        <TextInput
          style={styles.input}
          placeholder="Search stations..."
          placeholderTextColor={theme.colors.textMuted}
          value={query}
          onChangeText={handleChange}
          returnKeyType="search"
          autoCorrect={false}
          aria-label="Search charging stations"
        />
        {isSearching && (
          <ActivityIndicator size="small" color={theme.colors.primary} style={styles.spinner} />
        )}
        {query.length > 0 && (
          <TouchableOpacity onPress={onClear} style={styles.clearButton} aria-label="Clear search">
            <Text style={styles.clearText}>✕</Text>
          </TouchableOpacity>
        )}
      </View>
      {error && (
        <View style={styles.errorRow} aria-live="polite">
          <Text style={styles.errorText}>{error}</Text>
          <TouchableOpacity
            style={styles.retryBtn}
            onPress={() => onSearch(query)}
            aria-label="Retry search"
          >
            <Text style={styles.retryText}>Retry</Text>
          </TouchableOpacity>
        </View>
      )}
      {!isSearching && !error && query.length >= 2 && results?.length === 0 && (
        <View style={styles.emptyRow} aria-live="polite">
          <Text style={styles.emptyText}>No stations found. Try widening your search area.</Text>
        </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    position: 'absolute',
    top: Platform.OS === 'web' ? 8 : 12,
    left: 12,
    right: 12,
    zIndex: 40,
  },
  inputRow: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#FFFFFF',
    borderRadius: theme.borderRadius.md,
    paddingHorizontal: 12,
    height: 44,
    elevation: 4,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
  },
  input: {
    flex: 1,
    fontSize: 14,
    color: theme.colors.textPrimary,
    outlineStyle: 'none',
    borderWidth: 0,
    paddingVertical: 0,
  },
  spinner: { marginLeft: 8 },
  clearButton: { marginLeft: 8, padding: 4 },
  clearText: { fontSize: 16, color: theme.colors.textMuted },
  errorRow: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#FFEBEE',
    borderRadius: theme.borderRadius.sm,
    padding: 10,
    marginTop: 6,
  },
  errorText: { flex: 1, fontSize: 12, color: '#C62828' },
  retryBtn: { paddingHorizontal: 12, paddingVertical: 4, backgroundColor: '#FFFFFF', borderRadius: 6, marginLeft: 8 },
  retryText: { fontSize: 12, fontWeight: '600', color: '#C62828' },
  emptyRow: {
    backgroundColor: '#F5F5F5',
    borderRadius: theme.borderRadius.sm,
    padding: 12,
    marginTop: 6,
  },
  emptyText: { fontSize: 13, color: theme.colors.textSecondary, textAlign: 'center' },
});
