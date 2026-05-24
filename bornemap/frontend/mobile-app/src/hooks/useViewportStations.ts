import { useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { quantizeBounds } from "@bornemap/geo-models";
import type { MapViewportModel } from "@bornemap/geo-models";
import { apiFetch } from "../lib/apiClient";

interface ViewportStation {
  id: string;
  name: string;
  coord: [number, number];
  is_active: boolean;
  under_maintenance: boolean;
}

interface StationListResponse {
  viewport: { west: number; south: number; east: number; north: number };
  markers: ViewportStation[];
  truncated: boolean;
}

export function useViewportStations(viewport: MapViewportModel | null) {
  const [debouncedViewport, setDebouncedViewport] =
    useState<MapViewportModel | null>(null);

  useEffect(() => {
    if (!viewport) return;
    const timer = setTimeout(() => {
      setDebouncedViewport(quantizeBounds(viewport));
    }, 300);
    return () => clearTimeout(timer);
  }, [viewport]);

  const queryKey = debouncedViewport
    ? [
        "stations",
        `${debouncedViewport.west},${debouncedViewport.south},${debouncedViewport.east},${debouncedViewport.north}`,
      ]
    : ["stations", "none"];

  return useQuery({
    queryKey,
    queryFn: async () => {
      if (!debouncedViewport) return null;
      const bbox = `${debouncedViewport.west},${debouncedViewport.south},${debouncedViewport.east},${debouncedViewport.north}`;
      return apiFetch<StationListResponse>(`/api/v1/stations?bbox=${bbox}`);
    },
    enabled: !!debouncedViewport,
  });
}
