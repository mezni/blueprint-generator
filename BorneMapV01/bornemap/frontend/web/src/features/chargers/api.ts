import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { get, post, patch, del } from "../../lib/api";
import { getToken } from "../../lib/auth";
import type { Charger, ChargerCreate, ChargerUpdate } from "./types";

const BASE = "/api/v1/chargers";

export function useChargers() {
  return useQuery<Charger[]>({ queryKey: ["chargers"], queryFn: () => get<Charger[]>(`${BASE}`, getToken()) });
}

export function useCreateCharger() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: ChargerCreate) => post<Charger>(`${BASE}`, data, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["chargers"] }),
  });
}

export function useUpdateCharger() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: ChargerUpdate }) =>
      patch<Charger>(`${BASE}/${id}`, data, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["chargers"] }),
  });
}

export function useToggleChargerActive() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => patch<Charger>(`${BASE}/${id}/toggle-active`, {}, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["chargers"] }),
  });
}

export function useDeleteCharger() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => del(`${BASE}/${id}`, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["chargers"] }),
  });
}
