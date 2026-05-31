import React, { useRef, useCallback } from 'react';
import { View, Text, TouchableOpacity, ActivityIndicator, Animated, PanResponder, StyleSheet, Dimensions } from 'react-native';
import theme from '../styles/theme';

const PEEK_RATIO = 0.35;
const EXPANDED_RATIO = 0.7;
const SCREEN_HEIGHT = Dimensions.get('window').height;
const PEEK_HEIGHT = SCREEN_HEIGHT * PEEK_RATIO;
const EXPANDED_HEIGHT = SCREEN_HEIGHT * EXPANDED_RATIO;

export default function StationDetailSheet({ station, isLoading, error, sheetMode, setSheetMode, onClose, onRetry, onNavigate }) {
  const translateY = useRef(new Animated.Value(0)).current;
  const isExpanded = sheetMode === 'expanded';

  const handleKeyDown = useCallback((e) => {
    if (e.key === 'Escape') onClose?.();
  }, [onClose]);

  const panResponder = useRef(
    PanResponder.create({
      onStartShouldSetPanResponder: () => true,
      onMoveShouldSetPanResponder: (_, gesture) => Math.abs(gesture.dy) > 5,
      onPanResponderMove: (_, gesture) => {
        translateY.setValue(Math.max(0, gesture.dy));
      },
      onPanResponderRelease: (_, gesture) => {
        if (gesture.dy > 80) {
          if (isExpanded) {
            setSheetMode('peek');
          } else {
            onClose();
          }
        } else if (gesture.dy < -80 && !isExpanded) {
          setSheetMode('expanded');
        }
        Animated.spring(translateY, { toValue: 0, useNativeDriver: true }).start();
      },
    })
  ).current;

  const containerHeight = isExpanded ? EXPANDED_HEIGHT : PEEK_HEIGHT;

  if (!station && !isLoading && !error) return null;

  return (
    <Animated.View
      style={[styles.container, { height: containerHeight, transform: [{ translateY }] }]}
      {...panResponder.panHandlers}
      onKeyDown={handleKeyDown}
      accessibilityLabel="Station details"
    >
      <View style={styles.handle} />

      {isLoading ? (
        <View style={styles.skeleton} aria-live="polite">
          <View style={styles.skeletonTitle} />
          <View style={styles.skeletonRow} />
        </View>
      ) : error && !station ? (
        <View style={styles.errorRow} aria-live="polite">
          <Text style={styles.errorText}>{error}</Text>
          <TouchableOpacity onPress={onRetry} style={styles.retryBtn}>
            <Text style={styles.retryText}>Retry</Text>
          </TouchableOpacity>
        </View>
      ) : station ? (
        <>
          <View style={styles.header}>
            <View style={{ flex: 1 }}>
              <Text style={styles.title}>{station.station_name}</Text>
              <Text style={styles.address}>{station.address}</Text>
            </View>
            {isExpanded && (
              <TouchableOpacity onPress={onClose} style={styles.closeBtn}>
                <Text style={styles.closeText}>✕</Text>
              </TouchableOpacity>
            )}
          </View>

          {isExpanded && (
            <>
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
                        <Text style={styles.chipText}>{type.replace(/_/g, ' ').toUpperCase()}</Text>
                      </View>
                    ))}
                  </View>
                </View>
              )}

              {station.navigate_url && (
                <TouchableOpacity style={styles.navBtn} onPress={() => onNavigate?.(station.navigate_url)}>
                  <Text style={styles.navBtnText}>Navigate</Text>
                </TouchableOpacity>
              )}
            </>
          )}

          {!isExpanded && (
            <Text style={styles.peekHint}>Swipe up for details</Text>
          )}
        </>
      ) : null}
    </Animated.View>
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
    paddingTop: 8,
    elevation: 8,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: -2 },
    shadowOpacity: 0.15,
    shadowRadius: 8,
    zIndex: 40,
  },
  handle: {
    width: 36,
    height: 4,
    borderRadius: 2,
    backgroundColor: '#DDDDDD',
    alignSelf: 'center',
    marginBottom: 8,
  },
  header: { flexDirection: 'row', alignItems: 'flex-start', marginBottom: 8 },
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
  peekHint: { fontSize: 12, color: theme.colors.textMuted, textAlign: 'center', marginTop: 4 },
  navBtn: {
    backgroundColor: theme.colors.primary,
    borderRadius: 12,
    paddingVertical: 12,
    alignItems: 'center',
    marginTop: 8,
  },
  navBtnText: { color: '#FFFFFF', fontSize: 14, fontWeight: '600' },
  skeleton: { gap: 8, paddingTop: 12 },
  skeletonTitle: { height: 16, width: '60%', backgroundColor: '#EEEEEE', borderRadius: 4 },
  skeletonRow: { height: 12, width: '40%', backgroundColor: '#F5F5F5', borderRadius: 4 },
  errorRow: { flexDirection: 'row', alignItems: 'center', paddingTop: 12 },
  errorText: { flex: 1, fontSize: 13, color: '#C62828' },
  retryBtn: { paddingHorizontal: 12, paddingVertical: 6, backgroundColor: '#FFEBEE', borderRadius: 6 },
  retryText: { fontSize: 12, fontWeight: '600', color: '#C62828' },
});
