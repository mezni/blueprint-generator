import { useState } from 'react'
import { useAuth } from '@/hooks/useAuth'
import { Modal } from './ui/modal'
import { Button } from './ui/button'

interface AuthModalProps {
  isOpen: boolean
  onClose: () => void
  onSuccess: () => void
}

function AuthModal({ isOpen: open, onClose, onSuccess }: AuthModalProps) {
  const { login } = useAuth()
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  const handleLogin = async () => {
    setLoading(true)
    setError(null)
    try {
      await login()
      onSuccess()
      onClose()
    } catch {
      setError('Authentication failed. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  return (
    <Modal open={open} onClose={onClose}>
      <div className="flex flex-col gap-4 p-6">
        <h2 className="text-xl font-semibold text-[var(--color-text-base)]">
          Sign in to continue
        </h2>
        <p className="text-sm text-[var(--color-text-muted)]">
          You need to be signed in to perform this action.
        </p>
        {error && (
          <p className="rounded-md bg-[var(--color-error-bg)] px-3 py-2 text-sm text-[var(--color-error-base)]">
            {error}
          </p>
        )}
        <div className="flex gap-3">
          <Button variant="outline" onClick={onClose} disabled={loading}>
            Cancel
          </Button>
          <Button onClick={handleLogin} disabled={loading}>
            {loading ? 'Signing in...' : 'Sign in'}
          </Button>
        </div>
      </div>
    </Modal>
  )
}

export default AuthModal
