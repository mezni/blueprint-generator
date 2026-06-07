import { useState, useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { StationListItem, SearchQuery } from '@/lib/types'

export function useSearch(query: SearchQuery, debounceMs = 300) {
  const [debouncedQuery, setDebouncedQuery] = useState(query)

  useEffect(() => {
    const timer = setTimeout(() => setDebouncedQuery(query), debounceMs)
    return () => clearTimeout(timer)
  }, [query.q, query.city, query.connector_type, query.availability, query.page, query.size, debounceMs])

  const enabled = !!debouncedQuery.q && debouncedQuery.q.length > 0

  const searchParams = new URLSearchParams()
  if (debouncedQuery.q) searchParams.set('q', debouncedQuery.q)
  if (debouncedQuery.city) searchParams.set('city', debouncedQuery.city)
  if (debouncedQuery.connector_type) searchParams.set('connector_type', debouncedQuery.connector_type)
  if (debouncedQuery.availability) searchParams.set('availability', debouncedQuery.availability)
  if (debouncedQuery.page) searchParams.set('page', String(debouncedQuery.page))
  if (debouncedQuery.size) searchParams.set('size', String(debouncedQuery.size))

  return useQuery({
    queryKey: ['stations', 'search', debouncedQuery],
    queryFn: () =>
      apiClient.get<{ success: boolean; data: StationListItem[]; meta: { total?: number } }>(
        `/stations/search?${searchParams.toString()}`,
      ),
    select: (res) => ({
      results: res.data,
      totalResults: res.meta?.total ?? res.data.length,
    }),
    enabled,
    staleTime: 30_000,
  })
}
