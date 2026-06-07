import React from 'react';
import { StyleSheet, View, Text, ScrollView } from 'react-native';
import { useNavigation, useRoute } from '@react-navigation/native';
import { Ionicons } from '@expo/vector-icons';

export function StationDetailPage() {
  const navigation = useNavigation();
  const route = useRoute();

  const station = route.params as any;

  return (
    <ScrollView style={styles.container}>
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Ionicons name="arrow-back" size={24} color="#111827" />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Station Details</Text>
        <View style={styles.headerActions} />
      </View>

      {station && (
        <View style={styles.content}>
          <View style={styles.iconContainer}>
            <Ionicons name="car" size={32} color="#2563EB" />
          </View>
          <Text style={styles.stationName}>{station.name}</Text>
          <Text style={styles.stationDescription}>
            {station.description || 'No description available'}
          </Text>

          <View style={styles.infoCard}>
            <View style={styles.infoRow}>
              <Ionicons name="location-outline" size={20} color="#6B7280" />
              <Text style={styles.infoText}>
                {station.latitude.toFixed(4)}, {station.longitude.toFixed(4)}
              </Text>
            </View>

            <View style={styles.infoRow}>
              <Ionicons name="information-circle-outline" size={20} color="#6B7280" />
              <Text style={styles.infoText}>
                {station.is_live ? 'Live' : 'Offline'}
              </Text>
            </View>

            <View style={styles.infoRow}>
              <Ionicons name="business-outline" size={20} color="#6B7280" />
              <Text style={styles.infoText}>
                {station.is_public ? 'Public Station' : 'Private Station'}
              </Text>
            </View>
          </View>

          <View style={styles.chargerCard}>
            <Text style={styles.sectionTitle}>Chargers</Text>
            <Text style={styles.chargerCount}>
              {station.chargers?.length || 0} {station.chargers?.length === 1 ? 'charger' : 'chargers'} available
            </Text>
          </View>

          <View style={styles.mapCard}>
            <Text style={styles.sectionTitle}>Location</Text>
            <View style={styles.mapPlaceholder}>
              <Ionicons name="map" size={48} color="#D1D5DB" />
              <Text style={styles.mapPlaceholderText}>
                Map view would be shown here
              </Text>
            </View>
          </View>

          <View style={styles.actionsContainer}>
            <TouchableOpacity style={styles.actionButton}>
              <Ionicons name="share-outline" size={20} color="#FFFFFF" />
              <Text style={styles.actionButtonText}>Share</Text>
            </TouchableOpacity>

            <TouchableOpacity style={styles.actionButton}>
              <Ionicons name="navigate-outline" size={20} color="#FFFFFF" />
              <Text style={styles.actionButtonText}>Directions</Text>
            </TouchableOpacity>

            <TouchableOpacity style={styles.actionButton}>
              <Ionicons name="star-outline" size={20} color="#FFFFFF" />
              <Text style={styles.actionButtonText}>Rate</Text>
            </TouchableOpacity>
          </View>
        </View>
      )}
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFFFFF',
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: 16,
    paddingVertical: 16,
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  headerTitle: {
    fontSize: 18,
    fontWeight: '700',
    color: '#111827',
  },
  headerActions: {
    padding: 8,
  },
  content: {
    padding: 16,
  },
  iconContainer: {
    width: 64,
    height: 64,
    borderRadius: 16,
    backgroundColor: '#EFF6FF',
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: 12,
  },
  stationName: {
    fontSize: 18,
    fontWeight: '700',
    color: '#111827',
    marginBottom: 4,
    textAlign: 'center',
  },
  stationDescription: {
    fontSize: 14,
    color: '#6B7280',
    textAlign: 'center',
    marginBottom: 16,
  },
  infoCard: {
    padding: 16,
    backgroundColor: '#F9FAFB',
    borderRadius: 12,
    marginBottom: 16,
  },
  infoRow: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  infoText: {
    fontSize: 14,
    color: '#111827',
    marginLeft: 12,
  },
  chargerCard: {
    padding: 16,
    backgroundColor: '#F9FAFB',
    borderRadius: 12,
    marginBottom: 16,
  },
  sectionTitle: {
    fontSize: 16,
    fontWeight: '700',
    color: '#111827',
    marginBottom: 8,
  },
  chargerCount: {
    fontSize: 14,
    color: '#6B7280',
  },
  mapCard: {
    padding: 16,
    backgroundColor: '#F9FAFB',
    borderRadius: 12,
    marginBottom: 16,
  },
  mapPlaceholder: {
    height: 200,
    backgroundColor: '#E5E7EB',
    borderRadius: 8,
    justifyContent: 'center',
    alignItems: 'center',
    marginTop: 8,
  },
  mapPlaceholderText: {
    fontSize: 14,
    color: '#6B7280',
    marginTop: 8,
  },
  actionsContainer: {
    flexDirection: 'row',
    paddingHorizontal: 16,
    paddingVertical: 16,
    borderTopWidth: 1,
    borderTopColor: '#E5E7EB',
    marginBottom: 16,
  },
  actionButton: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: '#2563EB',
    padding: 12,
    borderRadius: 8,
    marginHorizontal: 8,
  },
  actionButtonText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#FFFFFF',
    marginLeft: 8,
  },
});
