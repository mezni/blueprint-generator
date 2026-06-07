import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { PartnerListResponse, PartnerItemResponse, PartnerCreate, PartnerUpdate } from '@/lib/types'

export function useAdminPartners() {
  return useQuery<PartnerListResponse>({
    queryKey: ['admin', 'partners'],
    queryFn: () => apiClient.get('/partners'),
  })
}

export function useCreatePartner() {
  const qc = useQueryClient()
  return useMutation<PartnerItemResponse, Error, PartnerCreate>({
    mutationFn: (body) => apiClient.post('/partners', { body, headers: { 'Idempotency-Key': crypto.randomUUID() } }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['admin', 'partners'] }),
  })
}

export function useUpdatePartner() {
  const qc = useQueryClient()
  return useMutation<PartnerItemResponse, Error, { id: string; data: PartnerUpdate }>({
    mutationFn: ({ id, data }) => apiClient.patch(`/partners/${id}`, { body: data }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['admin', 'partners'] }),
  })
}

export function useDeletePartner() {
  const qc = useQueryClient()
  return useMutation<void, Error, string>({
    mutationFn: (id) => apiClient.delete(`/partners/${id}`),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['admin', 'partners'] }),
  })
}
