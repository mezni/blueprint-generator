import { useState } from 'react'
import { useAuth } from '@/hooks/useAuth'
import { useReviewMutation } from '@/hooks/useReviews'
import AuthModal from './AuthModal'
import { Button } from './ui/button'

interface ReviewFormProps {
  stationId: string
}

function ReviewForm({ stationId }: ReviewFormProps) {
  const [rating, setRating] = useState(0)
  const [comment, setComment] = useState('')
  const [hovered, setHovered] = useState(0)
  const [showAuth, setShowAuth] = useState(false)
  const { isAuthenticated } = useAuth()
  const { create } = useReviewMutation(stationId)

  const handleSubmit = async () => {
    if (rating < 1) return

    if (!isAuthenticated) {
      setShowAuth(true)
      return
    }

    create.mutate(
      { station_id: stationId, rating, comment: comment || undefined },
      {
        onSuccess: () => {
          setRating(0)
          setComment('')
        },
      },
    )
  }

  return (
    <div className="flex flex-col gap-3">
      <h4 className="text-sm font-semibold text-[var(--color-text-base)]">
        Leave a review
      </h4>
      <div className="flex gap-1">
        {[1, 2, 3, 4, 5].map((star) => (
          <button
            key={star}
            type="button"
            onClick={() => setRating(star)}
            onMouseEnter={() => setHovered(star)}
            onMouseLeave={() => setHovered(0)}
            className="p-0.5 text-lg transition-colors"
          >
            {star <= (hovered || rating) ? '★' : '☆'}
          </button>
        ))}
      </div>
      <textarea
        value={comment}
        onChange={(e) => setComment(e.target.value)}
        placeholder="Share your experience..."
        className="min-h-[80px] rounded-md border border-[var(--color-border-base)] bg-transparent p-2 text-sm text-[var(--color-text-base)] placeholder:text-[var(--color-text-muted)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
      />
      {create.isError && (
        <p className="text-sm text-[var(--color-error-base)]">
          {(create.error as { message?: string })?.message ?? 'Failed to submit review'}
        </p>
      )}
      <Button
        onClick={handleSubmit}
        disabled={rating < 1 || create.isPending}
        className="self-start"
      >
        {create.isPending ? 'Submitting...' : 'Submit'}
      </Button>
      <AuthModal
        isOpen={showAuth}
        onClose={() => setShowAuth(false)}
        onSuccess={() => handleSubmit()}
      />
    </div>
  )
}

export default ReviewForm
