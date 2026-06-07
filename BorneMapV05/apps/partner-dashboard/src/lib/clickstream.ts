import type { EventEnvelope, EventName } from '@bornemap/event-taxonomy'

const GATEWAY_BASE_URL = (import.meta.env.VITE_GATEWAY_BASE_URL as string | undefined) ?? ''
const CLICKSTREAM_URL = `${GATEWAY_BASE_URL}/api/v1/clickstream/events`

let sessionId: string | null = null
function getSessionId(): string {
  if (!sessionId) { sessionId = crypto.randomUUID() }
  return sessionId
}

function createEnvelope(eventName: EventName, payload?: Record<string, unknown>): EventEnvelope {
  const now = new Date().toISOString()
  return {
    event_id: crypto.randomUUID(),
    event_version: 1,
    schema_namespace: 'clickstream',
    event_name: eventName,
    occurred_at: now,
    ingested_at: now,
    channel: 'partner_dashboard',
    session_id: getSessionId(),
    actor_role: 'partner',
    path: window.location.pathname,
    payload,
  }
}

export function emitEvent(eventName: EventName, payload?: Record<string, unknown>): void {
  const envelope = createEnvelope(eventName, payload)
  fetch(CLICKSTREAM_URL, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ events: [envelope] }),
  }).catch(() => {})
}
