import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import type { UserListResponse, User } from '@/lib/types'

export function useAdminUsers() {
  return useQuery<UserListResponse>({
    queryKey: ['admin', 'users'],
    queryFn: () => apiClient.get('/users'),
  })
}

export function useUpdateUser() {
  const qc = useQueryClient()
  return useMutation<User, Error, { id: string; data: Partial<Pick<User, 'role'>> }>({
    mutationFn: ({ id, data }) => apiClient.patch(`/users/${id}`, { body: data }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['admin', 'users'] }),
  })
}
