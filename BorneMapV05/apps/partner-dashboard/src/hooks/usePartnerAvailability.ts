import { useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import { emitEvent } from '@/lib/clickstream'
import type {
  AvailabilityUpdate,
  AvailabilityResponse,
} from '@/lib/types'

export function useUpdateAvailability() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({
      stationId,
      data,
    }: {
      stationId: string
      data: AvailabilityUpdate
    }) =>
      apiClient.patch<AvailabilityResponse>(
        `/stations/${stationId}/availability`,
        data,
      ),
    onSuccess: () => {
      emitEvent('partner_availability.updated')
      queryClient.invalidateQueries({ queryKey: ['partner', 'stations'] })
    },
  })
}
