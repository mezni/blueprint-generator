import type { CoordinateModel } from "./coordinate";

export interface StationMarkerModel {
  id: string;
  name: string;
  coord: CoordinateModel;
  isActive: boolean;
  underMaintenance: boolean;
}

export interface BorneMapMapStyles {
  pins: {
    activeGreen: string;
    inactiveRed: string;
  };
  cluster: {
    radius: number;
    minMarkers: number;
  };
}

export const BORNE_MAP_STYLES: BorneMapMapStyles = {
  pins: {
    activeGreen: "#22c55e",
    inactiveRed: "#ef4444",
  },
  cluster: {
    radius: 40,
    minMarkers: 15,
  },
};

export function pinColor(marker: StationMarkerModel): string {
  if (marker.isActive && !marker.underMaintenance) {
    return BORNE_MAP_STYLES.pins.activeGreen;
  }
  return BORNE_MAP_STYLES.pins.inactiveRed;
}
