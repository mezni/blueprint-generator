import { useQuery } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { ProfileResponse } from '@/lib/types'

export function usePartnerProfile() {
  return useQuery({
    queryKey: ['partner', 'profile'],
    queryFn: () => apiClient.get<ProfileResponse>('/me'),
    select: (res) => res.data,
    staleTime: 60_000,
  })
}
