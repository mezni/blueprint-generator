import { useQuery } from "@tanstack/react-query";
import { apiFetch } from "../../lib/apiClient";
import { quantizeBounds, insideViewport, pinColor } from "@bornemap/geo-models";
import type { MapViewportModel, StationMarkerModel } from "@bornemap/geo-models";
import { useEffect, useState, useCallback } from "react";

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

  const query = useQuery({
    queryKey,
    queryFn: async () => {
      if (!debouncedViewport) return null;
      const bbox = `${debouncedViewport.west},${debouncedViewport.south},${debouncedViewport.east},${debouncedViewport.north}`;
      return apiFetch<StationListResponse>(
        `/api/v1/stations?bbox=${bbox}`
      );
    },
    enabled: !!debouncedViewport,
  });

  const markers: StationMarkerModel[] = (query.data?.markers || [])
    .filter((m) => {
      if (!debouncedViewport) return true;
      return insideViewport(m.coord, debouncedViewport);
    })
    .map((m) => ({
      id: m.id,
      name: m.name,
      coord: m.coord,
      isActive: m.is_active,
      underMaintenance: m.under_maintenance,
    }));

  return {
    markers,
    truncated: query.data?.truncated ?? false,
    isLoading: query.isLoading,
  };
}
