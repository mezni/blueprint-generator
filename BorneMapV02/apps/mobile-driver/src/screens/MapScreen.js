import React, { useState } from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';
import MapView from '../components/MapView';
import ZoomControls from '../components/ZoomControls';
import StationDetailSheet from '../components/StationDetailSheet';
import mockStations from '../data/mockData';

const TUNISIA_CENTER = {
  latitude: 36.8065,
  longitude: 10.1815,
  latitudeDelta: 0.08,
  longitudeDelta: 0.04,
};

export default function MapScreen() {
  const stations = mockStations;
  const [selectedStation, setSelectedStation] = useState(null);
  const [sheetMode, setSheetMode] = useState('closed');

  const handleMarkerPress = (station) => {
    setSelectedStation(station);
    setSheetMode('peek');
  };

  const closeSheet = () => {
    setSelectedStation(null);
    setSheetMode('closed');
  };

  return (
    <View style={{ flex: 1 }}>
      <View style={styles.floatingHeader}>
        <Text style={styles.brandText}>BorneMap</Text>
        <TouchableOpacity style={styles.registerCapsule}>
          <Text style={styles.registerText}>REGISTER</Text>
        </TouchableOpacity>
      </View>

      <MapView
        style={StyleSheet.absoluteFillObject}
        initialRegion={TUNISIA_CENTER}
        stations={stations}
        onMarkerPress={handleMarkerPress}
      />

      <ZoomControls
        onZoomIn={() => {}}
        onZoomOut={() => {}}
        onLocateMe={() => {}}
        locationDisabled={false}
      />

      <StationDetailSheet
        station={selectedStation}
        isLoading={false}
        error={null}
        sheetMode={sheetMode}
        setSheetMode={setSheetMode}
        onClose={closeSheet}
        onRetry={() => {}}
        onNavigate={() => {}}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  floatingHeader: {
    position: 'absolute',
    top: 50,
    left: 16,
    right: 16,
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    zIndex: 50,
  },
  brandText: {
    fontSize: 20,
    fontWeight: '800',
    color: '#FFFFFF',
    textShadowColor: 'rgba(0,0,0,0.5)',
    textShadowOffset: { width: 0, height: 1 },
    textShadowRadius: 4,
  },
  registerCapsule: {
    backgroundColor: '#00B653',
    borderRadius: 20,
    paddingHorizontal: 16,
    paddingVertical: 8,
  },
  registerText: {
    color: '#FFFFFF',
    fontSize: 13,
    fontWeight: '700',
  },
});
