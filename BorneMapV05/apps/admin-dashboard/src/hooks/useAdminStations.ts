import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { StationListResponse, StationItemResponse, StationUpdate } from '@/lib/types'

export function useAdminStations(showDeleted = false) {
  return useQuery<StationListResponse>({
    queryKey: ['admin', 'stations', { showDeleted }],
    queryFn: () => apiClient.get(`/stations?show_deleted=${showDeleted}`),
  })
}

export function useUpdateStation() {
  const qc = useQueryClient()
  return useMutation<StationItemResponse, Error, { id: string; data: StationUpdate }>({
    mutationFn: ({ id, data }) => apiClient.patch(`/stations/${id}`, { body: data }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['admin', 'stations'] }),
  })
}

export function useDeleteStation() {
  const qc = useQueryClient()
  return useMutation<void, Error, string>({
    mutationFn: (id) => apiClient.delete(`/stations/${id}`),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['admin', 'stations'] }),
  })
}
