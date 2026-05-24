import { z } from "zod";

export type CoordinateModel = readonly [number, number];

export const coordinateSchema = z.tuple([z.number(), z.number()]);
