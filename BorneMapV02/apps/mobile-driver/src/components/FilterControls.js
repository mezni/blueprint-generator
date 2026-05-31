import React, { useState } from 'react';
import { View, Text, TouchableOpacity, StyleSheet, Platform } from 'react-native';
import theme from '../styles/theme';

const CONNECTOR_OPTIONS = [
  { key: 'type_2', label: 'Type 2' },
  { key: 'ccs', label: 'CCS' },
  { key: 'chademo', label: 'CHAdeMO' },
  { key: 'tesla', label: 'Tesla' },
];

const STATUS_OPTIONS = [
  { key: 'available', label: 'Available' },
  { key: 'busy', label: 'Busy' },
  { key: 'offline', label: 'Offline' },
];

export default function FilterControls({ filters, onFiltersChange }) {
  const [expanded, setExpanded] = useState(false);

  const toggleConnector = (key) => {
    const current = filters?.connector_types || [];
    const next = current.includes(key)
      ? current.filter((c) => c !== key)
      : [...current, key];
    onFiltersChange({ ...filters, connector_types: next });
  };

  const toggleStatus = (key) => {
    const current = filters?.status || [];
    const next = current.includes(key)
      ? current.filter((s) => s !== key)
      : [...current, key];
    onFiltersChange({ ...filters, status: next });
  };

  const hasActive = (filters?.connector_types?.length || 0) + (filters?.status?.length || 0) > 0;

  return (
    <View style={styles.container}>
      <TouchableOpacity
        onPress={() => setExpanded(!expanded)}
        style={styles.toggle}
        aria-label={expanded ? 'Close filters' : 'Open filters'}
      >
        <Text style={styles.toggleText}>
          Filters{hasActive ? ` (${(filters?.connector_types?.length || 0) + (filters?.status?.length || 0)})` : ''}
        </Text>
        <Text style={styles.arrow}>{expanded ? '▲' : '▼'}</Text>
      </TouchableOpacity>

      {expanded && (
        <View style={styles.panel}>
          <Text style={styles.sectionTitle}>Connector Type</Text>
          <View style={styles.chipRow}>
            {CONNECTOR_OPTIONS.map((opt) => (
              <TouchableOpacity
                key={opt.key}
                onPress={() => toggleConnector(opt.key)}
                style={[
                  styles.chip,
                  (filters?.connector_types || []).includes(opt.key) && styles.chipActive,
                ]}
                aria-label={`Filter by ${opt.label}`}
                aria-pressed={(filters?.connector_types || []).includes(opt.key)}
              >
                <Text style={[
                  styles.chipText,
                  (filters?.connector_types || []).includes(opt.key) && styles.chipTextActive,
                ]}>{opt.label}</Text>
              </TouchableOpacity>
            ))}
          </View>

          <Text style={styles.sectionTitle}>Status</Text>
          <View style={styles.chipRow}>
            {STATUS_OPTIONS.map((opt) => (
              <TouchableOpacity
                key={opt.key}
                onPress={() => toggleStatus(opt.key)}
                style={[
                  styles.chip,
                  (filters?.status || []).includes(opt.key) && styles.chipActive,
                ]}
                aria-label={`Filter by ${opt.label}`}
                aria-pressed={(filters?.status || []).includes(opt.key)}
              >
                <Text style={[
                  styles.chipText,
                  (filters?.status || []).includes(opt.key) && styles.chipTextActive,
                ]}>{opt.label}</Text>
              </TouchableOpacity>
            ))}
          </View>
        </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    position: 'absolute',
    top: Platform.OS === 'web' ? 60 : 64,
    left: 12,
    right: 12,
    zIndex: 39,
  },
  toggle: {
    flexDirection: 'row',
    alignItems: 'center',
    alignSelf: 'flex-start',
    backgroundColor: '#FFFFFF',
    borderRadius: theme.borderRadius.sm,
    paddingHorizontal: 12,
    paddingVertical: 8,
    elevation: 3,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.08,
    shadowRadius: 3,
    minHeight: 44,
  },
  toggleText: { fontSize: 13, fontWeight: '600', color: theme.colors.textPrimary, marginRight: 6 },
  arrow: { fontSize: 10, color: theme.colors.textSecondary },
  panel: {
    marginTop: 6,
    backgroundColor: '#FFFFFF',
    borderRadius: theme.borderRadius.md,
    padding: 12,
    elevation: 4,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
  },
  sectionTitle: { fontSize: 11, fontWeight: '700', color: theme.colors.textMuted, textTransform: 'uppercase', letterSpacing: 0.5, marginBottom: 6, marginTop: 8 },
  chipRow: { flexDirection: 'row', flexWrap: 'wrap', gap: 6 },
  chip: {
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: 16,
    backgroundColor: '#F5F5F5',
    borderWidth: 1,
    borderColor: '#EEEEEE',
    minHeight: 44,
    justifyContent: 'center',
  },
  chipActive: { backgroundColor: '#E3F2FD', borderColor: '#007AFF' },
  chipText: { fontSize: 12, fontWeight: '500', color: theme.colors.textSecondary },
  chipTextActive: { color: '#007AFF', fontWeight: '600' },
});
