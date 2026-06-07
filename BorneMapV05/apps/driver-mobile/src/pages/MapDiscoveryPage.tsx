import React, { useEffect, useRef, useState } from 'react';
import MapView, { Marker, Callout, Region } from 'react-native-maps';
import { View, Text, StyleSheet, TouchableOpacity, ActivityIndicator } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useStations } from '@/hooks/useStations';
import { StationCard } from '@/components/ui/StationCard';
import { useAuth } from '@/hooks/useAuth';
import { formatDistance } from '@/utils/rtl-utils';

export function MapDiscoveryPage() {
  const { isAuthenticated } = useAuth();
  const { data: stations, isLoading } = useStations();
  const [region, setRegion] = useState<Region>({
    latitude: 36.8065,
    longitude: 10.1815,
    latitudeDelta: 0.1,
    longitudeDelta: 0.1,
  });
  const [selectedStation, setSelectedStation] = useState<any>(null);
  const [showFavorites, setShowFavorites] = useState(false);
  const mapRef = useRef<MapView>(null);
  const [showCallout, setShowCallout] = useState(false);
  const [markerPositions, setMarkerPositions] = useState<Region[]>([]);

  // Update marker positions when stations change
  useEffect(() => {
    if (stations && stations.length > 0) {
      const positions = stations.map(station => ({
        latitude: station.latitude,
        longitude: station.longitude,
        latitudeDelta: 0.0001,
        longitudeDelta: 0.0001,
      }));
      setMarkerPositions(positions);
    }
  }, [stations]);

  // Focus map on user location when available
  useEffect(() => {
    if (region && mapRef.current) {
      mapRef.current.animateToRegion(region, 1000);
    }
  }, [region]);

  if (isLoading) {
    return (
      <View style={styles.container}>
        <ActivityIndicator size="large" color="#2563EB" />
      </View>
    );
  }

  if (!isAuthenticated) {
    return (
      <View style={styles.container}>
        <Text style={styles.placeholderText}>Please login to view stations on map</Text>
      </View>
    );
  }

  const displayedStations = showFavorites
    ? stations?.filter(station => favorites.has(station.id))
    : stations;

  return (
    <View style={styles.container}>
      {/* Map View */}
      <MapView
        ref={mapRef}
        style={styles.map}
        region={region}
        onRegionChangeComplete={setRegion}
        showsUserLocation={true}
        showsMyLocationButton={true}
        onLongPress={(e) => {
          const { latitude, longitude } = e.nativeEvent.coordinate;
          setRegion({
            latitude,
            longitude,
            latitudeDelta: 0.1,
            longitudeDelta: 0.1,
          });
        }}
      >
        {markers}
      </MapView>

      {/* Floating Header */}
      <View style={styles.header}>
        <View style={styles.headerContent}>
          <View style={styles.titleContainer}>
            <Ionicons name="map" size={28} color="#2563EB" />
            <View style={styles.titleText}>
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
      </View>

      {/* Station Cards Below Map */}
      {displayedStations && displayedStations.length > 0 ? (
        <View style={styles.listContainer}>
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
        </View>
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
  map: {
    flex: 1,
  },
  header: {
    position: 'absolute',
    top: 16,
    left: 16,
    right: 16,
    zIndex: 10,
  },
  headerContent: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    backgroundColor: 'rgba(255, 255, 255, 0.95)',
    padding: 12,
    borderRadius: 12,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  titleContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  titleText: {
    marginLeft: 12,
  },
  headerTitle: {
    fontSize: 16,
    fontWeight: '700',
    color: '#111827',
  },
  headerSubtitle: {
    fontSize: 12,
    color: '#6B7280',
    marginTop: 1,
  },
  filterButton: {
    padding: 8,
    borderRadius: 8,
  },
  filterButtonActive: {
    backgroundColor: '#FEF2F2',
  },
  listContainer: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    backgroundColor: '#FFFFFF',
    borderTopLeftRadius: 16,
    borderTopRightRadius: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: -2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
    maxHeight: '70%',
  },
  listContent: {
    padding: 16,
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
    marginTop: 100,
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
