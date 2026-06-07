import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { ReviewListResponse, ReviewItemResponse, ReviewStatus } from '@/lib/types'

export function useAdminReviews() {
  return useQuery<ReviewListResponse>({
    queryKey: ['admin', 'reviews'],
    queryFn: () => apiClient.get('/reviews'),
  })
}

export function useModerateReview() {
  const qc = useQueryClient()
  return useMutation<ReviewItemResponse, Error, { id: string; status: ReviewStatus }>({
    mutationFn: ({ id, status }) => apiClient.patch(`/reviews/${id}/status`, { body: { status } }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['admin', 'reviews'] }),
  })
}
