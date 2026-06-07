import { useAdminUsers, useUpdateUser } from '@/hooks/useAdminUsers'
import { Button } from '@/components/ui/button'
import type { User } from '@/lib/types'

export default function UsersPage() {
  const { data, isLoading, isError, refetch } = useAdminUsers()
  const updateUser = useUpdateUser()

  const handleToggleStatus = (user: User) => {
    const newRole = user.role === 'admin' ? 'registered_driver' : 'admin'
    updateUser.mutate({ id: user.id, data: { role: newRole } })
  }

  if (isLoading) {
    return (
      <div className="space-y-3">
        {[...Array(5)].map((_, i) => (
          <div key={i} className="h-12 animate-pulse rounded bg-[var(--color-surface-base)] border border-[var(--color-border-muted)]" />
        ))}
      </div>
    )
  }

  if (isError) {
    return (
      <div className="flex flex-col items-center gap-3 py-20">
        <p className="text-[var(--color-text-muted)]">Failed to load users</p>
        <Button onClick={() => refetch()}>Retry</Button>
      </div>
    )
  }

  const users = data?.data ?? []

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-xl font-bold text-[var(--color-text-base)]">Users</h1>
      </div>

      {users.length === 0 ? (
        <div className="flex flex-col items-center gap-3 py-20">
          <p className="text-[var(--color-text-muted)]">No users found</p>
        </div>
      ) : (
        <div className="overflow-x-auto rounded-lg border border-[var(--color-border-muted)]">
          <table className="w-full text-sm">
            <thead>
              <tr className="bg-[var(--color-surface-hover)] text-left text-[var(--color-text-muted)]">
                <th className="px-4 py-3 font-medium">Email</th>
                <th className="px-4 py-3 font-medium">Role</th>
                <th className="px-4 py-3 font-medium">Created</th>
                <th className="px-4 py-3 font-medium">Updated</th>
                <th className="px-4 py-3 font-medium">Actions</th>
              </tr>
            </thead>
            <tbody>
              {users.map((u: User) => (
                <tr key={u.id} className="border-t border-[var(--color-border-muted)] hover:bg-[var(--color-surface-hover)]">
                  <td className="px-4 py-3 text-[var(--color-text-base)]">{u.email || '-'}</td>
                  <td className="px-4 py-3">
                    <span className="text-[var(--color-text-base)]">{u.role || '-'}</span>
                  </td>
                  <td className="px-4 py-3 text-[var(--color-text-muted)]">{new Date(u.created_at).toLocaleDateString()}</td>
                  <td className="px-4 py-3 text-[var(--color-text-muted)]">{new Date(u.updated_at).toLocaleDateString()}</td>
                  <td className="px-4 py-3">
                    <button
                      onClick={() => handleToggleStatus(u)}
                      disabled={updateUser.isPending}
                      className="text-sm text-[var(--color-primary-base)] hover:underline disabled:opacity-50"
                    >
                      Update Status
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}
