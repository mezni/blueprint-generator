import React, { useCallback, useState, useEffect } from 'react';
import { View, Text, TouchableOpacity, ActivityIndicator, StyleSheet } from 'react-native';
import theme from '../styles/theme';
import { isDesktop } from '../utils/platform';

export default function StationDetailPanel({ station, isLoading, error, onClose, onRetry, onNavigate }) {
  const [viewportHeight, setViewportHeight] = useState(
    typeof window !== 'undefined' ? window.innerHeight : 700
  );

  useEffect(() => {
    if (!isDesktop || typeof window === 'undefined') return;
    const handle = () => setViewportHeight(window.innerHeight);
    window.addEventListener('resize', handle);
    return () => window.removeEventListener('resize', handle);
  }, []);

  const isMinimized = isDesktop && viewportHeight < 500;

  const handleKeyDown = useCallback((e) => {
    if (e.key === 'Escape') onClose?.();
  }, [onClose]);

  if (isLoading) {
    return (
      <View style={styles.container} aria-live="polite" onKeyDown={handleKeyDown}>
        <View style={styles.skeleton}>
          <View style={styles.skeletonTitle} />
          <View style={styles.skeletonRow} />
          <View style={styles.skeletonRow} />
        </View>
      </View>
    );
  }

  if (error && !station) {
    return (
      <View style={styles.container} aria-live="polite">
        <View style={styles.errorRow}>
          <Text style={styles.errorText}>{error}</Text>
          <TouchableOpacity onPress={onRetry} style={styles.retryBtn} aria-label="Retry loading station details">
            <Text style={styles.retryText}>Retry</Text>
          </TouchableOpacity>
          <TouchableOpacity onPress={onClose} style={styles.closeBtn} aria-label="Close detail panel">
            <Text style={styles.closeText}>✕</Text>
          </TouchableOpacity>
        </View>
      </View>
    );
  }

  if (!station) return null;

  if (isMinimized) {
    return (
      <View style={styles.minimizedBar} onKeyDown={handleKeyDown}>
        <View style={{ flex: 1 }}>
          <Text style={styles.minimizedTitle} numberOfLines={1}>{station.station_name}</Text>
          <Text style={styles.minimizedMeta}>{station.available_chargers}/{station.total_chargers} available — {station.status}</Text>
        </View>
        <TouchableOpacity onPress={onClose} style={styles.closeBtn} aria-label="Close detail panel">
          <Text style={styles.closeText}>✕</Text>
        </TouchableOpacity>
      </View>
    );
  }

  return (
    <View style={styles.container} onKeyDown={handleKeyDown}>
      <View style={styles.header}>
        <View style={{ flex: 1 }}>
          <Text style={styles.title}>{station.station_name}</Text>
          <Text style={styles.address}>{station.address}</Text>
        </View>
        <TouchableOpacity onPress={onClose} style={styles.closeBtn} aria-label="Close detail panel">
          <Text style={styles.closeText}>✕</Text>
        </TouchableOpacity>
      </View>

      <View style={styles.statsRow}>
        <View style={styles.stat}>
          <Text style={styles.statValue}>{station.available_chargers}/{station.total_chargers}</Text>
          <Text style={styles.statLabel}>Available</Text>
        </View>
        <View style={styles.stat}>
          <View style={[styles.statusDot, { backgroundColor: station.status === 'Available' ? theme.colors.success : theme.colors.warning }]} />
          <Text style={styles.statLabel}>{station.status}</Text>
        </View>
      </View>

      {station.connector_types?.length > 0 && (
        <View style={styles.connectors}>
          <Text style={styles.sectionLabel}>Connectors</Text>
          <View style={styles.chipRow}>
            {station.connector_types.map((type) => (
              <View key={type} style={styles.chip}>
                <Text style={styles.chipText}>{type.replace('_', ' ').toUpperCase()}</Text>
              </View>
            ))}
          </View>
        </View>
      )}

      {station.navigate_url && (
        <TouchableOpacity
          style={styles.navBtn}
          onPress={() => onNavigate?.(station.navigate_url)}
          aria-label="Navigate to station"
        >
          <Text style={styles.navBtnText}>Navigate</Text>
        </TouchableOpacity>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    backgroundColor: '#FFFFFF',
    borderTopLeftRadius: 16,
    borderTopRightRadius: 16,
    padding: 16,
    elevation: 8,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: -2 },
    shadowOpacity: 0.1,
    shadowRadius: 8,
    zIndex: 40,
    maxHeight: 300,
  },
  header: { flexDirection: 'row', alignItems: 'flex-start', marginBottom: 12 },
  title: { fontSize: 16, fontWeight: '700', color: theme.colors.textPrimary },
  address: { fontSize: 12, color: theme.colors.textSecondary, marginTop: 2 },
  closeBtn: { padding: 8, marginLeft: 8 },
  closeText: { fontSize: 18, color: theme.colors.textMuted },
  statsRow: { flexDirection: 'row', gap: 24, marginBottom: 12 },
  stat: { flexDirection: 'row', alignItems: 'center', gap: 6 },
  statValue: { fontSize: 20, fontWeight: '700', color: theme.colors.textPrimary },
  statLabel: { fontSize: 12, color: theme.colors.textSecondary },
  statusDot: { width: 10, height: 10, borderRadius: 5 },
  connectors: { marginBottom: 12 },
  sectionLabel: { fontSize: 11, fontWeight: '700', color: theme.colors.textMuted, textTransform: 'uppercase', letterSpacing: 0.5, marginBottom: 6 },
  chipRow: { flexDirection: 'row', flexWrap: 'wrap', gap: 6 },
  chip: { paddingHorizontal: 10, paddingVertical: 4, borderRadius: 12, backgroundColor: '#F5F5F5' },
  chipText: { fontSize: 11, fontWeight: '600', color: theme.colors.textSecondary },
  navBtn: {
    backgroundColor: theme.colors.primary,
    borderRadius: 12,
    paddingVertical: 12,
    alignItems: 'center',
  },
  navBtnText: { color: '#FFFFFF', fontSize: 14, fontWeight: '600' },
  skeleton: { gap: 8 },
  skeletonTitle: { height: 16, width: '60%', backgroundColor: '#EEEEEE', borderRadius: 4 },
  skeletonRow: { height: 12, width: '40%', backgroundColor: '#F5F5F5', borderRadius: 4 },
  errorRow: { flexDirection: 'row', alignItems: 'center' },
  errorText: { flex: 1, fontSize: 13, color: '#C62828' },
  retryBtn: { paddingHorizontal: 12, paddingVertical: 6, backgroundColor: '#FFEBEE', borderRadius: 6, marginRight: 8 },
  retryText: { fontSize: 12, fontWeight: '600', color: '#C62828' },
  minimizedBar: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 16,
    paddingVertical: 10,
    backgroundColor: '#FFFFFF',
    borderTopLeftRadius: 12,
    borderTopRightRadius: 12,
    elevation: 6,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: -2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    zIndex: 40,
  },
  minimizedTitle: { fontSize: 14, fontWeight: '700', color: theme.colors.textPrimary },
  minimizedMeta: { fontSize: 12, color: theme.colors.textSecondary, marginTop: 2 },
});
