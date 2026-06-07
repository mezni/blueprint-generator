import { useState } from 'react'
import { useAuth } from '@/hooks/useAuth'
import { useReviews, useReviewMutation } from '@/hooks/useReviews'

interface ReviewListProps {
  stationId: string
}

function ReviewList({ stationId }: ReviewListProps) {
  const { data: reviews, isLoading } = useReviews(stationId)
  const { user } = useAuth()
  const { remove } = useReviewMutation(stationId)
  const [deletingId, setDeletingId] = useState<string | null>(null)

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-4">
        <div className="h-6 w-6 animate-spin rounded-full border-4 border-[var(--color-border-base)] border-t-[var(--color-primary-base)]" />
      </div>
    )
  }

  if (!reviews || reviews.length === 0) {
    return (
      <p className="py-4 text-center text-sm text-[var(--color-text-muted)]">
        No reviews yet
      </p>
    )
  }

  return (
    <div className="flex flex-col gap-3">
      <h4 className="text-sm font-semibold text-[var(--color-text-base)]">
        Reviews ({reviews.length})
      </h4>
      {reviews.map((review) => {
        const isOwn = user?.id === review.user_id

        return (
          <div
            key={review.id}
            className="rounded-lg border border-[var(--color-border-muted)] p-3"
          >
            <div className="flex items-center justify-between">
              <div className="flex gap-0.5 text-sm">
                {[1, 2, 3, 4, 5].map((star) => (
                  <span
                    key={star}
                    className={star <= review.rating ? 'text-yellow-500' : 'text-[var(--color-text-muted)]'}
                  >
                    ★
                  </span>
                ))}
              </div>
              {isOwn && (
                <button
                  onClick={() => {
                    setDeletingId(review.id)
                    remove.mutate(review.id, {
                      onSettled: () => setDeletingId(null),
                    })
                  }}
                  disabled={deletingId === review.id}
                  className="text-xs text-[var(--color-error-base)] hover:underline disabled:opacity-50"
                >
                  {deletingId === review.id ? 'Deleting...' : 'Delete'}
                </button>
              )}
            </div>
            {review.comment && (
              <p className="mt-1 text-sm text-[var(--color-text-base)]">
                {review.comment}
              </p>
            )}
            <p className="mt-1 text-xs text-[var(--color-text-muted)]">
              {new Date(review.created_at).toLocaleDateString()}
            </p>
          </div>
        )
      })}
    </div>
  )
}

export default ReviewList
