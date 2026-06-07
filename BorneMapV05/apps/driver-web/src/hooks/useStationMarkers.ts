import { useQuery } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { StationListItem, StationListParams } from '@/lib/types'

export function useStationMarkers(params: StationListParams) {
  const searchParams = new URLSearchParams({
    lat: String(params.lat),
    lng: String(params.lng),
    radius_km: String(params.radiusKm),
  })
  if (params.connectorType) searchParams.set('connector_type', params.connectorType)
  if (params.availability) searchParams.set('availability', params.availability)

  return useQuery({
    queryKey: ['stations', 'list', params],
    queryFn: () =>
      apiClient.get<{ success: boolean; data: StationListItem[] }>(
        `/stations?${searchParams.toString()}`,
      ),
    select: (res) => res.data,
    staleTime: 30_000,
  })
}
