import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { Ionicons } from '@expo/vector-icons';

interface StationCardProps {
  station: {
    id: string;
    name: string;
    description: string | null;
    status: string;
    is_live: boolean;
    is_public: boolean;
  };
  onPress?: (station: any) => void;
  onFavorite?: (station: any) => void;
  isFavorite?: boolean;
}

export function StationCard({ station, onPress, onFavorite, isFavorite }: StationCardProps) {
  return (
    <TouchableOpacity 
      style={styles.card}
      onPress={() => onPress?.(station)}
      activeOpacity={0.7}
    >
      <View style={styles.header}>
        <View style={styles.iconContainer}>
          <Ionicons name="car-outline" size={24} color="#2563EB" />
        </View>
        <View style={styles.titleContainer}>
          <Text style={styles.title} numberOfLines={1}>{station.name}</Text>
          {station.is_live ? (
            <View style={styles.statusBadge}>
              <Text style={styles.statusText}>LIVE</Text>
            </View>
          ) : (
            <View style={[styles.statusBadge, styles.offlineBadge]}>
              <Text style={styles.statusText}>OFFLINE</Text>
            </View>
          )}
        </View>
      </View>
      
      {station.description && (
        <Text style={styles.description} numberOfLines={2}>
          {station.description}
        </Text>
      )}
      
      <View style={styles.footer}>
        <View style={styles.venueType}>
          <Ionicons name="business-outline" size={16} color="#6B7280" />
          <Text style={styles.venueTypeText}>Public Station</Text>
        </View>
      </View>

      {onFavorite && (
        <TouchableOpacity
          style={[styles.favoriteButton, isFavorite && styles.favoriteButtonActive]}
          onPress={(e) => {
            e.stopPropagation();
            onFavorite(station);
          }}
        >
          <Ionicons
            name={isFavorite ? "heart" : "heart-outline"}
            size={24}
            color={isFavorite ? "#EF4444" : "#6B7280"}
          />
        </TouchableOpacity>
      )}
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  card: {
    backgroundColor: '#FFFFFF',
    borderRadius: 8,
    padding: 16,
    marginBottom: 12,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 2,
    position: 'relative',
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 8,
  },
  iconContainer: {
    width: 48,
    height: 48,
    borderRadius: 12,
    backgroundColor: '#EFF6FF',
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: 12,
  },
  titleContainer: {
    flex: 1,
  },
  title: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 4,
  },
  statusBadge: {
    backgroundColor: '#D1FAE5',
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 4,
  },
  statusText: {
    fontSize: 10,
    fontWeight: '600',
    color: '#059669',
  },
  offlineBadge: {
    backgroundColor: '#FEE2E2',
  },
  offlineBadgeText: {
    color: '#DC2626',
  },
  description: {
    fontSize: 14,
    color: '#6B7280',
    marginBottom: 12,
    lineHeight: 20,
  },
  footer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  venueType: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  venueTypeText: {
    fontSize: 12,
    color: '#6B7280',
    marginLeft: 4,
  },
  favoriteButton: {
    position: 'absolute',
    top: 16,
    right: 16,
    padding: 8,
  },
  favoriteButtonActive: {
    color: '#EF4444',
  },
});
