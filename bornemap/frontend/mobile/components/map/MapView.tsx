import { View, StyleSheet } from "react-native";
import React from "react";

type MapViewProps = {
  // Forward ref for snap-sheet compatibility (Phase 2 needs bottomSheetRef)
};

const MapView = React.forwardRef<View, MapViewProps>((_props, ref) => {
  return (
    <View
      ref={ref}
      style={styles.container}
    />
  );
});

MapView.displayName = "MapView";

export default MapView;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    borderWidth: 2,
    borderColor: "#22c55e",
    borderRadius: 16,
  },
});
