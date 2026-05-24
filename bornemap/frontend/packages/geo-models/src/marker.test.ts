import { describe, it, expect } from "vitest";
import { pinColor, BORNE_MAP_STYLES } from "./marker";
import type { StationMarkerModel } from "./marker";

describe("pinColor", () => {
  const base: StationMarkerModel = {
    id: "1",
    name: "Test",
    coord: [10.0, 36.0],
    isActive: true,
    underMaintenance: false,
  };

  it("returns green for active, not in maintenance", () => {
    expect(pinColor(base)).toBe(BORNE_MAP_STYLES.pins.activeGreen);
  });

  it("returns red for inactive", () => {
    expect(pinColor({ ...base, isActive: false })).toBe(
      BORNE_MAP_STYLES.pins.inactiveRed,
    );
  });

  it("returns red for under maintenance even if active", () => {
    expect(pinColor({ ...base, underMaintenance: true })).toBe(
      BORNE_MAP_STYLES.pins.inactiveRed,
    );
  });

  it("returns red for inactive + under maintenance", () => {
    expect(
      pinColor({ ...base, isActive: false, underMaintenance: true }),
    ).toBe(BORNE_MAP_STYLES.pins.inactiveRed);
  });
});
