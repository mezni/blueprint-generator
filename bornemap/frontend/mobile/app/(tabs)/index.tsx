import { View, Text, StyleSheet } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

export default function MapTab() {
  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.mapPlaceholder}>
        <Text style={styles.placeholderText}>Map View</Text>
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "#f9fafb",
  },
  mapPlaceholder: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    borderWidth: 2,
    borderColor: "#22c55e",
    borderRadius: 16,
    margin: 8,
  },
  placeholderText: {
    fontSize: 18,
    fontWeight: "600",
    color: "#6b7280",
  },
});
