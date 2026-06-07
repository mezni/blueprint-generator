import { useState } from 'react'
import type { ReviewStatus } from '@/lib/types'
import { Button } from '@/components/ui/button'

interface ReviewModerationProps {
  currentStatus: ReviewStatus
  onModerate: (status: ReviewStatus) => Promise<void>
  onCancel: () => void
}

function allowedTransitions(current: ReviewStatus): ReviewStatus[] {
  switch (current) {
    case 'submitted': return ['published', 'flagged']
    case 'published': return ['flagged']
    case 'flagged': return ['hidden', 'published', 'submitted']
    case 'hidden': return ['flagged']
    default: return []
  }
}

export function ReviewModeration({ currentStatus, onModerate, onCancel }: ReviewModerationProps) {
  const [moderating, setModerating] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const transitions = allowedTransitions(currentStatus)

  const handleModerate = async (status: ReviewStatus) => {
    setModerating(true)
    setError(null)
    try {
      await onModerate(status)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Moderation failed')
    } finally {
      setModerating(false)
    }
  }

  if (transitions.length === 0) {
    return <p className="text-sm text-[var(--color-text-muted)]">No further transitions available.</p>
  }

  return (
    <div className="flex flex-col gap-3">
      <p className="text-sm font-medium text-[var(--color-text-base)]">
        Current status: <span className="capitalize">{currentStatus}</span>
      </p>
      <p className="text-xs text-[var(--color-text-muted)]">Change status to:</p>
      <div className="flex flex-wrap gap-2">
        {transitions.map((s) => (
          <Button
            key={s}
            variant="outline"
            size="sm"
            onClick={() => handleModerate(s)}
            disabled={moderating}
          >
            {s}
          </Button>
        ))}
      </div>
      {error && <p className="text-sm text-[var(--color-error-base)]">{error}</p>}
      <div className="flex justify-end">
        <Button variant="ghost" size="sm" onClick={onCancel}>Close</Button>
      </div>
    </div>
  )
}
