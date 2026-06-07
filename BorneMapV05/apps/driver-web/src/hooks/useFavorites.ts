import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import { emitEvent } from '@/lib/clickstream'

export function useFavorites() {
  return useQuery({
    queryKey: ['favorites'],
    queryFn: () =>
      apiClient.get<{ success: boolean; data: string[] }>('/favorites'),
    select: (res) => res.data,
    staleTime: 60_000,
  })
}

export function useFavoriteToggle() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      stationId,
      isFavorited,
    }: {
      stationId: string
      isFavorited: boolean
    }) => {
      if (isFavorited) {
        await apiClient.delete(`/favorites/${stationId}`)
      } else {
        await apiClient.post(`/favorites/${stationId}`)
      }
    },
    onMutate: async ({ stationId }) => {
      await queryClient.cancelQueries({ queryKey: ['favorites'] })
      const previous = queryClient.getQueryData<string[]>(['favorites'])

      queryClient.setQueryData<string[]>(['favorites'], (old) => {
        if (!old) return [stationId]
        if (old.includes(stationId)) return old.filter((id) => id !== stationId)
        return [...old, stationId]
      })

      return { previous }
    },
    onError: (_err, _vars, context) => {
      if (context?.previous) {
        queryClient.setQueryData(['favorites'], context.previous)
      }
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ['favorites'] })
    },
    onSuccess: (_data, { stationId, isFavorited }) => {
      emitEvent(
        isFavorited ? 'favorite_station.removed' : 'favorite_station.added',
        { stationId },
      )
    },
  })
}
