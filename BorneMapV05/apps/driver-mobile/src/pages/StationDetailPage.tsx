import React from 'react';
import { View, Text, StyleSheet, ScrollView, TouchableOpacity, ActivityIndicator } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useRoute, useNavigation } from '@react-navigation/native';
import { useStations } from '@/hooks/useStations';
import { useAuth } from '@/hooks/useAuth';

export function StationDetailPage() {
  const route = useRoute();
  const navigation = useNavigation();
  const { isAuthenticated } = useAuth();
  const { stations, isLoading } = useStations();
  
  const station = stations?.find((s: any) => s.id === route.params?.id);

  if (isLoading) {
    return (
      <View style={styles.container}>
        <ActivityIndicator size="large" color="#2563EB" />
      </View>
    );
  }

  if (!station) {
    return (
      <View style={styles.container}>
        <Text style={styles.errorText}>Station not found</Text>
      </View>
    );
  }

  return (
    <ScrollView style={styles.container} contentContainerStyle={styles.contentContainer}>
      {/* Header */}
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Ionicons name="arrow-back" size={24} color="#111827" />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>{station.name}</Text>
        <View style={styles.headerActions}>
          {station.is_live ? (
            <Ionicons name="wifi-outline" size={24} color="#10B981" />
          ) : (
            <Ionicons name="wifi-slash-outline" size={24} color="#EF4444" />
          )}
        </View>
      </View>

      {/* Station Info */}
      <View style={styles.infoCard}>
        <View style={styles.infoRow}>
          <Ionicons name="location-outline" size={20} color="#6B7280" />
          <View style={styles.infoContent}>
            <Text style={styles.infoLabel}>Location</Text>
            <Text style={styles.infoValue}>
              {station.latitude.toFixed(4)}, {station.longitude.toFixed(4)}
            </Text>
          </View>
        </View>

        <View style={styles.infoRow}>
          <Ionicons name="information-circle-outline" size={20} color="#6B7280" />
          <View style={styles.infoContent}>
            <Text style={styles.infoLabel}>Status</Text>
            <Text style={[styles.infoValue, station.is_live ? styles.statusLive : styles.statusOffline]}>
              {station.is_live ? 'Live' : 'Offline'}
            </Text>
          </View>
        </View>

        <View style={styles.infoRow}>
          <Ionicons name="business-outline" size={20} color="#6B7280" />
          <View style={styles.infoContent}>
            <Text style={styles.infoLabel}>Venue Type</Text>
            <Text style={styles.infoValue}>{station.is_public ? 'Public Station' : 'Private Station'}</Text>
          </View>
        </View>
      </View>

      {/* Description */}
      {station.description && (
        <View style={styles.descriptionCard}>
          <Text style={styles.sectionTitle}>About</Text>
          <Text style={styles.descriptionText}>{station.description}</Text>
        </View>
      )}

      {/* Charger Info */}
      <View style={styles.chargerCard}>
        <Text style={styles.sectionTitle}>Chargers</Text>
        <Text style={styles.chargerCount}>
          {station.chargers.length} {station.chargers.length === 1 ? 'charger' : 'chargers'} available
        </Text>
      </View>

      {/* Map Preview */}
      <View style={styles.mapCard}>
        <Text style={styles.sectionTitle}>Location</Text>
        <View style={styles.mapPlaceholder}>
          <Ionicons name="map" size={48} color="#D1D5DB" />
          <Text style={styles.mapPlaceholderText}>
            Map view would be shown here
          </Text>
        </View>
      </View>

      {/* Actions */}
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
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFFFFF',
  },
  contentContainer: {
    paddingBottom: 100,
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
  infoCard: {
    padding: 16,
    backgroundColor: '#F9FAFB',
    margin: 16,
    borderRadius: 12,
    borderLeftWidth: 4,
    borderLeftColor: '#2563EB',
  },
  infoRow: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  infoContent: {
    marginLeft: 12,
    flex: 1,
  },
  infoLabel: {
    fontSize: 12,
    color: '#6B7280',
    marginBottom: 2,
  },
  infoValue: {
    fontSize: 14,
    fontWeight: '500',
    color: '#111827',
  },
  statusLive: {
    color: '#10B981',
  },
  statusOffline: {
    color: '#EF4444',
  },
  descriptionCard: {
    padding: 16,
    margin: 16,
    backgroundColor: '#F9FAFB',
    borderRadius: 12,
  },
  sectionTitle: {
    fontSize: 16,
    fontWeight: '700',
    color: '#111827',
    marginBottom: 8,
  },
  descriptionText: {
    fontSize: 14,
    color: '#6B7280',
    lineHeight: 20,
  },
  chargerCard: {
    padding: 16,
    margin: 16,
    backgroundColor: '#F9FAFB',
    borderRadius: 12,
  },
  chargerCount: {
    fontSize: 14,
    color: '#6B7280',
  },
  mapCard: {
    padding: 16,
    margin: 16,
    backgroundColor: '#F9FAFB',
    borderRadius: 12,
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
    marginTop: 8,
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
  errorText: {
    fontSize: 16,
    color: '#EF4444',
    textAlign: 'center',
    marginTop: 40,
  },
});
