import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type {
  ChargerCreate,
  ChargerUpdate,
  ChargerListResponse,
  ChargerItemResponse,
} from '@/lib/types'

export function usePartnerChargers(stationId?: string, page = 1, size = 20) {
  const params = new URLSearchParams({ page: String(page), size: String(size) })
  if (stationId) params.set('station_id', stationId)

  return useQuery({
    queryKey: ['partner', 'chargers', { stationId, page, size }],
    queryFn: () =>
      apiClient.get<ChargerListResponse>(`/chargers?${params.toString()}`),
    select: (res) => ({ data: res.data, meta: res.meta }),
    staleTime: 30_000,
  })
}

export function useCreateCharger() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (data: ChargerCreate) =>
      apiClient.post<ChargerItemResponse>('/chargers', data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['partner', 'chargers'] })
    },
  })
}

export function useUpdateCharger() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({
      id,
      data,
      etag,
    }: {
      id: string
      data: ChargerUpdate
      etag: string
    }) =>
      apiClient.patch<ChargerItemResponse>(`/chargers/${id}`, data, {
        headers: { 'If-Match': etag },
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['partner', 'chargers'] })
    },
  })
}
