import { useQuery } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { StationDetail } from '@/lib/types'

export function useStationDetail(stationId: string | null) {
  return useQuery({
    queryKey: ['stations', 'detail', stationId],
    queryFn: () =>
      apiClient.get<{ success: boolean; data: StationDetail }>(
        `/stations/${stationId}`,
      ),
    select: (res) => res.data,
    enabled: !!stationId,
    staleTime: 60_000,
  })
}
