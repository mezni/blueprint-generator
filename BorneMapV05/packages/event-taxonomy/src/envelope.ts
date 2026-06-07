export type Channel = 'driver_web' | 'driver_mobile' | 'partner_dashboard' | 'admin_dashboard';

export type ActorRole = 'registered_driver' | 'partner' | 'admin' | 'anonymous';

export interface EventEnvelope {
  event_id: string;
  event_version: number;
  schema_namespace: 'clickstream';
  event_name: string;
  occurred_at: string;
  ingested_at: string;
  channel: Channel;
  session_id: string;
  correlation_id?: string;
  anonymous_id?: string;
  user_id?: string;
  actor_role?: ActorRole;
  path?: string;
  payload?: Record<string, unknown>;
  metadata?: Record<string, unknown>;
}
