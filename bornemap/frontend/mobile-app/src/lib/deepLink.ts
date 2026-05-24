import { Linking } from "react-native";
import type { CoordinateModel } from "@bornemap/geo-models";

export function buildNavigateUrl(coord: CoordinateModel): string {
  const [, lat] = coord;
  const [lng] = coord;
  return `https://www.google.com/maps/dir/?api=1&destination=${lat},${lng}`;
}

export async function openNavigate(coord: CoordinateModel): Promise<void> {
  const url = buildNavigateUrl(coord);
  try {
    await Linking.openURL(url);
  } catch {
    console.warn("Failed to open navigation URL:", url);
  }
}
