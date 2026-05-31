import React from 'react';
import { StyleSheet, View, Text } from 'react-native';

const STATUS_COLORS = {
  Available: { bg: '#E8F5E9', text: '#2E7D32' },
  Occupied: { bg: '#FFEBEE', text: '#C62828' },
  Offline: { bg: '#F5F5F5', text: '#757575' },
  Maintenance: { bg: '#FFF8E1', text: '#F57F17' },
};

export default function StationCard({ station }) {
  const statusStyle = STATUS_COLORS[station.status] || STATUS_COLORS.Offline;

  return (
    <View style={styles.card}>
      <View style={styles.titleRow}>
        <Text style={styles.title}>{station.name}</Text>
        {!station.is_live && (
          <View style={styles.stagedBadge}>
            <Text style={styles.stagedBadgeText}>STAGED</Text>
          </View>
        )}
      </View>

      {station.partner && (
        <Text style={styles.provider}>
          {station.partner.name} · {station.partner.type}
        </Text>
      )}

      <View style={styles.badgeContainer}>
        <View style={[styles.statusBadge, { backgroundColor: statusStyle.bg }]}>
          <Text style={[styles.badgeText, { color: statusStyle.text }]}>{station.status}</Text>
        </View>
      </View>

      <Text style={styles.chargerHeader}>Connectors:</Text>
      {(station.chargers || []).length === 0 ? (
        <Text style={styles.emptyChargers}>No chargers at this station</Text>
      ) : (
        station.chargers.map((charger) => {
          const chgStyle = STATUS_COLORS[charger.status] || STATUS_COLORS.Offline;
          return (
            <View key={charger.id} style={styles.chargerRow}>
              <Text style={styles.chargerText}>⚡ {charger.plug_type} ({charger.power_output} kW)</Text>
              <Text style={[styles.chargerStatus, { color: chgStyle.text }]}>
                {charger.status}
              </Text>
            </View>
          );
        })
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  card: { backgroundColor: '#FFFFFF', padding: 4 },
  titleRow: { flexDirection: 'row', alignItems: 'center', gap: 8 },
  title: { fontSize: 16, fontWeight: 'bold', color: '#111111', flexShrink: 1 },
  stagedBadge: { backgroundColor: '#FFF3E0', paddingHorizontal: 8, paddingVertical: 2, borderRadius: 4 },
  stagedBadgeText: { fontSize: 10, fontWeight: '700', color: '#E65100', letterSpacing: 0.5 },
  provider: { fontSize: 12, color: '#666666', marginTop: 2, textTransform: 'uppercase', letterSpacing: 0.5 },
  badgeContainer: { flexDirection: 'row', marginTop: 8 },
  statusBadge: { paddingHorizontal: 10, paddingVertical: 4, borderRadius: 12 },
  badgeText: { fontSize: 12, fontWeight: '600' },
  chargerHeader: { fontSize: 13, fontWeight: '600', color: '#444444', marginTop: 14, marginBottom: 4 },
  emptyChargers: { fontSize: 13, color: '#999999', fontStyle: 'italic', paddingVertical: 8 },
  chargerRow: { flexDirection: 'row', justifyContent: 'space-between', paddingVertical: 4, borderBottomWidth: 0.5, borderBottomColor: '#EEEEEE' },
  chargerText: { fontSize: 13, color: '#333333' },
  chargerStatus: { fontSize: 12, fontWeight: '500' },
});
