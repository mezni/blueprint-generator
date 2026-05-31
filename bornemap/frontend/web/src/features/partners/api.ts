import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { get, post, patch, del } from "../../lib/api";
import { getToken } from "../../lib/auth";
import type { Partner, PartnerCreate, PartnerUpdate } from "./types";

const BASE = "/api/v1/partners";

export function usePartners() {
  return useQuery<Partner[]>({ queryKey: ["partners"], queryFn: () => get<Partner[]>(`${BASE}`, getToken()) });
}

export function useCreatePartner() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: PartnerCreate) => post<Partner>(`${BASE}`, data, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["partners"] }),
  });
}

export function useUpdatePartner() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: PartnerUpdate }) =>
      patch<Partner>(`${BASE}/${id}`, data, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["partners"] }),
  });
}

export function useTogglePartnerActive() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => patch<Partner>(`${BASE}/${id}/toggle-active`, {}, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["partners"] }),
  });
}

export function useDeletePartner() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => del(`${BASE}/${id}`, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["partners"] }),
  });
}
