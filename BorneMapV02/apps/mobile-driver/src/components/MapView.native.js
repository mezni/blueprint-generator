import RNMapView, { Marker, PROVIDER_DEFAULT } from 'react-native-maps';
import { StyleSheet } from 'react-native';

export default function MapView({ stations, onMarkerPress, initialRegion, style }) {
  return (
    <RNMapView
      provider={PROVIDER_DEFAULT}
      style={[styles.map, style]}
      initialRegion={initialRegion}
    >
      {stations.map((s) => (
        <Marker
          key={s.id}
          coordinate={{ latitude: s.latitude, longitude: s.longitude }}
          onPress={() => onMarkerPress(s)}
          pinColor={!s.is_live ? '#FF9800' : s.status === 'Available' ? '#4CAF50' : '#F44336'}
        />
      ))}
    </RNMapView>
  );
}

const styles = StyleSheet.create({
  map: { ...StyleSheet.absoluteFillObject },
});
