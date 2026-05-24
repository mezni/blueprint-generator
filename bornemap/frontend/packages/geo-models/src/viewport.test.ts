import { describe, it, expect } from "vitest";
import { quantizeBounds, insideViewport } from "./viewport";
import type { MapViewportModel } from "./viewport";

describe("quantizeBounds", () => {
  it("rounds to 4 decimal places", () => {
    const input: MapViewportModel = {
      west: 9.12345,
      south: 33.67891,
      east: 12.11115,
      north: 38.22229,
    };
    const result = quantizeBounds(input);
    expect(result.west).toBe(9.1235);
    expect(result.south).toBe(33.6789);
    expect(result.east).toBe(12.1112);
    expect(result.north).toBe(38.2223);
  });

  it("handles negative values", () => {
    const input: MapViewportModel = {
      west: -10.12345,
      south: -33.67891,
      east: -8.11111,
      north: -30.22222,
    };
    const result = quantizeBounds(input);
    expect(result.west).toBe(-10.1234);
    expect(result.south).toBe(-33.6789);
    expect(result.east).toBe(-8.1111);
    expect(result.north).toBe(-30.2222);
  });

  it("handles zero values", () => {
    const input: MapViewportModel = {
      west: 0,
      south: 0,
      east: 0,
      north: 0,
    };
    const result = quantizeBounds(input);
    expect(result.west).toBe(0);
    expect(result.south).toBe(0);
    expect(result.east).toBe(0);
    expect(result.north).toBe(0);
  });
});

describe("insideViewport", () => {
  const viewport: MapViewportModel = {
    west: 9.0,
    south: 33.0,
    east: 12.0,
    north: 38.0,
  };

  it("returns true for a coord inside the viewport", () => {
    expect(insideViewport([10.0, 36.0], viewport)).toBe(true);
  });

  it("returns true for a coord on the west edge", () => {
    expect(insideViewport([9.0, 36.0], viewport)).toBe(true);
  });

  it("returns true for a coord on the east edge", () => {
    expect(insideViewport([12.0, 36.0], viewport)).toBe(true);
  });

  it("returns false for a coord west of the viewport", () => {
    expect(insideViewport([8.9, 36.0], viewport)).toBe(false);
  });

  it("returns false for a coord east of the viewport", () => {
    expect(insideViewport([12.1, 36.0], viewport)).toBe(false);
  });

  it("returns false for a coord south of the viewport", () => {
    expect(insideViewport([10.0, 32.9], viewport)).toBe(false);
  });

  it("returns false for a coord north of the viewport", () => {
    expect(insideViewport([10.0, 38.1], viewport)).toBe(false);
  });
});
