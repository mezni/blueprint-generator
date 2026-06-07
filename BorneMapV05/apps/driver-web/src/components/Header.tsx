import { useAuth } from '@/hooks/useAuth'

interface HeaderProps {
  onSearchToggle: () => void
}

function Header({ onSearchToggle }: HeaderProps) {
  const { isAuthenticated, user, login, logout } = useAuth()

  return (
    <header className="flex h-14 shrink-0 items-center gap-2 border-b border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-4">
      <span className="text-lg font-bold text-[var(--color-text-base)]">BorneMap</span>
      <div className="flex-1" />
      <button
        onClick={onSearchToggle}
        className="rounded-md px-3 py-1.5 text-sm text-[var(--color-text-muted)] transition-colors hover:bg-[var(--color-surface-hover)]"
      >
        Search
      </button>
      {isAuthenticated ? (
        <div className="flex items-center gap-2">
          <span className="text-sm text-[var(--color-text-muted)]">{user?.name ?? 'User'}</span>
          <button
            onClick={logout}
            className="rounded-md px-3 py-1.5 text-sm text-[var(--color-text-muted)] transition-colors hover:bg-[var(--color-surface-hover)]"
          >
            Logout
          </button>
        </div>
      ) : (
        <button
          onClick={() => login()}
          className="rounded-md bg-[var(--color-primary-base)] px-3 py-1.5 text-sm text-white transition-colors hover:bg-[var(--color-primary-hover)]"
        >
          Login
        </button>
      )}
    </header>
  )
}

export default Header
