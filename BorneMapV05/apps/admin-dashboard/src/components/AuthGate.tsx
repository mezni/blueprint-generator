import { type ReactNode, useState } from 'react'
import { useAuth } from '@/hooks/useAuth'
import { Button } from '@/components/ui/button'

interface AuthGateProps {
  children: ReactNode
}

export function AuthGate({ children }: AuthGateProps) {
  const { isInitialized, isAuthenticated, login } = useAuth()
  const [loggingIn, setLoggingIn] = useState(false)

  if (!isInitialized) {
    return (
      <div className="flex h-svh items-center justify-center">
        <div className="text-[var(--color-text-muted)]">Initializing...</div>
      </div>
    )
  }

  if (!isAuthenticated) {
    return (
      <div className="flex h-svh items-center justify-center">
        <div className="text-center max-w-sm">
          <h1 className="text-2xl font-bold text-[var(--color-text-base)] mb-2">
            Admin Dashboard
          </h1>
          <p className="text-[var(--color-text-muted)] mb-6">
            Sign in with your admin account to manage the platform.
          </p>
          <Button
            onClick={async () => {
              setLoggingIn(true)
              await login()
              setLoggingIn(false)
            }}
            disabled={loggingIn}
          >
            {loggingIn ? 'Redirecting...' : 'Sign in with Keycloak'}
          </Button>
        </div>
      </div>
    )
  }

  return <>{children}</>
}
