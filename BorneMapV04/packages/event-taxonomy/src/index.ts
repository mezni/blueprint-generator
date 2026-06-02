export interface EventEnvelope {
  eventId: string
  eventVersion: number
  eventName: string
  occurredAt: string
  channel: string
  sessionId: string
  payload: Record<string, unknown>
}

export type EventName =
  | 'page.viewed'
  | 'map.loaded'
  | 'search.performed'
  | 'station.opened'
  | 'station.favorited'
  | 'review.submitted'
  | 'auth.succeeded'
  | 'auth.failed'
