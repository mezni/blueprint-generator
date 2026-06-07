import React, { useState } from 'react';
import { View, Text, StyleSheet, FlatList, ActivityIndicator } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useStations } from '@/hooks/useStations';
import { StationCard } from '@/components/ui/StationCard';
import { useTheme } from '@/hooks/useTheme';
import { useAuth } from '@/hooks/useAuth';

export function DashboardPage() {
  const { mode } = useTheme();
  const { isAuthenticated } = useAuth();
  const { data: stations, isLoading, error } = useStations();
  const [selectedStation, setSelectedStation] = useState<any>(null);
  const [favorites, setFavorites] = useState<Set<string>>(new Set());
  const [showFavorites, setShowFavorites] = useState(false);

  const isRTL = mode === 'rtl';

  if (isLoading) {
    return (
      <View style={styles.container}>
        <ActivityIndicator size="large" color="#2563EB" />
      </View>
    );
  }

  if (error) {
    return (
      <View style={styles.container}>
        <Text style={styles.errorText}>Failed to load stations</Text>
      </View>
    );
  }

  if (!isAuthenticated) {
    return (
      <View style={styles.container}>
        <Text style={styles.placeholderText}>Please login to see stations</Text>
      </View>
    );
  }

  const displayedStations = showFavorites 
    ? stations?.filter(station => favorites.has(station.id))
    : stations;

  return (
    <View style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <View style={styles.titleContainer}>
          <Ionicons 
            name="map" 
            size={28} 
            color="#2563EB" 
            style={isRTL ? { transform: [{ scaleX: -1 }] } : {}} 
          />
          <View style={isRTL ? styles.titleTextRTL : styles.titleTextLTR}>
            <Text style={styles.headerTitle}>Map Discovery</Text>
            <Text style={styles.headerSubtitle}>
              {showFavorites 
                ? `Your ${favorites.size} favorite${favorites.size > 1 ? 's' : ''}`
                : `${stations?.length || 0} stations nearby`
              }
            </Text>
          </View>
        </View>

        <TouchableOpacity
          style={[styles.filterButton, showFavorites && styles.filterButtonActive]}
          onPress={() => setShowFavorites(!showFavorites)}
        >
          <Ionicons 
            name={showFavorites ? "heart" : "heart-outline"} 
            size={24} 
            color={showFavorites ? "#EF4444" : "#6B7280"} 
          />
        </TouchableOpacity>
      </View>

      {/* Stations List */}
      {displayedStations && displayedStations.length > 0 ? (
        <FlatList
          data={displayedStations}
          keyExtractor={(item) => item.id}
          renderItem={({ item }) => (
            <StationCard
              station={item}
              onPress={(station) => setSelectedStation(station)}
              onFavorite={(station) => {
                setFavorites(prev => {
                  const newFavorites = new Set(prev);
                  if (newFavorites.has(station.id)) {
                    newFavorites.delete(station.id);
                  } else {
                    newFavorites.add(station.id);
                  }
                  return newFavorites;
                });
              }}
              isFavorite={favorites.has(item.id)}
            />
          )}
          contentContainerStyle={styles.listContent}
          showsVerticalScrollIndicator={false}
        />
      ) : (
        <View style={styles.emptyContainer}>
          <Ionicons name="location-outline" size={64} color="#D1D5DB" />
          <Text style={styles.emptyTitle}>No stations found</Text>
          <Text style={styles.emptyText}>
            {showFavorites 
              ? "You don't have any favorite stations yet"
              : "No stations are available in your area"
            }
          </Text>
        </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#F3F4F6',
  },
  header: {
    backgroundColor: '#FFFFFF',
    paddingHorizontal: 16,
    paddingTop: 16,
    paddingBottom: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  titleContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  titleTextLTR: {
    marginLeft: 12,
  },
  titleTextRTL: {
    marginRight: 12,
  },
  headerTitle: {
    fontSize: 20,
    fontWeight: '700',
    color: '#111827',
  },
  headerSubtitle: {
    fontSize: 14,
    color: '#6B7280',
    marginTop: 2,
  },
  filterButton: {
    padding: 8,
    borderRadius: 8,
  },
  filterButtonActive: {
    backgroundColor: '#FEF2F2',
  },
  listContent: {
    padding: 16,
  },
  errorText: {
    fontSize: 16,
    color: '#EF4444',
    textAlign: 'center',
    marginTop: 40,
  },
  placeholderText: {
    fontSize: 16,
    color: '#6B7280',
    textAlign: 'center',
    marginTop: 40,
  },
  emptyContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 32,
  },
  emptyTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#111827',
    marginTop: 16,
    marginBottom: 8,
  },
  emptyText: {
    fontSize: 14,
    color: '#6B7280',
    textAlign: 'center',
  },
});
