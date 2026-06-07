import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import { emitEvent } from '@/lib/clickstream'
import type { Review, ReviewCreate, ReviewUpdate } from '@/lib/types'

export function useReviews(stationId: string) {
  const params = new URLSearchParams({ station_id: stationId })

  return useQuery({
    queryKey: ['reviews', stationId],
    queryFn: () =>
      apiClient.get<{ success: boolean; data: Review[] }>(
        `/reviews?${params.toString()}`,
      ),
    select: (res) => res.data,
    staleTime: 30_000,
  })
}

export function useReviewMutation(stationId: string) {
  const queryClient = useQueryClient()

  const invalidate = () => {
    queryClient.invalidateQueries({ queryKey: ['reviews', stationId] })
  }

  const create = useMutation({
    mutationFn: (data: ReviewCreate) =>
      apiClient.post<{ success: boolean; data: Review }>('/reviews', data),
    onSettled: invalidate,
    onSuccess: (_data, variables) => {
      emitEvent('review.submitted', { stationId: variables.station_id })
    },
  })

  const update = useMutation({
    mutationFn: ({ id, data }: { id: string; data: ReviewUpdate }) =>
      apiClient.patch<{ success: boolean; data: Review }>(
        `/reviews/${id}`,
        data,
      ),
    onSettled: invalidate,
    onSuccess: (_data, variables) => {
      emitEvent('review.updated', { reviewId: variables.id, stationId })
    },
  })

  const remove = useMutation({
    mutationFn: (id: string) =>
      apiClient.delete<{ success: boolean }>(`/reviews/${id}`),
    onSettled: invalidate,
  })

  return { create, update, remove }
}
