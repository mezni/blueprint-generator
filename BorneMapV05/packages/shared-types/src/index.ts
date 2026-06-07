export type Role = 'registered_driver' | 'partner' | 'admin';

export type StationStatus = 'active' | 'inactive' | 'maintenance' | 'draft';
export type StationAvailabilityStatus = 'available' | 'limited' | 'unavailable';
export type PartnerStatus = 'active' | 'suspended';
export type ChargerStatus = 'available' | 'offline' | 'fault';
export type ChargerType = 'CCS' | 'Type2' | 'CHAdeMO';
export type ReviewStatus = 'published' | 'hidden' | 'flagged' | 'deleted';
export type PartnerRole = 'owner' | 'manager' | 'operator' | 'viewer';
export type GisQueueStatus = 'pending' | 'processing' | 'done' | 'failed' | 'dead_letter';
export type AvailabilitySource = 'manual_partner' | 'system_sync' | 'admin';

export type EntityPrefix = 'USR' | 'PRT' | 'STN' | 'CHG' | 'REV' | 'EVT' | 'CLK' | 'SESS' | 'ANON';

export function formatId(prefix: EntityPrefix, ulid: string): string {
  return `${prefix}-${ulid}`;
}
