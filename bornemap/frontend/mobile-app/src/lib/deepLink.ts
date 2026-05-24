import { Linking } from "react-native";
import type { CoordinateModel } from "@bornemap/geo-models";

export function buildNavigateUrl(coord: CoordinateModel): string {
  const [, lat] = coord;
  const [lng] = coord;
  return `https://www.google.com/maps/dir/?api=1&destination=${lat},${lng}`;
}

export function openNavigate(coord: CoordinateModel): void {
  const url = buildNavigateUrl(coord);
  Linking.openURL(url);
}
