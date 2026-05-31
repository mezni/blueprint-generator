import React from "react";
import MapView, { PROVIDER_DEFAULT } from "react-native-maps";
import { StyleSheet } from "react-native";

interface MapViewProps {
  onMapPress?: () => void;
}

const BorneMapView = React.forwardRef<MapView, MapViewProps>((_props, ref) => {
  return (
    <MapView
      ref={ref}
      style={StyleSheet.absoluteFillObject}
      provider={PROVIDER_DEFAULT}
      initialRegion={{
        latitude: 33.8869,
        longitude: 9.5375,
        latitudeDelta: 10,
        longitudeDelta: 10,
      }}
      toolbarEnabled={false}
    />
  );
});

BorneMapView.displayName = "BorneMapView";

export default BorneMapView;
