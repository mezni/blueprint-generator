import type { ItemEnvelope, SuccessEnvelope } from '@bornemap/api-contracts'

export type StationStatus = 'active' | 'inactive' | 'maintenance' | 'draft'
export type StationAvailabilityStatus = 'available' | 'limited' | 'unavailable'
export type ChargerStatus = 'available' | 'offline' | 'fault'
export type ChargerType = 'CCS' | 'Type2' | 'CHAdeMO'

export interface Station {
  station_id: string
  partner_id: string
  name: string
  address: string | null
  latitude: number
  longitude: number
  status: StationStatus
  availability_status: StationAvailabilityStatus
  created_at: string
  updated_at: string
}

export interface StationCreate {
  name: string
  address?: string
  latitude: number
  longitude: number
}

export interface StationUpdate {
  name?: string
  address?: string
  latitude?: number
  longitude?: number
  status?: StationStatus
  availability_status?: StationAvailabilityStatus
}

export interface Charger {
  charger_id: string
  station_id: string
  charger_type: ChargerType
  power_kw: number
  status: ChargerStatus
  created_at: string
  updated_at: string
}

export interface ChargerCreate {
  station_id: string
  charger_type: ChargerType
  power_kw: number
  status: ChargerStatus
}

export interface ChargerUpdate {
  charger_type?: ChargerType
  power_kw?: number
  status?: ChargerStatus
}

export interface Profile {
  user_id: string
  email: string | null
  partner_id: string | null
  partner_name: string | null
  membership_role: string | null
}

export interface AvailabilityUpdate {
  status: StationAvailabilityStatus
}

export interface Availability {
  station_id: string
  availability_status: StationAvailabilityStatus
  source: string
  updated_at: string
}

export type StationListResponse = SuccessEnvelope<Station[]>
export type StationItemResponse = ItemEnvelope<Station>
export type ChargerListResponse = SuccessEnvelope<Charger[]>
export type ChargerItemResponse = ItemEnvelope<Charger>
export type ProfileResponse = ItemEnvelope<Profile>
export type AvailabilityResponse = ItemEnvelope<Availability>
