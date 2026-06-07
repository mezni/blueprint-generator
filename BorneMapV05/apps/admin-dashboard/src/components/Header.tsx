import { useAuth } from '@/hooks/useAuth'

interface HeaderProps {
  onToggleSidebar: () => void
}

export function Header({ onToggleSidebar }: HeaderProps) {
  const { isAuthenticated, user, logout } = useAuth()

  return (
    <header className="flex h-14 items-center justify-between border-b border-[var(--color-border-muted)] bg-[var(--color-surface-base)] px-4">
      <div className="flex items-center gap-3">
        <button
          onClick={onToggleSidebar}
          className="rounded-md p-1.5 text-[var(--color-text-muted)] hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-text-base)] transition-colors lg:hidden"
        >
          <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
            <path d="M3 5h14M3 10h14M3 15h14" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
          </svg>
        </button>
        <span className="text-lg font-bold text-[var(--color-primary-base)]">
          BorneMap Admin
        </span>
      </div>
      {isAuthenticated && (
        <div className="flex items-center gap-4">
          <span className="text-sm text-[var(--color-text-muted)]">
            {user?.email ?? 'Admin'}
          </span>
          <button
            onClick={logout}
            className="rounded-md px-3 py-1.5 text-sm font-medium text-[var(--color-text-muted)] hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-error-base)] transition-colors"
          >
            Logout
          </button>
        </div>
      )}
    </header>
  )
}
