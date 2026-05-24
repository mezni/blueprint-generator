import { z } from "zod";
import type { CoordinateModel } from "./coordinate";

export interface MapViewportModel {
  west: number;
  south: number;
  east: number;
  north: number;
}

export const viewportSchema = z.object({
  west: z.number().min(-180).max(180),
  south: z.number().min(-90).max(90),
  east: z.number().min(-180).max(180),
  north: z.number().min(-90).max(90),
});

export function quantizeBounds(viewport: MapViewportModel): MapViewportModel {
  return {
    west: Math.round(viewport.west * 10_000) / 10_000,
    south: Math.round(viewport.south * 10_000) / 10_000,
    east: Math.round(viewport.east * 10_000) / 10_000,
    north: Math.round(viewport.north * 10_000) / 10_000,
  };
}

export function insideViewport(
  coord: CoordinateModel,
  viewport: MapViewportModel
): boolean {
  const [lng, lat] = coord;
  return (
    lng >= viewport.west &&
    lng <= viewport.east &&
    lat >= viewport.south &&
    lat <= viewport.north
  );
}
