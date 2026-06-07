import { NavLink } from 'react-router'
import { useAuth } from '@/hooks/useAuth'

const navItems = [
  { to: '/stations', label: 'Stations' },
  { to: '/chargers', label: 'Chargers' },
  { to: '/profile', label: 'Profile' },
]

export function Header() {
  const { isAuthenticated, user, logout } = useAuth()

  return (
    <header className="flex h-14 items-center justify-between border-b border-[var(--color-border-muted)] bg-[var(--color-surface-base)] px-6">
      <div className="flex items-center gap-8">
        <span className="text-lg font-bold text-[var(--color-primary-base)]">
          BorneMap Partner
        </span>
        {isAuthenticated && (
          <nav className="flex items-center gap-1">
            {navItems.map((item) => (
              <NavLink
                key={item.to}
                to={item.to}
                className={({ isActive }) =>
                  `rounded-md px-3 py-1.5 text-sm font-medium transition-colors ${
                    isActive
                      ? 'bg-[var(--color-primary-muted)] text-[var(--color-primary-base)]'
                      : 'text-[var(--color-text-muted)] hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-text-base)]'
                  }`
                }
              >
                {item.label}
              </NavLink>
            ))}
          </nav>
        )}
      </div>
      {isAuthenticated && (
        <div className="flex items-center gap-4">
          <span className="text-sm text-[var(--color-text-muted)]">
            {user?.email ?? 'Partner'}
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
