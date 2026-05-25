import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { get } from "../../lib/api";
import type { Station, StationCreate, StationUpdate } from "./types";

const BASE = "/api/v1/stations";

async function post<T>(path: string, body: unknown): Promise<T> {
  const res = await fetch(`http://localhost:8000${path}`, {
    method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(body),
  });
  if (!res.ok) throw new Error((await res.json().catch(() => ({}))).detail ?? `POST ${path} failed`);
  return res.json();
}

async function patch<T>(path: string, body: unknown): Promise<T> {
  const res = await fetch(`http://localhost:8000${path}`, {
    method: "PATCH", headers: { "Content-Type": "application/json" }, body: JSON.stringify(body),
  });
  if (!res.ok) throw new Error((await res.json().catch(() => ({}))).detail ?? `PATCH ${path} failed`);
  return res.json();
}

async function del(path: string): Promise<void> {
  const res = await fetch(`http://localhost:8000${path}`, { method: "DELETE" });
  if (!res.ok) throw new Error((await res.json().catch(() => ({}))).detail ?? `DELETE ${path} failed`);
}

export function useStations() {
  return useQuery<Station[]>({ queryKey: ["stations"], queryFn: () => get<Station[]>(`${BASE}`) });
}

export function useCreateStation() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (data: StationCreate) => post<Station>(`${BASE}`, data), onSuccess: () => qc.invalidateQueries({ queryKey: ["stations"] }) });
}

export function useUpdateStation() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: ({ id, data }: { id: number; data: StationUpdate }) => patch<Station>(`${BASE}/${id}`, data), onSuccess: () => qc.invalidateQueries({ queryKey: ["stations"] }) });
}

export function useToggleStationActive() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (id: number) => patch<Station>(`${BASE}/${id}/toggle-active`, {}), onSuccess: () => qc.invalidateQueries({ queryKey: ["stations"] }) });
}

export function useDeleteStation() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (id: number) => del(`${BASE}/${id}`), onSuccess: () => qc.invalidateQueries({ queryKey: ["stations"] }) });
}
