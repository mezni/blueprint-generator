import { Component, type ReactNode } from "react";
import { View, Text, StyleSheet } from "react-native";

interface Props { children: ReactNode; }
interface State { hasError: boolean; error: Error | null; }

export class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, error: null };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  render() {
    if (this.state.hasError) {
      return (
        <View style={styles.container}>
          <Text style={styles.title}>Something went wrong</Text>
          <Text style={styles.message}>An unexpected error occurred. Please restart the app.</Text>
        </View>
      );
    }
    return this.props.children;
  }
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: "center", alignItems: "center", backgroundColor: "#f9fafb", padding: 24 },
  title: { fontSize: 24, fontWeight: "bold", marginBottom: 12, color: "#dc2626" },
  message: { fontSize: 16, color: "#6b7280", textAlign: "center" },
});
