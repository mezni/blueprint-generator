import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import { emitEvent } from '@/lib/clickstream'
import type {
  StationCreate,
  StationUpdate,
  StationListResponse,
  StationItemResponse,
} from '@/lib/types'

export function usePartnerStations(page = 1, size = 20) {
  return useQuery({
    queryKey: ['partner', 'stations', { page, size }],
    queryFn: () =>
      apiClient.get<StationListResponse>(
        `/stations?page=${page}&size=${size}`,
      ),
    select: (res) => ({ data: res.data, meta: res.meta }),
    staleTime: 30_000,
  })
}

export function useStationDetail(stationId: string | null) {
  return useQuery({
    queryKey: ['partner', 'stations', stationId],
    queryFn: () =>
      apiClient.get<StationItemResponse>(`/stations/${stationId}`),
    select: (res) => res.data,
    enabled: !!stationId,
    staleTime: 30_000,
  })
}

export function useCreateStation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (data: StationCreate) =>
      apiClient.post<StationItemResponse>('/stations', data, {
        headers: { 'Idempotency-Key': crypto.randomUUID() },
      }),
    onSuccess: () => {
      emitEvent('partner_station.created')
      queryClient.invalidateQueries({ queryKey: ['partner', 'stations'] })
    },
  })
}

export function useUpdateStation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({
      id,
      data,
      etag,
    }: {
      id: string
      data: StationUpdate
      etag: string
    }) =>
      apiClient.patch<StationItemResponse>(`/stations/${id}`, data, {
        headers: { 'If-Match': etag },
      }),
    onSuccess: () => {
      emitEvent('partner_station.updated')
      queryClient.invalidateQueries({ queryKey: ['partner', 'stations'] })
    },
  })
}

export function useDeleteStation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) =>
      apiClient.delete<StationItemResponse>(`/stations/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['partner', 'stations'] })
    },
  })
}
