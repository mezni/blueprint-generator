import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { get } from "../../lib/api";
import type { Charger, ChargerCreate, ChargerUpdate } from "./types";

const BASE = "/api/v1/chargers";

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

export function useChargers() {
  return useQuery<Charger[]>({ queryKey: ["chargers"], queryFn: () => get<Charger[]>(`${BASE}`) });
}

export function useCreateCharger() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (data: ChargerCreate) => post<Charger>(`${BASE}`, data), onSuccess: () => qc.invalidateQueries({ queryKey: ["chargers"] }) });
}

export function useUpdateCharger() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: ({ id, data }: { id: number; data: ChargerUpdate }) => patch<Charger>(`${BASE}/${id}`, data), onSuccess: () => qc.invalidateQueries({ queryKey: ["chargers"] }) });
}

export function useToggleChargerActive() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (id: number) => patch<Charger>(`${BASE}/${id}/toggle-active`, {}), onSuccess: () => qc.invalidateQueries({ queryKey: ["chargers"] }) });
}

export function useDeleteCharger() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (id: number) => del(`${BASE}/${id}`), onSuccess: () => qc.invalidateQueries({ queryKey: ["chargers"] }) });
}
