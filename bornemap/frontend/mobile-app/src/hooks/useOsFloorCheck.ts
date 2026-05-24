import { Platform } from "react-native";

const ANDROID_FLOOR = 29;
const IOS_FLOOR = 15;

export function useOsFloorCheck(): { isSupported: boolean } {
  const version = Number(Platform.Version);
  if (Platform.OS === "android") {
    return { isSupported: version >= ANDROID_FLOOR };
  }
  if (Platform.OS === "ios") {
    return { isSupported: version >= IOS_FLOOR };
  }
  return { isSupported: true };
}
