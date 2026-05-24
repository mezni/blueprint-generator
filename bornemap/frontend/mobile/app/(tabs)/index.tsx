import { View, Text, StyleSheet } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { Ionicons } from "@expo/vector-icons";
import BorneMapView from "../../components/map/MapView";

export default function MapTab() {
  return (
    <View style={styles.container}>
      <BorneMapView />

      <SafeAreaView style={styles.headerOverlay} edges={["top"]}>
        <View style={styles.header}>
          <View style={styles.logoRow}>
            <View style={styles.logo}>
              <Text style={styles.logoText}>BM</Text>
            </View>
            <Text style={styles.title}>BorneMap</Text>
          </View>
        </View>
      </SafeAreaView>

      <SafeAreaView style={styles.searchOverlay} edges={["top"]}>
        <View style={styles.searchBar}>
          <Ionicons name="search" size={16} color="#6b7280" />
          <Text style={styles.searchText}>Search stations...</Text>
        </View>
      </SafeAreaView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  headerOverlay: {
    position: "absolute",
    top: 0,
    left: 0,
    right: 0,
    alignItems: "center",
    paddingHorizontal: 16,
    paddingTop: 8,
  },
  header: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    width: "100%",
  },
  logoRow: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
    backgroundColor: "rgba(255,255,255,0.92)",
    paddingHorizontal: 12,
    paddingVertical: 8,
    borderRadius: 12,
    borderWidth: 1,
    borderColor: "rgba(255,255,255,0.6)",
  },
  logo: {
    width: 24,
    height: 24,
    borderRadius: 6,
    backgroundColor: "#22c55e",
    alignItems: "center",
    justifyContent: "center",
  },
  logoText: {
    fontSize: 10,
    fontWeight: "700",
    color: "#ffffff",
  },
  title: {
    fontSize: 14,
    fontWeight: "600",
    color: "#0a0a0a",
  },
  searchOverlay: {
    position: "absolute",
    top: 64,
    left: 0,
    right: 0,
    paddingHorizontal: 16,
  },
  searchBar: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
    backgroundColor: "rgba(255,255,255,0.92)",
    paddingHorizontal: 12,
    paddingVertical: 10,
    borderRadius: 12,
    borderWidth: 1,
    borderColor: "rgba(255,255,255,0.6)",
  },
  searchText: {
    fontSize: 13,
    color: "#6b7280",
  },
});
