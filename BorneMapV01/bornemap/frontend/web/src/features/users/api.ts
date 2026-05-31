import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { get, post, patch, del } from "../../lib/api";
import { getToken } from "../../lib/auth";
import type { User, UserCreate, UserUpdate } from "./types";

const BASE = "/api/v1/users";

export function useUsers() {
  return useQuery<User[]>({ queryKey: ["users"], queryFn: () => get<User[]>(`${BASE}`, getToken()) });
}

export function useCreateUser() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: UserCreate) => post<User>(`${BASE}`, data, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["users"] }),
  });
}

export function useUpdateUser() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: UserUpdate }) =>
      patch<User>(`${BASE}/${id}`, data, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["users"] }),
  });
}

export function useToggleActive() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => patch<User>(`${BASE}/${id}/toggle-active`, {}, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["users"] }),
  });
}

export function useDeleteUser() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => del(`${BASE}/${id}`, getToken()),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["users"] }),
  });
}
