import { useCallback } from 'react'
import type { EventName } from '@bornemap/event-taxonomy'
import { emitEvent } from '@/lib/clickstream'

export function useClickstream() {
  const emit = useCallback((eventName: EventName, payload?: Record<string, unknown>) => {
    emitEvent(eventName, payload)
  }, [])

  return { emit }
}
