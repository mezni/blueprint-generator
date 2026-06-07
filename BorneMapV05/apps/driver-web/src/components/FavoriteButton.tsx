import { useFavorites, useFavoriteToggle } from '@/hooks/useFavorites'
import { useAuth } from '@/hooks/useAuth'
import { useState } from 'react'
import AuthModal from './AuthModal'

interface FavoriteButtonProps {
  stationId: string
}

function FavoriteButton({ stationId }: FavoriteButtonProps) {
  const { data: favorites, isLoading: favsLoading } = useFavorites()
  const toggle = useFavoriteToggle()
  const { isAuthenticated } = useAuth()
  const [showAuth, setShowAuth] = useState(false)

  const isFavorited = favorites?.includes(stationId) ?? false

  const handleClick = async () => {
    if (!isAuthenticated) {
      setShowAuth(true)
      return
    }

    toggle.mutate({ stationId, isFavorited })
  }

  return (
    <>
      <button
        onClick={handleClick}
        disabled={favsLoading || toggle.isPending}
        className="rounded p-1 transition-colors hover:bg-[var(--color-surface-hover)] disabled:opacity-50"
      >
        <svg
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill={isFavorited ? 'var(--color-error-base)' : 'none'}
          stroke="var(--color-error-base)"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
        </svg>
      </button>
      <AuthModal
        isOpen={showAuth}
        onClose={() => setShowAuth(false)}
        onSuccess={() => {
          toggle.mutate({ stationId, isFavorited })
        }}
      />
    </>
  )
}

export default FavoriteButton
