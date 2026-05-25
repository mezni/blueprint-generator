import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { get } from "../../lib/api";
import type { Partner, PartnerCreate, PartnerUpdate } from "./types";

const BASE = "/api/v1/partners";

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

export function usePartners() {
  return useQuery<Partner[]>({ queryKey: ["partners"], queryFn: () => get<Partner[]>(`${BASE}`) });
}

export function useCreatePartner() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (data: PartnerCreate) => post<Partner>(`${BASE}`, data), onSuccess: () => qc.invalidateQueries({ queryKey: ["partners"] }) });
}

export function useUpdatePartner() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: ({ id, data }: { id: number; data: PartnerUpdate }) => patch<Partner>(`${BASE}/${id}`, data), onSuccess: () => qc.invalidateQueries({ queryKey: ["partners"] }) });
}

export function useTogglePartnerActive() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (id: number) => patch<Partner>(`${BASE}/${id}/toggle-active`, {}), onSuccess: () => qc.invalidateQueries({ queryKey: ["partners"] }) });
}

export function useDeletePartner() {
  const qc = useQueryClient();
  return useMutation({ mutationFn: (id: number) => del(`${BASE}/${id}`), onSuccess: () => qc.invalidateQueries({ queryKey: ["partners"] }) });
}
