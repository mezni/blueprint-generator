import { describe, it, expect } from "vitest";

describe("MapContainer performance benchmark", () => {
  it("validates MapContainer component exists", async () => {
    const { MapContainer } = await import("../map-container");
    expect(MapContainer).toBeDefined();
    expect(MapContainer.displayName).toBe("MapContainer");
  });

  it("measures module import time under 500ms", async () => {
    const start = performance.now();
    await import("../map-container");
    const duration = performance.now() - start;
    expect(duration).toBeLessThan(500);
  });
});
