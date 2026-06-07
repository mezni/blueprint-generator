import { Text, View } from "react-native";
import type { SuccessEnvelope } from "@bornemap/api-contracts";

export default function App() {
  const _verify: SuccessEnvelope | null = null;
  return (
    <View style={{ flex: 1, justifyContent: "center", alignItems: "center" }}>
      <Text>driver-mobile {_verify !== null ? "?" : ""}</Text>
    </View>
  );
}
