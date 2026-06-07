import { useQuery } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { OverviewResponse } from '@/lib/types'

export function useAdminOverview() {
  return useQuery<OverviewResponse>({
    queryKey: ['admin', 'overview'],
    queryFn: () => apiClient.get('/overview'),
  })
}
