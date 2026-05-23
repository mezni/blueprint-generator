import React, { useState } from "react";
import { View, StyleSheet } from "react-native";
import MapView, { Marker } from "react-native-maps";
import { pinColor } from "@bornemap/geo-models";
import type { MapViewportModel, StationMarkerModel } from "@bornemap/geo-models";
import { useViewportStations } from "../hooks/useViewportStations";

const TUNISIA_CENTER = { latitude: 36.8, longitude: 10.2 };

interface RawMarker {
  id: string;
  name: string;
  coord: [number, number];
  is_active: boolean;
  under_maintenance: boolean;
}

export function MapScreen() {
  const [viewport, setViewport] = useState<MapViewportModel | null>(null);
  const { data } = useViewportStations(viewport);

  const markers: StationMarkerModel[] = (data?.markers || []).map(
    (m: RawMarker) => ({
      id: m.id,
      name: m.name,
      coord: m.coord,
      isActive: m.is_active,
      underMaintenance: m.under_maintenance,
    })
  );

  return (
    <View style={styles.container}>
      <MapView
        style={styles.map}
        initialRegion={{
          ...TUNISIA_CENTER,
          latitudeDelta: 3,
          longitudeDelta: 3,
        }}
        onRegionChangeComplete={(region) => {
          setViewport({
            west: region.longitude - region.longitudeDelta / 2,
            south: region.latitude - region.latitudeDelta / 2,
            east: region.longitude + region.longitudeDelta / 2,
            north: region.latitude + region.latitudeDelta / 2,
          });
        }}
      >
        {markers.map((m) => (
          <Marker
            key={m.id}
            coordinate={{ latitude: m.coord[1], longitude: m.coord[0] }}
            title={m.name}
            pinColor={pinColor(m)}
            tracksViewChanges={false}
          />
        ))}
      </MapView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1 },
  map: { flex: 1 },
});
