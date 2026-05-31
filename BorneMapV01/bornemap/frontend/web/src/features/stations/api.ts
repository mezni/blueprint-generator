import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { get, post, patch, del } from "../../lib/api";
import { getToken } from "../../lib/auth";
import type { Station, StationCreate, StationUpdate } from "./types";

const BASE = "/api/v1/stations";

export function useStations() {
  return useQuery<Station[]>({ queryKey: ["stations"], queryFn: () => get<Station[]>(`${BASE}`, getToken()) });
}

export function useStationsByViewport(sw_lat: number, sw_lng: number, ne_lat: number, ne_lng: number) {
  return useQuery<Station[]>({
    queryKey: ["stations", "viewport", sw_lat, sw_lng, ne_lat, ne_lng],
    queryFn: () => get<Station[]>(`${BASE}?sw_lat=${sw_lat}&sw_lng=${sw_lng}&ne_lat=${ne_lat}&ne_lng=${ne_lng}`, getToken()),
    enabled: false,
  });
}

export function useNearbyStations(lat: number, lng: number, radius_m = 20000) {
  return useQuery<Station[]>({
    queryKey: ["stations", "nearby", lat, lng, radius_m],
    queryFn: () => get<Station[]>(`${BASE}/nearby?lat=${lat}&lng=${lng}&radius_m=${radius_m}`, getToken()),
    enabled: false,
  });
}

export function useCreateStation() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: StationCreate) => post<Station>(`${BASE}`, data, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["stations"] }),
  });
}

export function useUpdateStation() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: StationUpdate }) =>
      patch<Station>(`${BASE}/${id}`, data, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["stations"] }),
  });
}

export function useToggleStationActive() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => patch<Station>(`${BASE}/${id}/toggle-active`, {}, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["stations"] }),
  });
}

export function useDeleteStation() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => del(`${BASE}/${id}`, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["stations"] }),
  });
}
