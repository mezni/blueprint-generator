import { createContext, useContext, useState, useEffect, useCallback, type ReactNode } from 'react'
import * as auth from '@bornemap/auth-client'
import { emitEvent } from '@/lib/clickstream'

interface AuthState {
  isInitialized: boolean
  isAuthenticated: boolean
  user: { id?: string; email?: string; name?: string } | null
}

interface AuthContextValue extends AuthState {
  login: (provider?: string) => Promise<void>
  logout: () => Promise<void>
  getToken: () => Promise<string | null>
}

const AuthContext = createContext<AuthContextValue | null>(null)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<AuthState>({
    isInitialized: false,
    isAuthenticated: false,
    user: null,
  })

  useEffect(() => {
    let cancelled = false
    auth.initAuth()
      .then((authenticated) => {
        if (cancelled) return
        setState({
          isInitialized: true,
          isAuthenticated: authenticated,
          user: authenticated ? auth.getUser() : null,
        })
      })
      .catch(() => {
        if (cancelled) return
        setState({ isInitialized: true, isAuthenticated: false, user: null })
      })
    return () => { cancelled = true }
  }, [])

  const login = useCallback(async (provider?: string) => {
    emitEvent('auth.started')
    try {
      await auth.login(provider)
      setState((prev) => ({
        ...prev,
        isAuthenticated: auth.isAuthenticated(),
        user: auth.getUser(),
      }))
      emitEvent('auth.succeeded')
    } catch {
      emitEvent('auth.failed')
    }
  }, [])

  const logout = useCallback(async () => {
    await auth.logout()
    setState({ isInitialized: true, isAuthenticated: false, user: null })
  }, [])

  const getToken = useCallback(async () => auth.getToken(), [])

  return (
    <AuthContext.Provider value={{ ...state, login, logout, getToken }}>
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext)
  if (!ctx) throw new Error('useAuth must be used within <AuthProvider>')
  return ctx
}
