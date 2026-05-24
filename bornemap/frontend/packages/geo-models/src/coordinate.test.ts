import { describe, it, expect } from "vitest";
import { coordinateSchema } from "./coordinate";

describe("coordinateSchema", () => {
  it("accepts valid [lng, lat]", () => {
    expect(coordinateSchema.parse([10.0, 36.0])).toEqual([10.0, 36.0]);
  });

  it("rejects single number", () => {
    expect(() => coordinateSchema.parse([10.0])).toThrow();
  });

  it("rejects three numbers", () => {
    expect(() => coordinateSchema.parse([10.0, 36.0, 0.0])).toThrow();
  });

  it("rejects non-number values", () => {
    expect(() => coordinateSchema.parse(["a", "b"])).toThrow();
  });

  it("rejects null", () => {
    expect(() => coordinateSchema.parse(null)).toThrow();
  });
});
