import { useState } from 'react'
import { useAdminReviews, useModerateReview } from '@/hooks/useAdminReviews'
import { Modal } from '@/components/Modal'
import { ReviewModeration } from '@/components/ReviewModeration'
import { Button } from '@/components/ui/button'
import type { Review, ReviewStatus } from '@/lib/types'

export default function ReviewsPage() {
  const { data, isLoading, isError, refetch } = useAdminReviews()
  const moderateReview = useModerateReview()

  const [moderateId, setModerateId] = useState<string | null>(null)
  const [currentStatus, setCurrentStatus] = useState<ReviewStatus>('submitted')

  const selectedReview = data?.data?.find((r: Review) => r.id === moderateId)

  const handleModerate = async (status: ReviewStatus) => {
    if (!moderateId) return
    await moderateReview.mutateAsync({ id: moderateId, status })
    setModerateId(null)
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
        <p className="text-[var(--color-text-muted)]">Failed to load reviews</p>
        <Button onClick={() => refetch()}>Retry</Button>
      </div>
    )
  }

  const reviews = data?.data ?? []

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-xl font-bold text-[var(--color-text-base)]">Reviews</h1>
      </div>

      {reviews.length === 0 ? (
        <div className="flex flex-col items-center gap-3 py-20">
          <p className="text-[var(--color-text-muted)]">No reviews yet</p>
        </div>
      ) : (
        <div className="overflow-x-auto rounded-lg border border-[var(--color-border-muted)]">
          <table className="w-full text-sm">
            <thead>
              <tr className="bg-[var(--color-surface-hover)] text-left text-[var(--color-text-muted)]">
                <th className="px-4 py-3 font-medium">Station</th>
                <th className="px-4 py-3 font-medium">User</th>
                <th className="px-4 py-3 font-medium">Rating</th>
                <th className="px-4 py-3 font-medium">Comment</th>
                <th className="px-4 py-3 font-medium">Status</th>
                <th className="px-4 py-3 font-medium">Actions</th>
              </tr>
            </thead>
            <tbody>
              {reviews.map((r: Review) => (
                <tr key={r.id} className="border-t border-[var(--color-border-muted)] hover:bg-[var(--color-surface-hover)]">
                  <td className="px-4 py-3 text-[var(--color-text-base)]">{r.station_name}</td>
                  <td className="px-4 py-3 text-[var(--color-text-muted)]">{r.user_email}</td>
                  <td className="px-4 py-3">
                    <span className="text-[var(--color-accent-base)]">{'★'.repeat(r.rating)}{'☆'.repeat(5 - r.rating)}</span>
                  </td>
                  <td className="px-4 py-3 max-w-xs truncate text-[var(--color-text-muted)]">{r.comment}</td>
                  <td className="px-4 py-3">
                    <span className={`inline-block rounded-full px-2 py-0.5 text-xs font-medium capitalize ${
                      r.status === 'published' ? 'bg-[var(--color-success-muted)] text-[var(--color-success-base)]'
                        : r.status === 'flagged' ? 'bg-[var(--color-warning-muted)] text-[var(--color-warning-base)]'
                        : r.status === 'hidden' ? 'bg-[var(--color-surface-muted)] text-[var(--color-text-muted)]'
                        : 'bg-[var(--color-primary-muted)] text-[var(--color-primary-base)]'
                    }`}>
                      {r.status}
                    </span>
                  </td>
                  <td className="px-4 py-3">
                    <button
                      onClick={() => { setModerateId(r.id); setCurrentStatus(r.status) }}
                      className="text-sm text-[var(--color-primary-base)] hover:underline"
                    >
                      Moderate
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {moderateId && selectedReview && (
        <Modal open onClose={() => setModerateId(null)} title="Moderate Review">
          <ReviewModeration
            currentStatus={currentStatus}
            onModerate={handleModerate}
            onCancel={() => setModerateId(null)}
          />
        </Modal>
      )}
    </div>
  )
}
